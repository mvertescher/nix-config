//! Full screens assembled from the shared widget vocabulary.
//!
//! A screen is era-agnostic by construction: it takes a [`crate::style::
//! Style`] and never asks which era it is. That constraint is the point
//! -- it is what makes "four eras, one toolkit" a testable claim rather
//! than an aspiration.
//!
//! Two of the four -- the store and the dashboard -- are trace-driven:
//! the era table holds a `&'static [Prim]` scene transcribed from the
//! trace and [`scene`] is the one renderer that paints it.

pub mod dashboard;
pub mod login;
pub mod hub;
pub mod mail;
pub mod nav;
pub mod scene;
pub mod soft;
pub mod store;

pub use dashboard::Dashboard;
pub use hub::Hub;
pub use login::Login;
pub use mail::MailBox;
pub use store::Store;
