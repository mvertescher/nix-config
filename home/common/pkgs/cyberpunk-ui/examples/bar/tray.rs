//! The bar's tray: StatusNotifierItem, over the session bus.
//!
//! The protocol has three roles. An *item* is an application that wants
//! a tray icon. A *host* is a bar that draws them. Between them sits a
//! *watcher*, `org.kde.StatusNotifierWatcher`, a bus name that keeps the
//! list of registered items so that items and hosts never have to find
//! each other directly.
//!
//! Exactly one process owns the watcher name, so this module does both
//! jobs and lets the bus decide which one is live:
//!
//!   * It always exports the watcher interface and *asks* for the name
//!     without `ReplaceExisting`. On a desktop already running waybar,
//!     waybar keeps it and we are only a host; on a desktop running
//!     nothing else, we get it and become the watcher too.
//!   * It always reads the item list back off the watcher *by name*,
//!     never out of its own registry. One code path covers both cases,
//!     and the case where the name changes hands mid-session costs a
//!     poll rather than a restart.
//!
//! `AllowReplacement` is set but `ReplaceExisting` is not: we never take
//! the name from a bar that is already using it, but a bar started later
//! on purpose can take it from us -- which, because of the above, we
//! survive by quietly demoting ourselves to a host.
//!
//! Like the audio and network modules this owns a thread and publishes
//! snapshots. It has to: a tray item is another process, and asking one
//! for its title is a round trip through a program that may be busy or
//! wedged. An empty snapshot means no tray cells at all, which is also
//! what a machine with no session bus gets -- a missing subsystem is a
//! missing module, never a stalled bar.
//!
//! ## What wakes it up
//!
//! Unlike the other sensors this one is not on a timer. Everything the
//! tray draws is another process's *state*, and the protocol has a
//! signal for every way that state can move; a poll would either be
//! slow enough to show a stale icon or fast enough to `GetAll` every
//! tray application once a second forever. So the thread selects over:
//!
//!   * the watcher's `StatusNotifierItemRegistered` / `Unregistered`,
//!   * `NameOwnerChanged`, filtered down in the loop to the watcher
//!     name and to names we are currently holding an item for -- the
//!     bus is chatty and a full re-read per activation would not be,
//!   * every signal on either item interface, caught with one match
//!     rule per interface rather than a subscription per item, so an
//!     item appearing costs no bookkeeping,
//!   * clicks arriving from the drawing thread,
//!   * and a slow [`SAFETY_NET`] timer, which is what makes a missed
//!     signal cost thirty seconds rather than the session.
//!
//! The item rule is deliberately *not* narrowed to senders we already
//! hold an item for. It could be, and the cost of not doing it is one
//! wasted re-read when a process that never registered emits `NewIcon`
//! -- but a key is whatever name the watcher published, which for an
//! item that registered under a well-known name is not the unique name
//! its signals arrive from, so the filter would sometimes drop a real
//! one. A spurious re-read is cheap; a dropped `NewIcon` is a wrong
//! icon until the safety net.
//!
//! Icons are resolved in `icon.rs`, on this thread, and arrive at the
//! bar already decoded. See that module for why all five of the
//! protocol's ways of describing an icon collapse into one.
//!
//! ## Still missing
//!
//! `com.canonical.dbusmenu`. Right-clicking calls the item's own
//! `ContextMenu`, which is what the spec says to do and what an item
//! with `ItemIsMenu = false` expects -- but an item that instead
//! exports a `Menu` object wants the host to *draw* that menu, and
//! drawing it means a second layer surface and a menu widget this
//! crate does not have yet.

use crate::icon;
use crate::sensor::{Latest, Snapshot};
use cyberpunk_ui::bar::{TrayAction, TrayItem};
use futures_lite::stream::{or, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use zbus::fdo::{DBusProxy, PropertiesProxy, RequestNameFlags};
use zbus::names::{BusName, InterfaceName};
use zbus::object_server::SignalEmitter;
use zbus::proxy::CacheProperties;
use zbus::zvariant::OwnedValue;
use zbus::{interface, proxy, Connection, MatchRule, MessageStream};

/// The well-known name the watcher lives at, and the object path the
/// spec fixes for it.
const WATCHER_NAME: &str = "org.kde.StatusNotifierWatcher";
const WATCHER_PATH: &str = "/StatusNotifierWatcher";

/// The path an item is assumed to sit at when it registers by bus name
/// alone. The spec's own default.
const DEFAULT_ITEM_PATH: &str = "/StatusNotifierItem";

/// Items answer on one of two interface names depending on how old
/// their toolkit is. `org.kde.` came first and is still what
/// libappindicator and Qt use; the freedesktop spelling turns up in
/// newer implementations. Tried in this order per item.
const ITEM_INTERFACES: [&str; 2] = [
    "org.kde.StatusNotifierItem",
    "org.freedesktop.StatusNotifierItem",
];

/// Signals that mean an item's *pixels* moved, as opposed to its title
/// or its status. These are the ones that have to invalidate the icon
/// memo, because an item may repaint behind an unchanged `IconName`.
const ICON_SIGNALS: [&str; 3] = ["NewIcon", "NewAttentionIcon", "NewOverlayIcon"];

/// How often the whole list is re-read regardless of signals.
///
/// Not the update path -- the signals are -- but the thing that bounds
/// how wrong the tray can get if one is missed, if a watcher hands the
/// name over without saying so, or if an application dies in a way that
/// produces no `NameOwnerChanged` we were listening for. Slow enough to
/// cost nothing, fast enough that nobody restarts the bar over it.
const SAFETY_NET: Duration = Duration::from_secs(30);

/// How long to wait before reconnecting after the bus goes away. Long,
/// because the usual reason is that there is no session bus at all and
/// there never will be one this session.
const RETRY: Duration = Duration::from_secs(10);

/// Characters of the item's own name used as its label when no icon can
/// be resolved. Four keeps a tray cell about as wide as `CPU 12%`, so a
/// tray appearing does not shove the clock off a narrow screen.
const LABEL_CHARS: usize = 4;

/// How the bar wants its tray drawn.
#[derive(Clone)]
pub struct Config {
    /// Draw items whose `Status` is `Passive`.
    ///
    /// The spec says a host *should* hide them, and hiding them is the
    /// default, but "should" is not "must" and an application that
    /// parks itself as passive and never comes back is invisible with
    /// no way to see that it is there.
    pub show_passive: bool,
    /// The icon theme to search, ahead of its inherit chain and
    /// `hicolor`. `None` reads the desktop's GTK setting.
    pub icon_theme: Option<String>,
    /// The size icons are drawn at, from `cyberpunk_ui::bar::icon_size`,
    /// so that what is decoded matches what is drawn.
    pub icon_size: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            show_passive: false,
            icon_theme: None,
            icon_size: 16,
        }
    }
}

/// One drawable item and the key it can be talked to on.
///
/// The key travels with the item so that a click on the *n*th cell
/// reaches the application that drew it rather than the one that
/// happens to be *n*th when the click arrives.
#[derive(Clone)]
struct Entry {
    key: String,
    item: TrayItem,
}

/// A click on its way from the drawing thread to the bus.
struct Command {
    key: String,
    action: TrayAction,
}

/// The bar's handle on the tray.
pub struct Monitor {
    latest: Latest<Vec<Entry>>,
    /// Keys of the entries last handed to the bar, so an index into
    /// what was drawn resolves to the item that was drawn there.
    keys: Vec<String>,
    commands: async_channel::Sender<Command>,
}

impl Monitor {
    /// Start watching. Returns immediately with an empty tray, which
    /// draws nothing -- the same as a desktop whose applications have
    /// no tray icons, which is the honest reading until the first read
    /// lands.
    pub fn spawn(config: Config) -> Self {
        let shared = Snapshot::new(Vec::new());
        let writer = shared.clone();
        // Bounded: a click that cannot be delivered within a hundred
        // queued clicks is a click nobody is waiting for any more.
        let (sender, receiver) = async_channel::bounded(100);

        let _ = thread::Builder::new()
            .name("cyberpunk-bar-tray".to_string())
            .spawn(move || run(&config, &writer, &receiver));

        Monitor {
            latest: Latest::new(shared, Vec::new()),
            keys: Vec::new(),
            commands: sender,
        }
    }

    /// The tray as of the last completed read, without blocking.
    pub fn reading(&mut self) -> Vec<TrayItem> {
        let entries = self.latest.get();
        self.keys = entries.iter().map(|entry| entry.key.clone()).collect();
        entries.into_iter().map(|entry| entry.item).collect()
    }

    /// Send a pointer event to the item drawn at `index`.
    ///
    /// Never blocks and never fails loudly: the bus call happens on the
    /// tray thread, and an application that will not answer a click is
    /// not a reason for the bar to stop repainting.
    pub fn dispatch(&self, index: usize, action: TrayAction) {
        // A touchpad reports the end of a gesture as a zero delta, and
        // the compositor sends one after every wheel detent. Rounding
        // cannot turn those into a detent, so they arrive here as
        // `Scroll(0)` -- a bus call telling an application that nothing
        // happened.
        if let TrayAction::Scroll(0) = action {
            return;
        }
        let Some(key) = self.keys.get(index).cloned() else {
            return;
        };
        let _ = self.commands.try_send(Command { key, action });
    }
}

fn run(
    config: &Config,
    shared: &Snapshot<Vec<Entry>>,
    commands: &async_channel::Receiver<Command>,
) {
    loop {
        // `serve` only returns when the connection itself is gone.
        let _ = zbus::block_on(serve(config, shared, commands));
        // No bus, or the bus went away. Either way the honest reading
        // is that this machine has no tray, which draws nothing.
        shared.set(Vec::new());
        thread::sleep(RETRY);
    }
}

/// The item list, shared between the watcher interface (which is driven
/// by incoming method calls) and the loop (which prunes it).
#[derive(Clone, Default)]
struct Registry(Arc<Mutex<Vec<String>>>);

impl Registry {
    /// Poisoning is recovered from rather than propagated, for the same
    /// reason as in `sensor.rs`: one bad write should cost one reading.
    fn with<T>(&self, f: impl FnOnce(&mut Vec<String>) -> T) -> T {
        let mut guard = self
            .0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        f(&mut guard)
    }

    fn items(&self) -> Vec<String> {
        self.with(|items| items.clone())
    }

    /// True when the key was new, so the caller knows whether a
    /// registration signal is owed.
    fn insert(&self, key: String) -> bool {
        self.with(|items| {
            if items.contains(&key) {
                return false;
            }
            items.push(key);
            // Sorted so the bar's cells keep a stable order rather than
            // shuffling every time an application restarts.
            items.sort();
            true
        })
    }

    fn remove(&self, key: &str) {
        self.with(|items| items.retain(|held| held != key));
    }
}

/// Our half of `org.kde.StatusNotifierWatcher`, exported whether or not
/// we end up owning the name.
struct Watcher {
    items: Registry,
    /// How many hosts have registered. Applications consult this before
    /// deciding a tray exists at all.
    hosts: u32,
}

#[interface(name = "org.kde.StatusNotifierWatcher")]
impl Watcher {
    /// `service` is either the caller's bus name or, in the newer
    /// convention, just an object path -- in which case the bus name is
    /// the sender's. Both are normalised to one "name/path" key here so
    /// that the rest of the module has a single shape to handle.
    async fn register_status_notifier_item(
        &mut self,
        service: &str,
        #[zbus(header)] header: zbus::message::Header<'_>,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> zbus::fdo::Result<()> {
        let sender = header.sender().map(|s| s.to_string()).unwrap_or_default();
        let key = item_key(&sender, service);
        if self.items.insert(key.clone()) {
            Watcher::status_notifier_item_registered(&emitter, &key).await?;
            // The property changed too. Our own reads go through the
            // list rather than through notification, but a third-party
            // host driving off `PropertiesChanged` would otherwise
            // never see an item appear.
            self.registered_status_notifier_items_changed(&emitter)
                .await?;
        }
        Ok(())
    }

    /// The counterpart nobody calls. Well-behaved applications do; the
    /// common case is a crash, which the loop notices instead.
    async fn unregister_status_notifier_item(
        &mut self,
        service: &str,
        #[zbus(header)] header: zbus::message::Header<'_>,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> zbus::fdo::Result<()> {
        let sender = header.sender().map(|s| s.to_string()).unwrap_or_default();
        let key = item_key(&sender, service);
        self.items.remove(&key);
        Watcher::status_notifier_item_unregistered(&emitter, &key).await?;
        self.registered_status_notifier_items_changed(&emitter)
            .await?;
        Ok(())
    }

    /// Hosts are counted rather than listed: the only thing anyone asks
    /// is whether there is at least one, and a host that dies without
    /// saying so would leave a stale name in a list nobody reads.
    async fn register_status_notifier_host(
        &mut self,
        _service: &str,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> zbus::fdo::Result<()> {
        let first = self.hosts == 0;
        self.hosts = self.hosts.saturating_add(1);
        Watcher::status_notifier_host_registered(&emitter).await?;
        if first {
            self.is_status_notifier_host_registered_changed(&emitter)
                .await?;
        }
        Ok(())
    }

    #[zbus(property)]
    fn registered_status_notifier_items(&self) -> Vec<String> {
        self.items.items()
    }

    #[zbus(property)]
    fn is_status_notifier_host_registered(&self) -> bool {
        self.hosts > 0
    }

    /// The spec has only ever defined version 0.
    #[zbus(property)]
    fn protocol_version(&self) -> i32 {
        0
    }

    #[zbus(signal)]
    async fn status_notifier_item_registered(
        emitter: &SignalEmitter<'_>,
        service: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn status_notifier_item_unregistered(
        emitter: &SignalEmitter<'_>,
        service: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn status_notifier_host_registered(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;
}

/// The watcher as seen from outside -- which is how this module always
/// reads it, even when the owner is us.
#[proxy(
    interface = "org.kde.StatusNotifierWatcher",
    default_service = "org.kde.StatusNotifierWatcher",
    default_path = "/StatusNotifierWatcher"
)]
trait StatusNotifierWatcher {
    fn register_status_notifier_host(&self, service: &str) -> zbus::Result<()>;

    #[zbus(property)]
    fn registered_status_notifier_items(&self) -> zbus::Result<Vec<String>>;

    #[zbus(signal)]
    fn status_notifier_item_registered(&self, service: String) -> zbus::Result<()>;

    #[zbus(signal)]
    fn status_notifier_item_unregistered(&self, service: String) -> zbus::Result<()>;
}

/// Everything the loop needs, established once per connection.
struct Session {
    conn: Connection,
    dbus: DBusProxy<'static>,
    watcher: StatusNotifierWatcherProxy<'static>,
    registry: Registry,
    host_name: String,
    /// Unique name of the process currently owning the watcher, so a
    /// change of owner can be noticed and re-registered with.
    known_owner: String,
    /// Keys as of the last read, so a `NameOwnerChanged` for a name we
    /// do not care about can be dropped without a bus round trip.
    keys: Vec<String>,
    icons: icon::Icons,
    config: Config,
}

/// Why the loop woke up.
enum Wake {
    /// Re-read the item list.
    Refresh,
    /// Re-read it, and distrust the icon memo while doing so.
    Repaint,
    /// A name appeared or vanished; whether it matters is decided in
    /// the loop, where the current key set is.
    Owner(String),
    Click(Command),
}

async fn connect(config: &Config) -> zbus::Result<Session> {
    let conn = Connection::session().await?;
    let registry = Registry::default();

    // Exported before any name is requested, so an application that
    // registers in the same instant as we win the name is not answered
    // with UnknownObject.
    conn.object_server()
        .at(
            WATCHER_PATH,
            Watcher {
                items: registry.clone(),
                hosts: 0,
            },
        )
        .await?;

    // The spec wants a host to own a name of this shape. Nothing reads
    // it in practice, but an application that checks before registering
    // has something to find.
    //
    // `ReplaceExisting` is deliberately absent from these flags, here
    // and for the watcher name below: starting this bar must not take
    // the tray away from a waybar already serving one. `AllowReplacement`
    // is present because losing a name costs us nothing -- the item list
    // is always read off whoever owns the watcher, so being replaced
    // just demotes us to a plain host.
    let host_name = format!("org.kde.StatusNotifierHost-{}", std::process::id());
    let _ = conn
        .request_name_with_flags(
            host_name.as_str(),
            RequestNameFlags::DoNotQueue | RequestNameFlags::AllowReplacement,
        )
        .await;

    let dbus = DBusProxy::new(&conn).await?;
    // Properties are read fresh every time: a cached proxy would hold
    // the first watcher's values across a change of owner.
    let watcher = StatusNotifierWatcherProxy::builder(&conn)
        .cache_properties(CacheProperties::No)
        .build()
        .await?;

    Ok(Session {
        conn,
        dbus,
        watcher,
        registry,
        host_name,
        known_owner: String::new(),
        keys: Vec::new(),
        icons: icon::Icons::new(config.icon_theme.clone()),
        config: config.clone(),
    })
}

/// One connection's worth of tray, from opening the bus to losing it.
async fn serve(
    config: &Config,
    shared: &Snapshot<Vec<Entry>>,
    commands: &async_channel::Receiver<Command>,
) -> zbus::Result<()> {
    let mut session = connect(config).await?;

    let registered = session
        .watcher
        .receive_status_notifier_item_registered()
        .await?
        .map(|_| Wake::Refresh);
    let unregistered = session
        .watcher
        .receive_status_notifier_item_unregistered()
        .await?
        .map(|_| Wake::Refresh);
    let owners = session
        .dbus
        .receive_name_owner_changed()
        .await?
        .map(|signal| {
            Wake::Owner(
                signal
                    .args()
                    .map(|args| args.name().to_string())
                    .unwrap_or_default(),
            )
        });
    let items = item_signals(&session.conn).await?;
    let clicks = commands.clone().map(Wake::Click);
    // The safety net, and also the first read: the timer fires
    // immediately and then every `SAFETY_NET`.
    let ticks =
        async_io::Timer::interval_at(std::time::Instant::now(), SAFETY_NET).map(|_| Wake::Refresh);

    // Clicks first. `or` is biased towards its left operand, and a
    // click is the one event here with a person waiting on it.
    //
    // Boxed because `async_channel`'s receiver is `!Unpin`, and this
    // one allocation for the life of the connection is cheaper to read
    // than a `pin!` of a type this long.
    let mut events = Box::pin(or(
        clicks,
        or(or(registered, unregistered), or(items, or(owners, ticks))),
    ));

    while let Some(wake) = events.next().await {
        match wake {
            Wake::Click(command) => {
                dispatch(&session.conn, &command).await;
                continue;
            }
            Wake::Owner(name) => {
                // The bus announces every activation; almost none of
                // them are ours. Cheaper to check the key set we
                // already have than to `GetAll` the world.
                let ours = name == WATCHER_NAME
                    || session
                        .keys
                        .iter()
                        .any(|key| split_key(key).0 == name.as_str());
                if !ours {
                    continue;
                }
            }
            Wake::Repaint => session.icons.forget(),
            Wake::Refresh => {}
        }
        refresh(&mut session, shared).await?;
    }
    Ok(())
}

/// Every signal on either item interface, as one stream.
///
/// A match rule per *interface* rather than a subscription per item:
/// the bus does the filtering, an item appearing needs no bookkeeping
/// here, and an item that dies takes its signals with it.
async fn item_signals(conn: &Connection) -> zbus::Result<impl futures_lite::Stream<Item = Wake>> {
    let mut streams = Vec::new();
    for name in ITEM_INTERFACES {
        let rule = MatchRule::builder()
            .msg_type(zbus::message::Type::Signal)
            .interface(name)?
            .build();
        streams.push(MessageStream::for_match_rule(rule, conn, Some(64)).await?);
    }
    // `ITEM_INTERFACES` has two entries and this is written for two.
    let (second, first) = (streams.pop(), streams.pop());
    let both = or(
        first.expect("two interfaces"),
        second.expect("two interfaces"),
    );
    Ok(both.map(|message| {
        let member = message
            .as_ref()
            .ok()
            .and_then(|m| m.header().member().map(|m| m.to_string()))
            .unwrap_or_default();
        if ICON_SIGNALS.contains(&member.as_str()) {
            Wake::Repaint
        } else {
            Wake::Refresh
        }
    }))
}

/// One read of the whole tray.
///
/// `Err` means the connection itself is gone -- every lesser failure
/// (an item that will not answer, a watcher that is not there yet)
/// resolves to a shorter tray rather than an error.
async fn refresh(session: &mut Session, shared: &Snapshot<Vec<Entry>>) -> zbus::Result<()> {
    // Cheap, and the only thing that makes taking over from a bar that
    // exited automatic rather than a restart.
    let _ = session
        .conn
        .request_name_with_flags(
            WATCHER_NAME,
            RequestNameFlags::DoNotQueue | RequestNameFlags::AllowReplacement,
        )
        .await;

    let watcher_name = BusName::try_from(WATCHER_NAME).map_err(zbus::Error::from)?;
    // `NameHasOwner` answers false rather than erroring when nobody
    // holds the name, so a failure here is the transport, which is the
    // one thing worth giving up on.
    if !session.dbus.name_has_owner(watcher_name.clone()).await? {
        session.known_owner.clear();
        session.keys.clear();
        shared.set(Vec::new());
        return Ok(());
    }

    let owner = session
        .dbus
        .get_name_owner(watcher_name)
        .await
        .map(|owner| owner.to_string())
        .unwrap_or_default();

    // A new watcher has no idea we exist, so say so again. Also covers
    // the first pass, where `known_owner` is empty.
    if owner != session.known_owner {
        session.known_owner = owner;
        let _ = session
            .watcher
            .register_status_notifier_host(&session.host_name)
            .await;
    }

    let keys = session
        .watcher
        .registered_status_notifier_items()
        .await
        .unwrap_or_default();

    prune(&session.conn, &session.dbus, &session.registry, &keys).await;

    let mut entries = Vec::new();
    for key in &keys {
        if let Some(item) = describe(&session.conn, key, &mut session.icons, &session.config).await
        {
            entries.push(Entry {
                key: key.clone(),
                item,
            });
        }
    }

    session.keys = entries.iter().map(|entry| entry.key.clone()).collect();
    shared.set(entries);
    Ok(())
}

/// Drop items whose application is gone.
///
/// Only meaningful when we own the watcher -- when we do not, the list
/// is someone else's and the registry is empty, so this is a no-op.
/// Well-behaved applications call `UnregisterStatusNotifierItem`, but
/// the common case is a crash, and a tray that keeps drawing a dead
/// application is worse than one that is a read behind.
async fn prune(conn: &Connection, dbus: &DBusProxy<'_>, registry: &Registry, keys: &[String]) {
    for key in registry.items() {
        if !keys.contains(&key) {
            continue;
        }
        let (service, _) = split_key(&key);
        let alive = match BusName::try_from(service.to_string()) {
            Ok(name) => dbus.name_has_owner(name).await.unwrap_or(true),
            // An unparseable name cannot be checked, and cannot be
            // talked to either.
            Err(_) => false,
        };
        if alive {
            continue;
        }
        registry.remove(&key);
        announce_unregistered(conn, &key).await;
    }
}

/// Tell the bus an item is gone, both ways: the protocol's own signal,
/// and `PropertiesChanged` on the list.
async fn announce_unregistered(conn: &Connection, key: &str) {
    let Ok(iface) = conn
        .object_server()
        .interface::<_, Watcher>(WATCHER_PATH)
        .await
    else {
        return;
    };
    let emitter = iface.signal_emitter();
    let _ = Watcher::status_notifier_item_unregistered(emitter, key).await;
    let _ = iface
        .get()
        .await
        .registered_status_notifier_items_changed(emitter)
        .await;
}

/// Ask one item what it is, or give up on it.
///
/// `None` covers every way this goes wrong -- the application exited
/// between the list and the question, it speaks neither interface name,
/// it is wedged, it asked to be hidden -- and the caller's answer to
/// all of them is the same: leave the cell out.
async fn describe(
    conn: &Connection,
    key: &str,
    icons: &mut icon::Icons,
    config: &Config,
) -> Option<TrayItem> {
    let (service, path) = split_key(key);

    let props = PropertiesProxy::builder(conn)
        .destination(service.to_string())
        .ok()?
        .path(path.to_string())
        .ok()?
        .build()
        .await
        .ok()?;

    for name in ITEM_INTERFACES {
        let Ok(interface) = InterfaceName::try_from(name) else {
            continue;
        };
        let Ok(all) = props.get_all(interface).await else {
            continue;
        };
        if all.is_empty() {
            continue;
        }
        return item_from(&all, icons, config);
    }

    None
}

fn item_from(
    all: &HashMap<String, OwnedValue>,
    icons: &mut icon::Icons,
    config: &Config,
) -> Option<TrayItem> {
    let status = text(all, "Status");
    // The spec's own reading of `Passive` is that the host should hide
    // the item; applications that never set a status at all report
    // `Active` or nothing.
    if status == "Passive" && !config.show_passive {
        return None;
    }
    let attention = status == "NeedsAttention";

    // An item that is shouting draws its attention icon if it has one.
    // Falling back rather than requiring one matters: plenty of items
    // set the status and never define the icon, and the cell has its
    // own way of shouting.
    let mut request = icon::Request {
        name: text(all, "IconName"),
        pixmaps: pixmaps(all, "IconPixmap"),
        overlay_name: text(all, "OverlayIconName"),
        overlay_pixmaps: pixmaps(all, "OverlayIconPixmap"),
        theme_path: text(all, "IconThemePath"),
    };
    if attention {
        let name = text(all, "AttentionIconName");
        let pixmaps = pixmaps(all, "AttentionIconPixmap");
        if !name.is_empty() || !pixmaps.is_empty() {
            request.name = name;
            request.pixmaps = pixmaps;
        }
    }

    let title = text(all, "Title");
    let id = text(all, "Id");

    Some(TrayItem {
        label: label(&title, &id),
        icon: icons.resolve(&request, config.icon_size),
        attention,
    })
}

fn text(all: &HashMap<String, OwnedValue>, key: &str) -> String {
    all.get(key)
        .and_then(|value| value.downcast_ref::<String>().ok())
        .unwrap_or_default()
}

/// One of the protocol's `a(iiay)` properties.
///
/// A malformed entry is dropped rather than failing the property: an
/// item that sends one bad size and three good ones should draw.
fn pixmaps(all: &HashMap<String, OwnedValue>, key: &str) -> Vec<icon::Pixmap> {
    let Some(value) = all.get(key) else {
        return Vec::new();
    };
    let Ok(entries) = Vec::<(i32, i32, Vec<u8>)>::try_from(value.clone()) else {
        return Vec::new();
    };
    entries
        .into_iter()
        .map(|(width, height, argb)| icon::Pixmap {
            width,
            height,
            argb,
        })
        .collect()
}

/// A short stand-in when no icon could be resolved.
///
/// `Title` first because it is the human-facing name; `Id` is a fallback
/// because plenty of applications leave the title empty. Non-alphanumerics
/// are dropped so that "nm-applet" reads as NMAP rather than NM-A.
fn label(title: &str, id: &str) -> String {
    let source = if title.trim().is_empty() { id } else { title };
    let short: String = source
        .chars()
        .filter(|c| c.is_alphanumeric())
        .take(LABEL_CHARS)
        .collect();
    if short.is_empty() {
        "SNI".to_string()
    } else {
        short.to_uppercase()
    }
}

/// Send one pointer event to one item.
///
/// The coordinates are zero. The protocol asks for the pointer's
/// position in *screen* space so that the item can put a menu there,
/// and a wlr-layer-shell client is never told where it is on the
/// screen -- so a number here would be a guess dressed as a fact, and
/// every implementation treats `0, 0` as "you decide".
/// Raw calls rather than a `#[proxy]` trait because the interface name
/// is the thing that varies and a proxy fixes it at compile time; two
/// proxies for four methods would be the same call written eight times.
/// The first interface that answers wins, and an item that answers
/// neither is an item that has exited, which is not an error worth
/// reporting to a bar.
async fn dispatch(conn: &Connection, command: &Command) {
    let (service, path) = split_key(&command.key);
    for interface in ITEM_INTERFACES {
        let at =
            |method| conn.call_method(Some(service), path, Some(interface), method, &(0i32, 0i32));
        let sent = match command.action {
            TrayAction::Activate => at("Activate").await.is_ok(),
            TrayAction::Secondary => at("SecondaryActivate").await.is_ok(),
            TrayAction::Context => at("ContextMenu").await.is_ok(),
            // The protocol's axis is a string, and only an item that
            // reads it knows what to do with a vertical scroll.
            TrayAction::Scroll(delta) => conn
                .call_method(
                    Some(service),
                    path,
                    Some(interface),
                    "Scroll",
                    &(delta, "vertical"),
                )
                .await
                .is_ok(),
        };
        if sent {
            return;
        }
    }
}

/// Normalise a registration into one "name/path" key.
fn item_key(sender: &str, service: &str) -> String {
    if service.starts_with('/') {
        format!("{sender}{service}")
    } else if service.contains('/') {
        service.to_string()
    } else {
        format!("{service}{DEFAULT_ITEM_PATH}")
    }
}

/// Split a key back into bus name and object path. A key with no path
/// -- which only a foreign watcher could produce -- gets the spec's
/// default rather than being dropped.
fn split_key(key: &str) -> (&str, &str) {
    match key.find('/') {
        Some(at) => (&key[..at], &key[at..]),
        None => (key, DEFAULT_ITEM_PATH),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registration_by_bus_name_gets_the_default_path() {
        assert_eq!(
            item_key(":1.42", "org.example.Tray"),
            "org.example.Tray/StatusNotifierItem"
        );
    }

    #[test]
    fn registration_by_path_alone_is_credited_to_the_sender() {
        assert_eq!(
            item_key(":1.42", "/org/ayatana/item"),
            ":1.42/org/ayatana/item"
        );
    }

    #[test]
    fn a_registration_that_is_already_a_key_is_left_alone() {
        assert_eq!(
            item_key(":1.42", ":1.7/StatusNotifierItem"),
            ":1.7/StatusNotifierItem"
        );
    }

    #[test]
    fn keys_split_back_into_the_pair_they_were_built_from() {
        assert_eq!(
            split_key(":1.42/org/ayatana/item"),
            (":1.42", "/org/ayatana/item")
        );
        // A foreign watcher could hand us a bare name; the spec's
        // default path is the right guess, not a reason to drop it.
        assert_eq!(
            split_key("org.example.Tray"),
            ("org.example.Tray", "/StatusNotifierItem")
        );
    }

    #[test]
    fn the_label_prefers_the_title_and_drops_punctuation() {
        assert_eq!(label("Blueman", "blueman-applet"), "BLUE");
        assert_eq!(label("", "nm-applet"), "NMAP");
        assert_eq!(label("  ", "steam"), "STEA");
    }

    #[test]
    fn an_item_that_names_itself_nothing_still_gets_a_cell() {
        assert_eq!(label("", ""), "SNI");
        assert_eq!(label("---", "!!!"), "SNI");
    }

    /// A property bag shaped like the one `GetAll` returns.
    fn props(pairs: &[(&str, &str)]) -> HashMap<String, OwnedValue> {
        pairs
            .iter()
            .map(|(key, value)| {
                (
                    key.to_string(),
                    OwnedValue::try_from(zbus::zvariant::Value::from(*value)).expect("a string"),
                )
            })
            .collect()
    }

    fn icons() -> icon::Icons {
        // A theme name nothing can match, so these tests are about the
        // property reading rather than about what happens to be
        // installed on the machine running them.
        icon::Icons::new(Some("CyberpunkUiNoSuchTheme".to_string()))
    }

    #[test]
    fn a_passive_item_is_hidden_by_default_and_shown_on_request() {
        let all = props(&[("Status", "Passive"), ("Id", "syncthing")]);
        assert!(item_from(&all, &mut icons(), &Config::default()).is_none());

        let config = Config {
            show_passive: true,
            ..Config::default()
        };
        let item = item_from(&all, &mut icons(), &config).expect("shown on request");
        assert_eq!(item.label, "SYNC");
        assert!(!item.attention);
    }

    #[test]
    fn needs_attention_reaches_the_cell() {
        let all = props(&[("Status", "NeedsAttention"), ("Title", "Element")]);
        let item = item_from(&all, &mut icons(), &Config::default()).expect("active");
        assert!(item.attention);
        assert_eq!(item.label, "ELEM");
    }

    #[test]
    fn an_item_with_no_findable_icon_falls_back_to_its_label() {
        // `IconName` names something no theme has, which is the case
        // that must not produce an empty cell.
        let all = props(&[("IconName", "no-such-icon-anywhere"), ("Id", "flameshot")]);
        let item = item_from(&all, &mut icons(), &Config::default()).expect("active");
        assert!(item.icon.is_none());
        assert_eq!(item.label, "FLAM");
    }

    #[test]
    fn an_inline_pixmap_becomes_an_icon() {
        let mut all = props(&[("Title", "flameshot")]);
        // One 2x2 ARGB32 image, the shape Qt sends.
        let pixmap: Vec<(i32, i32, Vec<u8>)> = vec![(2, 2, vec![0xff; 16])];
        all.insert(
            "IconPixmap".to_string(),
            OwnedValue::try_from(zbus::zvariant::Value::from(pixmap)).expect("a(iiay)"),
        );

        let item = item_from(&all, &mut icons(), &Config::default()).expect("active");
        assert!(item.icon.is_some(), "the pixmap should have been decoded");
    }
}
