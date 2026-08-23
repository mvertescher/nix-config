//! The resolved colour set a screen draws with.
//!
//! Two sources feed this. The desktop theme layer publishes the seven
//! semantic roles to `$XDG_CONFIG_HOME/theme/current.toml` (see
//! [`crate::theme`]), and each era carries a reference-sampled fallback
//! so the toolkit still runs standalone. Roles that the theme layer does
//! not model -- the fill a *selected* element takes, and the colour of
//! text sitting on it -- come from the era.
//!
//! Keeping selection out of the seven roles is deliberate. Across the
//! four eras it is not one colour with four values; it is four different
//! ideas that happen to share a slot:
//!
//! | era       | selected element is...                    |
//! |-----------|-------------------------------------------|
//! | neomil    | filled with the fill red                  |
//! | entropism | filled with solid sage                    |
//! | kitsch    | filled with yellow -- *selection*, not alarm |
//! | neokitsch | filled with wood veneer -- a material      |
//!
//! `alert` stays a separate role because in three of the four eras it is
//! a different colour from `select`. In kitsch it is the same one, which
//! is exactly the sort of thing a shared `accent` slot would have hidden.

use crate::theme::{Roles, Theme};
use iced::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    /// Page ground.
    pub bg: Color,
    /// Raised surface: bar, popup, titlebar. Published by the theme
    /// layer, so it is whatever the desktop wants a panel to be.
    pub panel: Color,
    /// 1px structure.
    pub border: Color,
    /// Captions, rules, anything deliberately quiet.
    pub dim: Color,
    /// Body and heading text.
    pub fg: Color,
    /// Failure and warning only.
    pub alert: Color,
    /// Improvised label accent; the orange bezel in kitsch, veneer in
    /// neokitsch.
    pub tape: Color,
    /// Fill for the selected element.
    pub select: Color,
    /// Text and line-work drawn on top of `select`.
    pub on_select: Color,
    /// Fill for the one affirmative control on a screen -- ENTER,
    /// NEXT, LOGIN. Era-owned, and not merely an alias for `select`:
    /// three eras do use their selection colour here, but neokitsch's
    /// references reserve amber for exactly this and fill selection
    /// with veneer instead. Collapsing the two would have made its
    /// login button a plank of wood.
    pub cta: Color,

    /// The colour of the era's background bloom, where it has one.
    /// Era-owned rather than a published role: `panel` has to stay a
    /// usable bar background on the desktop, and the two want opposite
    /// things -- a bloom is a saturated wash, a bar is not.
    pub bloom: Color,
    /// Optional highlight band behind key figures, as `(fill, ink)`.
    /// Kitsch runs a mint band under the weapon stats; the ink is
    /// carried with it because the band is light in an otherwise dark
    /// palette, and the era's body colour is illegible on it.
    /// `None` in eras whose references have no such band.
    pub emphasis: Option<(Color, Color)>,
}

impl Palette {
    /// Overlay the theme layer's seven roles onto an era's sampled
    /// fallback. `select`/`on_select` are era-owned and never come from
    /// the theme file.
    pub fn with_roles(self, roles: Roles) -> Self {
        Palette {
            bg: roles.bg.into(),
            panel: roles.panel.into(),
            border: roles.border.into(),
            dim: roles.dim.into(),
            fg: roles.fg.into(),
            alert: roles.alert.into(),
            tape: roles.tape.into(),
            ..self
        }
    }

    /// Same, but driven by a loaded [`Theme`].
    pub fn with_theme(self, theme: &Theme) -> Self {
        self.with_roles(theme.colors)
    }

    /// `self` at `alpha`. Used for washes and inactive states rather
    /// than mixing a second colour into the palette.
    pub fn faded(color: Color, alpha: f32) -> Color {
        Color { a: alpha, ..color }
    }
}

/// Compile-time `#rrggbb`, so the sampled palettes read the way they do
/// in the design docs instead of as float triples.
pub const fn rgb(hex: u32) -> Color {
    Color {
        r: ((hex >> 16) & 0xff) as f32 / 255.0,
        g: ((hex >> 8) & 0xff) as f32 / 255.0,
        b: (hex & 0xff) as f32 / 255.0,
        a: 1.0,
    }
}
