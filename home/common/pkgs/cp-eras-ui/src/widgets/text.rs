//! Text at the four sizes the reference screens actually use.
//!
//! Sizes come from [`crate::style::Metrics`] rather than call sites, so
//! an era that runs its captions smaller does it once.

use crate::style::Style;
use iced::widget::text::IntoFragment;
use iced::widget::{text, Text};
use iced::Color;

fn base<'a>(content: impl IntoFragment<'a>, size: u16, color: Color) -> Text<'a> {
    text(content).size(f32::from(size)).color(color)
}

pub fn title<'a>(style: &Style, content: impl IntoFragment<'a>) -> Text<'a> {
    base(content, style.metrics.text_title, style.palette.fg)
}

pub fn body<'a>(style: &Style, content: impl IntoFragment<'a>) -> Text<'a> {
    base(content, style.metrics.text_body, style.palette.fg)
}

/// Tertiary text: present but deliberately receding. Meta labels, a
/// card's class line, the internal rules of a list.
pub fn label<'a>(style: &Style, content: impl IntoFragment<'a>) -> Text<'a> {
    base(content, style.metrics.text_body, style.palette.dim)
}

/// The ink between `dim` and `fg`.
///
/// The published role vocabulary has one quiet colour; the reference
/// screens use two, and collapsing them is what made entropism's store
/// screen unreadable. In `docs/entropism/target-app.svg` the tertiary
/// strings -- meta labels, footnote bodies, the compliance caption --
/// are `#3d4d38`, which is `dim`; but the *structural* small text --
/// segmented bar labels, `S T O R E`, the DPS/PNT/ACC/ROF heads and
/// every `EMPTY SOCKET` -- is `#728f76`, a full stop brighter. On that
/// era's ground `dim` measures 2.1:1 and `#728f76` measures 5.5:1, so
/// drawing the second set in the first colour is not a shade of wrong,
/// it is illegible.
///
/// Derived rather than published, because the eras disagree about where
/// it sits and none of them treats it as a colour of its own: kitsch and
/// neokitsch draw both of those sets at `fg`, and entropism at 60% of
/// the way from `dim` to `fg` -- which is `#718f6f` against a sampled
/// `#728f76`. So 0.6 is the entropism reading, and it lands the
/// maximalist eras a little under theirs, which is the safe direction.
pub fn mid_ink(style: &Style) -> Color {
    crate::style::Ink::Mid.of(&style.palette)
}

/// Secondary text, in [`mid_ink`]: small, structural, and still meant
/// to be read.
pub fn mid<'a>(style: &Style, content: impl IntoFragment<'a>) -> Text<'a> {
    base(content, style.metrics.text_body, mid_ink(style))
}

/// The tiny maintenance strings the references are covered in --
/// compliance notices, build numbers, "SERVING CUSTOMERS SINCE 2006".
pub fn caption<'a>(style: &Style, content: impl IntoFragment<'a>) -> Text<'a> {
    base(content, style.metrics.text_caption, style.palette.dim)
}

/// Text drawn on top of a selected surface.
pub fn on_select<'a>(style: &Style, content: impl IntoFragment<'a>) -> Text<'a> {
    base(content, style.metrics.text_body, style.palette.on_select)
}
