//! mpris-status — MPRIS "now playing" line for waybar.
//!
//! A port of the upstream cybr-waybar `mediaplayer.py`, which needed python3
//! plus PyGObject and the Playerctl GIR typelib. This talks D-Bus directly via
//! zbus, so the only runtime dependency is the session bus itself.
//!
//! Emits one JSON object per line on stdout, as waybar's `return-type: json`
//! custom module expects:
//!
//!     {"text": "Artist - Title", "class": "custom-spotify", "alt": "spotify"}
//!
//! and a bare empty line when no player is worth showing.

use std::collections::HashMap;
use std::io::Write;

use futures_util::StreamExt;
use serde_json::json;
use zbus::zvariant::{OwnedValue, Value};
use zbus::{Connection, MatchRule, MessageStream, Proxy};

const MPRIS_PREFIX: &str = "org.mpris.MediaPlayer2.";
const MPRIS_PATH: &str = "/org/mpris/MediaPlayer2";
const PLAYER_IFACE: &str = "org.mpris.MediaPlayer2.Player";

/// Nerd Font pause-circle (U+F28B), prefixed when the shown player is not
/// actually playing. Byte-identical to what mediaplayer.py emitted.
const PAUSED_PREFIX: &str = "\u{f28b} ";

struct Args {
    player: Option<String>,
    exclude: Vec<String>,
}

fn parse_args() -> Args {
    let mut player = None;
    let mut exclude = Vec::new();
    let mut argv = std::env::args().skip(1);

    while let Some(arg) = argv.next() {
        match arg.as_str() {
            "--player" => player = argv.next(),
            "-x" | "--exclude" => {
                if let Some(list) = argv.next() {
                    exclude = list
                        .split(',')
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(str::to_owned)
                        .collect();
                }
            }
            // Accepted for compatibility with the python script's flags; this
            // port logs to stderr unconditionally and has no log levels.
            "-v" | "--verbose" | "--enable-logging" => {}
            "-h" | "--help" => {
                println!("usage: mpris-status [--player NAME] [-x|--exclude A,B]");
                std::process::exit(0);
            }
            other => eprintln!("mpris-status: ignoring unknown argument {other:?}"),
        }
    }

    Args { player, exclude }
}

/// playerctl-style short name: `org.mpris.MediaPlayer2.firefox.instance_1_7`
/// becomes `firefox`, which is what the CSS classes in style.css key off.
fn short_name(bus_name: &str) -> String {
    let rest = bus_name.strip_prefix(MPRIS_PREFIX).unwrap_or(bus_name);
    match rest.find(".instance") {
        Some(idx) => rest[..idx].to_owned(),
        None => rest.to_owned(),
    }
}

fn as_str(value: &Value) -> Option<String> {
    match value {
        Value::Str(s) => Some(s.to_string()),
        Value::ObjectPath(p) => Some(p.to_string()),
        Value::Value(inner) => as_str(inner),
        _ => None,
    }
}

/// `xesam:artist` is normally an array of strings, but some players publish a
/// bare string. Accept either and take the first entry.
fn first_artist(value: &Value) -> Option<String> {
    match value {
        Value::Array(items) => items.iter().find_map(as_str),
        Value::Value(inner) => first_artist(inner),
        other => as_str(other),
    }
}

struct Snapshot {
    bus_name: String,
    status: String,
    metadata: HashMap<String, OwnedValue>,
}

impl Snapshot {
    fn is_playing(&self) -> bool {
        self.status == "Playing"
    }

    /// Mirrors mediaplayer.py's on_metadata_changed: Spotify adverts get their
    /// own label, otherwise "artist - title", falling back to just the title.
    fn text(&self) -> String {
        let name = short_name(&self.bus_name);
        let artist = self.metadata.get("xesam:artist").and_then(|v| first_artist(v));
        let title = self
            .metadata
            .get("xesam:title")
            .and_then(|v| as_str(v))
            .map(|t| t.replace('&', "&amp;"));

        let is_ad = name == "spotify"
            && self
                .metadata
                .get("mpris:trackid")
                .and_then(|v| as_str(v))
                .is_some_and(|id| id.contains(":ad:"));

        let track = if is_ad {
            "Advertisement".to_owned()
        } else {
            match (artist, title) {
                (Some(a), Some(t)) => format!("{a} - {t}"),
                (_, Some(t)) => t,
                _ => String::new(),
            }
        };

        if track.is_empty() || self.is_playing() {
            track
        } else {
            format!("{PAUSED_PREFIX}{track}")
        }
    }
}

async fn list_players(conn: &Connection) -> zbus::Result<Vec<String>> {
    let dbus = Proxy::new(
        conn,
        "org.freedesktop.DBus",
        "/org/freedesktop/DBus",
        "org.freedesktop.DBus",
    )
    .await?;
    let names: Vec<String> = dbus.call("ListNames", &()).await?;
    Ok(names
        .into_iter()
        .filter(|n| n.starts_with(MPRIS_PREFIX))
        .collect())
}

async fn snapshot(conn: &Connection, bus_name: &str) -> zbus::Result<Snapshot> {
    let props = Proxy::new(
        conn,
        bus_name.to_owned(),
        MPRIS_PATH,
        "org.freedesktop.DBus.Properties",
    )
    .await?;
    let all: HashMap<String, OwnedValue> = props.call("GetAll", &(PLAYER_IFACE,)).await?;

    let status = all
        .get("PlaybackStatus")
        .and_then(|v| as_str(v))
        .unwrap_or_default();
    let metadata = match all.get("Metadata").map(|v| HashMap::try_from(v.clone())) {
        Some(Ok(map)) => map,
        _ => HashMap::new(),
    };

    Ok(Snapshot {
        bus_name: bus_name.to_owned(),
        status,
        metadata,
    })
}

/// Discovery order matters: the python original preferred the most recently
/// appeared player among those playing, so keep our own ordered list rather
/// than trusting ListNames, whose order is arbitrary.
fn reconcile(order: &mut Vec<String>, live: &[String], args: &Args) {
    order.retain(|name| live.contains(name));
    for name in live {
        let short = short_name(name);
        if args.exclude.iter().any(|e| *e == short || e == name) {
            continue;
        }
        if let Some(only) = &args.player {
            if short != *only && name != only {
                continue;
            }
        }
        if !order.contains(name) {
            order.push(name.clone());
        }
    }
}

async fn render(conn: &Connection, order: &[String]) -> String {
    let mut snapshots = Vec::new();
    for name in order {
        // A player can vanish between ListNames and GetAll; that is normal,
        // so drop it rather than treating it as an error.
        if let Ok(snap) = snapshot(conn, name).await {
            snapshots.push(snap);
        }
    }

    // Prefer the most recently added player that is actually playing, else the
    // first known player, else show nothing.
    let chosen = snapshots
        .iter()
        .rev()
        .find(|s| s.is_playing())
        .or_else(|| snapshots.first());

    match chosen {
        Some(snap) => {
            let name = short_name(&snap.bus_name);
            json!({
                "text": snap.text(),
                "class": format!("custom-{name}"),
                "alt": name,
            })
            .to_string()
        }
        None => String::new(),
    }
}

fn emit(line: &str) -> std::io::Result<()> {
    let mut out = std::io::stdout().lock();
    writeln!(out, "{line}")?;
    out.flush()
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args();
    let conn = Connection::session().await?;

    // One match rule catches property changes from every player at once, so we
    // never have to add or drop per-player subscriptions as they come and go.
    let props_rule = MatchRule::builder()
        .msg_type(zbus::message::Type::Signal)
        .interface("org.freedesktop.DBus.Properties")?
        .member("PropertiesChanged")?
        .path(MPRIS_PATH)?
        .build();
    let props = MessageStream::for_match_rule(props_rule, &conn, Some(16)).await?;

    let dbus = Proxy::new(
        &conn,
        "org.freedesktop.DBus",
        "/org/freedesktop/DBus",
        "org.freedesktop.DBus",
    )
    .await?;
    let names = dbus.receive_signal("NameOwnerChanged").await?;

    let mut events = futures_util::stream::select(props.map(|_| ()), names.map(|_| ()));

    let mut order: Vec<String> = Vec::new();
    let mut last = String::from("\u{0}"); // sentinel: never equal to a real line

    loop {
        let live = list_players(&conn).await.unwrap_or_default();
        reconcile(&mut order, &live, &args);

        let line = render(&conn, &order).await;
        if line != last {
            // waybar closing the pipe is a normal shutdown, not a failure.
            if emit(&line).is_err() {
                return Ok(());
            }
            last = line;
        }

        if events.next().await.is_none() {
            return Ok(());
        }
    }
}

fn main() {
    if let Err(err) = zbus::block_on(run()) {
        eprintln!("mpris-status: {err}");
        std::process::exit(1);
    }
}
