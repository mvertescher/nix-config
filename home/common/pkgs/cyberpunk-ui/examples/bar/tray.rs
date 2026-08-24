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
//! **Icons are not drawn yet.** `IconName` and `IconPixmap` both need an
//! icon-theme lookup and an image pipeline the bar does not have, so an
//! item is currently a short label taken from its own `Title` or `Id`.
//! That is a stand-in, not the finished module.

use crate::sensor::{Latest, Snapshot};
use cyberpunk_ui::bar::TrayItem;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use zbus::fdo::{DBusProxy, PropertiesProxy, RequestNameFlags};
use zbus::names::{BusName, InterfaceName};
use zbus::object_server::SignalEmitter;
use zbus::proxy::CacheProperties;
use zbus::zvariant::OwnedValue;
use zbus::{interface, proxy, Connection};

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

/// How often the item list is re-read. Matches the network module: a
/// tray icon appearing a second late is not a defect worth a signal
/// subscription per item.
const PERIOD: Duration = Duration::from_secs(2);

/// How long to wait before reconnecting after the bus goes away. Long,
/// because the usual reason is that there is no session bus at all and
/// there never will be one this session.
const RETRY: Duration = Duration::from_secs(10);

/// Characters of the item's own name used as its stand-in label. Four
/// keeps a tray cell about as wide as `CPU 12%`, so a tray appearing
/// does not shove the clock off a narrow screen.
const LABEL_CHARS: usize = 4;

/// The bar's handle on the tray.
pub struct Monitor {
    latest: Latest<Vec<TrayItem>>,
}

impl Monitor {
    /// Start watching. Returns immediately with an empty tray, which
    /// draws nothing -- the same as a desktop whose applications have
    /// no tray icons, which is the honest reading until the first poll
    /// lands.
    pub fn spawn() -> Self {
        let shared = Snapshot::new(Vec::new());
        let writer = shared.clone();

        let _ = thread::Builder::new()
            .name("cyberpunk-bar-tray".to_string())
            .spawn(move || run(&writer));

        Monitor {
            latest: Latest::new(shared, Vec::new()),
        }
    }

    /// The tray as of the last completed poll, without blocking.
    pub fn reading(&mut self) -> Vec<TrayItem> {
        self.latest.get()
    }
}

fn run(shared: &Snapshot<Vec<TrayItem>>) {
    loop {
        if let Ok(mut session) = zbus::block_on(connect()) {
            // The sleep is deliberately outside the async block. This
            // thread runs the future itself, and parking it inside one
            // would keep a `block_on` frame alive for the whole period
            // to no purpose.
            while zbus::block_on(poll(&mut session, shared)).is_ok() {
                thread::sleep(PERIOD);
            }
        }
        // No bus, or the bus went away. Either way the honest reading
        // is that this machine has no tray, which draws nothing.
        shared.set(Vec::new());
        thread::sleep(RETRY);
    }
}

/// The item list, shared between the watcher interface (which is driven
/// by incoming method calls) and the poll loop (which prunes it).
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
        }
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
        self.hosts = self.hosts.saturating_add(1);
        Watcher::status_notifier_host_registered(&emitter).await?;
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
}

/// Everything the poll loop needs, established once per connection.
struct Session {
    conn: Connection,
    dbus: DBusProxy<'static>,
    watcher: StatusNotifierWatcherProxy<'static>,
    registry: Registry,
    host_name: String,
    /// Unique name of the process currently owning the watcher, so a
    /// change of owner can be noticed and re-registered with.
    known_owner: String,
}

async fn connect() -> zbus::Result<Session> {
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
    // Properties are read fresh every poll: a cached proxy would hold
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
    })
}

/// One pass. `Err` means the connection itself is gone -- every lesser
/// failure (an item that will not answer, a watcher that is not there
/// yet) resolves to a shorter tray rather than an error.
async fn poll(session: &mut Session, shared: &Snapshot<Vec<TrayItem>>) -> zbus::Result<()> {
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

    let mut items = Vec::new();
    for key in &keys {
        if let Some(item) = describe(&session.conn, key).await {
            items.push(item);
        }
    }

    shared.set(items);
    Ok(())
}

/// Drop items whose application is gone.
///
/// Only meaningful when we own the watcher -- when we do not, the list
/// is someone else's and the registry is empty, so this is a no-op.
/// Well-behaved applications call `UnregisterStatusNotifierItem`, but
/// the common case is a crash, and a tray that keeps drawing a dead
/// application is worse than one that is a poll behind.
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
        if let Ok(emitter) = SignalEmitter::new(conn, WATCHER_PATH) {
            let _ = Watcher::status_notifier_item_unregistered(&emitter, &key).await;
        }
    }
}

/// Ask one item what it is, or give up on it.
///
/// `None` covers every way this goes wrong -- the application exited
/// between the list and the question, it speaks neither interface name,
/// it is wedged -- and the caller's answer to all of them is the same:
/// leave the cell out.
async fn describe(conn: &Connection, key: &str) -> Option<TrayItem> {
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
        return item_from(&all);
    }

    None
}

/// `None` for a passive item: the spec's own reading of `Passive` is
/// that the host should hide it, and applications that never set a
/// status at all report `Active` or nothing.
fn item_from(all: &HashMap<String, OwnedValue>) -> Option<TrayItem> {
    let status = text(all, "Status");
    if status == "Passive" {
        return None;
    }

    let title = text(all, "Title");
    let id = text(all, "Id");

    Some(TrayItem {
        label: label(&title, &id),
        attention: status == "NeedsAttention",
    })
}

fn text(all: &HashMap<String, OwnedValue>, key: &str) -> String {
    all.get(key)
        .and_then(|value| value.downcast_ref::<String>().ok())
        .unwrap_or_default()
}

/// A short stand-in for the icon this module cannot draw yet.
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
}
