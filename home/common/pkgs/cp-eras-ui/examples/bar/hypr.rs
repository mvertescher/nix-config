//! The bar's Hyprland module: the workspace list and the active
//! window's title.
//!
//! Both facts come from the compositor's unix socket, and the socket
//! is the one reading on the tick that can silently take far longer
//! than a frame: a Hyprland that is busy layoutting, reloading its
//! config, or answering an earlier IPC flood can hold a request for
//! most of the 200 ms timeout, and three of them back to back is a
//! stalled bar. So like audio, network and tray this owns a thread and
//! publishes snapshots, and the bar's tick takes whatever is present
//! without ever waiting. See `sensor.rs` for the contract.
//!
//! The failure mode is the other sensors' too: no compositor, no
//! socket, or one that stops answering collapses to an empty reading
//! -- no workspaces and no window -- which [`cp_eras_ui::bar`] draws
//! as no module at all. A bar showing a stale workspace is a better
//! outcome than one in a frame stall.

use crate::sensor::{Latest, Snapshot};
use cp_eras_ui::bar::Workspace;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

/// One read of the compositor, as of that instant.
#[derive(Debug, Clone, Default)]
pub struct Hypr {
    pub workspaces: Vec<Workspace>,
    pub window: String,
}

/// How often the socket is asked. The bar's tick is once a second and
/// this matches it, so a healthy socket updates the drawing at the
/// same cadence the inline reads used to.
const PERIOD: Duration = Duration::from_secs(1);

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
        shared.set(Hypr {
            workspaces: workspaces(),
            window: active_window(),
        });
        thread::sleep(PERIOD);
    }
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
    let sig = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").ok()?;
    let runtime = std::env::var("XDG_RUNTIME_DIR").ok()?;
    let path = format!("{runtime}/hypr/{sig}/.socket.sock");

    let mut sock = std::os::unix::net::UnixStream::connect(path).ok()?;
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