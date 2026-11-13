//! Which of the bar's own surfaces are up, and how long it has been
//! without one.
//!
//! The bar runs in `StartMode::AllScreens`, so `layershellev` puts a
//! bar on every output as it is announced and does not end the event
//! loop when the last surface goes (see the module header of the
//! binary). That is what lets an output be removed and re-added without
//! the bar going with it. It also means that a surface the compositor
//! closes for some other reason, its output still present, would leave
//! a process running with nothing on screen and nothing to bring it
//! back -- the unit restarts the bar on exit, not on absence. So a bar
//! that has had no surface for [`GRACE`] after losing its last one
//! gives up, and the restart takes it from there.
//!
//! A process that has never had a surface does not give up: that is a
//! bar started with no output, waiting for one, which is right.

use std::time::{Duration, Instant};

/// How long to wait for a replacement surface before exiting. An output
/// re-handshake re-announces the output within the same second, so this
/// is generous; it only has to be shorter than a person's patience.
pub const GRACE: Duration = Duration::from_secs(10);

/// The bar surfaces currently up, by window id. Generic so the tests
/// need no iced runtime; the binary uses `iced::window::Id`.
#[derive(Debug)]
pub struct Surfaces<I> {
    live: Vec<I>,
    /// When the last live surface closed, if none has opened since.
    barless_since: Option<Instant>,
}

impl<I: PartialEq> Surfaces<I> {
    pub fn new() -> Self {
        Surfaces {
            live: Vec::new(),
            barless_since: None,
        }
    }

    /// A bar surface was drawn for the first time.
    pub fn opened(&mut self, id: I) {
        if !self.live.contains(&id) {
            self.live.push(id);
        }
        self.barless_since = None;
    }

    /// The compositor closed a surface. Ids that are not a live bar --
    /// the tray menu's, or one already forgotten -- change nothing.
    pub fn closed(&mut self, id: &I, now: Instant) {
        let before = self.live.len();
        self.live.retain(|live| live != id);
        if self.live.len() < before && self.live.is_empty() {
            self.barless_since = Some(now);
        }
    }

    /// Whether the bar has been without a surface long enough that no
    /// replacement is coming.
    pub fn given_up(&self, now: Instant) -> bool {
        self.barless_since
            .is_some_and(|since| now.saturating_duration_since(since) >= GRACE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bar_that_never_had_a_surface_waits() {
        let surfaces = Surfaces::<u32>::new();
        let later = Instant::now() + GRACE * 100;
        assert!(!surfaces.given_up(later));
    }

    #[test]
    fn an_output_re_added_in_time_keeps_the_bar() {
        let t0 = Instant::now();
        let mut surfaces = Surfaces::new();
        surfaces.opened(1);
        surfaces.closed(&1, t0);
        assert!(!surfaces.given_up(t0 + GRACE / 2));
        // The library put a new surface on the re-announced output.
        surfaces.opened(2);
        assert!(!surfaces.given_up(t0 + GRACE * 10));
    }

    #[test]
    fn no_replacement_within_grace_gives_up() {
        let t0 = Instant::now();
        let mut surfaces = Surfaces::new();
        surfaces.opened(1);
        surfaces.closed(&1, t0);
        assert!(!surfaces.given_up(t0 + GRACE - Duration::from_millis(1)));
        assert!(surfaces.given_up(t0 + GRACE));
    }

    #[test]
    fn closing_something_that_is_not_a_bar_changes_nothing() {
        let t0 = Instant::now();
        let mut surfaces = Surfaces::new();
        surfaces.opened(1);
        // The tray menu's surface, or a stale id.
        surfaces.closed(&7, t0);
        assert!(!surfaces.given_up(t0 + GRACE * 10));
        // And on a bar with no surfaces, it does not arm the clock.
        surfaces.closed(&1, t0);
        surfaces.opened(2);
        surfaces.closed(&9, t0);
        assert!(!surfaces.given_up(t0 + GRACE * 10));
    }

    #[test]
    fn one_output_of_two_going_away_is_not_barless() {
        let t0 = Instant::now();
        let mut surfaces = Surfaces::new();
        surfaces.opened(1);
        surfaces.opened(2);
        surfaces.closed(&1, t0);
        assert!(!surfaces.given_up(t0 + GRACE * 10));
        surfaces.closed(&2, t0);
        assert!(surfaces.given_up(t0 + GRACE));
    }

    #[test]
    fn opening_the_same_surface_twice_counts_once() {
        let t0 = Instant::now();
        let mut surfaces = Surfaces::new();
        surfaces.opened(1);
        surfaces.opened(1);
        surfaces.closed(&1, t0);
        assert!(surfaces.given_up(t0 + GRACE));
    }
}
