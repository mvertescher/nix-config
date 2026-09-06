//! The clock the screens move against, and the traces' SMIL played back.
//!
//! A trace carries its motion as `<animate>` elements on the things
//! that move (`docs/PIPELINE.md`, "Motion"), and the screens play them
//! back from here. Two rules make a frame from the app comparable with
//! a frame from `scripts/frame.sh`:
//!
//! - **Time is counted from one origin**, the moment the process first
//!   asks for it, the way a trace's document begins at 0 when it
//!   loads. Every animation is a function of `at - origin`, so two
//!   screens asked for the same instant agree.
//! - **The clock can be frozen.** `--at-ms <n>` on the command line, or
//!   `CP_ERAS_UI_AT_MS=<n>` in the environment for the harnesses that
//!   pass no arguments (`scripts/render.sh`, `tests/visual.nix`), pins
//!   [`now`] to `origin + n ms` for the life of the process and turns
//!   [`frozen`] on so the screens stop asking for ticks. The goldens
//!   are captured at 0 -- frame 0 of the trace is what they hold, and
//!   a golden that depends on when the compositor got round to the
//!   capture is not a golden.
//!
//! What is here is what the vocabulary in `docs/PIPELINE.md` needs and
//! iced's `Animation` does not give: the discrete cycle
//! (`calcMode="discrete"`), which is a phase and not an interpolation.
//! Eased transitions go through `iced::animation::Animation` with the
//! [`now`] from here as their `at`.

use std::sync::OnceLock;
use std::time::{Duration, Instant};

/// How the login traces blink their caret: `docs/<era>/login-trace.svg`
/// `#caret-blink`, `values="1;0" keyTimes="0;0.5" calcMode="discrete"
/// dur="1.2s" repeatCount="indefinite"` -- lit for the first 600 ms of
/// every 1.2 s, counted from the document's begin.
pub const CARET_BLINK: Duration = Duration::from_millis(1200);

struct Clock {
    origin: Instant,
    frozen: Option<Duration>,
}

fn clock() -> &'static Clock {
    static CLOCK: OnceLock<Clock> = OnceLock::new();
    CLOCK.get_or_init(|| Clock {
        origin: Instant::now(),
        frozen: at_ms().map(Duration::from_millis),
    })
}

/// The `--at-ms` flag, else the `CP_ERAS_UI_AT_MS` variable. A value
/// that is not a number is no value.
fn at_ms() -> Option<u64> {
    crate::shell::flag(std::env::args().skip(1), "--at-ms")
        .or_else(|| std::env::var("CP_ERAS_UI_AT_MS").ok())
        .and_then(|v| v.trim().parse().ok())
}

/// The moment the clock started: t = 0 of every trace.
pub fn origin() -> Instant {
    clock().origin
}

/// The time to draw at: the wall clock, or the pinned moment when the
/// process was started with one.
pub fn now() -> Instant {
    let clock = clock();
    match clock.frozen {
        Some(at) => clock.origin + at,
        None => Instant::now(),
    }
}

/// Whether the clock is pinned. A screen with motion subscribes to a
/// tick only when this is false: a frozen frame never changes, and a
/// redraw that draws the same thing is work the capture waits on.
pub fn frozen() -> bool {
    clock().frozen.is_some()
}

/// A discrete two-state cycle -- `values="1;0" keyTimes="0;0.5"` --
/// read at `at`: true during the first half of every `period` since
/// the origin. Frame 0 is lit, so a trace whose element is drawn in
/// its lit state animates this way without moving its frame 0.
pub fn blink(period: Duration, at: Instant) -> bool {
    blink_since(period, at.saturating_duration_since(origin()))
}

fn blink_since(period: Duration, elapsed: Duration) -> bool {
    let period = period.as_micros().max(1);
    (elapsed.as_micros() % period) * 2 < period
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lit_for_the_first_half_of_each_period() {
        let p = Duration::from_millis(1200);
        assert!(blink_since(p, Duration::from_millis(0)));
        assert!(blink_since(p, Duration::from_millis(599)));
        assert!(!blink_since(p, Duration::from_millis(600)));
        assert!(!blink_since(p, Duration::from_millis(1199)));
        assert!(blink_since(p, Duration::from_millis(1200)));
        assert!(!blink_since(p, Duration::from_millis(1800)));
        // The seconds frame.sh was checked at.
        assert!(blink_since(CARET_BLINK, Duration::from_millis(300)));
        assert!(!blink_since(CARET_BLINK, Duration::from_millis(900)));
    }

    #[test]
    fn origin_is_frame_zero() {
        assert!(blink(CARET_BLINK, origin()));
        assert!(now() >= origin());
    }
}
