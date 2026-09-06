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
//!   are captured at [`REST`] -- the trace at rest is what they hold,
//!   and a golden that depends on when the compositor got round to
//!   the capture is not a golden.
//!
//! What is here is what the vocabulary in `docs/PIPELINE.md` needs and
//! iced's `Animation` does not give: the discrete cycle
//! (`calcMode="discrete"`), which is a phase and not an interpolation,
//! and the one-shot eased transition ([`progress`]) read off the same
//! clock, so a scene table can carry a `<animate>` as data
//! ([`Motion`](crate::style::Motion)) instead of each screen keeping an
//! `iced::animation::Animation` per moving thing. lilt's `Easing`
//! is the curve, as the vocabulary table says.

use crate::style::Motion;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

/// How the login traces blink their caret: `docs/<era>/login-trace.svg`
/// `#caret-blink`, `values="1;0" keyTimes="0;0.5" calcMode="discrete"
/// dur="1.2s" repeatCount="indefinite"` -- lit for the first 600 ms of
/// every 1.2 s, counted from the document's begin.
pub const CARET_BLINK: Duration = Duration::from_millis(1200);

/// The moment the traces are at rest: every boot-in has frozen at its
/// `to` and every cycle is at its frame-0 phase. This is the frame the
/// static pipeline sees -- rsvg plays no SMIL, so a trace draws each
/// moving element at the value its animation freezes at, and a frame
/// of the app at `REST` is what the goldens and `render.sh` capture by
/// default (`docs/PIPELINE.md`, "The rest frame is the trace").
///
/// 2.4 s: after the longest boot-in any trace annotates, with room,
/// and two whole [`CARET_BLINK`]s so the caret is lit here as it is
/// at 0. Lengthen it in that step if a trace ever needs longer.
pub const REST: Duration = Duration::from_millis(2400);

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

/// How far along a one-shot transition is at `elapsed` since the
/// origin, 0..=1 through its easing: 0 (its `from`) until `begin`, its
/// eased fraction through `dur`, and 1 (its `to`) ever after, which is
/// `fill="freeze"`. A hold before `begin` is SMIL's `begin="0.4s"` and
/// lilt's `.delay(..)`: the value stays at `from` until then.
pub fn progress(motion: &Motion, elapsed: Duration) -> f32 {
    let begin = Duration::from_millis(u64::from(motion.begin));
    let dur = Duration::from_millis(u64::from(motion.dur));
    let Some(into) = elapsed.checked_sub(begin) else { return 0.0 };
    if dur.is_zero() || into >= dur {
        return 1.0;
    }
    motion.ease.value(into.as_secs_f32() / dur.as_secs_f32())
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

    /// `#panel-open` as neomil's dashboard table carries it: at `from`
    /// before it begins, frozen at `to` after, eased between -- and at
    /// REST, done, which is what makes the rest frame the trace.
    #[test]
    fn a_transition_holds_then_eases_then_freezes() {
        use crate::style::{Change, Motion};
        use iced::animation::Easing;
        let open = Motion {
            id: "panel-open",
            begin: 100,
            dur: 360,
            ease: Easing::EaseOutCubic,
            change: Change::Clip { x: 0.0, y: 0.0, w: (1.0, 1.0), h: (0.0, 1.0) },
        };
        let ms = Duration::from_millis;
        assert_eq!(progress(&open, ms(0)), 0.0);
        assert_eq!(progress(&open, ms(99)), 0.0);
        assert_eq!(progress(&open, ms(100)), 0.0);
        let mid = progress(&open, ms(250));
        // EaseOutCubic at 150/360: 1 - (1 - 0.4167)^3.
        assert!((mid - 0.8016).abs() < 0.01, "{mid}");
        assert_eq!(progress(&open, ms(460)), 1.0);
        assert_eq!(progress(&open, REST), 1.0);
        assert!(REST > ms(460));
    }

    /// The caret is lit at REST as it is at 0, so the login's rest
    /// frame is its frame 0.
    #[test]
    fn rest_is_a_whole_number_of_blinks() {
        assert_eq!(REST.as_millis() % CARET_BLINK.as_millis(), 0);
        assert!(blink_since(CARET_BLINK, REST));
    }

    #[test]
    fn origin_is_frame_zero() {
        assert!(blink(CARET_BLINK, origin()));
        assert!(now() >= origin());
    }
}
