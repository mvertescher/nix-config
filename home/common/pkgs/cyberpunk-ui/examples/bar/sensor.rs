//! The primitive the bar's background sensors publish through.
//!
//! Two of the bar's modules cannot be read synchronously on the tick.
//! PulseAudio wants a mainloop of its own, and asking a wireless driver
//! for an SSID means forking a process; either would put an unbounded
//! wait in front of a frame. So each of those sensors owns a thread,
//! writes its latest reading here, and the bar takes whatever is
//! present when it draws.
//!
//! The contract that makes this safe is [`Latest::get`]: it never
//! blocks. If the sensor thread happens to hold the lock at that
//! instant, or has died holding it, the bar redraws the previous
//! reading and tries again a second later. A bar is a display of
//! recent facts, not a transaction.

use std::sync::{Arc, Mutex, TryLockError};

/// The sensor's end: a slot it overwrites whenever it learns something.
pub struct Snapshot<T>(Arc<Mutex<T>>);

// Derived `Clone` would demand `T: Clone`, which is not what is being
// cloned -- the handle is.
impl<T> Clone for Snapshot<T> {
    fn clone(&self) -> Self {
        Snapshot(Arc::clone(&self.0))
    }
}

impl<T> Snapshot<T> {
    pub fn new(initial: T) -> Self {
        Snapshot(Arc::new(Mutex::new(initial)))
    }

    /// Publish a reading. Recovers from poisoning rather than
    /// propagating it: a panic in one probe should cost that reading,
    /// not every reading after it.
    pub fn set(&self, value: T) {
        let mut slot = self
            .0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        *slot = value;
    }
}

/// The bar's end: a non-blocking view of a [`Snapshot`], holding the
/// last value it managed to see.
pub struct Latest<T> {
    shared: Snapshot<T>,
    last: T,
}

impl<T: Clone> Latest<T> {
    pub fn new(shared: Snapshot<T>, initial: T) -> Self {
        Latest {
            shared,
            last: initial,
        }
    }

    /// The most recent reading available *without waiting*.
    pub fn get(&mut self) -> T {
        match self.shared.0.try_lock() {
            Ok(slot) => self.last = slot.clone(),
            // The sensor thread panicked mid-write. The value is still
            // whatever it last wrote, and refusing to look at it for
            // the rest of the session helps nobody.
            Err(TryLockError::Poisoned(poisoned)) => self.last = poisoned.into_inner().clone(),
            // Contended for the microsecond of an assignment. Skip this
            // frame's update rather than stall it.
            Err(TryLockError::WouldBlock) => {}
        }
        self.last.clone()
    }
}
