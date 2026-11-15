//! The bar's Hyprland module: the workspace list and the active
//! window's title, and switching workspace when a cell is clicked.
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
use std::collections::BTreeSet;
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
    changed: async_channel::Receiver<()>,
}

impl Monitor {
    /// Start watching. Returns immediately with an empty reading; the
    /// first real one lands once the socket has answered, and until
    /// then the bar draws no workspaces and no window -- the same as a
    /// machine with no compositor at all.
    pub fn spawn() -> Self {
        let shared = Snapshot::new(Hypr::default());
        let writer = shared.clone();
        // Capacity one: a wake is a fact, not a count. If the bar has
        // not yet taken the last one, another would say nothing new.
        let (wake, changed) = async_channel::bounded(1);

        // Deliberately never joined, like the other sensors. If the
        // thread fails to start, or later dies, the reading stays
        // whatever was last written and the bar carries on.
        let _ = thread::Builder::new()
            .name("cp-eras-hypr".to_string())
            .spawn(move || run(&writer, &wake));

        Monitor {
            latest: Latest::new(shared, Hypr::default()),
            changed,
        }
    }

    /// The last completed read, without blocking.
    pub fn reading(&mut self) -> Hypr {
        self.latest.get()
    }

    /// Rings once per completed read, so the bar can redraw when the
    /// compositor changed something rather than at the next tick. A
    /// workspace switch that showed up to a second late was the one
    /// thing waybar was visibly better at (2026-09-06, at the desk);
    /// the thread had the answer within 50 ms, and the tick was the
    /// whole delay.
    ///
    /// A clone of one queue, like the tray's `opened`, so the
    /// subscription can hold its own handle.
    pub fn changed(&self) -> async_channel::Receiver<()> {
        self.changed.clone()
    }
}

fn run(shared: &Snapshot<Hypr>, wake: &async_channel::Sender<()>) {
    // Which workspaces hold a window that asked for attention. Hyprland
    // has no request that answers this: the only record is the
    // `urgent>>ADDRESS` event, so the thread keeps it.
    let mut urgent = BTreeSet::new();
    loop {
        publish(shared, wake, &mut urgent, &[]);
        if let Some(events) = connect(".socket2.sock") {
            follow(events, |raised| publish(shared, wake, &mut urgent, raised));
        }
        thread::sleep(RETRY);
    }
}

/// Read both facts, publish them, and ring the bar. A full queue
/// means a wake is already waiting and this one is the same news; a
/// closed one means the bar is gone, which the thread finds out when
/// the process does.
///
/// `raised` are the windows that asked for attention since the last
/// read; their workspaces join `urgent`, which [`fold`] then clears of
/// the active workspace and of any that no longer exist.
fn publish(
    shared: &Snapshot<Hypr>,
    wake: &async_channel::Sender<()>,
    urgent: &mut BTreeSet<i32>,
    raised: &[String],
) {
    if !raised.is_empty() {
        if let Some(clients) = hypr("clients") {
            urgent.extend(workspaces_of(&parse_clients(&clients), raised));
        }
    }
    shared.set(Hypr {
        workspaces: workspaces(urgent),
        window: active_window(),
    });
    let _ = wake.try_send(());
}

/// Ask the compositor to switch to workspace `id`, without waiting for
/// it: the request goes out on a thread of its own, so a busy
/// compositor stalls nothing, and its answer comes back the usual way,
/// as `workspace>>` on the event stream.
pub fn switch(id: i32) {
    let command = switch_command(id);
    let _ = thread::Builder::new()
        .name("cp-eras-hypr-dispatch".to_string())
        .spawn(move || hypr(&command));
}

/// The request [`switch`] sends: the dispatcher SUPER+<n> runs too, so
/// a click does exactly what the key does, `on-created-empty` included.
fn switch_command(id: i32) -> String {
    format!("dispatch workspace {id}")
}

/// Sit on the event stream until it ends. Re-reads after any burst of
/// relevant lines, and every `SAFETY_NET` regardless.
///
/// Hyprland writes each event as its own line and never a partial
/// one, but a read timeout can still land inside a line that is in
/// flight; `line` is only cleared after a complete one, so the next
/// read finishes it rather than starting a new one from its tail.
///
/// `publish` is handed the addresses of the windows that raised
/// `urgent` during the burst, which only the stream reports.
fn follow(events: UnixStream, mut publish: impl FnMut(&[String])) {
    let mut reader = BufReader::new(events);
    let mut line = String::new();
    let mut pending = false;
    let mut raised = Vec::new();

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
                if let Some(address) = urgent_address(&line) {
                    raised.push(address.to_string());
                }
                line.clear();
            }
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                publish(&raised);
                raised.clear();
                pending = false;
            }
            Err(_) => return,
        }
    }
}

/// The window an `urgent>>ADDRESS` line names, as the bare hex address
/// `clients` heads its blocks with.
fn urgent_address(line: &str) -> Option<&str> {
    let address = line.strip_prefix("urgent>>")?.trim();
    (!address.is_empty()).then_some(address)
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
/// monitor <m>:" followed by its "windows: <count>", and
/// `activeworkspace` names the current one.
fn workspaces(urgent: &mut BTreeSet<i32>) -> Vec<Workspace> {
    let active = hypr("activeworkspace")
        .and_then(|s| parse_workspace_id(&s))
        .unwrap_or(-1);
    let listed = hypr("workspaces")
        .map(|s| parse_workspaces(&s))
        .unwrap_or_default();
    fold(listed, active, urgent)
}

/// The listed workspaces as the bar's readings, sorted and deduplicated
/// -- the compositor lists them in creation order, which jumps around as
/// workspaces come and go, and the bar wants them stable. Visiting a
/// workspace is what clears its urgency, as it does Hyprland's own
/// border colour, and a workspace that is gone takes its urgency with it.
fn fold(mut listed: Vec<(i32, u32)>, active: i32, urgent: &mut BTreeSet<i32>) -> Vec<Workspace> {
    listed.sort_unstable();
    listed.dedup_by_key(|(id, _)| *id);

    if listed.is_empty() && active >= 0 {
        listed.push((active, 0));
    }

    urgent.remove(&active);
    urgent.retain(|id| listed.iter().any(|(listed, _)| listed == id));

    listed
        .into_iter()
        .map(|(id, windows)| Workspace {
            id,
            active: id == active,
            occupied: windows > 0,
            urgent: urgent.contains(&id),
        })
        .collect()
}

/// Each workspace of a `workspaces` reply with its window count. A
/// block with no `windows:` line counts as occupied, as every inactive
/// workspace Hyprland keeps is.
fn parse_workspaces(reply: &str) -> Vec<(i32, u32)> {
    let mut out: Vec<(i32, u32)> = Vec::new();
    for line in reply.lines() {
        if let Some(id) = parse_workspace_line(line) {
            out.push((id, 1));
        } else if let (Some(last), Some(count)) = (out.last_mut(), line.trim().strip_prefix("windows: ")) {
            last.1 = count.trim().parse().unwrap_or(1);
        }
    }
    out
}

/// Each window of a `clients` reply, as its address ("Window
/// 59f5865b3040 -> title:") and the id on its "workspace: <n> (<name>)"
/// line.
fn parse_clients(reply: &str) -> Vec<(String, i32)> {
    let mut out = Vec::new();
    let mut address: Option<&str> = None;
    for line in reply.lines() {
        if let Some(rest) = line.strip_prefix("Window ") {
            address = rest.split_whitespace().next();
        } else if let (Some(a), Some(rest)) = (address, line.trim().strip_prefix("workspace: ")) {
            if let Some(Ok(id)) = rest.split_whitespace().next().map(str::parse) {
                out.push((a.to_string(), id));
            }
            address = None;
        }
    }
    out
}

/// The workspaces the `raised` windows are on.
fn workspaces_of(clients: &[(String, i32)], raised: &[String]) -> Vec<i32> {
    clients
        .iter()
        .filter(|(address, _)| raised.iter().any(|r| r.trim_start_matches("0x") == address.trim_start_matches("0x")))
        .map(|&(_, id)| id)
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
            follow(reader, |_| reads += 1);
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

    /// The `urgent` lines of a burst reach the read that follows it, and
    /// only those of that burst.
    #[test]
    fn urgent_windows_reach_the_read() {
        let (mut writer, reader) = UnixStream::pair().unwrap();
        let handle = thread::spawn(move || {
            let mut reads: Vec<Vec<String>> = Vec::new();
            follow(reader, |raised| reads.push(raised.to_vec()));
            reads
        });
        writer.write_all(b"urgent>>59f5865b3040\nactivewindow>>,\n").unwrap();
        thread::sleep(COALESCE * 4);
        writer.write_all(b"workspace>>4\n").unwrap();
        thread::sleep(COALESCE * 4);
        drop(writer);
        assert_eq!(handle.join().unwrap(), [vec!["59f5865b3040".to_string()], vec![]]);
        assert_eq!(urgent_address("urgent>>\n"), None);
        assert_eq!(urgent_address("urgentish\n"), None);
    }

    const WORKSPACES: &str = "workspace ID 3 (3) on monitor DP-3:\n\tmonitorID: 0\n\twindows: 2\n\thasfullscreen: 0\n\tlastwindow: 0x59f5865b3040\n\tlastwindowtitle: nvim\n\nworkspace ID 1 (1) on monitor DP-3:\n\tmonitorID: 0\n\twindows: 1\n\nworkspace ID 10 (10) on monitor DP-3:\n\tmonitorID: 0\n\twindows: 0\n\nworkspace ID -98 (special:magic) on monitor DP-3:\n\tmonitorID: 0\n\twindows: 1\n";

    const CLIENTS: &str = "Window 59f5865b3040 -> nvim:\n\tmapped: 1\n\thidden: 0\n\tat: 10,40\n\tsize: 1900,1000\n\tworkspace: 3 (3)\n\tfloating: 0\n\nWindow 59f5865b4000 -> Firefox:\n\tmapped: 1\n\tworkspace: 1 (1)\n\nWindow 59f5865b5000 -> scratch:\n\tworkspace: -98 (special:magic)\n";

    #[test]
    fn workspaces_parse_with_their_window_counts() {
        assert_eq!(parse_workspaces(WORKSPACES), [(3, 2), (1, 1), (10, 0), (-98, 1)]);
        assert_eq!(
            parse_clients(CLIENTS),
            [("59f5865b3040".to_string(), 3), ("59f5865b4000".to_string(), 1), ("59f5865b5000".to_string(), -98)]
        );
        assert_eq!(workspaces_of(&parse_clients(CLIENTS), &["0x59f5865b4000".to_string()]), [1]);
    }

    /// Occupied from the window count, urgency kept until the workspace
    /// is visited or goes away.
    #[test]
    fn urgency_lasts_until_the_workspace_is_visited() {
        let mut urgent = BTreeSet::from([1, 7]);
        let shown = fold(parse_workspaces(WORKSPACES), 10, &mut urgent);
        let row: Vec<_> = shown.iter().map(|w| (w.id, w.active, w.occupied, w.urgent)).collect();
        assert_eq!(
            row,
            [(-98, false, true, false), (1, false, true, true), (3, false, true, false), (10, true, false, false)]
        );
        // Workspace 7 no longer exists, so it is no longer urgent.
        assert_eq!(urgent, BTreeSet::from([1]));
        // Going to 1 clears it.
        let shown = fold(parse_workspaces(WORKSPACES), 1, &mut urgent);
        assert!(shown.iter().all(|w| !w.urgent));
        assert!(urgent.is_empty());
    }

    /// A click on a badge reaches the compositor as the dispatcher the
    /// workspace keys run.
    #[test]
    fn a_click_dispatches_the_workspace_it_names() {
        use cp_eras_ui::bar::{shown_workspaces, workspace_target, WorkspaceAction};
        let listed = fold(parse_workspaces(WORKSPACES), 3, &mut BTreeSet::new());
        let shown = shown_workspaces(10, &listed);
        let command = |action| workspace_target(&shown, action).map(switch_command);
        assert_eq!(command(WorkspaceAction::Switch(10)).as_deref(), Some("dispatch workspace 10"));
        assert_eq!(command(WorkspaceAction::Switch(9)).as_deref(), Some("dispatch workspace 9"));
        assert_eq!(command(WorkspaceAction::Switch(3)), None);
        assert_eq!(command(WorkspaceAction::Step(1)).as_deref(), Some("dispatch workspace 4"));
        assert_eq!(command(WorkspaceAction::Step(-1)).as_deref(), Some("dispatch workspace 2"));
    }
}
