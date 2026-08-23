//! The bar's network module: whether there is a way out, and over what.
//!
//! Read straight out of `/proc` and `/sys` rather than through a
//! NetworkManager or systemd-networkd client. The question the bar
//! actually answers is "does this machine have a default route", and
//! the kernel answers that in two small text files -- whereas a
//! manager-specific client would be a dbus dependency that is wrong on
//! any host running the other manager, or none.
//!
//! The SSID is the one fact with no cheap kernel source: it lives
//! behind nl80211 netlink and appears nowhere in `/sys`. So it is
//! best-effort, from `iw` or `iwgetid` if either is on `PATH`, resolved
//! once at startup so a host with neither never pays for a failed fork.
//! Without them the module shows the interface name, which is still the
//! useful half of the reading.
//!
//! Like the audio module this owns a thread and publishes snapshots;
//! the file reads are microseconds, but the SSID probe is a process,
//! and a process must never happen on the way to a frame.

use crate::sensor::{Latest, Snapshot};
use cyberpunk_ui::bar::Network;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

/// How often the route is re-read. Fast enough that plugging in a cable
/// shows up before you have finished plugging it in.
const PERIOD: Duration = Duration::from_secs(2);

/// The SSID cannot change without the link changing with it, so it is
/// re-read only every `SSID_EVERY` passes -- plus whenever the
/// interface changes, which is the case that actually matters. Forking
/// a process every two seconds for a string that almost never moves
/// would be the expensive kind of correct.
const SSID_EVERY: u32 = 8;

/// `RTF_UP`, from `linux/route.h`. A route entry that is not up is
/// still listed.
const RTF_UP: u32 = 0x0001;

/// The bar's handle on the link.
pub struct Monitor {
    latest: Latest<Network>,
}

impl Monitor {
    /// Start watching. Returns immediately, with the reading at
    /// `Network::Unknown` until the first pass lands -- the bar draws
    /// nothing for that moment rather than claiming the machine is
    /// offline.
    pub fn spawn() -> Self {
        let shared = Snapshot::new(Network::Unknown);
        let writer = shared.clone();

        let _ = thread::Builder::new()
            .name("cyberpunk-bar-network".to_string())
            .spawn(move || run(&writer));

        Monitor {
            latest: Latest::new(shared, Network::Unknown),
        }
    }

    /// The last known link state, without blocking.
    pub fn reading(&mut self) -> Network {
        self.latest.get()
    }
}

fn run(shared: &Snapshot<Network>) {
    let ssid_source = SsidSource::detect();
    // (interface, ssid) -- kept so a wireless link keeps its name
    // between probes.
    let mut cached: Option<(String, String)> = None;
    // Passes remaining before the SSID is worth re-reading; zero means
    // this one.
    let mut until_ssid: u32 = 0;

    loop {
        let reading = match default_interface() {
            None => {
                cached = None;
                Network::Offline
            }
            Some(interface) if is_wireless(&interface) => {
                let stale =
                    until_ssid == 0 || cached.as_ref().is_none_or(|(known, _)| known != &interface);
                if stale {
                    cached = Some((interface.clone(), ssid_source.read(&interface)));
                    until_ssid = SSID_EVERY;
                }
                let ssid = cached
                    .as_ref()
                    .map(|(_, ssid)| ssid.clone())
                    .unwrap_or_default();
                Network::Wireless { interface, ssid }
            }
            Some(interface) => {
                cached = None;
                Network::Wired { interface }
            }
        };

        shared.set(reading);
        until_ssid = until_ssid.saturating_sub(1);
        thread::sleep(PERIOD);
    }
}

/// The interface carrying the default route, IPv4 first.
///
/// IPv4 first rather than "best": on a dual-stack host both point the
/// same way, and on a v6-only host the v4 table simply has no default
/// to find.
fn default_interface() -> Option<String> {
    default_route_v4().or_else(default_route_v6)
}

/// `/proc/net/route`, one route per line after the header:
/// `Iface Destination Gateway Flags RefCnt Use Metric Mask ...`, with
/// the addresses as little-endian hex. The default route is the one to
/// 0.0.0.0/0; the lowest metric wins, which is how the kernel picks too.
fn default_route_v4() -> Option<String> {
    let table = std::fs::read_to_string("/proc/net/route").ok()?;
    let mut best: Option<(u32, String)> = None;

    for line in table.lines().skip(1) {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 8 || fields[1] != "00000000" || fields[7] != "00000000" {
            continue;
        }
        if u32::from_str_radix(fields[3], 16).unwrap_or(0) & RTF_UP == 0 {
            continue;
        }
        let metric = fields[6].parse::<u32>().unwrap_or(u32::MAX);
        if best.as_ref().is_none_or(|(known, _)| metric < *known) {
            best = Some((metric, fields[0].to_string()));
        }
    }

    best.map(|(_, interface)| interface)
}

/// `/proc/net/ipv6_route` has no header and packs the address as 32 hex
/// digits: `dest prefix src srcprefix nexthop metric refcnt use flags
/// iface`. The default route is ::/0.
fn default_route_v6() -> Option<String> {
    let table = std::fs::read_to_string("/proc/net/ipv6_route").ok()?;

    for line in table.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 10 || fields[1] != "00" {
            continue;
        }
        if fields[0] != "00000000000000000000000000000000" {
            continue;
        }
        if u32::from_str_radix(fields[8], 16).unwrap_or(0) & RTF_UP == 0 {
            continue;
        }
        // The kernel keeps unreachable and loopback defaults in here
        // too; neither is a way out.
        if fields[9] == "lo" {
            continue;
        }
        return Some(fields[9].to_string());
    }

    None
}

/// Every wireless interface gets a `wireless` directory; wired ones
/// never do. Cheaper and more portable than asking the driver.
fn is_wireless(interface: &str) -> bool {
    Path::new(&format!("/sys/class/net/{interface}/wireless")).is_dir()
}

/// Where an SSID can be had, resolved once at startup.
enum SsidSource {
    /// `iw dev <if> link`, whose output carries a `SSID: <name>` line.
    Iw(PathBuf),
    /// `iwgetid <if> -r`, which prints the name and nothing else.
    IwGetId(PathBuf),
    /// Neither tool is installed. The module falls back to the
    /// interface name, which is a downgrade rather than a failure.
    Absent,
}

impl SsidSource {
    fn detect() -> Self {
        if let Some(path) = on_path("iw") {
            return SsidSource::Iw(path);
        }
        if let Some(path) = on_path("iwgetid") {
            return SsidSource::IwGetId(path);
        }
        SsidSource::Absent
    }

    /// Empty on any failure at all: no tool, a non-zero exit, an
    /// interface that is associated with nothing. The caller treats an
    /// empty SSID as "show the interface instead".
    fn read(&self, interface: &str) -> String {
        let mut command = match self {
            SsidSource::Absent => return String::new(),
            SsidSource::Iw(bin) => {
                let mut c = Command::new(bin);
                c.args(["dev", interface, "link"]);
                c
            }
            SsidSource::IwGetId(bin) => {
                let mut c = Command::new(bin);
                c.args([interface, "-r"]);
                c
            }
        };

        // Nothing is typed at these and nothing should be read from
        // the bar's own stderr.
        let output = command.stdin(Stdio::null()).stderr(Stdio::null()).output();
        let Ok(output) = output else {
            return String::new();
        };
        if !output.status.success() {
            return String::new();
        }

        let text = String::from_utf8_lossy(&output.stdout);
        match self {
            SsidSource::Iw(_) => text
                .lines()
                .find_map(|line| line.trim().strip_prefix("SSID:"))
                .unwrap_or_default()
                .trim()
                .to_string(),
            _ => text.trim().to_string(),
        }
    }
}

/// A `which` for one name. Resolved once so that a host without the
/// tool costs a directory scan at startup instead of a failed fork
/// every probe.
fn on_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|dir| dir.join(name))
        .find(|candidate| candidate.is_file())
}
