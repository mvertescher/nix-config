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
use crate::widgets::surface::{backdrop, surface, Surface};
use crate::widgets::text;
use iced::widget::image;
use iced::widget::{container, mouse_area, row, Space};
use iced::{mouse, Element, Length, Padding};

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

/// One tray icon, as StatusNotifierItem describes it.
///
/// Deliberately not the protocol's own shape. `IconName`,
/// `IconPixmap`, `AttentionIconName`, `OverlayIconName` and
/// `IconThemePath` are five ways of saying one thing -- *these pixels*
/// -- and resolving between them means an icon-theme lookup, a PNG
/// decoder and an SVG rasteriser. All of that is the binary's problem
/// (`examples/bar/tray.rs`, `examples/bar/icon.rs`); what arrives here
/// is already decoded, already composited with its overlay, and
/// already the right size, so `bar()` stays a pure function of what it
/// is handed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrayItem {
    /// Four characters of the item's own name, drawn when no icon
    /// could be resolved. Not a placeholder for one that is coming:
    /// plenty of items name an icon no installed theme has, and a cell
    /// that says `SYNC` is worth more than a blank one.
    pub label: String,
    /// The resolved icon, RGBA, or `None` to fall back to [`label`].
    ///
    /// [`label`]: TrayItem::label
    pub icon: Option<image::Handle>,
    /// The item is asking to be looked at (`Status = NeedsAttention`).
    /// The host has already swapped in `AttentionIcon*` where the item
    /// offered one; this is what makes the *cell* shout as well.
    pub attention: bool,
}

/// What a pointer did to a tray cell.
///
/// Named for the protocol methods rather than for the buttons, because
/// the mapping from button to method is the host's convention and the
/// method names are the fixed part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayAction {
    /// `Activate`. Left button, the usual "open this application".
    Activate,
    /// `SecondaryActivate`. Middle button.
    ///
    /// Wired, and unreachable on a layer surface today:
    /// `iced_layershell` 0.13.7 maps every Wayland button code except
    /// `BTN_RIGHT` to `mouse::Button::Left` (`src/event.rs`), so a
    /// middle click on `cyberpunk-ui-bar` arrives as
    /// [`Activate`](TrayAction::Activate). Left wired rather than
    /// removed because `bar()` is not a layer-shell function -- it is
    /// correct in an ordinary window today, and correct everywhere once
    /// that map grows a third arm.
    Secondary,
    /// `ContextMenu`. Right button.
    Context,
    /// `Scroll` along the vertical axis, in whole wheel detents, never
    /// zero.
    ///
    /// The sign is whatever the host's scroll event carried, passed
    /// through rather than normalised, and it is the one thing here
    /// that has *not* been checked against a running item: no
    /// StatusNotifierItem on this desktop acts on `Scroll`, and the
    /// virtual-pointer tools available for driving one produce the same
    /// sign in both directions.
    ///
    /// An earlier version of this comment claimed the two hosts of
    /// `bar()` disagree, on the grounds that `iced_layershell` forwards
    /// the raw `wl_pointer` axis. **That is wrong.** 0.13.7's
    /// `src/event.rs` negates on all four paths -- `-vertical.discrete`,
    /// `-vertical.absolute` and the horizontal pair -- so it already
    /// matches iced's own `ScrollDelta` convention, positive away from
    /// the user. What remains unverified is only whether a real item
    /// interprets a detent the way we send it.
    Scroll(i32),
}

/// One row of an item's `com.canonical.dbusmenu`.
///
/// Deliberately not the protocol's own shape, for the same reason
/// [`TrayItem`] is not: a menu row over that interface is a numeric id
/// and a bag of thirty optional properties, of which a bar draws four.
/// Reading them is the host's problem (`examples/bar/tray.rs`); what
/// arrives here is already a row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuEntry {
    /// The dbusmenu id, sent back with `Event` when the row is
    /// clicked. Meaningless to the bar and never displayed.
    pub id: i32,
    pub label: String,
    /// `enabled = false`. Drawn in the quiet ink and not clickable,
    /// rather than hidden: an application greys a row to say the
    /// command exists and is not available now, and dropping it would
    /// say something else.
    pub enabled: bool,
    pub kind: MenuKind,
}

/// What kind of row it is. The protocol's `type`, `toggle-type`,
/// `toggle-state` and `children-display` collapse to this, because
/// those are four ways of describing one thing: what the row does when
/// it is clicked and what it looks like before it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuKind {
    /// A plain command.
    Command,
    /// A rule between groups. Carries no id worth dispatching.
    Separator,
    /// A checkmark or a radio, and whether it is set. Drawn as the
    /// era's own *selection* rather than as a tick glyph: selection is
    /// the thing the era vocabulary already has four opinions about,
    /// and a `[x]` would be the same box in all four.
    Toggle(bool),
    /// A row with children. Marked and inert -- see [`TrayMenu`].
    Submenu,
}

/// An item's context menu, as far as this bar draws one.
///
/// Flat on purpose. `GetLayout` will return the whole tree, and a row
/// whose `children-display` is `submenu` is drawn with a marker and
/// does nothing when clicked, because opening it means a second
/// surface stacked on the first one and a grab this crate has no way
/// to take. Marked-and-inert rather than hidden: an item whose entire
/// menu is one submenu should look like a menu that will not open, not
/// like an empty one.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TrayMenu {
    pub entries: Vec<MenuEntry>,
}

/// Characters of a menu label drawn before it is clipped.
///
/// An application chooses its own labels and some of them are
/// sentences; the panel is placed against the pointer, so one long row
/// would push the whole menu off the side of the screen rather than
/// just looking untidy.
const MENU_LABEL_CHARS: usize = 40;

/// How wide the menu panel is drawn, in pixels.
///
/// Public for the same reason [`icon_size`] is: whoever places the
/// panel has to know how wide it will be, and the alternative is two
/// constants that agree until one of them is edited. Measured from the
/// labels rather than filled to the content, because a menu that
/// resizes itself per era's `Corner` is one that lands somewhere
/// different per era.
pub fn menu_width(style: &Style, menu: &TrayMenu) -> f32 {
    let per_char = style.metrics.text_body as f32 * 0.58;
    let widest = menu
        .entries
        .iter()
        .map(|entry| entry.label.chars().count().min(MENU_LABEL_CHARS))
        .max()
        .unwrap_or(0);
    // The floor is what keeps a menu of `OK` from being a stamp; the
    // ceiling is `MENU_LABEL_CHARS` worth of the largest era's body
    // text, so the clip above is what bounds this and not the clamp.
    ((widest as f32 * per_char).ceil() + 24.0).clamp(140.0, 460.0)
}

/// One menu row.
fn menu_row<'a, Message: Clone + 'static>(
    style: &Style,
    entry: &MenuEntry,
    on_entry: fn(i32) -> Message,
) -> Element<'a, Message> {
    if let MenuKind::Separator = entry.kind {
        // A rule, not a row: the era's border colour at the stroke
        // width it declares, with air either side.
        let border = style.palette.border;
        return container(
            container(Space::new(
                Length::Fill,
                Length::Fixed(style.metrics.stroke),
            ))
            .style(move |_: &iced::Theme| container::Style {
                background: Some(border.into()),
                ..container::Style::default()
            })
            .width(Length::Fill),
        )
        .padding(Padding::from([4, 6]))
        .width(Length::Fill)
        .into();
    }

    let label = clip(&entry.label, MENU_LABEL_CHARS);
    let selected = matches!(entry.kind, MenuKind::Toggle(true));

    let text = if selected {
        text::on_select(style, label)
    } else if entry.enabled {
        text::body(style, label)
    } else {
        text::body(style, label).color(style.palette.dim)
    };

    // The marker is the one place a row says something the label does
    // not: that clicking it would have opened something this bar
    // cannot open.
    let trailing: Element<'a, Message> = match entry.kind {
        MenuKind::Submenu => text::mid(style, ">").into(),
        _ => Space::new(Length::Shrink, Length::Shrink).into(),
    };

    let inner = row![text, Space::new(Length::Fill, Length::Shrink), trailing]
        .align_y(iced::Alignment::Center)
        .width(Length::Fill);

    // `backdrop` rather than `surface`: a bar cell pins its own height
    // and a menu row does not, and a `surface` whose caller does not
    // pin one grows to whatever the column offers it.
    let face: Element<'a, Message> = if selected {
        backdrop(Surface::selected(style), Padding::from([3, 8]), inner)
    } else {
        container(inner).padding(Padding::from([3, 8])).into()
    };

    // A disabled row and a submenu row are both drawn and neither
    // answers: sending `clicked` to a row an application has greyed is
    // asking it to do something it just said it would not do, and
    // sending it to a submenu parent is asking for a menu we have
    // nowhere to put.
    if !entry.enabled || matches!(entry.kind, MenuKind::Submenu) {
        return face;
    }

    mouse_area(face)
        .on_press(on_entry(entry.id))
        .interaction(mouse::Interaction::Pointer)
        .into()
}

/// An item's context menu, in the era's own dress.
///
/// Filled rather than outlined, unlike every other surface on the bar:
/// this is the one thing the bar draws that floats over other
/// applications, and an unfilled panel would show a terminal through
/// its own rows.
pub fn tray_menu<'a, Message: Clone + 'static>(
    style: &Style,
    menu: &TrayMenu,
    on_entry: fn(i32) -> Message,
) -> Element<'a, Message> {
    let mut rows = iced::widget::column![].width(Length::Fill);
    for entry in &menu.entries {
        rows = rows.push(menu_row(style, entry, on_entry));
    }

    container(backdrop(
        Surface::filled(style, style.palette.bg).stroke(style.palette.border),
        Padding::from([6, 4]),
        rows,
    ))
    .width(Length::Fixed(menu_width(style, menu)))
    .height(Length::Shrink)
    .into()
}

/// How the bar reports a pointer event on a tray cell to its host.
///
/// `usize` indexes [`Readings::tray`]. `None` at the call site means
/// the bar draws tray cells but does not listen -- which is what
/// `cyberpunk-ui-bar-window` wants, since a golden of a still life
/// should not carry hit-testing it never exercises.
///
/// A bare function pointer rather than a closure: the caller is a
/// message constructor and has nothing to capture, and a `&dyn Fn`
/// would have to outlive the `Element`, which in an iced `view` means
/// finding somewhere `'static` to keep a closure that does nothing.
pub type OnTray<Message> = fn(usize, TrayAction) -> Message;

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
    /// Empty when there is no tray, no session bus, or nothing has
    /// registered an icon. All three are ordinary states and all three
    /// draw nothing, so `Vec` rather than `Option<Vec>`: there is no
    /// fourth case for the bar to tell apart.
    pub tray: Vec<TrayItem>,
    pub network: Network,
    pub clock: String,
    pub date: String,
}

/// Width a label needs, in pixels.
///
/// A [`Surface`] paints the box it is handed and its canvas fills
/// whatever space it is given, so in a shrink-width row the cells
/// collapse and clip their own text -- a five-character hostname came
/// out as three.
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

/// How large a tray icon is drawn, in pixels.
///
/// Derived from the bar's own height rather than fixed, so an era that
/// declares a taller bar gets larger icons instead of a small one
/// floating in a big cell. The margin is the cell's stroke, its 2px of
/// vertical padding and the bar's own 3px, doubled.
///
/// Public because whoever gathers the readings has to decode icons at
/// the size they will be drawn, and the alternative is two constants
/// that agree until one of them is edited.
pub fn icon_size(style: &Style) -> f32 {
    (style.bar.height as f32 - 12.0).clamp(10.0, 24.0)
}

/// One tray item drawn as its icon.
///
/// The icon is handed over already decoded and composited, so all this
/// decides is the box around it -- which is the whole argument for
/// having built this bar: a tray icon is a [`cell`], so it wears the
/// era's corner for free.
fn icon_cell<'a, Message: 'static>(
    style: &Style,
    icon: &image::Handle,
    attention: bool,
) -> Element<'a, Message> {
    let size = icon_size(style);

    // An item shouting has usually swapped its own icon for
    // `AttentionIcon*` already, but many define none -- so the cell
    // says it too, in the era's published `alert` role, the same ink
    // `alert_cell` moves for a muted sink.
    let bg = if attention {
        Surface::outlined(style).stroke(style.palette.alert)
    } else {
        Surface::outlined(style)
    };

    container(surface(
        bg,
        Padding::from([2, 6]),
        container(
            image(icon.clone())
                .width(Length::Fixed(size))
                .height(Length::Fixed(size))
                // Nearest would alias a 22px item icon scaled to 14;
                // these are photographs as far as the bar is concerned.
                .filter_method(image::FilterMethod::Linear),
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill),
    ))
    .width(Length::Fixed(size + 18.0))
    .height(Length::Fill)
    .into()
}

/// One tray item, and the pointer events it answers to.
fn tray_cell<'a, Message: Clone + 'static>(
    style: &Style,
    item: &TrayItem,
    index: usize,
    on_tray: Option<OnTray<Message>>,
) -> Element<'a, Message> {
    let face = match &item.icon {
        Some(icon) => icon_cell(style, icon, item.attention),
        // Clipped for the same reason the SSID is: a label is whatever
        // the application chose to call itself, and one long one must
        // not push the clock off the screen.
        None => {
            let label = clip(&item.label, 6);
            if item.attention {
                alert_cell(style, label)
            } else {
                cell(style, label, false)
            }
        }
    };

    let Some(on_tray) = on_tray else {
        return face;
    };

    mouse_area(face)
        .on_press(on_tray(index, TrayAction::Activate))
        .on_middle_press(on_tray(index, TrayAction::Secondary))
        .on_right_press(on_tray(index, TrayAction::Context))
        .on_scroll(move |delta| on_tray(index, TrayAction::Scroll(detents(delta))))
        // The one cell on the bar that does anything, so it is the one
        // cell that should look like it does.
        .interaction(mouse::Interaction::Pointer)
        .into()
}

/// A wheel movement as whole detents, keeping the delta's own sign.
///
/// Both of iced's deltas are folded into one number because the
/// protocol's `Scroll` takes an integer and an axis, not a pixel
/// count. A pixel delta comes from a touchpad, where 15px is the
/// conventional detent; rounding away from zero means a small flick
/// still counts as one, which is what the item is waiting to be told.
/// Zero stays zero, and the host is expected to drop it rather than
/// tell an application that nothing happened.
fn detents(delta: mouse::ScrollDelta) -> i32 {
    let lines = match delta {
        mouse::ScrollDelta::Lines { y, .. } => y,
        mouse::ScrollDelta::Pixels { y, .. } => y / 15.0,
    };
    if lines == 0.0 {
        0
    } else {
        lines.abs().ceil().copysign(lines) as i32
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
///
/// `on_tray` is how a clicked tray cell gets back to whoever can talk
/// to the item; `None` draws the same bar with no hit-testing at all.
/// It is a parameter rather than a field on [`Readings`] because it is
/// the one thing here that is not a *reading* -- the readings are what
/// the machine is doing, and this is what the bar should do about it.
pub fn bar<'a, Message: Clone + 'static>(
    style: &Style,
    r: &'a Readings,
    on_tray: Option<OnTray<Message>>,
) -> Element<'a, Message> {
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
    // Tray first, so the modules that are always present keep a fixed
    // distance from the right edge; an application starting up should
    // not move the clock.
    for (index, item) in r.tray.iter().enumerate() {
        right = right.push(tray_cell(style, item, index, on_tray));
    }
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
