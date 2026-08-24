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
//!
//! On top of the seven sit the *ornamental* roles ([`Ornaments`], and
//! `emphasis`). Those are the maximalist half of the vocabulary: the
//! notched accent band, the two edges of a bevel, the page-curl, the
//! recessed well. They are optional in both directions -- an era may
//! declare none, and the theme layer may publish none -- and a widget
//! reads them through the accessors on [`Palette`], which degrade to
//! the base seven rather than returning `Option`.

use crate::theme::{Rgb, Roles, Theme};
use iced::Color;

/// The ornamental roles, which only a maximalist era declares.
///
/// Kept in their own struct, and every one of them an `Option`, because
/// that is the honest shape of the thing: entropism and neomil have
/// none of this, and even kitsch and neokitsch each lack one of the
/// five. A widget asks through [`Palette`]'s accessors, which name the
/// fallback they use, rather than unwrapping.
///
/// The vocabulary is `extraNames` in `home/themes/lib/roles.nix`; each
/// entry is justified there by where it appears in
/// `docs/{kitsch,neokitsch}/target-*.svg`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Ornaments {
    /// Notched accent band as `(fill, ink)`. Kitsch's shelf band pokes
    /// past the card's left edge and carries compliance glyphs;
    /// neokitsch's is the footer nameplate and the BASKET panel. The
    /// ink is carried with the fill for the same reason `emphasis`
    /// carries one: the band is light in a dark palette and `fg` is not
    /// legible on it.
    pub banner: Option<(Color, Color)>,
    /// The two edges of a raised or extruded surface, as
    /// `(bevel, shade)` -- lit face and receding side. Kitsch's fan-menu
    /// slabs and neokitsch's double-stroked device frame.
    pub relief: Option<(Color, Color)>,
    /// Solid, non-structural decoration: kitsch's page-curl, neokitsch's
    /// strata wedge. Never encloses anything, which is what separates it
    /// from `border`.
    pub ornament: Option<Color>,
    /// Recessed fill -- input wells and sockets. Neokitsch's login
    /// field; `bg` is the page and `panel` is the raised thing, neither
    /// is the hole in it.
    pub inset: Option<Color>,
}

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
    ///
    /// Belongs to the same optional vocabulary as [`Ornaments`] and is
    /// published under the same rules -- it is flat here only because
    /// it predates the rest and widgets already read it by this path.
    pub emphasis: Option<(Color, Color)>,
    /// The accent band's `(fill, ink)` when it sits on a *selected*
    /// element, where the era sampled one.
    ///
    /// Era-owned rather than a published role, for the same kind of
    /// reason `bloom` is -- see [`Palette::banner_on_select`], which
    /// records the argument and the fallback.
    pub banner_selected: Option<(Color, Color)>,

    /// Everything else a maximalist era declares. Default-empty, so an
    /// era that says nothing is spelled `Ornaments::default()`.
    pub ornaments: Ornaments,
}

impl Palette {
    /// Overlay the theme layer's roles onto an era's sampled fallback.
    /// `select`/`on_select` are era-owned and never come from the theme
    /// file.
    ///
    /// The seven always overwrite. The optional roles only overwrite
    /// what the file actually declares: a theme that publishes none of
    /// them -- every theme written before the vocabulary existed, and
    /// both minimalist eras today -- leaves the era's own ornaments
    /// exactly as sampled. That is what keeps this additive.
    pub fn with_roles(self, roles: Roles) -> Self {
        let x = roles.extra;
        // An ink without its fill is meaningless, so a pair exists only
        // when the fill does; a fill without its ink falls back to the
        // published ground, which the nix side also does.
        let band = |fill: Option<Rgb>, ink: Option<Rgb>| {
            fill.map(|f| (f.into(), ink.unwrap_or(roles.bg).into()))
        };

        Palette {
            bg: roles.bg.into(),
            panel: roles.panel.into(),
            border: roles.border.into(),
            dim: roles.dim.into(),
            fg: roles.fg.into(),
            alert: roles.alert.into(),
            tape: roles.tape.into(),

            emphasis: band(x.emphasis, x.on_emphasis).or(self.emphasis),
            ornaments: Ornaments {
                banner: band(x.banner, x.on_banner).or(self.ornaments.banner),
                // A bevel is a pair by nature: half of one is a flat
                // edge, so a lone side pulls its partner from `border`.
                relief: match (x.bevel, x.shade) {
                    (None, None) => self.ornaments.relief,
                    (bevel, shade) => Some((
                        bevel.unwrap_or(roles.border).into(),
                        shade.unwrap_or(roles.border).into(),
                    )),
                },
                ornament: x.ornament.map(Into::into).or(self.ornaments.ornament),
                inset: x.inset.map(Into::into).or(self.ornaments.inset),
            },
            ..self
        }
    }

    /// The accent banner as `(fill, ink)`.
    ///
    /// An era that declares none gets its improvised label accent over
    /// the page ground, which is what a banner degrades to: a tape
    /// label. Never `None`, so a banner widget has no era branch in it.
    pub fn banner(&self) -> (Color, Color) {
        self.ornaments.banner.unwrap_or((self.tape, self.bg))
    }

    /// The accent banner as `(fill, ink)` on a selected element.
    ///
    /// ## Why this is a colour and not a role
    ///
    /// A band in the selection fill, on a card in the selection fill,
    /// is invisible, so both maximalist eras move it -- and they move
    /// it in opposite directions:
    ///
    /// * kitsch darkens. `docs/kitsch/target-app.svg` fills the
    ///   selected card's shelf band `#f0a80a` against `#fcc428` on the
    ///   other three, ink unchanged at `#37220f`; the same `#f0a80a`
    ///   is the folded corner of the callout panel in
    ///   `target-components.svg`. It is the yellow one stop down.
    /// * neokitsch inverts. `target-components.svg` draws the
    ///   unselected footer nameplate `#d3b279` with `#3a2410` text and
    ///   the selected one `#3a2410` with `#e7c686` -- the ink is the
    ///   era's `fg`, not its band fill.
    ///
    /// This used to be inferred from [`crate::style::Selection`], and
    /// the inference was wrong twice over: a 15% mix towards the ink
    /// lands on `#deab24`, which is muddier and *less* saturated than
    /// the sampled `#f0a80a` (no mix can reach it -- the target's blue
    /// channel is 10 and both endpoints are above it), and a straight
    /// swap of the pair gives neokitsch champagne ink where the target
    /// has gold. Two sampled pairs are shorter than the derivation
    /// they replace and land on the reference exactly.
    ///
    /// It is not a published role, and the reasoning matters more than
    /// the outcome because the vocabulary in
    /// `home/themes/lib/roles.nix` would now accept one cheaply --
    /// `extraNames` is additive and an era that does not declare a
    /// name is unaffected by it. The case against is that only one
    /// era has anything to publish. Neokitsch's pair is `(onBanner,
    /// fg)`, both already published, so a role there would restate two
    /// values it already has; that leaves kitsch's single `#f0a80a`,
    /// and a role with one era's value and one era's tautology is a
    /// constant with a long name. The precedent is the repo TODO's
    /// "Roles for maximalist eras": a role it is not, a colour the era
    /// needs it is.
    ///
    /// The live cost of that call is that a *variant* cannot retint
    /// it: kitsch's `bleach` publishes `banner = "#e8a80f"` and would
    /// still get the dark-theme `#f0a80a` selected. That is tolerable
    /// because the two are within a stop of each other and because the
    /// variant's own note says the band "stays hot -- it is what
    /// 'selected' looks like". If it ever stops being tolerable, the
    /// additive path is one `extraNames` entry and one `ExtraRoles`
    /// field, and it touches no era that stays quiet.
    ///
    /// Total, like every accessor here. An era that sampled no pair
    /// falls back to swapping fill and ink, which is a legible band in
    /// any palette and is what the two minimalist eras' tape-label
    /// degradation reduces to.
    pub fn banner_on_select(&self) -> (Color, Color) {
        let (fill, ink) = self.banner();
        self.banner_selected.unwrap_or((ink, fill))
    }

    /// The highlight band as `(fill, ink)`, falling back to a quiet
    /// panel with ordinary body text -- an era with no band still gets
    /// a legible one.
    pub fn emphasis_band(&self) -> (Color, Color) {
        self.emphasis.unwrap_or((self.panel, self.fg))
    }

    /// The two edges of a raised surface as `(bevel, shade)`. With no
    /// relief declared both are `border`, which draws the flat 1px box
    /// the minimalist eras already use.
    pub fn relief(&self) -> (Color, Color) {
        self.ornaments.relief.unwrap_or((self.border, self.border))
    }

    /// Decoration colour, falling back to `fg`: an era with no ornament
    /// draws its flourishes in line-work, or draws none.
    pub fn ornament(&self) -> Color {
        self.ornaments.ornament.unwrap_or(self.fg)
    }

    /// Recessed fill, falling back to `bg` -- a well that reads as a
    /// hole cut through to the page, which is what entropism's and
    /// neomil's input boxes are.
    pub fn inset(&self) -> Color {
        self.ornaments.inset.unwrap_or(self.bg)
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
