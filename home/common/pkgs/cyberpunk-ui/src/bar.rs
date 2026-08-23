//! The status bar, in any era.
//!
//! This is the screen that made building our own bar worth it. waybar
//! styles with CSS and ashell with a closed `Islands | Solid | Gradient`
//! enum, and neither can express a chamfered or clipped corner -- so on
//! the one surface that is always visible, two of the four eras could
//! never look like themselves. Here a module is a [`Surface`], so it
//! wears whatever `Corner` the era declares, for free.
//!
//! Everything here is a pure function of [`Style`] and [`Readings`].
//! Collecting the readings is the binary's job, which keeps this file
//! testable and keeps the layer-shell dependency out of the library.

use crate::style::Style;
use crate::widgets::surface::{surface, Surface};
use crate::widgets::text;
use iced::widget::{container, row, Space};
use iced::{Element, Length, Padding};

/// One workspace as the compositor reports it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    pub id: i32,
    pub active: bool,
}

/// Everything the bar draws, already gathered.
#[derive(Debug, Clone, Default)]
pub struct Readings {
    pub host: String,
    pub workspaces: Vec<Workspace>,
    pub window: String,
    /// Whole percent, so the bar never reflows on a decimal.
    pub cpu: u8,
    pub memory: u8,
    pub clock: String,
    pub date: String,
}

/// Width a label needs, in pixels.
///
/// A [`Surface`] paints the box it is handed and its canvas fills
/// whatever space it is given, so in a shrink-width row the cells
/// collapse and clip their own text -- "terra" came out as "ter".
/// Sizing from the label is also the better behaviour for a bar: cells
/// stop reflowing every time CPU% ticks from 9 to 10.
fn width_for(style: &Style, label: &str) -> f32 {
    let per_char = style.metrics.text_body as f32 * 0.58;
    (label.chars().count() as f32 * per_char).ceil() + 26.0
}

/// A bar module: outlined, in the era's own silhouette.
fn cell<'a, Message: 'static>(
    style: &Style,
    label: impl Into<String>,
    filled: bool,
) -> Element<'a, Message> {
    let label: String = label.into();
    let width = width_for(style, &label);

    let bg = if filled {
        Surface::selected(style)
    } else {
        Surface::outlined(style)
    };
    let content = if filled {
        text::on_select(style, label)
    } else {
        text::body(style, label)
    };
    container(surface(bg, Padding::from([2, 10]), content))
        .width(Length::Fixed(width))
        .height(Length::Fill)
        .into()
}

/// The hostname tape at the far left. Uses `tape`, the role that exists
/// precisely for improvised labelling.
fn host_tape<'a, Message: 'static>(style: &Style, host: &'a str) -> Element<'a, Message> {
    container(surface(
        Surface::filled(style, style.palette.tape).no_stroke(),
        Padding::from([2, 12]),
        text::body(style, host).color(style.palette.bg),
    ))
    .width(Length::Fixed(width_for(style, host) + 4.0))
    .height(Length::Fill)
    .into()
}

/// The whole bar. `height` should match `Metrics::bar`, which is also
/// what the layer surface reserves as its exclusive zone.
pub fn bar<'a, Message: 'static>(style: &Style, r: &'a Readings) -> Element<'a, Message> {
    let gap = style.metrics.gap * 0.4;

    let mut left = row![].spacing(gap).height(Length::Fill);
    if style.bar.host_tape && !r.host.is_empty() {
        left = left.push(host_tape(style, &r.host));
    }
    for ws in &r.workspaces {
        left = left.push(cell(style, ws.id.to_string(), ws.active));
    }

    let centre: Element<'a, Message> = if r.window.is_empty() {
        Space::new(Length::Shrink, Length::Shrink).into()
    } else {
        container(text::label(style, r.window.as_str()))
            .center_y(Length::Fill)
            .into()
    };

    let right = row![
        cell(style, format!("CPU {:>2}%", r.cpu), false),
        cell(style, format!("MEM {:>2}%", r.memory), false),
        cell(style, r.date.as_str(), false),
        cell(style, r.clock.as_str(), true),
    ]
    .spacing(gap)
    .height(Length::Fill);

    container(
        row![
            left,
            Space::new(Length::Fill, Length::Shrink),
            centre,
            Space::new(Length::Fill, Length::Shrink),
            right,
        ]
        .align_y(iced::Alignment::Center)
        .height(Length::Fill),
    )
    .padding(Padding::from([3, 6]))
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
