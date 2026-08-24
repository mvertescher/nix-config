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
//! ## The context menu
//!
//! An item that exports a `Menu` object wants the host to *draw* its
//! menu rather than to call `ContextMenu` and let it draw its own. So
//! a right click resolves in that order: `Menu` if there is one, the
//! `ContextMenu` method if there is not. `ItemIsMenu` moves the left
//! button onto the same path, which is what an application that has
//! only ever had a menu means by it.
//!
//! The menu itself is `com.canonical.dbusmenu` -- a numbered tree,
//! fetched with `GetLayout` and clicked with `Event`. What leaves this
//! module is a [`TrayMenu`] of the same shape, for the same reason
//! [`TrayItem`] is not the protocol's own: the thirty optional
//! properties of a dbusmenu row describe six things a bar draws.
//!
//! `GetLayout` is asked for the whole tree ([`MENU_DEPTH`]), so a
//! submenu is drawn from the reply that drew its parent and opening
//! one costs no round trip. The bar draws it inside the surface it
//! already has -- see `cyberpunk_ui::bar::tray_menu` for why that is
//! the only option here that dismisses.
//!
//! Three protocol courtesies are kept because applications do act on
//! them: `AboutToShow` before the layout is read, which is how an
//! application with a dynamic menu is told to populate it; the same
//! call again on a *submenu* when the bar opens one, followed by a
//! re-read ([`Command::Expand`]), which is the only way a submenu
//! built on demand ever has anything in it; and `Event(0, "closed")`
//! when the bar puts the menu away.
//!
//! Menu-row icons go through `icon.rs` like every other icon here.
//! Note the one place the two protocols riding this bus disagree:
//! dbusmenu's `icon-data` is a PNG file, where the item interface's
//! `IconPixmap` is raw ARGB32.

use crate::icon;
use crate::sensor::{Latest, Snapshot};
use cyberpunk_ui::bar::{MenuEntry, MenuKind, TrayAction, TrayItem, TrayMenu};
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

/// The menu interface an item points at with its `Menu` property.
/// One name, unlike the item interface: nothing has ever spelled this
/// one two ways.
const MENU_INTERFACE: &str = "com.canonical.dbusmenu";

/// How deep `GetLayout` is asked to go. `-1` is the whole tree.
///
/// One level was enough while a submenu was drawn as a marker and
/// nothing else. Now that the bar opens them, asking for everything at
/// once is what makes opening one instant: the reply that draws a menu
/// already contains its submenus, so a click has nothing to wait for.
/// The depth is the *item's* choice, though, so the recursion on this
/// side is bounded by [`MENU_LEVELS`] rather than by trust.
const MENU_DEPTH: i32 = -1;

/// Levels of submenu kept, counting the top level as one.
///
/// Nothing sane nests a tray menu five deep, and the chain is drawn
/// across the screen rather than down it -- so this is the bound that
/// keeps a pathological item from walking a menu off the left edge.
const MENU_LEVELS: usize = 5;

/// Rows drawn before a menu -- or one of its submenus -- is cut short.
///
/// A menu is another application's data and the panel is placed
/// against the pointer, so an item with a thousand-row menu would
/// otherwise draw a thousand-row panel down through the screen and out
/// the bottom.
const MENU_ROWS: usize = 40;

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
    /// The same for a menu row's icon, from
    /// `cyberpunk_ui::bar::menu_icon_size`. A separate number because a
    /// tray cell is sized by the bar's height and a menu row by its own
    /// line of text, and on a tall bar those are not close.
    pub menu_icon_size: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            show_passive: false,
            icon_theme: None,
            icon_size: 16,
            menu_icon_size: 16,
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
    menu: MenuRef,
}

/// Where an item keeps its menu, if it keeps one.
///
/// Read off the item alongside everything else rather than asked for
/// when a click arrives: the click is the moment a person is waiting,
/// and the answer has not changed since the last `GetAll`.
#[derive(Clone, Default)]
struct MenuRef {
    /// The `Menu` property: an object path on the item's own bus name.
    /// Empty when the item exports none, which is the case
    /// `ContextMenu` exists for.
    path: String,
    /// `ItemIsMenu`. The item is saying it has no primary action and
    /// that a plain click should open the menu too.
    is_menu: bool,
}

/// Something on its way from the drawing thread to the bus.
enum Command {
    /// A pointer event on the cell.
    Pointer { key: String, action: TrayAction },
    /// A row of the item's own menu was clicked.
    Entry { key: String, entry: i32 },
    /// A submenu was opened. `AboutToShow` on that row's id, then a
    /// re-read of the whole layout -- which is what an application
    /// that builds a submenu on demand is waiting for, and which
    /// reaches the bar down the same channel the first menu did.
    Expand { key: String, entry: i32 },
    /// The bar has put the menu away. Sent so an application that
    /// tracks its own menu state is not left thinking it is still up.
    Closed { key: String },
}

/// A menu on its way back: in answer to a right click, or as the
/// re-read that follows a submenu being opened.
///
/// One type for both because they are the same thing -- an item's menu
/// as of now. Whether the bar has one on screen already is what tells
/// the two apart, and that is the bar's own state to consult.
#[derive(Debug, Clone)]
pub struct Opened {
    /// The item it belongs to, which is also what a click on one of
    /// its rows has to be sent back with. Opaque to the bar.
    pub key: String,
    pub menu: TrayMenu,
}

/// The bar's handle on the tray.
pub struct Monitor {
    latest: Latest<Vec<Entry>>,
    /// Keys of the entries last handed to the bar, so an index into
    /// what was drawn resolves to the item that was drawn there.
    keys: Vec<String>,
    commands: async_channel::Sender<Command>,
    opened: async_channel::Receiver<Opened>,
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
        // A channel rather than a snapshot, because a menu is an
        // *event* and not a reading: the bar wants the one that
        // answers the click it just made, not the latest one to exist.
        // Small, for the same reason -- a queue of menus is a queue of
        // stale menus.
        let (menus, opened) = async_channel::bounded(4);

        let _ = thread::Builder::new()
            .name("cyberpunk-bar-tray".to_string())
            .spawn(move || run(&config, &writer, &receiver, &menus));

        Monitor {
            latest: Latest::new(shared, Vec::new()),
            keys: Vec::new(),
            commands: sender,
            opened,
        }
    }

    /// Menus as they are fetched, for the bar to subscribe to.
    ///
    /// Cloned rather than borrowed: an iced `Subscription` is built
    /// afresh on every `view` and wants a stream by value, and an
    /// `async_channel` receiver is a handle on the one queue however
    /// many times it is cloned.
    pub fn opened(&self) -> async_channel::Receiver<Opened> {
        self.opened.clone()
    }

    /// The key of the item drawn at `index`, so the bar can tell
    /// whether an arriving menu is still the one it asked for.
    pub fn key(&self, index: usize) -> Option<String> {
        self.keys.get(index).cloned()
    }

    /// Whether the last reading still contains this item. An
    /// application that exits with its menu on screen leaves a panel
    /// answering to nobody.
    pub fn holds(&self, key: &str) -> bool {
        self.keys.iter().any(|held| held == key)
    }

    /// Click one row of an item's menu.
    pub fn activate(&self, key: &str, entry: i32) {
        let _ = self.commands.try_send(Command::Entry {
            key: key.to_string(),
            entry,
        });
    }

    /// Tell the item one of its submenus is being opened.
    ///
    /// The bar has usually drawn that submenu already, from the tree it
    /// is holding; this is the protocol courtesy that gives an
    /// application which fills a submenu on demand its chance to. The
    /// answer arrives as another [`Opened`] for the same key, and it is
    /// what makes a submenu that was *empty* in the first layout open
    /// at all.
    pub fn expand(&self, key: &str, entry: i32) {
        let _ = self.commands.try_send(Command::Expand {
            key: key.to_string(),
            entry,
        });
    }

    /// Tell the item its menu is no longer on screen.
    pub fn closed(&self, key: &str) {
        let _ = self.commands.try_send(Command::Closed {
            key: key.to_string(),
        });
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
        let _ = self.commands.try_send(Command::Pointer { key, action });
    }
}

fn run(
    config: &Config,
    shared: &Snapshot<Vec<Entry>>,
    commands: &async_channel::Receiver<Command>,
    menus: &async_channel::Sender<Opened>,
) {
    loop {
        // `serve` only returns when the connection itself is gone.
        let _ = zbus::block_on(serve(config, shared, commands, menus));
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
    /// Where each of those items keeps its menu. Held apart from the
    /// snapshot the bar reads because it is the *host* half of an
    /// item: the bar never sees a menu object path.
    menus: HashMap<String, MenuRef>,
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
        menus: HashMap::new(),
        icons: icon::Icons::new(config.icon_theme.clone()),
        config: config.clone(),
    })
}

/// One connection's worth of tray, from opening the bus to losing it.
async fn serve(
    config: &Config,
    shared: &Snapshot<Vec<Entry>>,
    commands: &async_channel::Receiver<Command>,
    menus: &async_channel::Sender<Opened>,
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
                dispatch(&mut session, &command, menus).await;
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
        if let Some((item, menu)) =
            describe(&session.conn, key, &mut session.icons, &session.config).await
        {
            entries.push(Entry {
                key: key.clone(),
                item,
                menu,
            });
        }
    }

    session.keys = entries.iter().map(|entry| entry.key.clone()).collect();
    session.menus = entries
        .iter()
        .map(|entry| (entry.key.clone(), entry.menu.clone()))
        .collect();
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
) -> Option<(TrayItem, MenuRef)> {
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
) -> Option<(TrayItem, MenuRef)> {
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

    let menu = MenuRef {
        path: object_path(all, "Menu"),
        is_menu: all
            .get("ItemIsMenu")
            .and_then(|value| value.downcast_ref::<bool>().ok())
            .unwrap_or(false),
    };

    Some((
        TrayItem {
            label: label(&title, &id),
            icon: icons.resolve(&request, config.icon_size),
            attention,
        },
        menu,
    ))
}

fn text(all: &HashMap<String, OwnedValue>, key: &str) -> String {
    all.get(key)
        .and_then(|value| value.downcast_ref::<String>().ok())
        .unwrap_or_default()
}

/// A property the spec types as `o`.
///
/// Falls back to reading it as a string, because an item that got the
/// type wrong still knows where its own menu is, and a menu that does
/// not open is indistinguishable to a person from one that is not
/// there.
fn object_path(all: &HashMap<String, OwnedValue>, key: &str) -> String {
    let Some(value) = all.get(key) else {
        return String::new();
    };
    if let Ok(path) = value.downcast_ref::<zbus::zvariant::ObjectPath<'_>>() {
        return path.to_string();
    }
    text(all, key)
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

/// Act on one thing the drawing thread asked for.
///
/// Every arm is best-effort and none of them reports failure: the
/// caller is a bar that must keep repainting, and the usual reason a
/// call fails is that the application exited between the click and the
/// bus.
///
/// Takes the session by `&mut` for one reason: resolving a menu row's
/// icon fills the same memo a tray cell's does, and the memo lives on
/// the session.
async fn dispatch(session: &mut Session, command: &Command, menus: &async_channel::Sender<Opened>) {
    // Cloned rather than borrowed so the arms below can hold `&mut
    // session.icons` at the same time. A `Connection` is a handle.
    let conn = session.conn.clone();
    let conn = &conn;
    match command {
        Command::Entry { key, entry } => {
            let Some(menu) = session.menus.get(key) else {
                return;
            };
            let (service, _) = split_key(key);
            // `clicked` with a zero data variant: the protocol carries
            // one for the events that have a payload, and this is not
            // one of them.
            let _ = conn
                .call_method(
                    Some(service),
                    menu.path.as_str(),
                    Some(MENU_INTERFACE),
                    "Event",
                    &(
                        *entry,
                        "clicked",
                        zbus::zvariant::Value::from(0i32),
                        timestamp(),
                    ),
                )
                .await;
        }
        Command::Expand { key, entry } => {
            let Some(path) = session
                .menus
                .get(key)
                .map(|menu| menu.path.clone())
                .filter(|path| !path.is_empty())
            else {
                return;
            };
            // The re-read is unconditional. `AboutToShow` answers
            // whether the layout changed, and an application that
            // built the submenu just now says `true` -- but one that
            // had it all along says `false` and we would still have
            // drawn from a tree fetched before the click. Reading
            // either way costs one round trip on this thread and
            // nothing at all on the drawing one, which already has a
            // panel up.
            if let Some(menu) = layout(
                conn,
                key,
                &path,
                *entry,
                &mut session.icons,
                &session.config,
            )
            .await
            {
                let _ = menus.try_send(Opened {
                    key: key.clone(),
                    menu,
                });
            }
        }
        Command::Closed { key } => {
            let Some(menu) = session.menus.get(key) else {
                return;
            };
            let (service, _) = split_key(key);
            let _ = conn
                .call_method(
                    Some(service),
                    menu.path.as_str(),
                    Some(MENU_INTERFACE),
                    "Event",
                    &(
                        0i32,
                        "closed",
                        zbus::zvariant::Value::from(0i32),
                        timestamp(),
                    ),
                )
                .await;
        }
        Command::Pointer { key, action } => {
            // A right click on an item that exports a menu is a
            // request to draw *that* menu; `ContextMenu` is what the
            // spec offers an item which has none. `ItemIsMenu` says
            // the same about the left button.
            let menu = session.menus.get(key).cloned();
            let wants_menu = match action {
                TrayAction::Context => true,
                TrayAction::Activate => menu.as_ref().is_some_and(|menu| menu.is_menu),
                _ => false,
            };
            if wants_menu {
                if let Some(menu) = menu.filter(|menu| !menu.path.is_empty()) {
                    // Zero: the root, which is what the protocol calls
                    // the menu as a whole.
                    if let Some(open) = layout(
                        conn,
                        key,
                        &menu.path,
                        0,
                        &mut session.icons,
                        &session.config,
                    )
                    .await
                    {
                        // Dropped rather than awaited: the queue only
                        // fills if the drawing thread has stopped
                        // reading it, and a bar that is not drawing is
                        // not one to block the bus for.
                        let _ = menus.try_send(Opened {
                            key: key.clone(),
                            menu: open,
                        });
                        return;
                    }
                }
            }
            pointer(conn, key, *action).await;
        }
    }
}

/// Milliseconds since the process started, which is all the protocol's
/// `timestamp` is used for: applications compare two of ours to each
/// other and never to a clock of their own.
fn timestamp() -> u32 {
    use std::sync::OnceLock;
    static START: OnceLock<std::time::Instant> = OnceLock::new();
    START
        .get_or_init(std::time::Instant::now)
        .elapsed()
        .as_millis() as u32
}

/// Read one item's menu as the tree a bar draws.
///
/// `AboutToShow` first, because an application with a menu it builds
/// on demand has nothing in `GetLayout` until it is asked. `about` is
/// the row being shown -- `0` for the menu as a whole, a submenu's own
/// id when the bar opens that submenu. Its answer -- whether the
/// layout changed -- is ignored: we read the layout either way, so the
/// only thing knowing would save is the read we are about to do
/// anyway.
async fn layout(
    conn: &Connection,
    key: &str,
    path: &str,
    about: i32,
    icons: &mut icon::Icons,
    config: &Config,
) -> Option<TrayMenu> {
    let (service, _) = split_key(key);

    let _ = conn
        .call_method(
            Some(service),
            path,
            Some(MENU_INTERFACE),
            "AboutToShow",
            &about,
        )
        .await;

    // Always from the root, whichever row was announced. `GetLayout`
    // takes a parent id and could be asked for the subtree alone, but
    // then the caller would have to splice a reply into a tree it is
    // holding by index -- and the whole menu is a few kilobytes.
    let reply = conn
        .call_method(
            Some(service),
            path,
            Some(MENU_INTERFACE),
            "GetLayout",
            &(0i32, MENU_DEPTH, Vec::<String>::new()),
        )
        .await
        .ok()?;

    // `(u(ia{sv}av))`: a revision nobody here tracks, and the root
    // node. The children are variants because the type is recursive
    // and D-Bus has no way to say so.
    let (_revision, root): (u32, Node) = reply.body().deserialize().ok()?;

    let entries = rows(&root.2, 1, icons, config);

    // An item whose menu is empty -- or whose every row is hidden --
    // has not given us a menu to draw, and an empty panel under the
    // pointer says less than nothing.
    (!entries.is_empty()).then_some(TrayMenu { entries })
}

/// One `com.canonical.dbusmenu` node: id, properties, children.
type Node = (i32, HashMap<String, OwnedValue>, Vec<OwnedValue>);

/// One level of the tree, as the rows a bar draws.
///
/// `level` counts from one at the top, and a node whose level is
/// already [`MENU_LEVELS`] keeps its marker and loses its children --
/// the same shape a submenu the application has not filled in yet
/// comes out as. The bar draws both the same way: marked, and clicking
/// one asks the application for its contents rather than opening a
/// panel with nothing in it.
fn rows(
    children: &[OwnedValue],
    level: usize,
    icons: &mut icon::Icons,
    config: &Config,
) -> Vec<MenuEntry> {
    children
        .iter()
        .filter_map(|child| Node::try_from(child.clone()).ok())
        .filter_map(|node| entry_from(&node, level, icons, config))
        .take(MENU_ROWS)
        .collect()
}

/// One node as a row, or `None` when the item asked for it not to be
/// drawn.
fn entry_from(
    node: &Node,
    level: usize,
    icons: &mut icon::Icons,
    config: &Config,
) -> Option<MenuEntry> {
    let (id, props, children) = node;

    // `visible` is the item saying "not now"; the spec defaults it to
    // true, and most rows never mention it.
    if !menu_flag(props, "visible", true) {
        return None;
    }

    let kind = if menu_string(props, "type") == "separator" {
        MenuKind::Separator
    } else if menu_string(props, "children-display") == "submenu" {
        MenuKind::Submenu
    } else if !menu_string(props, "toggle-type").is_empty() {
        // `toggle-state` is 1 on, 0 off, -1 indeterminate. A row that
        // does not know is drawn as off rather than as a third thing:
        // the era vocabulary has selected and not-selected and no
        // third state, and inventing one here would be a widget.
        MenuKind::Toggle(menu_int(props, "toggle-state") == 1)
    } else {
        MenuKind::Command
    };

    let label = mnemonic(&menu_string(props, "label"));
    // A separator's label is never drawn, so an unlabelled command is
    // the only case left -- and a blank row is not a row.
    if label.is_empty() && !matches!(kind, MenuKind::Separator) {
        return None;
    }

    // Only a submenu's children are read. Plenty of applications hang
    // nodes off a row that is not one -- a radio group's members are a
    // common case -- and `children-display` is the property that says
    // which of them the user is meant to be shown.
    let children = if matches!(kind, MenuKind::Submenu) && level < MENU_LEVELS {
        rows(children, level + 1, icons, config)
    } else {
        Vec::new()
    };

    Some(MenuEntry {
        id: *id,
        label,
        enabled: menu_flag(props, "enabled", true),
        kind,
        // A separator is a rule; an icon on one would be an icon on a
        // row that is not drawn as a row.
        icon: match kind {
            MenuKind::Separator => None,
            _ => icons.menu(
                &menu_string(props, "icon-name"),
                &menu_bytes(props, "icon-data"),
                config.menu_icon_size,
            ),
        },
        children,
    })
}

/// A property the protocol types as `ay`. dbusmenu's `icon-data` is
/// the only one, and it is a PNG file rather than raw pixels.
fn menu_bytes(props: &HashMap<String, OwnedValue>, key: &str) -> Vec<u8> {
    props
        .get(key)
        .and_then(|value| Vec::<u8>::try_from(value.clone()).ok())
        .unwrap_or_default()
}

fn menu_string(props: &HashMap<String, OwnedValue>, key: &str) -> String {
    props
        .get(key)
        .and_then(|value| value.downcast_ref::<String>().ok())
        .unwrap_or_default()
}

fn menu_flag(props: &HashMap<String, OwnedValue>, key: &str, default: bool) -> bool {
    props
        .get(key)
        .and_then(|value| value.downcast_ref::<bool>().ok())
        .unwrap_or(default)
}

fn menu_int(props: &HashMap<String, OwnedValue>, key: &str) -> i32 {
    props
        .get(key)
        .and_then(|value| value.downcast_ref::<i32>().ok())
        .unwrap_or_default()
}

/// Strip a label's keyboard mnemonics.
///
/// GTK's convention, which dbusmenu inherited: one underscore marks the
/// next character, two are a literal underscore. The bar takes no
/// keyboard input, so a marker it cannot act on is just a typo on
/// screen.
fn mnemonic(label: &str) -> String {
    let mut out = String::with_capacity(label.len());
    let mut chars = label.chars();
    while let Some(c) = chars.next() {
        if c != '_' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('_') => out.push('_'),
            Some(next) => out.push(next),
            None => {}
        }
    }
    out
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
async fn pointer(conn: &Connection, key: &str, action: TrayAction) {
    let (service, path) = split_key(key);
    for interface in ITEM_INTERFACES {
        let at =
            |method| conn.call_method(Some(service), path, Some(interface), method, &(0i32, 0i32));
        let sent = match action {
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
        let (item, _) = item_from(&all, &mut icons(), &config).expect("shown on request");
        assert_eq!(item.label, "SYNC");
        assert!(!item.attention);
    }

    #[test]
    fn needs_attention_reaches_the_cell() {
        let all = props(&[("Status", "NeedsAttention"), ("Title", "Element")]);
        let (item, _) = item_from(&all, &mut icons(), &Config::default()).expect("active");
        assert!(item.attention);
        assert_eq!(item.label, "ELEM");
    }

    #[test]
    fn an_item_with_no_findable_icon_falls_back_to_its_label() {
        // `IconName` names something no theme has, which is the case
        // that must not produce an empty cell.
        let all = props(&[("IconName", "no-such-icon-anywhere"), ("Id", "flameshot")]);
        let (item, _) = item_from(&all, &mut icons(), &Config::default()).expect("active");
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

        let (item, _) = item_from(&all, &mut icons(), &Config::default()).expect("active");
        assert!(item.icon.is_some(), "the pixmap should have been decoded");
    }

    #[test]
    fn an_item_that_exports_a_menu_says_where_it_is() {
        let mut all = props(&[("Title", "nm-applet")]);
        all.insert(
            "Menu".to_string(),
            OwnedValue::try_from(zbus::zvariant::Value::from(
                zbus::zvariant::ObjectPath::try_from("/org/ayatana/NotificationItem/nm/Menu")
                    .expect("a path"),
            ))
            .expect("o"),
        );
        all.insert(
            "ItemIsMenu".to_string(),
            OwnedValue::try_from(zbus::zvariant::Value::from(true)).expect("b"),
        );

        let (_, menu) = item_from(&all, &mut icons(), &Config::default()).expect("active");
        assert_eq!(menu.path, "/org/ayatana/NotificationItem/nm/Menu");
        assert!(menu.is_menu);
    }

    #[test]
    fn an_item_with_no_menu_property_is_a_context_menu_call() {
        let all = props(&[("Title", "flameshot")]);
        let (_, menu) = item_from(&all, &mut icons(), &Config::default()).expect("active");
        assert!(menu.path.is_empty());
        assert!(!menu.is_menu);
    }

    /// A dbusmenu node, as `GetLayout` would hand one over.
    fn node(id: i32, pairs: &[(&str, zbus::zvariant::Value<'static>)]) -> Node {
        nest(id, pairs, Vec::new())
    }

    /// The same, with children hung off it.
    fn nest(
        id: i32,
        pairs: &[(&str, zbus::zvariant::Value<'static>)],
        children: Vec<Node>,
    ) -> Node {
        let props = pairs
            .iter()
            .map(|(key, value)| {
                (
                    key.to_string(),
                    OwnedValue::try_from(value.try_clone().expect("cloneable")).expect("a value"),
                )
            })
            .collect();
        let children = children
            .into_iter()
            .map(|child| OwnedValue::try_from(zbus::zvariant::Value::from(child)).expect("a node"))
            .collect();
        (id, props, children)
    }

    fn string(value: &str) -> zbus::zvariant::Value<'static> {
        zbus::zvariant::Value::from(value.to_string())
    }

    /// One node as the row it becomes, at the top level.
    fn row(node: &Node) -> Option<MenuEntry> {
        entry_from(node, 1, &mut icons(), &Config::default())
    }

    /// A row marked as having a submenu, with `children` under it.
    fn submenu(id: i32, label: &str, children: Vec<Node>) -> Node {
        nest(
            id,
            &[
                ("label", string(label)),
                ("children-display", string("submenu")),
            ],
            children,
        )
    }

    #[test]
    fn a_plain_row_keeps_its_id_and_loses_its_mnemonic() {
        let entry = row(&node(7, &[("label", string("_Quit"))])).expect("a row");
        assert_eq!(entry.id, 7);
        assert_eq!(entry.label, "Quit");
        assert!(entry.enabled);
        assert_eq!(entry.kind, MenuKind::Command);
        assert!(entry.children.is_empty());
    }

    #[test]
    fn the_four_row_shapes_come_out_of_four_different_properties() {
        let separator = row(&node(1, &[("type", string("separator"))])).expect("a rule");
        assert_eq!(separator.kind, MenuKind::Separator);

        let entry = row(&submenu(2, "Networks", Vec::new())).expect("a submenu");
        assert_eq!(entry.kind, MenuKind::Submenu);

        let on = row(&node(
            3,
            &[
                ("label", string("Enable")),
                ("toggle-type", string("checkmark")),
                ("toggle-state", zbus::zvariant::Value::from(1i32)),
            ],
        ))
        .expect("a toggle");
        assert_eq!(on.kind, MenuKind::Toggle(true));

        // Indeterminate is drawn as off: the era vocabulary has two
        // states and inventing a third here would be a widget.
        let unknown = row(&node(
            4,
            &[
                ("label", string("Enable")),
                ("toggle-type", string("checkmark")),
                ("toggle-state", zbus::zvariant::Value::from(-1i32)),
            ],
        ))
        .expect("a toggle");
        assert_eq!(unknown.kind, MenuKind::Toggle(false));
    }

    #[test]
    fn a_submenus_children_come_back_with_it() {
        let entry = row(&submenu(
            2,
            "Default S_ink",
            vec![
                node(
                    3,
                    &[
                        ("label", string("Dummy Output")),
                        ("toggle-type", string("radio")),
                        ("toggle-state", zbus::zvariant::Value::from(1i32)),
                    ],
                ),
                node(4, &[("label", string("_Headphones"))]),
            ],
        ))
        .expect("a submenu");

        assert_eq!(entry.kind, MenuKind::Submenu);
        assert_eq!(entry.children.len(), 2);
        assert_eq!(entry.children[0].kind, MenuKind::Toggle(true));
        // The whole row treatment applies at every level, not just the
        // top one: this child's mnemonic is gone too.
        assert_eq!(entry.children[1].label, "Headphones");
        assert_eq!(entry.children[1].id, 4);
    }

    #[test]
    fn only_a_row_that_says_submenu_keeps_its_children() {
        // Applications hang nodes off rows that are not submenus --
        // the members of a radio group are the common case -- and
        // `children-display` is the property that says which of them
        // the user is meant to be shown.
        let entry = row(&nest(
            2,
            &[("label", string("Volume"))],
            vec![node(3, &[("label", string("Hidden"))])],
        ))
        .expect("a row");
        assert_eq!(entry.kind, MenuKind::Command);
        assert!(entry.children.is_empty());
    }

    #[test]
    fn nesting_stops_at_menu_levels() {
        // A chain one level deeper than the bound, built from the
        // bottom up.
        let mut deepest = submenu(100, "bottom", vec![node(101, &[("label", string("leaf"))])]);
        for id in (1..=MENU_LEVELS as i32).rev() {
            deepest = submenu(id, "down", vec![deepest]);
        }

        // Walk what came back and count the panels a bar could open.
        let mut entry = row(&deepest).expect("a submenu");
        let mut levels = 1;
        while let Some(child) = entry.children.first() {
            entry = child.clone();
            levels += 1;
        }
        assert_eq!(levels, MENU_LEVELS);
        // The deepest row kept its marker; it is the children that
        // were dropped, which the bar draws as marked-and-inert.
        assert_eq!(entry.kind, MenuKind::Submenu);
        assert!(entry.children.is_empty());
    }

    #[test]
    fn a_rows_icon_comes_from_its_own_png_when_no_theme_has_the_name() {
        // `icon-data` is a PNG file, not the raw ARGB32 the item
        // interface sends -- the one place the two protocols on this
        // bus disagree about what a picture is.
        let png = one_pixel_png();
        let entry = row(&node(
            9,
            &[
                ("label", string("Preferences")),
                ("icon-name", string("no-such-icon-anywhere")),
                ("icon-data", zbus::zvariant::Value::from(png)),
            ],
        ))
        .expect("a row");
        assert!(entry.icon.is_some(), "the icon-data should have decoded");
    }

    #[test]
    fn a_row_with_neither_icon_property_has_no_icon() {
        let entry = row(&node(9, &[("label", string("Quit"))])).expect("a row");
        assert!(entry.icon.is_none());
    }

    #[test]
    fn a_separator_never_carries_an_icon() {
        let entry = row(&nest(
            9,
            &[
                ("type", string("separator")),
                ("icon-data", zbus::zvariant::Value::from(one_pixel_png())),
            ],
            Vec::new(),
        ))
        .expect("a rule");
        assert!(entry.icon.is_none());
    }

    /// One opaque pixel as a PNG file, which is the shape dbusmenu
    /// asks for. Encoded rather than kept as a fixture: `png` is
    /// already a dependency for the decoding half, and a literal blob
    /// in a test file is a thing nobody can check by reading.
    fn one_pixel_png() -> Vec<u8> {
        let mut out = Vec::new();
        let mut encoder = png::Encoder::new(&mut out, 1, 1);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder
            .write_header()
            .expect("a header")
            .write_image_data(&[0xff, 0x00, 0x88, 0xff])
            .expect("a pixel");
        out
    }

    #[test]
    fn a_hidden_or_unlabelled_row_is_not_drawn() {
        assert!(row(&node(
            5,
            &[
                ("label", string("Secret")),
                ("visible", zbus::zvariant::Value::from(false)),
            ],
        ))
        .is_none());
        // A command with nothing to say is a blank row.
        assert!(row(&node(6, &[])).is_none());
        // A separator has no label and is still a rule.
        assert!(row(&node(7, &[("type", string("separator"))])).is_some());
    }

    #[test]
    fn a_disabled_row_is_drawn_and_does_not_answer() {
        let entry = row(&node(
            8,
            &[
                ("label", string("Disconnect")),
                ("enabled", zbus::zvariant::Value::from(false)),
            ],
        ))
        .expect("a row");
        assert!(!entry.enabled);
    }

    #[test]
    fn a_doubled_underscore_is_a_literal_one() {
        assert_eq!(mnemonic("_File"), "File");
        assert_eq!(mnemonic("Save __as"), "Save _as");
        assert_eq!(mnemonic("no marks here"), "no marks here");
        // A trailing marker marks nothing and is dropped.
        assert_eq!(mnemonic("Quit_"), "Quit");
    }
}
