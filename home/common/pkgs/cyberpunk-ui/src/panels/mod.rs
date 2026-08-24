//! Screens that are era-agnostic but not yet in the golden matrix.
//!
//! Same contract as [`crate::screens`] -- a screen takes a
//! [`crate::style::Style`] and never asks which era it is -- and the
//! same shape (`new`/`title`/`update`/`view`).
//!
//! Only `mail` is left here, and it is not waiting to move: it is the
//! interactive counterpart to the display-only [`crate::screens::mail`]
//! -- selection, focus, scrolling and deletion wired up -- rather than
//! a second copy of it. `dashboard` was the one that was merely in the
//! wrong place, and it now sits in [`crate::screens`] beside `store`,
//! `login` and `mailbox`, where `tests.dashboard.<era>` covers it.
//!
//! Everything here used to hardcode the neo-militarism palette. None of
//! it does now: `crate::colors` is gone, and so is its last reader,
//! `widgets::message_card` -- an unreachable mail card whose geometry
//! predated the era generalisation. `mail`'s own `message_row` draws
//! that row from [`crate::widgets::surface::Surface`], so it wears each
//! era's corner and selection idiom rather than neomil's chamfer.

pub mod mail;

pub use mail::{mail_panel, Email, MailFocus, ThreadMessage};
