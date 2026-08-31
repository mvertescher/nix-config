//! The bar's audio module: the default sink's volume and mute state.
//!
//! Spoken over PulseAudio's native protocol via `libpulse-binding`
//! (Apache-2.0), rather than by shelling out to `pactl` on every tick.
//! The protocol also *pushes*: subscribing to sink events means the bar
//! reacts to a volume key immediately instead of up to a second later,
//! which is the difference between a bar that feels wired to the
//! machine and one that feels like it is polling it.
//!
//! PulseAudio's client library wants a mainloop, so this owns a thread.
//! Nothing in the bar ever waits on it. Every failure -- no server, a
//! server that stops answering, a sink that disappears -- collapses to
//! `None`, which [`cp_eras_ui::bar`] draws as no module at all rather
//! than as a volume of zero it cannot vouch for.

use crate::sensor::{Latest, Snapshot};
use cp_eras_ui::bar::Audio;
use libpulse_binding::callbacks::ListResult;
use libpulse_binding::context::subscribe::{Facility, InterestMaskSet};
use libpulse_binding::context::{Context, FlagSet, State};
use libpulse_binding::mainloop::standard::{IterateResult, Mainloop};
use libpulse_binding::proplist::{properties, Proplist};
use libpulse_binding::volume::Volume;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

/// How long to wait for the server handshake before giving up on this
/// attempt. A socket that exists but never answers must not pin the
/// sensor thread for the life of the session.
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(5);

/// Reconnection backoff. Starts patient enough to catch a restarting
/// server and ends slow enough that a machine with no sound at all is
/// not paying for a connect attempt every second all day.
const RETRY_MIN: Duration = Duration::from_secs(1);
const RETRY_MAX: Duration = Duration::from_secs(30);

/// How often to ask a quiet server to confirm it is still alive. A
/// healthy PulseAudio server sends nothing while the volume is
/// untouched, so without a standing request the watchdog could not
/// tell a working server from a hung one that keeps the connection
/// open.
const PROBE_INTERVAL: Duration = Duration::from_secs(5);

/// How much silence is allowed before the connection is declared
/// stalled. Three probe windows: a server that is alive answers one of
/// the standing requests in that time, and a hung one answers none.
/// Generous enough not to punish a server that is merely busy, short
/// enough that a dead volume reading cannot outlive its welcome.
const STALL_TIMEOUT: Duration = Duration::from_secs(15);

/// Pacing for the non-blocking mainloop loop. Ten wakeups a second is
/// nothing measurable, while keeping a probe reply -- and therefore a
/// stalled-server verdict -- within a tenth of a second of the truth.
const PUMP_SLEEP: Duration = Duration::from_millis(100);

/// The bar's handle on the sink.
pub struct Monitor {
    latest: Latest<Option<Audio>>,
}

impl Monitor {
    /// Start watching. Returns immediately; the first reading arrives
    /// once the server has answered, and until then the module is
    /// simply absent.
    pub fn spawn() -> Self {
        let shared = Snapshot::new(None);
        let writer = shared.clone();

        // Deliberately never joined. If the thread fails to start, or
        // later dies, the reading stays `None` and the bar carries on
        // without an audio module -- which is exactly the behaviour
        // wanted on a machine with no sound server.
        let _ = thread::Builder::new()
            .name("cp-eras-audio".to_string())
            .spawn(move || run(&writer));

        Monitor {
            latest: Latest::new(shared, None),
        }
    }

    /// The last known sink state, without blocking.
    pub fn reading(&mut self) -> Option<Audio> {
        self.latest.get()
    }
}

/// Connect, follow the sink until the connection dies, repeat.
fn run(shared: &Snapshot<Option<Audio>>) {
    let mut backoff = RETRY_MIN;

    loop {
        if session(shared) {
            // We had a working session, so the server exists and is
            // merely restarting; be eager about picking it back up.
            backoff = RETRY_MIN;
        }

        // The connection is gone. Drop the reading rather than leave a
        // volume on screen that nothing is maintaining any more.
        shared.set(None);

        thread::sleep(backoff);
        backoff = (backoff * 2).min(RETRY_MAX);
    }
}

/// One connection, from handshake to hang-up.
///
/// Returns whether the context ever reached `Ready`, which is the only
/// evidence available that there is a server worth waiting for.
fn session(shared: &Snapshot<Option<Audio>>) -> bool {
    let Some(mainloop) = Mainloop::new() else {
        return false;
    };
    let mainloop = Rc::new(RefCell::new(mainloop));

    let Some(mut proplist) = Proplist::new() else {
        return false;
    };
    // Errors here only cost the name PulseAudio shows in a mixer, so
    // they are not worth failing the connection over.
    let _ = proplist.set_str(properties::APPLICATION_NAME, "cp-eras-ui-bar");
    let _ = proplist.set_str(properties::APPLICATION_ID, "cp-eras-ui-bar");

    let context = {
        let ml = mainloop.borrow();
        match Context::new_with_proplist(&*ml, "cp-eras-ui-bar", &proplist) {
            Some(context) => context,
            None => return false,
        }
    };
    let context = Rc::new(RefCell::new(context));

    if context
        .borrow_mut()
        .connect(None, FlagSet::NOFLAGS, None)
        .is_err()
    {
        // No server on the socket. The common case on a headless or
        // sound-less machine, not an error worth logging every second.
        return false;
    }

    if !wait_ready(&mainloop, &context) {
        return false;
    }

    // The state we came for, plus a standing order to re-read it. SINK
    // covers volume and mute; SERVER covers the default sink itself
    // moving to different hardware.
    request(&context, shared);
    {
        let context_for_events = Rc::clone(&context);
        let shared_for_events = shared.clone();
        context.borrow_mut().set_subscribe_callback(Some(Box::new(
            move |facility, _op, _index| {
                if matches!(facility, Some(Facility::Sink) | Some(Facility::Server)) {
                    request(&context_for_events, &shared_for_events);
                }
            },
        )));
    }
    // The success flag is not actionable: if the subscription fails the
    // module simply stops updating until the next reconnect.
    let _op = context
        .borrow_mut()
        .subscribe(InterestMaskSet::SINK | InterestMaskSet::SERVER, |_| {});

    pump(&mainloop, &context, shared);

    // The subscribe callback holds an `Rc` to the context and the
    // context owns the callback: a cycle that would strand one
    // connection's worth of state on every reconnect. Cut it before the
    // session goes out of scope.
    context.borrow_mut().set_subscribe_callback(None);
    true
}

/// Run the mainloop until the server stops being believable.
///
/// The loop must not block on `iterate(true)`: a server that hangs
/// *without* dropping the connection would leave a blocked iterate --
/// and the last volume -- on screen forever. So the loop iterates
/// without blocking and sleeps briefly instead. `iterate` reports how
/// many sources it dispatched, and a count above zero is exactly "the
/// server said something", which is the heartbeat the watchdog needs.
/// Real events are dispatched on the next iterate, at most one sleep
/// later, so a volume key still lands well inside a bar frame.
fn pump(
    mainloop: &Rc<RefCell<Mainloop>>,
    context: &Rc<RefCell<Context>>,
    shared: &Snapshot<Option<Audio>>,
) {
    // The last moment the server actually spoke: a subscription event,
    // or a reply to one of the probe requests sent below. Seeded to
    // "now" so a server that never answers even the first request is
    // only given the stall window, not a head start.
    let mut last_talk = Instant::now();
    let mut last_probe = Instant::now();

    loop {
        match mainloop.borrow_mut().iterate(false) {
            IterateResult::Success(dispatched) => {
                // Zero is a normal quiet iteration; anything above
                // zero is the server answering us.
                if dispatched > 0 {
                    last_talk = Instant::now();
                }
            }
            IterateResult::Quit(_) | IterateResult::Err(_) => return,
        }
        if !matches!(context.borrow().get_state(), State::Ready) {
            // Dropped, failed, or never got here: the connection is
            // over either way.
            return;
        }

        let now = Instant::now();
        if now.duration_since(last_talk) >= STALL_TIMEOUT {
            // The whole stall window with nothing back: the server is
            // hung but still holding the connection. Believe it no
            // longer -- the caller drops the reading and reconnects,
            // so the module is absent rather than stale.
            return;
        }
        if now.duration_since(last_probe) >= PROBE_INTERVAL {
            // Ask for the sink list again. An idle-but-alive server
            // answers, which keeps `last_talk` fresh; a hung one does
            // not, which is what the stall check above is waiting for.
            last_probe = now;
            request(context, shared);
        }
        thread::sleep(PUMP_SLEEP);
    }
}

/// Pump the mainloop until the context is usable, or until it is clear
/// it will not be.
fn wait_ready(mainloop: &Rc<RefCell<Mainloop>>, context: &Rc<RefCell<Context>>) -> bool {
    let deadline = Instant::now() + HANDSHAKE_TIMEOUT;

    loop {
        // Non-blocking, because a blocking iterate on a server that
        // never answers would ignore the deadline entirely.
        match mainloop.borrow_mut().iterate(false) {
            IterateResult::Success(_) => {}
            IterateResult::Quit(_) | IterateResult::Err(_) => return false,
        }
        match context.borrow().get_state() {
            State::Ready => return true,
            State::Failed | State::Terminated => return false,
            _ => {}
        }
        if Instant::now() >= deadline {
            return false;
        }
        // Paced rather than spun: this is a handshake, not a hot path.
        thread::sleep(Duration::from_millis(20));
    }
}

/// Ask for the default sink's volume and mute, in two hops.
///
/// There is no single call for "the default sink": the server names it,
/// and the sink itself has to be looked up by that name.
fn request(context: &Rc<RefCell<Context>>, shared: &Snapshot<Option<Audio>>) {
    let context_for_sink = Rc::clone(context);
    let shared_for_sink = shared.clone();

    // Dropping the returned `Operation` does not cancel it; the
    // binding's callback proxy owns the closure and frees it when the
    // reply lands.
    let _op = context.borrow().introspect().get_server_info(move |info| {
        let Some(name) = info.default_sink_name.as_deref() else {
            return;
        };
        let name = name.to_string();
        let shared = shared_for_sink.clone();

        let _op = context_for_sink
            .borrow()
            .introspect()
            .get_sink_info_by_name(&name, move |result| {
                if let ListResult::Item(sink) = result {
                    shared.set(Some(Audio {
                        volume: percent(sink.volume.avg()),
                        muted: sink.mute,
                    }));
                }
            });
    });
}

/// PulseAudio's volume scale is not percent -- `Volume::NORMAL` is
/// 100%, and values above it are real amplification, so this rounds
/// rather than clamps.
fn percent(volume: Volume) -> u16 {
    let normal = u64::from(Volume::NORMAL.0);
    if normal == 0 {
        return 0;
    }
    let scaled = (u64::from(volume.0) * 100 + normal / 2) / normal;
    scaled.min(u64::from(u16::MAX)) as u16
}
