//! The era as an iced theme.
//!
//! iced dresses its built-in widgets through a theme type: each widget
//! has a `Catalog` trait the theme implements, and the theme decides
//! what a `scrollable` or a `button` looks like unless the call site
//! says otherwise. Until 2026-09-05 this crate had no such type. Every
//! element was `Element<'_, Message, iced::Theme>`, so a built-in
//! either took iced's own dark-theme reading or an ad-hoc closure at
//! the call site (`panels::mail` carried one for `scrollable`,
//! `widgets::chrome` one for a `container` rule) -- and the form
//! controls still to come would each have carried their own.
//!
//! The theme type is [`Style`] itself. Not a wrapper: a `Style` *is*
//! an era's palette plus its tables, which is exactly what a theme
//! holds, and making it the theme means a canvas program's `draw`
//! receives the era it should paint against instead of an unused
//! `iced::Theme`. Each `Catalog` here resolves [`Ink`] roles through
//! the palette, so the nix theme layer's overrides
//! ([`crate::palette::Palette::with_roles`]) reach the built-ins the
//! same way they reach the canvas screens.
//!
//! Classes are iced's own `StyleFn` boxes, so `.style(closure)` still
//! works at a call site that has a reading the catalog does not --
//! the closure just gets a `&Style` now. The named functions in the
//! per-widget modules (`scrollable::rail`, ...) are what the defaults
//! point at, and are public so a site can pass a variant the same way
//! iced's `button::secondary` is passed.

use crate::palette::Palette;
use crate::style::{Coat, Era, Ink, Style};
use iced::overlay::menu;
use iced::theme;
use iced::widget::{
    checkbox, container, pick_list, progress_bar, radio, rule, scrollable, slider, text,
    text_input, toggler,
};
use iced::{Border, Color, Shadow};

/// The era a program dresses in when nothing has chosen one: the same
/// fallback [`Style::from_desktop`] takes for an unrecognised theme
/// file, and the era `theme::Theme::fallback` keeps inline.
impl Default for Style {
    fn default() -> Self {
        Era::Neomil.style()
    }
}

impl theme::Base for Style {
    fn default(_preference: theme::Mode) -> Self {
        // Every era is a dark design; there is no light variant to
        // offer a preference.
        <Style as Default>::default()
    }

    fn mode(&self) -> theme::Mode {
        theme::Mode::Dark
    }

    fn base(&self) -> theme::Style {
        theme::Style {
            background_color: self.palette.bg,
            text_color: self.palette.fg,
        }
    }

    /// The runtime's debugging palette, mapped from the roles that
    /// mean the same thing. Nothing this crate draws reads it.
    fn palette(&self) -> Option<theme::Palette> {
        let p = &self.palette;
        Some(theme::Palette {
            background: p.bg,
            text: p.fg,
            primary: p.cta,
            success: p.select,
            warning: p.tape,
            danger: p.alert,
        })
    }

    fn name(&self) -> &str {
        self.era.name()
    }
}

impl container::Catalog for Style {
    type Class<'a> = container::StyleFn<'a, Self>;

    /// Transparent: a container is layout until a site says otherwise,
    /// as the crate has always used it.
    fn default<'a>() -> Self::Class<'a> {
        Box::new(container::transparent)
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        class(self)
    }
}

/// A container filled flat in the era's ground, for a surface that
/// paints its own rather than taking the theme base -- the bar under a
/// transparent application. Pass as `.style(catalog::ground)`.
pub fn ground(style: &Style) -> container::Style {
    container::Style {
        background: Some(style.palette.bg.into()),
        ..container::Style::default()
    }
}

impl text::Catalog for Style {
    type Class<'a> = text::StyleFn<'a, Self>;

    /// Inherited: the base style's `fg`, or whatever the enclosing
    /// widget set. `widgets::text` sets its inks explicitly.
    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_: &Style| text::Style { color: None })
    }

    fn style(&self, class: &Self::Class<'_>) -> text::Style {
        class(self)
    }
}

impl scrollable::Catalog for Style {
    type Class<'a> = scrollable::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(rail)
    }

    fn style(&self, class: &Self::Class<'_>, status: scrollable::Status) -> scrollable::Style {
        class(self, status)
    }
}

/// The era's scroll rail: a track in the border ink and a thumb in the
/// dim ink, no outline on either.
///
/// Only one reference draws a rail -- neomil's mailbox, `rect
/// (542,313,6x565) #3a0f12` with a `#a8282b` thumb -- and those come
/// within a few units of `border` at 60% over the ground and of `dim`,
/// which is the reading the roles give the other three eras too. The width is the
/// widget's (`Scrollbar::new().width(..)`), not the style's; the
/// mailbox's is 6.
///
/// `auto_scroll` is iced 0.14's middle-click autoscroll overlay and
/// has no era reading; it is drawn as a panel-coloured disc with the
/// foreground arrow rather than invented further.
pub fn rail(style: &Style, _status: scrollable::Status) -> scrollable::Style {
    rail_in(style, 1.0)
}

/// [`rail`] with both inks faded to `alpha`, for a pane that has lost
/// focus and recedes with the rest of its chrome.
pub fn faded_rail(alpha: f32) -> impl Fn(&Style, scrollable::Status) -> scrollable::Style {
    move |style, _status| rail_in(style, alpha)
}

fn rail_in(style: &Style, alpha: f32) -> scrollable::Style {
    let p = &style.palette;
    let no_border = Border {
        color: Color::TRANSPARENT,
        width: 0.0,
        radius: 0.0.into(),
    };
    let track = Palette::faded(Ink::Border.of(p), 0.6 * alpha);
    let thumb = Palette::faded(Ink::Dim.of(p), alpha);
    let vertical_rail = scrollable::Rail {
        background: Some(track.into()),
        border: no_border,
        scroller: scrollable::Scroller {
            background: thumb.into(),
            border: no_border,
        },
    };
    scrollable::Style {
        container: container::Style::default(),
        vertical_rail,
        horizontal_rail: vertical_rail,
        gap: None,
        auto_scroll: scrollable::AutoScroll {
            background: p.panel.into(),
            border: Border {
                color: p.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
            icon: p.fg,
        },
    }
}

// --- form controls ---
//
// Everything below reads `Style::controls` (see the `--- controls ---`
// block of `style.rs`). The per-era material covers two button dresses,
// a disabled one, and a field; the rest is derived from the same coats
// and roles, and each function says which it is.
//
// No status treatment: the traces are stills and record no hover or
// press, and "Motion" is its own TODO item. A disabled control takes
// the `disabled` coat whatever its class.

/// A coat as the pieces a built-in style is assembled from.
struct Dressed {
    fill: Option<Color>,
    edge: Border,
    ink: Color,
}

impl Style {
    fn dress(&self, coat: Coat) -> Dressed {
        let p = &self.palette;
        Dressed {
            fill: self.ink(coat.fill),
            edge: Border {
                color: self.ink(coat.edge).unwrap_or(Color::TRANSPARENT),
                width: if coat.edge == Ink::None { 0.0 } else { coat.weight },
                radius: self.controls.radius.into(),
            },
            ink: coat.ink.of(p),
        }
    }

    /// The selection highlight: the era's selection fill, thinned so
    /// the value stays legible through it. Derived; no trace shows a
    /// text selection.
    fn selection(&self) -> Color {
        Palette::faded(self.palette.select, 0.35)
    }
}

pub mod button {
    //! `button` in the era's coats. Pass as `.style(catalog::button::ghost)`.
    //!
    //! A built-in is a rectangle; an era whose buttons chamfer or step
    //! (neomil, kitsch, neokitsch's tab) gets the coat on a plain
    //! rectangle here, and `widgets::surface` for the silhouette.

    use super::*;
    use iced::widget::button::{Status, Style as ButtonStyle};

    fn coat(style: &Style, coat: Coat, status: Status) -> ButtonStyle {
        let coat = match status {
            Status::Disabled => style.controls.disabled,
            _ => coat,
        };
        let d = style.dress(coat);
        ButtonStyle {
            background: d.fill.map(Into::into),
            text_color: d.ink,
            border: d.edge,
            shadow: Shadow::default(),
            snap: true,
        }
    }

    /// The affirmative control: `Controls::primary`.
    pub fn primary(style: &Style, status: Status) -> ButtonStyle {
        coat(style, style.controls.primary, status)
    }

    /// The bare control: `Controls::ghost`.
    pub fn ghost(style: &Style, status: Status) -> ButtonStyle {
        coat(style, style.controls.ghost, status)
    }

    /// No chrome at all: for a button whose face is drawn by what it
    /// wraps -- a `widgets::surface` plate, an icon. Disabled still
    /// dims the ink.
    pub fn bare(style: &Style, status: Status) -> ButtonStyle {
        let ink = match status {
            Status::Disabled => style.controls.disabled.ink,
            _ => Ink::Fg,
        };
        ButtonStyle {
            background: None,
            text_color: ink.of(&style.palette),
            border: Border::default(),
            shadow: Shadow::default(),
            snap: true,
        }
    }
}

impl iced::widget::button::Catalog for Style {
    type Class<'a> = iced::widget::button::StyleFn<'a, Self>;

    /// [`button::ghost`]: the bare one is the common case; a screen has
    /// one affirmative control and asks for it.
    fn default<'a>() -> Self::Class<'a> {
        Box::new(button::ghost)
    }

    fn style(
        &self,
        class: &Self::Class<'_>,
        status: iced::widget::button::Status,
    ) -> iced::widget::button::Style {
        class(self, status)
    }
}

/// `text_input` in the era's field coat. Focus and hover leave it as
/// it is: the traces show every field in one state (entropism's caret
/// underline and kitsch's cursor are the value's, which iced draws in
/// the value ink).
pub fn field(style: &Style, status: text_input::Status) -> text_input::Style {
    let coat = match status {
        text_input::Status::Disabled => style.controls.disabled,
        _ => style.controls.field,
    };
    let d = style.dress(coat);
    text_input::Style {
        background: d.fill.unwrap_or(Color::TRANSPARENT).into(),
        border: d.edge,
        icon: d.ink,
        placeholder: style.controls.placeholder.of(&style.palette),
        value: d.ink,
        selection: style.selection(),
    }
}

impl text_input::Catalog for Style {
    type Class<'a> = text_input::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(field)
    }

    fn style(&self, class: &Self::Class<'_>, status: text_input::Status) -> text_input::Style {
        class(self, status)
    }
}

/// `checkbox`: the field coat as the box, the mark in the affirmative
/// fill. Derived -- no trace has one.
pub fn check(style: &Style, status: checkbox::Status) -> checkbox::Style {
    let (checked, coat) = match status {
        checkbox::Status::Active { is_checked } | checkbox::Status::Hovered { is_checked } => {
            (is_checked, style.controls.field)
        }
        checkbox::Status::Disabled { is_checked } => (is_checked, style.controls.disabled),
    };
    let d = style.dress(coat);
    let p = &style.palette;
    // A ticked box takes the affirmative fill so the state reads at a
    // glance; an unfilled field coat (entropism) would otherwise leave
    // only the mark.
    let background = if checked {
        style.controls.primary.fill.of(p)
    } else {
        d.fill.unwrap_or(Color::TRANSPARENT)
    };
    checkbox::Style {
        background: background.into(),
        icon_color: if checked {
            style.controls.primary.ink.of(p)
        } else {
            d.ink
        },
        border: d.edge,
        text_color: None,
    }
}

impl checkbox::Catalog for Style {
    type Class<'a> = checkbox::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(check)
    }

    fn style(&self, class: &Self::Class<'_>, status: checkbox::Status) -> checkbox::Style {
        class(self, status)
    }
}

/// `toggler`: the field coat as the track, the knob in the field's ink
/// off and the affirmative fill on. Derived.
pub fn toggle(style: &Style, status: toggler::Status) -> toggler::Style {
    let (on, coat) = match status {
        toggler::Status::Active { is_toggled } | toggler::Status::Hovered { is_toggled } => {
            (is_toggled, style.controls.field)
        }
        toggler::Status::Disabled { is_toggled } => (is_toggled, style.controls.disabled),
    };
    let d = style.dress(coat);
    let p = &style.palette;
    let knob = if on {
        style.controls.primary.fill.of(p)
    } else {
        d.ink
    };
    toggler::Style {
        background: d.fill.unwrap_or(Color::TRANSPARENT).into(),
        background_border_width: d.edge.width,
        background_border_color: d.edge.color,
        foreground: knob.into(),
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: None,
        border_radius: Some(style.controls.radius.into()),
        padding_ratio: 0.2,
    }
}

impl toggler::Catalog for Style {
    type Class<'a> = toggler::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(toggle)
    }

    fn style(&self, class: &Self::Class<'_>, status: toggler::Status) -> toggler::Style {
        class(self, status)
    }
}

/// `radio`: the field coat as the ring, the dot in the affirmative
/// fill. Derived.
pub fn choice(style: &Style, _status: radio::Status) -> radio::Style {
    let d = style.dress(style.controls.field);
    radio::Style {
        background: d.fill.unwrap_or(Color::TRANSPARENT).into(),
        dot_color: style.controls.primary.fill.of(&style.palette),
        border_width: d.edge.width.max(1.0),
        border_color: if d.edge.width > 0.0 {
            d.edge.color
        } else {
            d.ink
        },
        text_color: None,
    }
}

impl radio::Catalog for Style {
    type Class<'a> = radio::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(choice)
    }

    fn style(&self, class: &Self::Class<'_>, status: radio::Status) -> radio::Style {
        class(self, status)
    }
}

/// `slider`: the scroll rail's track and thumb readings laid on their
/// side -- border-ink track, the run so far in the affirmative fill, a
/// foreground-ink bar for a handle. Derived; iced's slider has no
/// ticks, so "slider with ticks" is not a style and stays open.
pub fn slide(style: &Style, _status: slider::Status) -> slider::Style {
    let p = &style.palette;
    slider::Style {
        rail: slider::Rail {
            backgrounds: (
                style.controls.primary.fill.of(p).into(),
                Palette::faded(Ink::Border.of(p), 0.6).into(),
            ),
            width: 4.0,
            border: Border::default(),
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Rectangle {
                width: 6,
                border_radius: style.controls.radius.into(),
            },
            background: p.fg.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
    }
}

impl slider::Catalog for Style {
    type Class<'a> = slider::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(slide)
    }

    fn style(&self, class: &Self::Class<'_>, status: slider::Status) -> slider::Style {
        class(self, status)
    }
}

/// `pick_list`: a field with the foreground ink for its handle. Derived.
pub fn pick(style: &Style, _status: pick_list::Status) -> pick_list::Style {
    let d = style.dress(style.controls.field);
    pick_list::Style {
        text_color: d.ink,
        placeholder_color: style.controls.placeholder.of(&style.palette),
        handle_color: d.ink,
        background: d.fill.unwrap_or(Color::TRANSPARENT).into(),
        border: d.edge,
    }
}

impl pick_list::Catalog for Style {
    type Class<'a> = pick_list::StyleFn<'a, Self>;

    fn default<'a>() -> pick_list::StyleFn<'a, Self> {
        Box::new(pick)
    }

    fn style(&self, class: &pick_list::StyleFn<'_, Self>, status: pick_list::Status) -> pick_list::Style {
        class(self, status)
    }
}

/// The drop-down under a `pick_list`: the panel colour edged in the
/// field's edge, the chosen row in the selection pair. Derived; the
/// bar's tray menus are canvas (`bar::tray_menu`) and read their own
/// table.
pub fn drop_down(style: &Style) -> menu::Style {
    let p = &style.palette;
    let d = style.dress(style.controls.field);
    menu::Style {
        background: p.panel.into(),
        border: Border {
            color: if d.edge.width > 0.0 { d.edge.color } else { p.border },
            width: d.edge.width.max(1.0),
            radius: style.controls.radius.into(),
        },
        text_color: p.fg,
        selected_text_color: p.on_select,
        selected_background: p.select.into(),
        shadow: Shadow::default(),
    }
}

impl menu::Catalog for Style {
    type Class<'a> = menu::StyleFn<'a, Self>;

    fn default<'a>() -> menu::StyleFn<'a, Self> {
        Box::new(drop_down)
    }

    fn style(&self, class: &menu::StyleFn<'_, Self>) -> menu::Style {
        class(self)
    }
}

/// `rule`: a 1px line in the border ink, as every era's dividers are.
pub fn divider(style: &Style) -> rule::Style {
    rule::Style {
        color: style.palette.border,
        radius: 0.0.into(),
        fill_mode: rule::FillMode::Full,
        snap: true,
    }
}

impl rule::Catalog for Style {
    type Class<'a> = rule::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(divider)
    }

    fn style(&self, class: &Self::Class<'_>) -> rule::Style {
        class(self)
    }
}

/// `progress_bar`: the scroll rail's track under the affirmative fill.
/// Derived; the "Feedback" TODO item's segmented meter is not this.
pub fn progress(style: &Style) -> progress_bar::Style {
    let p = &style.palette;
    progress_bar::Style {
        background: Palette::faded(Ink::Border.of(p), 0.6).into(),
        bar: style.controls.primary.fill.of(p).into(),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: style.controls.radius.into(),
        },
    }
}

impl progress_bar::Catalog for Style {
    type Class<'a> = progress_bar::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(progress)
    }

    fn style(&self, class: &Self::Class<'_>) -> progress_bar::Style {
        class(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::theme::Base;

    #[test]
    fn base_is_the_palette() {
        for era in Era::ALL {
            let style = era.style();
            let base = style.base();
            assert_eq!(base.background_color, style.palette.bg, "{}", era.name());
            assert_eq!(base.text_color, style.palette.fg, "{}", era.name());
            assert_eq!(style.name(), era.name());
        }
    }

    /// The one measured rail: neomil's mailbox track `#3a0f12` and
    /// thumb `#a8282b`, each within a few units of its role reading
    /// (the trace's are hand-picked colours, not blends).
    #[test]
    fn neomil_rail_matches_the_trace() {
        let style = Era::Neomil.style();
        let status = scrollable::Status::Active {
            is_horizontal_scrollbar_disabled: false,
            is_vertical_scrollbar_disabled: false,
        };
        let s = rail(&style, status);
        let over = |c: Color, ground: Color| {
            let mix = |a: f32, b: f32| (a * c.a + b * (1.0 - c.a)) * 255.0;
            (
                mix(c.r, ground.r).round() as u8,
                mix(c.g, ground.g).round() as u8,
                mix(c.b, ground.b).round() as u8,
            )
        };
        let iced::Background::Color(track) = s.vertical_rail.background.unwrap() else {
            panic!("rail track is a colour");
        };
        let iced::Background::Color(thumb) = s.vertical_rail.scroller.background else {
            panic!("rail thumb is a colour");
        };
        let (r, g, b) = over(track, style.palette.bg);
        assert!(
            (r as i32 - 0x3a).abs() <= 6 && (g as i32 - 0x0f).abs() <= 6 && (b as i32 - 0x12).abs() <= 6,
            "track {r:02x}{g:02x}{b:02x}"
        );
        let (r, g, b) = over(thumb, style.palette.bg);
        assert!(
            (r as i32 - 0xa8).abs() <= 6 && (g as i32 - 0x28).abs() <= 6 && (b as i32 - 0x2b).abs() <= 6,
            "thumb {r:02x}{g:02x}{b:02x}"
        );
    }

    fn colour(bg: iced::Background) -> Color {
        let iced::Background::Color(c) = bg else {
            panic!("a colour, not a gradient");
        };
        c
    }

    /// Every era's affirmative button is the call-to-action fill under
    /// its ink, and disabling any button swaps in the disabled coat.
    #[test]
    fn primary_button_is_the_cta() {
        for era in Era::ALL {
            let style = era.style();
            let s = button::primary(&style, iced::widget::button::Status::Active);
            assert_eq!(colour(s.background.unwrap()), style.palette.cta, "{}", era.name());
            let off = button::primary(&style, iced::widget::button::Status::Disabled);
            let ghost_off = button::ghost(&style, iced::widget::button::Status::Disabled);
            assert_eq!(off.background, ghost_off.background, "{}", era.name());
            assert_eq!(off.text_color, ghost_off.text_color, "{}", era.name());
        }
    }

    /// Kitsch's PROTECTED reading from the component sheet: `#122724`
    /// under a 1px `#1d3f3a` edge, lettered `#7fe0c8`.
    #[test]
    fn kitsch_disabled_is_protected() {
        let style = Era::Kitsch.style();
        let s = button::ghost(&style, iced::widget::button::Status::Disabled);
        assert_eq!(colour(s.background.unwrap()), crate::eras::kitsch::LOCKED);
        assert_eq!(s.border.color, crate::eras::kitsch::LOCKED_EDGE);
        assert_eq!(s.border.width, 1.0);
        assert_eq!(s.text_color, crate::eras::kitsch::ANNOTATION);
    }

    /// Entropism's field is outlined only: no fill, the 1.25 border
    /// stroke, square corners. The radius is the era's everywhere.
    #[test]
    fn fields_follow_the_coat() {
        let e = Era::Entropism.style();
        let s = field(&e, text_input::Status::Active);
        assert_eq!(colour(s.background), Color::TRANSPARENT);
        assert_eq!(s.border.width, 1.25);
        assert_eq!(s.border.color, e.palette.border);
        assert_eq!(s.border.radius, 0.0.into());
        let k = Era::Kitsch.style();
        let s = field(&k, text_input::Status::Focused { is_hovered: false });
        assert_eq!(colour(s.background), crate::eras::kitsch::WELL);
        assert_eq!(s.border.radius, 2.0.into());
        assert_eq!(s.placeholder, k.palette.dim);
    }

    /// A coat with `Ink::None` for an edge draws no border at all,
    /// whatever its weight says.
    #[test]
    fn no_edge_means_no_border() {
        let style = Era::Neokitsch.style();
        let s = field(&style, text_input::Status::Active);
        assert_eq!(s.border.width, 0.0);
    }
}
