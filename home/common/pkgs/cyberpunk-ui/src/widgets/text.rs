//! Text at the four sizes the reference screens actually use.
//!
//! Sizes come from [`crate::style::Metrics`] rather than call sites, so
//! an era that runs its captions smaller does it once.

use crate::style::Style;
use iced::widget::text::IntoFragment;
use iced::widget::{text, Text};
use iced::Color;

fn base<'a>(content: impl IntoFragment<'a>, size: u16, color: Color) -> Text<'a> {
    text(content).size(size).color(color)
}

pub fn title<'a>(style: &Style, content: impl IntoFragment<'a>) -> Text<'a> {
    base(content, style.metrics.text_title, style.palette.fg)
}

pub fn body<'a>(style: &Style, content: impl IntoFragment<'a>) -> Text<'a> {
    base(content, style.metrics.text_body, style.palette.fg)
}

/// Secondary text: present but deliberately receding.
pub fn label<'a>(style: &Style, content: impl IntoFragment<'a>) -> Text<'a> {
    base(content, style.metrics.text_body, style.palette.dim)
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
