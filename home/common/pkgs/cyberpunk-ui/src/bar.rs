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

/// The default audio sink, as the sound server reports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Audio {
    /// Whole percent of the server's notion of normal volume. May
    /// exceed 100: PulseAudio allows amplification, and a bar that
    /// clamps it silently is lying about the state of the machine.
    pub volume: u16,
    pub muted: bool,
}

/// How the machine reaches the outside world.
///
/// `Unknown` and `Offline` are deliberately not the same variant. A bar
/// that announces "offline" because its first probe has not landed yet
/// is worse than one that shows nothing for a second, so `Unknown`
/// draws no module at all.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Network {
    #[default]
    Unknown,
    /// Nothing holds a default route.
    Offline,
    Wired {
        interface: String,
    },
    /// `ssid` is empty when the name was not cheaply available; the
    /// interface stands in for it rather than the module vanishing.
    Wireless {
        interface: String,
        ssid: String,
    },
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
    /// `None` when no sound server answered. A machine without one is a
    /// normal state rather than a failure, so the module leaves the row
    /// instead of showing a zero it cannot vouch for.
    pub audio: Option<Audio>,
    pub network: Network,
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

/// A module whose reading is a warning: the sink is muted, or there is
/// no route out. Same silhouette as [`cell`] -- only the ink moves, to
/// the era's published `alert` role, so this stays era-agnostic and a
/// fifth era gets its own idea of alarm for free.
fn alert_cell<'a, Message: 'static>(
    style: &Style,
    label: impl Into<String>,
) -> Element<'a, Message> {
    let label: String = label.into();
    let width = width_for(style, &label);

    container(surface(
        Surface::outlined(style),
        Padding::from([2, 10]),
        text::body(style, label).color(style.palette.alert),
    ))
    .width(Length::Fixed(width))
    .height(Length::Fill)
    .into()
}

/// Clip to `max` characters, counting characters rather than bytes so a
/// non-ASCII SSID does not get cut mid-codepoint.
fn clip(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_string();
    }
    let head: String = text.chars().take(max.saturating_sub(1)).collect();
    format!("{head}…")
}

/// Volume and mute in one module.
///
/// `VOL` and `MUT` rather than a speaker glyph: the icon set is still a
/// to-do, and three characters either way means the cell keeps its
/// width when the sink is muted, so nothing to its left reflows.
fn audio_cell<'a, Message: 'static>(style: &Style, audio: &Audio) -> Element<'a, Message> {
    // Three digits covers PulseAudio's amplification range without the
    // cell growing; past that the number is not the interesting fact.
    let volume = audio.volume.min(999);

    if audio.muted {
        alert_cell(style, format!("MUT {volume:>3}%"))
    } else {
        cell(style, format!("VOL {volume:>3}%"), false)
    }
}

/// The route out, or nothing at all while it is still unknown.
fn network_cell<'a, Message: 'static>(
    style: &Style,
    network: &Network,
) -> Option<Element<'a, Message>> {
    match network {
        Network::Unknown => None,
        Network::Offline => Some(alert_cell(style, "NET --")),
        Network::Wired { interface } => {
            Some(cell(style, format!("NET {}", clip(interface, 12)), false))
        }
        Network::Wireless { interface, ssid } => {
            let name = if ssid.is_empty() { interface } else { ssid };
            Some(cell(style, format!("WIFI {}", clip(name, 16)), false))
        }
    }
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

    // Built by pushing rather than as a literal, because the audio and
    // network modules are absent -- not blank -- when their subsystem
    // has nothing to say.
    let mut right = row![].spacing(gap).height(Length::Fill);
    if let Some(network) = network_cell(style, &r.network) {
        right = right.push(network);
    }
    if let Some(audio) = &r.audio {
        right = right.push(audio_cell(style, audio));
    }
    let right = right
        .push(cell(style, format!("CPU {:>2}%", r.cpu), false))
        .push(cell(style, format!("MEM {:>2}%", r.memory), false))
        .push(cell(style, r.date.as_str(), false))
        .push(cell(style, r.clock.as_str(), true));

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
