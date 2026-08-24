//! Screens that are era-agnostic but not yet in the golden matrix.
//!
//! Same contract as [`crate::screens`] -- a screen takes a
//! [`crate::style::Style`] and never asks which era it is -- and the
//! same shape (`new`/`title`/`update`/`view`). The split is temporary:
//! `dashboard` belongs beside `store`, `login` and `mailbox` and moves
//! there once `tests.dashboard.<era>` is wired, and `mail` is the
//! interactive counterpart to the display-only [`crate::screens::mail`]
//! rather than a second copy of it.
//!
//! Everything here used to hardcode the neo-militarism palette. What is
//! left of that is noted where it survives -- `widgets::message_card`
//! still resolves its selected-row ink through `crate::colors`.

pub mod dashboard;
pub mod mail;

pub use dashboard::Dashboard;
pub use mail::{mail_panel, Email, MailFocus, ThreadMessage};
