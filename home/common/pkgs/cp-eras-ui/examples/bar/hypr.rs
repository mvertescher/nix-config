//! The bar's Hyprland module: the workspace list and the active
//! window's title.
//!
//! Both facts come from the compositor's request socket, and the
//! socket is the one reading on the tick that can silently take far
//! longer than a frame: a Hyprland that is busy layoutting, reloading
//! its config, or answering an earlier IPC flood can hold a request
//! for most of the 200 ms timeout, and three of them back to back is
//! a stalled bar. So like audio, network and tray this owns a thread
//! and publishes snapshots, and the bar's tick takes whatever is
//! present without ever waiting. See `sensor.rs` for the contract.
//!
//! The thread does not poll. Hyprland has a second socket,
//! `.socket2.sock`, that streams one `EVENT>>DATA` line per thing
//! that happens, and the thread sits on it and re-reads the request
//! socket only when a line says the answer may have changed. Polling
//! every second was three requests a tick, ~10k a session logged at
//! DEBUG in `hyprland.log`, for a fact that changes a few times a
//! minute; and the answer could be a second stale on top of the
//! tick, where now it is as fresh as the tick. Like the tray, a slow
//! re-read regardless of events bounds how wrong a missed or
//! misparsed line can leave the bar.
//!
//! The failure mode is the other sensors' too: no compositor, no
//! socket, or one that stops answering collapses to an empty reading
//! -- no workspaces and no window -- which [`cp_eras_ui::bar`] draws
//! as no module at all. A bar showing a stale workspace is a better
//! outcome than one in a frame stall.

use crate::sensor::{Latest, Snapshot};
use cp_eras_ui::bar::Workspace;
use std::io::{BufRead, BufReader, ErrorKind, Read, Write};
use std::os::unix::net::UnixStream;
use std::thread;
use std::time::Duration;

/// One read of the compositor, as of that instant.
#[derive(Debug, Clone, Default)]
pub struct Hypr {
    pub workspaces: Vec<Workspace>,
    pub window: String,
}

/// How often the state is re-read regardless of events.
///
/// Not the update path -- the event stream is -- but the bound on how
/// long a missed line, or a compositor that changed something without
/// saying so, can leave the bar wrong. The tray's figure, for the
/// tray's reasons.
const SAFETY_NET: Duration = Duration::from_secs(30);

/// How long to keep draining the stream after a relevant event before
/// re-reading. One workspace switch is half a dozen lines
/// (`workspacev2`, `activewindow`, `windowtitlev2`, ...) inside a
/// millisecond; reading once after the burst instead of once per line
/// is the difference between three requests and twenty.
const COALESCE: Duration = Duration::from_millis(50);

/// How long to wait before trying the event socket again after it
/// could not be opened or went away. The usual reason is that there
/// is no compositor and there will not be one this session, so the
/// wait is long; a reading is still taken each time round in case the
/// request socket answers when the event one does not.
const RETRY: Duration = Duration::from_secs(5);

/// The bar's handle on the compositor.
pub struct Monitor {
    latest: Latest<Hypr>,
}

impl Monitor {
    /// Start watching. Returns immediately with an empty reading; the
    /// first real one lands once the socket has answered, and until
    /// then the bar draws no workspaces and no window -- the same as a
    /// machine with no compositor at all.
    pub fn spawn() -> Self {
        let shared = Snapshot::new(Hypr::default());
        let writer = shared.clone();

        // Deliberately never joined, like the other sensors. If the
        // thread fails to start, or later dies, the reading stays
        // whatever was last written and the bar carries on.
        let _ = thread::Builder::new()
            .name("cp-eras-hypr".to_string())
            .spawn(move || run(&writer));

        Monitor {
            latest: Latest::new(shared, Hypr::default()),
        }
    }

    /// The last completed read, without blocking.
    pub fn reading(&mut self) -> Hypr {
        self.latest.get()
    }
}

fn run(shared: &Snapshot<Hypr>) {
    loop {
        publish(shared);
        if let Some(events) = connect(".socket2.sock") {
            follow(events, || publish(shared));
        }
        thread::sleep(RETRY);
    }
}

/// Read both facts and publish them.
fn publish(shared: &Snapshot<Hypr>) {
    shared.set(Hypr {
        workspaces: workspaces(),
        window: active_window(),
    });
}

/// Sit on the event stream until it ends. Re-reads after any burst of
/// relevant lines, and every `SAFETY_NET` regardless.
///
/// Hyprland writes each event as its own line and never a partial
/// one, but a read timeout can still land inside a line that is in
/// flight; `line` is only cleared after a complete one, so the next
/// read finishes it rather than starting a new one from its tail.
fn follow(events: UnixStream, mut publish: impl FnMut()) {
    let mut reader = BufReader::new(events);
    let mut line = String::new();
    let mut pending = false;

    loop {
        let wait = if pending { COALESCE } else { SAFETY_NET };
        if reader.get_ref().set_read_timeout(Some(wait)).is_err() {
            return;
        }
        match reader.read_line(&mut line) {
            // EOF: the compositor closed the stream, which it does
            // only on its way out.
            Ok(0) => return,
            Ok(_) => {
                pending |= wakes(&line);
                line.clear();
            }
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                publish();
                pending = false;
            }
            Err(_) => return,
        }
    }
}

/// Whether an event line can change what the bar shows.
///
/// Matched on the event name's stem so the `v2` forms count too:
/// Hyprland emits both for every one of these and the pairs carry the
/// same news. The list is what touches the workspace set, which one is
/// current, or the focused window's title. `monitoradded`,
/// `fullscreen`, `submap` and the rest are left to the safety net,
/// which is to say ignored.
fn wakes(line: &str) -> bool {
    const STEMS: [&str; 12] = [
        "workspace",
        "focusedmon",
        "createworkspace",
        "destroyworkspace",
        "renameworkspace",
        "moveworkspace",
        "urgent",
        "activewindow",
        "windowtitle",
        "openwindow",
        "closewindow",
        "movewindow",
    ];
    let Some((event, _)) = line.split_once(">>") else {
        return false;
    };
    STEMS.iter().any(|stem| event.starts_with(stem))
}

/// Open one of the compositor's sockets by file name.
fn connect(name: &str) -> Option<UnixStream> {
    let sig = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").ok()?;
    let runtime = std::env::var("XDG_RUNTIME_DIR").ok()?;
    UnixStream::connect(format!("{runtime}/hypr/{sig}/{name}")).ok()
}

/// One request/response over Hyprland's IPC socket.
///
/// Returns None rather than propagating: a bar that vanishes because
/// the compositor socket hiccuped is worse than one showing a stale
/// workspace, and every caller here has a reasonable empty case. Each
/// hop is bounded -- connect fails fast on a missing socket, and the
/// read timeout keeps a dead-but-present one from pinning the sensor
/// thread for the life of the session.
fn hypr(command: &str) -> Option<String> {
    let mut sock = connect(".socket.sock")?;
    sock.set_read_timeout(Some(Duration::from_millis(200)))
        .ok()?;
    sock.write_all(command.as_bytes()).ok()?;

    let mut out = String::new();
    sock.read_to_string(&mut out).ok()?;
    Some(out)
}

/// Parsed from the plain-text form rather than `j/`, to avoid taking a
/// JSON dependency for two fields.
///
/// `workspaces` lists every workspace as "workspace ID <n> (<name>) on
/// monitor <m>:", and `activeworkspace` names the current one.
fn workspaces() -> Vec<Workspace> {
    let active = hypr("activeworkspace")
        .and_then(|s| parse_workspace_id(&s))
        .unwrap_or(-1);

    let mut ids: Vec<i32> = hypr("workspaces")
        .map(|s| s.lines().filter_map(parse_workspace_line).collect())
        .unwrap_or_default();

    // The compositor lists them in creation order, which jumps around as
    // workspaces come and go; the bar wants them stable.
    ids.sort_unstable();
    ids.dedup();

    if ids.is_empty() && active >= 0 {
        ids.push(active);
    }

    ids.into_iter()
        .map(|id| Workspace {
            id,
            active: id == active,
        })
        .collect()
}

fn parse_workspace_line(line: &str) -> Option<i32> {
    let rest = line.trim().strip_prefix("workspace ID ")?;
    rest.split_whitespace().next()?.parse().ok()
}

fn parse_workspace_id(reply: &str) -> Option<i32> {
    reply.lines().find_map(parse_workspace_line)
}

/// `activewindow` returns a block of "key: value" lines; the title is
/// the one worth showing. Empty when nothing is focused, which is a
/// normal state rather than a failure.
fn active_window() -> String {
    let Some(reply) = hypr("activewindow") else {
        return String::new();
    };
    reply
        .lines()
        .find_map(|l| l.trim().strip_prefix("title: "))
        .map(|t| {
            let t = t.trim();
            // Long titles push the right-hand modules off a narrow
            // screen, so clip rather than let the layout fight back.
            if t.chars().count() > 90 {
                let clipped: String = t.chars().take(89).collect();
                format!("{clipped}…")
            } else {
                t.to_string()
            }
        })
        .unwrap_or_default()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_and_window_events_wake() {
        for line in [
            "workspace>>9\n",
            "workspacev2>>9,9\n",
            "createworkspacev2>>9,9\n",
            "destroyworkspace>>9\n",
            "focusedmon>>DP-3,1\n",
            "activewindow>>firefox,Restore Session — Mozilla Firefox\n",
            "activewindowv2>>59f5867ab140\n",
            "windowtitlev2>>59f5865b3040,✳ cyberpunk ui builder\n",
            "openwindow>>59f5865b3040,9,Alacritty,Alacritty\n",
            "closewindow>>59f5865b3040\n",
            "urgent>>59f5865b3040\n",
        ] {
            assert!(wakes(line), "{line:?}");
        }
    }

    #[test]
    fn unrelated_and_malformed_lines_do_not() {
        for line in [
            "monitoradded>>DP-3\n",
            "fullscreen>>1\n",
            "submap>>resize\n",
            "screencast>>1,0\n",
            "configreloaded>>\n",
            "not an event\n",
            "",
        ] {
            assert!(!wakes(line), "{line:?}");
        }
    }

    /// One workspace switch is a burst of lines; the stream is read
    /// once after it, not once per line, and a line that cannot change
    /// the drawing does not read at all. EOF ends the follow.
    #[test]
    fn a_burst_reads_once_and_noise_not_at_all() {
        let (mut writer, reader) = UnixStream::pair().unwrap();
        let handle = thread::spawn(move || {
            let mut reads = 0;
            follow(reader, || reads += 1);
            reads
        });

        writer
            .write_all(
                b"createworkspacev2>>9,9\nactivewindow>>,\nactivewindowv2>>\n\
                  workspace>>9\nworkspacev2>>9,9\nwindowtitlev2>>59f5865b3040,Alacritty\n",
            )
            .unwrap();
        thread::sleep(COALESCE * 4);
        writer.write_all(b"submap>>resize\nfullscreen>>0\n").unwrap();
        thread::sleep(COALESCE * 4);
        drop(writer);

        assert_eq!(handle.join().unwrap(), 1);
    }

    #[test]
    fn workspace_lines_parse() {
        assert_eq!(
            parse_workspace_line("workspace ID 3 (3) on monitor DP-3:"),
            Some(3)
        );
        assert_eq!(parse_workspace_line("\tmonitorID: 0"), None);
        assert_eq!(
            parse_workspace_id("workspace ID -98 (special) on monitor DP-3:\n"),
            Some(-98)
        );
    }
}
