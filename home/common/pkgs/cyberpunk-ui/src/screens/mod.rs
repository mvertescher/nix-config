//! Full screens assembled from the shared widget vocabulary.
//!
//! A screen is era-agnostic by construction: it takes a [`crate::style::
//! Style`] and never asks which era it is. That constraint is the point
//! -- it is what makes "four eras, one toolkit" a testable claim rather
//! than an aspiration.

pub mod login;
pub mod mail;
pub mod store;

pub use login::Login;
pub use mail::MailBox;
pub use store::Store;
