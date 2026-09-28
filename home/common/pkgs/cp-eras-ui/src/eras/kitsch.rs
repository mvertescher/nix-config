//! Kitsch -- "style over substance".
//!
//! Teal line-work and yellow selection over a rose bloom on warm black.
//! Sampled from Behance Part 1, gallery positions 44-52 (title card 43)
//! per `docs/sources.md`; the "doc #34-42" this comment used to give is
//! from an earlier, smaller scrape and is entropism's run.
//!
//! This comment used to say "everything rounded, no chamfers anywhere".
//! The traces disagree: `docs/kitsch/store-trace.svg` `#card` is r6 at
//! the top-left and a 24px chamfer at the top-right (`M 6.5,0.5 H 237
//! L 261.5,24.5 ...`), its yellow band ends in a 45-degree chamfer
//! (`V 71.5 L 235.5,94 Z`), the nav is a peaked chevron
//! (`M 0,39 V 19 L 18,0 L 27,3 H 214 ... V 11 L 190,39 Z`), and
//! `mailbox-trace.svg` chamfers its row ends ((491,325)->(471,346)) and
//! its tab's top-right ((1105,313)->(1127,337)). Rounding survives at
//! r8 (fan cards, chips), r6 and r2 (badges), not the r16 of
//! `Corner::Round` below, which only bar/dashboard widgets read; see
//! `ERAS-DELTA.md`.
//!
//! Note the inversion the era forces on the role vocabulary: yellow is
//! *selection*, not alarm. Failure states are essentially absent from
//! the reference, so `alert` and `select` are the same colour here --
//! the only era where that is true.

use crate::palette::{rgb, Ornaments, Palette};
use crate::style::{
    Banner, Bar, BarChrome, BarGround, BarMenu, BarOrnament, Chrome, Coat, Compliance, Controls,
    Corner, Destination, Dress,
    Era, Face, Footnotes, Ground, Ink, MenuMarker, MenuRule, Metrics, Nameplate,
    PanelEcho, PlateStates, Selection, Style, Ticket, WindowLabel,
};
use crate::widgets::surface::{Corners, Cut};
use super::magnum_art::{RIFLE, RIFLE_BRIGHT};
// --- login ---
use crate::style::{
    Access, Blink, Caret, Colophon, Emblem, Entry, Fixture, Legend, Masthead, Plate, Plot, Slot,
};
// --- end login ---

pub const BG: iced::Color = rgb(0x0b0b07);
pub const BLOOM: iced::Color = rgb(0xa63355);
pub const TEAL: iced::Color = rgb(0x7ddec8);
/// The outline teal: what the store and mailbox traces sample off card
/// frames and dividers, a stop under the body teal. The `border` role,
/// and the same value the published theme resolves it to.
pub const TEAL_OUTLINE: iced::Color = rgb(0x5fd6c2);
pub const TEAL_SOLID: iced::Color = rgb(0x1cb39b);
pub const MINT: iced::Color = rgb(0x87f4d9);
pub const YELLOW: iced::Color = rgb(0xfcc428);
pub const ON_YELLOW: iced::Color = rgb(0x37220f);
pub const BEZEL: iced::Color = rgb(0xf08c1e);
pub const TEAL_DIM: iced::Color = rgb(0x4d9484);
/// Ink for figures sitting on the mint stat band.
pub const ON_MINT: iced::Color = rgb(0x0b3b31);
/// Lit face of an extruded fan-menu slab, and the darker teal its
/// stacked outlines recede in. Sampled off the braindance screens; see
/// the old `docs/kitsch/target-components.svg`, "EXTRUDED FAN MENU" (the
/// by-eye sheet replaced 2026-09-03 by `components.svg`, rebuilt from the
/// traces; `dashboard-trace.svg` has 162x50 r8 cards with ghosts fading
/// 0.58 to 0.12, not extruded slabs). The trace fills those cards
/// `#2c9798` under a 1.8px `#a9e6df` stroke (`<use href="#card"
/// fill="#2c9798" stroke="#a9e6df" stroke-width="1.8"/>`); `SLAB` stays
/// `#2bc4ac` because its only reader is `relief` -> `Palette::relief()`,
/// consumed by bar / menu / chrome widgets, and in every gated render
/// `home/themes/kitsch/palettes.nix` overrides `bevel`/`shade` anyway.
pub const SLAB: iced::Color = rgb(0x2bc4ac);
pub const SLAB_SHADE: iced::Color = rgb(0x177a6b);
/// The yellow one stop down: the shelf band on the *selected* card and
/// the folded corner of the callout panel, as drawn in
/// `docs/kitsch/target-app.svg` (deleted 2026-09-03). Not reachable by
/// mixing `YELLOW` towards `ON_YELLOW` -- it is darker *and* more
/// saturated, and its blue channel sits below both endpoints.
///
/// An earlier version of this comment claimed `store-trace.svg` carried
/// `M830 308 ... fill="#f0a80a"`. It does not: `#f0a80a` appears in no
/// trace. The grown card in `store-trace.svg` (group at translate(804
/// 218), `M 6,0 H 237 L 261,24 V 239 ...`) fills `#ffc233`, the plain cards' band is `#fec32f`, and the
/// grown card's band is a 1.1px `#a4583a` outline rather than a darker
/// fill.
///
/// Unconsumed as of 2026-09-03: the only reader is `banner_selected`
/// (below), which reaches `Palette::banner_on_select`; its widget
/// readers (`widgets::banner` / `widgets::card`) were deleted
/// 2026-09-05, so nothing draws it. Trace value would be `#ffc233` (grown-card fill) if the
/// band-on-selected pair is ever wired up.
pub const YELLOW_SHADE: iced::Color = rgb(0xf0a80a);

pub fn palette() -> Palette {
    Palette {
        bg: BG,
        panel: BLOOM,
        border: TEAL_OUTLINE,
        dim: TEAL_DIM,
        fg: TEAL,
        alert: YELLOW,
        tape: BEZEL,
        select: YELLOW,
        on_select: ON_YELLOW,
        emphasis: Some((MINT, ON_MINT)),
        // On the selected card the band darkens and keeps its ink: the
        // era shades the band rather than inverting it, because the
        // card underneath is already yellow.
        banner_selected: Some((YELLOW_SHADE, ON_YELLOW)),
        ornaments: Ornaments {
            // The shelf band on every product card: yellow, poking past
            // the card's left edge, its glyphs and brand tag in the
            // dark ink. Same fill as `select` here and a different one
            // in neokitsch, which is why it is not an alias for it.
            banner: Some((YELLOW, ON_YELLOW)),
            relief: Some((SLAB, SLAB_SHADE)),
            // The page-curl at the foot of the nav container -- one per
            // screen -- plus the chip squares and PROTECTED bars.
            ornament: Some(TEAL_SOLID),
            // No wells in the era: kitsch cards are unfilled outlines.
            inset: None,
        },
        cta: YELLOW,
        bloom: BLOOM,
    }
}

pub fn style() -> Style {
    Style {
        era: Era::Kitsch,
        palette: palette(),
        corner: Corner::Round { radius: 16.0 },
        selection: Selection::Solid,
        // Out of the top-right, heavily vignetted.
        ground: Ground::Bloom {
            x: 0.82,
            y: 0.0,
            radius: 0.75,
        },
        chrome: Chrome::Caption,
        nameplate: Nameplate::Header,
        // --- bar --- (docs/kitsch/bar.svg, IMPLEMENTATION DELTA)
        //
        // Thin bright line-work on .5 coordinates, the customer chip's
        // r8 for a readout, the store nav's chevron for a workspace,
        // the mailbox USER box for the tape, and the era's one solid
        // teal curl -- moved from the container foot, which is 3px
        // tall here, to the foot of the tray menu, the one container
        // the bar draws.
        bar: Bar {
            height: 31,
            host_tape: true,

            // The bracket lives in the 10px the strip leaves at its
            // left and the 3px under its cells.
            pad_left: 10.0,
            pad_right: 6.0,
            pad_y: 3.0,
            // The traces separate cells by air, never by rules.
            gap: 8.0,
            // mailbox pitch 53..55 on a 46px tab is a 7..9 gap; at
            // this scale, 6 on a 46 pitch.
            ws_gap: 6.0,
            ws_lead: 10.0,
            ws_width: 40.0,
            // The nav chevron of mailbox-trace's #chev, scaled 25/46:
            // the left edge rises to a peak 12 in and 13 up, the
            // top-right is the era's small radius, and the right end
            // chamfers back over the lower half.
            ws_corners: Some(
                Corners::square()
                    // #chev scaled 25/46: up from (0,13) to the peak at
                    // (12,0), down onto the brow at (15.2,4.9).
                    .with_top_left(Cut::Peak {
                        x: 12.0,
                        y: 13.0,
                        brow: (3.2, 4.9),
                    })
                    .with_top_right(Cut::Round { radius: 3.0 })
                    .with_bottom_right(Cut::Chamfer { x: 12.0, y: 12.0 }),
            ),
            pad_x: 13.0,
            trail: 13.0,
            em: 0.58,
            // The design sized its cells by counting characters flat.
            space_em: 0.58,
            alert_track: 0.0,
            // 1.2px full-brightness measured across the message
            // outline and the login bracket; 1.5 on integer
            // coordinates rendered as two half-bright pixels and read
            // dimmer.
            stroke: 1.25,
            icon_pad: 18.0,
            label_left: false,
            // A 1.25px line next to 400-weight Rajdhani reads heavier
            // than the text, so the era sets its labels Medium.
            face: Face::Medium,
            tape_extra: 4.0,
            tape_ticks: false,

            ground: BarGround::Plain,
            chrome: BarChrome::Loose,
            ornament: BarOrnament::Bracket,

            // The customer chip of store-trace: a 22px-tall rounded
            // outline holding a label. r8, not the 16 the era's cards
            // clamp to -- three independent measurements agree on 8.
            idle: Dress {
                corners: Corners::all(Cut::Round { radius: 8.0 }),
                fill: Ink::None,
                stroke: Ink::Border,
                ink: Ink::Fg,
                tab: false,
                step: None,
            },
            // The EVENTS card: the one selected blade of the fan,
            // filled solid yellow with dark ink.
            selected: Dress {
                corners: Corners::all(Cut::Round { radius: 8.0 }),
                fill: Ink::Select,
                stroke: Ink::None,
                ink: Ink::OnSelect,
                tab: false,
                step: None,
            },
            // The message panel's yellow outline: the shape does not
            // change, which is what the era does -- yellow is a fill
            // or a line, never a new silhouette.
            alert: Dress {
                corners: Corners::all(Cut::Round { radius: 8.0 }),
                fill: Ink::None,
                stroke: Ink::Alert,
                ink: Ink::Alert,
                tab: false,
                step: None,
            },
            // The GUES 7702 box of mailbox-trace, whose bottom edge
            // steps down under its first 55px through a 12px diagonal
            // (`M 155,185 H 349 V 228 H 220 L 208,236 H 155 Z`); here
            // the first 26px, through a 6px diagonal.
            tape: Dress {
                corners: Corners::square(),
                fill: Ink::Tape,
                stroke: Ink::None,
                ink: Ink::OnSelect,
                tab: false,
                step: Some((26.0, 4.0)),
            },
            tab: None,
            // The DESCRIPTION box: the same stepped box outlined, in
            // full teal -- dim teal is reserved for the 7.5px
            // in-fiction captions and every readable label is full.
            window: WindowLabel {
                dress: Some(Dress {
                    corners: Corners::square(),
                    fill: Ink::None,
                    stroke: Ink::Border,
                    ink: Ink::Fg,
                    tab: false,
                    step: Some((26.0, 4.0)),
                }),
                ink: Ink::Fg,
                leading: false,
                pad_x: 12.0,
                stroke: None,
                face: None,
            },

            alert_suffix: None,
            bold_tiers: false,
            clock_plain: None,

            menu: BarMenu {
                panel: Dress {
                    corners: Corners::all(Cut::Round { radius: 8.0 }),
                    fill: Ink::Bg,
                    stroke: Ink::Border,
                    ink: Ink::Fg,
                    tab: false,
                    step: None,
                },
                air: 6.0,
                side: 0.0,
                row_air: 2.8,
                row_side: 4.0,
                // The icon owns a cell, so it can never sit on the
                // label -- the overlap the sibling bars have.
                icon_col: 26.0,
                icon_gap: 8.0,
                // Abutting 1.25px outlines would read as one 3px line.
                level_gap: 4.0,
                level_pad: 24.0,
                row_divider: false,
                // The socket-row rules of the product card run edge to
                // edge.
                rule: MenuRule::Full,
                // The selected message row: straight for its top 9.3px,
                // then a 15.5-wide chamfer to the bottom.
                row: Dress {
                    corners: Corners::square()
                        .with_bottom_right(Cut::Chamfer { x: 15.5, y: 16.3 }),
                    fill: Ink::Select,
                    stroke: Ink::None,
                    ink: Ink::OnSelect,
                    tab: false,
                    step: None,
                },
                open: Dress {
                    corners: Corners::square()
                        .with_bottom_right(Cut::Chamfer { x: 15.5, y: 16.3 }),
                    fill: Ink::Select,
                    stroke: Ink::None,
                    ink: Ink::OnSelect,
                    tab: false,
                    step: None,
                },
                open_inset: (2.5, 4.0),
                // The other half of the selected row: an icon cell 26
                // wide with its bottom-right chamfered, then the 2px
                // gap the trace measures at x 196.
                row_split: Some((
                    26.0,
                    Corners::square().with_bottom_right(Cut::Chamfer { x: 12.0, y: 4.0 }),
                )),
                disabled: Ink::Dim,
                rule_ink: Ink::Border,
                row_inset: (2.5, 4.0),
                row_overshoot: 0.0,
                spine: 0.0,
                // The 20px foot the wave lives in.
                foot: 20.0,
                marker: MenuMarker::Text,
                echo: PanelEcho::Wave,
            },
        },
        // The shelf band hangs 12px past the card and steps its
        // trailing corner down 8; measured off the target-app.svg
        // composite (deleted 2026-09-03).
        //
        // Unconsumed as of 2026-09-03: `style.banner` was read only by
        // `widgets::banner::banner` and `widgets::card::product_card`,
        // which nothing called (the store is a canvas program) and
        // which were deleted 2026-09-05. Trace
        // value would be a 35px band with a 27px flag, no notch, and a
        // 45-degree chamfer at the right end -- `store-trace.svg` `#card`
        // band `M -27,94 V 72 L -3,50 V 59 H 256 Q 258,59 258,61 V 71.5
        // L 235.5,94 Z` (local y 59..94 on a card whose right edge is
        // x 261.5): overhang 27 on the left, none on the right.
        banner: Banner {
            overhang: 12.0,
            notch: 8.0,
        },
        // A halfway down the column under the page-curl, C under the
        // right of the shelf.
        footnotes: Footnotes::MidColumn,
        // Below the card rather than inside it, and on the selected
        // one too.
        compliance: Compliance::Below,
        // The nav pill's top-right juts out 18 and drops 15, on a body
        // that is otherwise the era's `radius: 16` pill. Sampled off
        // `M172 340 h158 l18 15 ...` in the target-app.svg composite
        // (deleted 2026-09-03).
        //
        // Unconsumed as of 2026-09-03: nothing reads `style.ticket`
        // (`widgets::pill` was deleted 2026-09-03). The `Ticket` type
        // survives as a parameter of `widgets::surface::{outline,
        // span_at, band_path}` and `Surface::ticket()`, but every
        // caller -- `bar.rs`, `screens::mail`, `Surface::{outlined,
        // filled, selected}` -- passes
        // `Ticket::default()`, and nothing calls `Surface::ticket()`.
        // Trace value would be a peaked chevron, not a pill --
        // `store-trace.svg` `#nav`
        // `M 0,39 V 19 L 18,0 L 27,3 H 214 Q 216,3 216,5 V 11 L 190,39 Z`
        // (216x39; the left edge stops 20 short of the top and the peak
        // is 18 in from it; 26x28 chamfer at the right), which
        // `Ticket { reach, drop }` cannot describe.
        ticket: Ticket {
            reach: 18.0,
            drop: 15.0,
        },
        // The dotted matrix, hollow square and hollow triangle that
        // head every shelf band and lead every socket row.
        glyphs: true,
        // --- controls --- (components.svg INPUT FIELD, ENTER BAR,
        // PROTECTED BAR, TAB CHEVRON)
        controls: Controls {
            // The sheet's ghost extrusion needs custom drawing.
            // Keep the coats until the complete treatment is wired.
            primary_states: Default::default(),
            ghost_states: Default::default(),
            field_states: Default::default(),
            // The role, not the login's mint: `ACCESS` pins ENTER to
            // `LIT` because the trace does, but the era's affirmative
            // fill everywhere else is the yellow, and the role is what
            // a theme can retint.
            primary: Coat::filled(Ink::Cta, Ink::OnSelect),
            // The mailbox chevrons: stroked 1.25 in the body teal.
            ghost: Coat::outlined(Ink::Fg, 1.25, Ink::Fg),
            // PROTECTED: near-ground fill, hairline edge one stop up,
            // mid-mint label.
            disabled: Coat::filled(Ink::Fixed(LOCKED), Ink::Fixed(ANNOTATION))
                .edged(Ink::Fixed(LOCKED_EDGE), 1.0),
            // The login well: solid dark teal, no outline.
            field: Coat::filled(Ink::Fixed(WELL), Ink::Fg),
            placeholder: Ink::Dim,
            // Login controls have source-fitted rounded path corners.
            radius: 2.0,
        },
        // --- end controls ---
        // --- login ---
        access: ACCESS,
        // --- end login ---
        // --- mailbox ---
        mailbox: mailbox(),
        // --- end mailbox ---
        // --- store ---
        store: STORE,
        store_selection: (1, 1),
        store_cursor: None,
        store_states: STORE_STATES,
        // --- end store ---
        // --- dashboard ---
        dashboard: DASHBOARD,
        dashboard_reference_fg: None,
        store_reference_fg: None,
        store_reference: None,
        // EVENTS, the fifth solid card in `dashboard-trace.svg` (group at
        // `translate(731 586) rotate(-30)`, the one `<use href="#card"
        // fill="#f5cb23" stroke="#fce89a">` under the comment "the
        // selection"); the other five fill `#2c9798`.
        dashboard_selection: 4,
        dashboard_cursor: false,
        mailbox_cursor: false,
        dashboard_states: DASHBOARD_STATES,
        dashboard_held_backdrops: DASHBOARD_HELD_BACKDROPS,
        // Both PRODUCTS blades (2 and 3) open the store; no blade says
        // "mail", and the mailbox is `m` from the hub instead
        // (`screens::hub`).
        dashboard_destinations: [None, None, Some(Destination::Store), Some(Destination::Store), None, None],
        // --- end dashboard ---
        metrics: Metrics {
            stroke: 1.5,
            gap: 20.0,
            pad: 18.0,
            ..Metrics::default()
        },
    }
}

// --- login ---
//
// The access screen, transcribed from `docs/kitsch/login-trace.svg` at
// 1600x900: a clock, a full-height bracket with a barcode standing in
// its foot, and three GUEST 7702 rows on a 393px pitch of which the
// first is live.
//
// The bar shape is the thing to notice. All three bars -- the mint
// ENTER and the two dark PROTECTED ones -- carry the same shoulder,
// their right two fifths standing 8px taller than their left. The
// input's lower edge follows the same rounded shoulder, keeping a
// 6.7px gap above ENTER. These native-source contours include the
// rounded joins that the earlier straight Step model omitted.

/// The teal a printed chip is filled with. Brighter and greener than
/// the era's `TEAL_SOLID` ornament colour, sampled off a 4x zoom of the
/// chip in the login photo.
pub const CHIP: iced::Color = rgb(0x1cb6ae);
/// The dark hexagon, wedge and marks printed on the chip.
pub const CHIP_INK: iced::Color = rgb(0x0e2b2a);
/// The recessed input well, and the lobe filling the elbow of the
/// bracket. Neither is a role: the era declares no `inset` because its
/// cards are unfilled outlines, and this screen is the one place it
/// sinks anything.
pub const WELL: iced::Color = rgb(0x162826);
pub const LOBE: iced::Color = rgb(0x0f2320);
/// The lit mint of the ENTER bar and the cursor, and the dark teal the
/// label on it is printed in.
pub const LIT: iced::Color = rgb(0x8afada);
pub const ON_LIT: iced::Color = rgb(0x0f3a33);
/// A PROTECTED bar: near-ground fill, a hairline edge one stop up.
pub const LOCKED: iced::Color = rgb(0x122724);
pub const LOCKED_EDGE: iced::Color = rgb(0x1d3f3a);
/// The mid mint the annotation text and the boxed letters are set in,
/// and the brighter one the guest names take.
pub const ANNOTATION: iced::Color = rgb(0x7fe0c8);
pub const NAME_INK: iced::Color = rgb(0xa9e6df);
/// The bright mint of the barcode and the footer line, and the teal of
/// the barcode's own label strip.
pub const BARCODE: iced::Color = rgb(0x8af0d8);
pub const BARCODE_TAB: iced::Color = rgb(0x16a49c);
/// The bright mint the *annotations* are set in.
///
/// Corrected 2026-09-03 by the trace's polish pass: the boxed letter
/// and the three lines beside it are the same `#82f0d3` as the foot
/// line and the barcode, drawn bold and wide, not the dimmer
/// [`ANNOTATION`] teal at a regular weight -- which came out 20% short
/// and carried a fifth of the photo's ink.
pub const BRIGHT: iced::Color = rgb(0x82f0d3);

const NOTE_1: &str = "ACCESS MANAGER WAS DEVE-";
const NOTE_2: &str = "LOPED BY SEOCHO. SERVING";
const NOTE_3: &str = "CUSTOMERS SINCE 2006.";

/// Native #50 retains the three-screen rose footprint, but its lower
/// page and the left cast are different. See docs/kitsch/ground-fit.md.
const LOGIN_PAGE: iced::Color = rgb(0x050604);
const LOGIN_BLOOM: &[(f32, iced::Color)] = &[
    (0.00, rgb(0xa4455e)),
    (0.20, rgb(0xa4455e)),
    (0.40, rgb(0xa34b5d)),
    (0.60, rgb(0x85354c)),
    (0.80, rgb(0x1a1215)),
    (1.00, LOGIN_PAGE),
];
const LOGIN_WASH: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x21231f)),
    (0.4, iced::Color { a: 0.85, ..rgb(0x202621) }),
    (0.7, iced::Color { a: 0.45, ..rgb(0x1f342a) }),
    (1.0, iced::Color { a: 0.0, ..rgb(0x5c8974) }),
];
const LOGIN_GROUND: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(LOGIN_PAGE)),
    Prim::Lobe { x: 750.0, y: -331.0, rx: 1600.0, ry: 929.0, stops: LOGIN_BLOOM },
    Prim::Lobe { x: 0.0, y: 434.0, rx: 700.0, ry: 433.0, stops: LOGIN_WASH },
];
const LOGIN_BACKDROP: &[Prim] = &[Prim::Soft { prims: LOGIN_GROUND }];

// Native source #50, 3840x2160 pixels divided by 2.4. The field has
// an 8px bottom step; each bar has the same rise at its top. Layout
// boxes remain available for input fitting while these curves govern
// painting and pointer hits. See login-trace.svg for source metrics.
const LOGIN_FIELD: &[Seg] = &[
    Seg::Line(589.5, 413.3),
    Seg::Quad { cx: 591.8, cy: 413.3, x: 591.8, y: 415.6 },
    Seg::Line(591.8, 453.4),
    Seg::Quad { cx: 591.8, cy: 455.7, x: 589.5, y: 455.7 },
    Seg::Line(432.0, 455.7),
    Seg::Quad { cx: 430.0, cy: 455.7, x: 428.4, y: 456.8 },
    Seg::Line(418.4, 462.6),
    Seg::Quad { cx: 416.8, cy: 463.7, x: 415.0, y: 463.7 },
    Seg::Line(259.5, 463.7),
    Seg::Quad { cx: 257.2, cy: 463.7, x: 257.2, y: 461.4 },
    Seg::Line(257.2, 415.6),
    Seg::Quad { cx: 257.2, cy: 413.3, x: 259.5, y: 413.3 },
];

const LOGIN_ENTER: &[Seg] = &[
    Seg::Line(416.4, 470.4),
    Seg::Quad { cx: 418.2, cy: 470.4, x: 419.8, y: 469.3 },
    Seg::Line(429.8, 463.5),
    Seg::Quad { cx: 431.4, cy: 462.4, x: 433.2, y: 462.4 },
    Seg::Line(589.8, 462.4),
    Seg::Quad { cx: 592.0, cy: 462.4, x: 592.0, y: 464.6 },
    Seg::Line(592.0, 495.5),
    Seg::Quad { cx: 592.0, cy: 497.7, x: 589.8, y: 497.7 },
    Seg::Line(259.2, 497.7),
    Seg::Quad { cx: 257.0, cy: 497.7, x: 257.0, y: 495.5 },
    Seg::Line(257.0, 472.6),
    Seg::Quad { cx: 257.0, cy: 470.4, x: 259.2, y: 470.4 },
];

const LOGIN_PROTECTED_2: &[Seg] = &[
    Seg::Line(809.4, 470.4),
    Seg::Quad { cx: 811.2, cy: 470.4, x: 812.8, y: 469.3 },
    Seg::Line(822.8, 463.5),
    Seg::Quad { cx: 824.4, cy: 462.4, x: 826.2, y: 462.4 },
    Seg::Line(982.3, 462.4),
    Seg::Quad { cx: 984.5, cy: 462.4, x: 984.5, y: 464.6 },
    Seg::Line(984.5, 495.5),
    Seg::Quad { cx: 984.5, cy: 497.7, x: 982.3, y: 497.7 },
    Seg::Line(651.8, 497.7),
    Seg::Quad { cx: 649.6, cy: 497.7, x: 649.6, y: 495.5 },
    Seg::Line(649.6, 472.6),
    Seg::Quad { cx: 649.6, cy: 470.4, x: 651.8, y: 470.4 },
];

const LOGIN_PROTECTED_3: &[Seg] = &[
    Seg::Line(1202.0, 470.4),
    Seg::Quad { cx: 1203.8, cy: 470.4, x: 1205.4, y: 469.3 },
    Seg::Line(1215.4, 463.5),
    Seg::Quad { cx: 1217.0, cy: 462.4, x: 1218.8, y: 462.4 },
    Seg::Line(1374.9, 462.4),
    Seg::Quad { cx: 1377.1, cy: 462.4, x: 1377.1, y: 464.6 },
    Seg::Line(1377.1, 495.5),
    Seg::Quad { cx: 1377.1, cy: 497.7, x: 1374.9, y: 497.7 },
    Seg::Line(1044.4, 497.7),
    Seg::Quad { cx: 1042.2, cy: 497.7, x: 1042.2, y: 495.5 },
    Seg::Line(1042.2, 472.6),
    Seg::Quad { cx: 1042.2, cy: 470.4, x: 1044.4, y: 470.4 },
];

pub const ACCESS: Access = Access {
    reference_fg: None,
    reference_backdrop: None,
    backdrop: LOGIN_BACKDROP,
    masthead: Masthead::Clock {
        labels: &[Legend::new("10:20 PM", 779.17, 74.9, 21.75, Ink::Fixed(rgb(0xb4ece3))).medium()],
    },
    slots: &[
        // Row 1, inside the bracket: the live one.
        Slot {
            mark: Some(Plate::filled(
                Plot::new(258.0, 338.0, 62.0, 61.0),
                Ink::Fixed(CHIP),
            )),
            emblem: Emblem::Chip,
            name: Some(
                Legend::new("GUEST 7702", 340.0, 355.0, 20.0, Ink::Fixed(NAME_INK)).medium(),
            ),
            badge: Some(Plate::outlined(
                Plot::new(339.3, 371.9, 24.5, 24.1),
                Ink::Fixed(BRIGHT),
                2.0,
            )),
            badge_letter: Some(
                Legend::new("A", 351.55, 390.4, 18.0, Ink::Fixed(BRIGHT))
                    .centred()
                    .medium()
                    .stretched(1.7),
            ),
            notes: &[
                Legend::new(NOTE_1, 372.1, 376.3, 8.0, Ink::Fixed(BRIGHT))
                    .bold()
                    .stretched(1.32),
                Legend::new(NOTE_2, 372.1, 385.5, 8.0, Ink::Fixed(BRIGHT))
                    .bold()
                    .stretched(1.32),
                Legend::new(NOTE_3, 372.1, 394.7, 8.0, Ink::Fixed(BRIGHT))
                    .bold()
                    .stretched(1.32),
            ],
            field: Some(
                Plate::filled(Plot::new(257.2, 413.3, 334.6, 50.4), Ink::Fixed(WELL))
                    .outlined_path((259.5, 413.3), LOGIN_FIELD),
            ),
            caret: Some(Plate::filled(
                Plot::new(266.0, 421.0, 2.0, 22.0),
                Ink::Fixed(BARCODE),
            )),
            // The trace shows the well empty but for the cursor, so the
            // rest run has no text; it says where a typed run goes: in
            // the cursor's mint, at the cursor's height (22, its y
            // 421..443), starting just past it, and the cursor trails.
            entry: Some(Entry {
                rest: Legend::new("", 272.0, 439.0, 22.0, Ink::Fixed(BARCODE)).medium(),
                mask: '*',
                tail: "",
                caret: Caret::Trails,
                blink: Blink::Caret,
                busy: "WAIT",
                failed: "DENIED",
            }),
            action: Some(
                Plate::filled(Plot::new(257.0, 462.4, 335.0, 35.3), Ink::Fixed(LIT))
                    .outlined_path((259.2, 470.4), LOGIN_ENTER),
            ),
            action_label: Some(
                Legend {
                    weight: iced::font::Weight::Medium,
                    ..Legend::new("ENTER", 510.625, 484.0, 12.7, Ink::Fixed(ON_LIT))
                        .centred()
                        .stretched(1.77)
                },
            ),
            ..Slot::EMPTY
        },
        // Rows 2 and 3: the same card, its bar dark and its label
        // PROTECTED. No field and no cursor.
        Slot {
            mark: Some(Plate::filled(
                Plot::new(651.0, 338.0, 62.0, 61.0),
                Ink::Fixed(CHIP),
            )),
            emblem: Emblem::Chip,
            name: Some(
                Legend::new("GUEST 7702", 733.0, 355.0, 20.0, Ink::Fixed(NAME_INK)).medium(),
            ),
            badge: Some(Plate::outlined(
                Plot::new(732.3, 371.9, 24.5, 24.1),
                Ink::Fixed(BRIGHT),
                2.0,
            )),
            badge_letter: Some(
                Legend::new("A", 744.55, 390.4, 18.0, Ink::Fixed(BRIGHT))
                    .centred()
                    .medium()
                    .stretched(1.7),
            ),
            notes: &[
                Legend::new(NOTE_1, 765.1, 376.3, 8.0, Ink::Fixed(BRIGHT))
                    .bold()
                    .stretched(1.32),
                Legend::new(NOTE_2, 765.1, 385.5, 8.0, Ink::Fixed(BRIGHT))
                    .bold()
                    .stretched(1.32),
                Legend::new(NOTE_3, 765.1, 394.7, 8.0, Ink::Fixed(BRIGHT))
                    .bold()
                    .stretched(1.32),
            ],
            action: Some(
                Plate::filled(Plot::new(649.6, 462.4, 334.9, 35.3), Ink::Fixed(LOCKED))
                    .outlined_path((651.8, 470.4), LOGIN_PROTECTED_2)
                    .edged(Ink::Fixed(LOCKED_EDGE), 1.0),
            ),
            action_label: Some(
                Legend {
                    weight: iced::font::Weight::Semibold,
                    ..Legend::new("PROTECTED", 903.54, 484.35, 13.2, Ink::Fixed(ANNOTATION))
                        .centred()
                        .stretched(1.7)
                        .tracked(0.15)
                },
            ),
            ..Slot::EMPTY
        },
        Slot {
            mark: Some(Plate::filled(
                Plot::new(1043.0, 338.0, 62.0, 61.0),
                Ink::Fixed(CHIP),
            )),
            emblem: Emblem::Chip,
            name: Some(
                Legend::new("GUEST 7702", 1125.0, 355.0, 20.0, Ink::Fixed(NAME_INK)).medium(),
            ),
            badge: Some(Plate::outlined(
                Plot::new(1124.3, 371.9, 24.5, 24.1),
                Ink::Fixed(BRIGHT),
                2.0,
            )),
            badge_letter: Some(
                Legend::new("A", 1136.55, 390.4, 18.0, Ink::Fixed(BRIGHT))
                    .centred()
                    .medium()
                    .stretched(1.7),
            ),
            notes: &[
                Legend::new(NOTE_1, 1157.1, 376.3, 8.0, Ink::Fixed(BRIGHT))
                    .bold()
                    .stretched(1.32),
                Legend::new(NOTE_2, 1157.1, 385.5, 8.0, Ink::Fixed(BRIGHT))
                    .bold()
                    .stretched(1.32),
                Legend::new(NOTE_3, 1157.1, 394.7, 8.0, Ink::Fixed(BRIGHT))
                    .bold()
                    .stretched(1.32),
            ],
            action: Some(
                Plate::filled(Plot::new(1042.2, 462.4, 334.9, 35.3), Ink::Fixed(LOCKED))
                    .outlined_path((1044.4, 470.4), LOGIN_PROTECTED_3)
                    .edged(Ink::Fixed(LOCKED_EDGE), 1.0),
            ),
            action_label: Some(
                Legend {
                    weight: iced::font::Weight::Semibold,
                    ..Legend::new("PROTECTED", 1296.14, 484.35, 13.2, Ink::Fixed(ANNOTATION))
                        .centred()
                        .stretched(1.7)
                        .tracked(0.15)
                },
            ),
            ..Slot::EMPTY
        },
    ],
    // The bracket runs the full height of the frame -- it fades into
    // the rose above y~130 but reaches the top edge -- breaks into a
    // ~57-degree diagonal at y 540 and rounds into its foot at y 731.
    fixture: Fixture::Bracket {
        left: 228.5,
        right: 611.5,
        knee: 540.0,
        foot: 731.0,
        barcode: Plot::new(370.0, 632.0, 220.0, 63.0),
        labels: &[
            Legend::new("0033 05 64 08 CP", 375.5, 693.0, 5.5, Ink::Fixed(CHIP_INK)).turned(),
            Legend::new("12345678123456789", 420.0, 696.0, 13.0, Ink::Fixed(BARCODE)),
        ],
    },
    colophon: Colophon::Notice {
        labels: &[
            // One weight for the whole line: the notice is as bold as
            // the brand in the photo (2026-09-03; drawn regular before,
            // at 0.116 coverage against the photo's 0.168).
            Legend::new("ARASAKA CONSUMER TECHNOLOGY", 503.0, 870.0, 9.0, Ink::Fixed(BRIGHT))
                .bold()
                .tracked(0.25),
            Legend::new(
                "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE.",
                641.0,
                870.0,
                9.0,
                Ink::Fixed(BRIGHT),
            )
            .bold()
            .tracked(0.24),
        ],
    },
};
// --- end login ---
// --- mailbox ---
//
// `docs/kitsch/mailbox-trace.svg`, read at its 1600x900 frame. The rose
// bloom and the grey-green left wash are `MAIL_GROUND`, from this
// trace's own defs (:146-147); until 2026-09-04 this header said they
// were `Ground::Bloom`'s business, and that generic disc out of the top
// right left a flat maroon `47 15 24` over the whole frame where the
// trace is page black by y 450 -- `triptych.sh --diff` lit every pixel.
// Everything else in the trace is below.
//
// Two shapes the era's `Corner::Round { radius: 16 }` cannot say, both
// carried as per-piece [`Trim`]s or as polylines: the USER and
// DESCRIPTION boxes step their bottom edge (y 228 on the right, y 236
// under the first 55px, joined by a short diagonal), and the selected
// row's body cuts a *diagonal* trailing corner on an era that rounds
// everything else.
use crate::style::{
    Change, Frame, Mail, MailBadges, MailButtons, MailEnvelope, MailList, MailRowType, MailRowCoat, MailRowEcho, MailRowStates, MailMotion, MailPanel, MailPart,
    Mailbox, Motion, Note, Piece, RowDecor, Run, Trim, FromAt, BL, BR, TL, TR,
};
use iced::animation::Easing;

const fn text(x: f32, y: f32, size: f32, ink: Ink, s: &'static str) -> Piece {
    Piece::Label(Note {
        at: Run::new(x, y, size, ink),
        text: s,
    })
}

/// Native #51's clear pixels agree with #49 and #52, so all three use
/// the same broad page, rose, and left wash; see ground-fit.md.
const MAIL_GROUND: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(PAGE)),
    Prim::Lobe { x: 750.0, y: -331.0, rx: 1600.0, ry: 929.0, stops: ROSE },
    Prim::Lobe { x: 0.0, y: 393.0, rx: 535.0, ry: 400.0, stops: MARGIN },
];
const MAIL_BACKDROP: &[Prim] = &[Prim::Soft { prims: MAIL_GROUND }];

/// The in-fiction micro-print: the re-cut trace measures it at size 8
/// weight 600 in the bright mint, not the dim teal an earlier pass set
/// (a quarter of the ink, 25% narrow).
const fn mid(x: f32, y: f32, s: &'static str) -> Piece {
    Piece::Label(Note {
        at: Run::new(x, y, 8.0, Ink::Fg).bold(),
        text: s,
    })
}

const fn letter(x: f32, y: f32, s: &'static str) -> Piece {
    Piece::Label(Note {
        at: Run::new(x, y, 18.0, Ink::Fg).bold().centered(),
        text: s,
    })
}

/// The USER box, and the DESCRIPTION box beside it: a rectangle whose
/// bottom edge steps down under its first 55px.
static USER_BOX: [(f32, f32); 6] = [
    (155.5, 185.5),
    (349.5, 185.5),
    (349.5, 228.5),
    (220.0, 228.5),
    (208.0, 236.5),
    (155.5, 236.5),
];
static DESC_BOX: [(f32, f32); 6] = [
    (575.5, 185.5),
    (769.5, 185.5),
    (769.5, 228.5),
    (640.0, 228.5),
    (628.0, 236.5),
    (575.5, 236.5),
];

/// The list bracket: a top edge that fades right, a rounded corner, a
/// left edge that FORKS at y~600, and the solid teal wave between the
/// two branches. The trace's quadratics are stepped into short
/// segments; at 2px they read the same.
static BRACKET: [(f32, f32); 16] = [
    (402.0, 268.0),
    (140.0, 268.0),
    (127.0, 271.0),
    (121.0, 281.0),
    (120.5, 288.0),
    (120.5, 606.0),
    (122.0, 628.0),
    (138.0, 642.0),
    (158.0, 644.0),
    (278.0, 644.0),
    (288.0, 647.0),
    (296.0, 653.0),
    (362.0, 741.0),
    (368.0, 747.0),
    (376.0, 748.0),
    (451.0, 748.0),
];
static BRACKET_FORK: [(f32, f32); 5] = [
    (120.5, 606.0),
    (120.5, 690.0),
    (124.0, 726.0),
    (146.0, 745.0),
    (180.0, 748.5),
];
static WAVE: [(f32, f32); 14] = [
    (120.5, 606.0),
    (120.5, 628.0),
    (132.0, 641.0),
    (158.0, 644.0),
    (278.0, 644.0),
    (288.0, 647.0),
    (296.0, 653.0),
    (362.0, 741.0),
    (368.0, 747.0),
    (376.0, 748.0),
    (180.0, 748.0),
    (146.0, 745.0),
    (124.0, 726.0),
    (120.5, 690.0),
];
/// The outlined flag band hanging off the message tab's left edge.
static FLAG: [(f32, f32); 5] = [
    (575.0, 327.0),
    (540.0, 363.0),
    (540.0, 384.2),
    (1104.0, 384.2),
    (1127.5, 360.7),
];

static CHROME: [Piece; 26] = [
    mid(205.0, 110.0, "SPARE TIME MANAGER WAS DEVELO-"),
    mid(205.0, 119.0, "PED BY SEOCHO. SERVING CUSTO-"),
    mid(205.0, 128.0, "MERS SINCE 2006."),
    mid(624.6, 110.0, "MAPS ARE PROVIDED BY SEOCHO."),
    mid(624.6, 119.0, "SATELITE SERVICES SINCE 2006."),
    mid(1259.1, 110.0, "SPARE TIME MANAGER WAS DEVELO-"),
    mid(1259.1, 119.0, "PED BY SEOCHO. SERVING CUSTO-"),
    mid(1259.1, 128.0, "MERS SINCE 2006."),
    Piece::Box {
        at: Frame::new(165.2, 106.2, 24.1, 23.8),
        fill: None,
        stroke: Some(Ink::Fg),
        width: 1.5,
        trim: Trim::NONE,
    },
    letter(177.25, 125.5, "A"),
    Piece::Label(Note { at: Run::new(166.0, 159.0, 12.3, Ink::Fg).bold(), text: "USER" }),
    Piece::Box {
        at: Frame::new(586.2, 106.2, 24.2, 23.8),
        fill: None,
        stroke: Some(Ink::Fg),
        width: 1.5,
        trim: Trim::NONE,
    },
    letter(598.3, 125.5, "B"),
    Piece::Label(Note { at: Run::new(587.0, 159.0, 12.3, Ink::Fg).bold(), text: "DESCRIPTION" }),
    Piece::Box {
        at: Frame::new(1216.6, 106.2, 24.4, 23.8),
        fill: None,
        stroke: Some(Ink::Fg),
        width: 1.5,
        trim: Trim::NONE,
    },
    letter(1228.8, 125.5, "C"),
    Piece::Label(Note { at: Run::new(1217.0, 159.0, 12.3, Ink::Fg).bold(), text: "SECURITY LEVEL" }),
    Piece::Poly {
        points: &USER_BOX,
        fill: None,
        stroke: Some(Ink::Fg),
        width: 1.25,
        close: true,
    },
    Piece::Poly {
        points: &DESC_BOX,
        fill: None,
        stroke: Some(Ink::Fg),
        width: 1.25,
        close: true,
    },
    Piece::Label(Note {
        at: Run::new(166.0, 213.0, 20.0, Ink::Fg).bold(),
        text: "GUES 7702",
    }),
    Piece::Label(Note {
        at: Run::new(588.0, 213.0, 20.0, Ink::Fg).bold(),
        text: "MAILBOX",
    }),
    Piece::Poly {
        points: &WAVE,
        fill: Some(Ink::Ornament),
        stroke: None,
        width: 0.0,
        close: true,
    },
    Piece::Poly {
        points: &BRACKET,
        fill: None,
        stroke: Some(Ink::Fg),
        width: 1.25,
        close: false,
    },
    Piece::Poly {
        points: &BRACKET_FORK,
        fill: None,
        stroke: Some(Ink::Fg),
        width: 1.25,
        close: false,
    },
    Piece::Label(Note {
        at: Run::new(503.0, 870.0, 9.0, Ink::Fg).bold(),
        text: "ARASAKA CONSUMER TECHNOLOGY",
    }),
    Piece::Label(Note {
        at: Run::new(641.0, 870.0, 9.0, Ink::Fg).bold(),
        text:
        "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE.",
    }),
];

/// The message's flag band and the two lines of micro-print on it:
/// the message's, extruded with it (`MAILBOX_MOTIONS`), not chrome.
static MESSAGE_FLAG: [Piece; 3] = [
    Piece::Poly {
        points: &FLAG,
        fill: None,
        stroke: Some(Ink::Select),
        width: 1.25,
        close: false,
    },
    Piece::Label(Note {
        at: Run::new(578.4, 366.1, 8.0, Ink::Fixed(rgb(0xe6b522))).semibold().stretched(1.09),
        text: "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE",
    }),
    Piece::Label(Note {
        at: Run::new(578.4, 374.0, 8.0, Ink::Fixed(rgb(0xe6b522))).semibold().stretched(1.09),
        text: "ALLOWED TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE.",
    }),
];

/// Dark selected printing uses a lighter sender face than bright idle printing.
/// The selected template follows selection, including after changing messages.
const MAIL_SELECTED_ROW_TYPE: MailRowType = MailRowType {
    title: Run::new(220.0, 27.0, 18.0, Ink::Fg),
    from: Run::new(220.0, 48.0, 13.5, Ink::Mid).semibold(),
};
static MAIL_ROW_TYPE: [MailRowType; 5] = [MailRowType {
    from: Run::new(220.0, 48.0, 13.5, Ink::Mid).bold(),
    ..MAIL_SELECTED_ROW_TYPE
}; 5];

/// `#message-extrude` (trace lines 164-170): the message -- its tab
/// with the title on it, its flag, the body outline and paragraphs,
/// and the DETAILS column
/// beside it -- is wiped on from the left over 0.45 s from 0,
/// `keySplines="0.33 1 0.68 1"` = EaseOutCubic, the way the hub's
/// depth extrudes; the list and the badges stand from frame 0. The
/// rect x 534 y 300 w 850 h 456 takes the whole group with margin.
pub const MAILBOX_MOTIONS: &[MailMotion] = &[MailMotion {
    motion: Motion {
        id: "message-extrude",
        begin: 0,
        dur: 450,
        ease: Easing::EaseOutCubic,
        change: Change::Clip { x: 534.0, y: 300.0, w: (0.0, 850.0), h: (456.0, 456.0) },
    },
    parts: &[MailPart::Pieces(&MESSAGE_FLAG), MailPart::Panel, MailPart::Title, MailPart::Buttons],
}];

static TABS: [&str; 4] = ["DETAILS", "MODS", "PRICE", "DAMAGE"];
static LEVELS: [&str; 4] = ["01", "02", "03", "04"];

/// Source #51 has an open flap on the first row, closed on the other four.
/// The open art is measured separately from the common envelope fallback;
/// `title_upper` preserves the photographed capital subjects.
// Four closed source symbols share a thin outline and both lower folds.
// The small local y offset preserves the message row/text pitch.
const MAIL_CLOSED_ENVELOPE: &[Piece] = &[
    Piece::Poly {
        points: &[(1.95, 1.5), (18.0, 1.5), (18.0, 11.45), (1.95, 11.45)],
        fill: None, stroke: Some(Ink::Fg), width: 0.9, close: true,
    },
    Piece::Poly {
        points: &[(1.95, 1.5), (10.0, 8.25), (18.0, 1.5)],
        fill: None, stroke: Some(Ink::Fg), width: 0.9, close: false,
    },
    Piece::Poly {
        points: &[(1.95, 11.45), (7.2, 5.9)],
        fill: None, stroke: Some(Ink::Fg), width: 0.9, close: false,
    },
    Piece::Poly {
        points: &[(18.0, 11.45), (12.75, 5.93)],
        fill: None, stroke: Some(Ink::Fg), width: 0.9, close: false,
    },
];
const MAIL_OPEN_ENVELOPE: &[Piece] = &[
    Piece::Poly {
        points: &[(1.95, 2.2), (10.0, -2.5), (18.0, 2.2), (18.0, 12.0), (1.95, 12.0)],
        fill: None, stroke: Some(Ink::Fg), width: 0.7, close: true,
    },
    Piece::Poly {
        points: &[(1.95, 2.2), (10.0, 7.7), (18.0, 2.2)],
        fill: None, stroke: Some(Ink::Fg), width: 0.7, close: false,
    },
];
static ROWS: [Mail; 5] = [
    Mail { subject: "You'll regret that", from: "Jackie", unread: true },
    Mail { subject: "Urgent information (!)", from: "Mom", unread: false },
    Mail { subject: "Heist data sent to you", from: "805000451", unread: false },
    Mail { subject: "I'm worried man", from: "Rachel Ross", unread: false },
    Mail { subject: "Special offer to you!", from: "JINX JINX STORE", unread: false },
];

/// The body, trace lines 267-276: 5 + 2 + 3 lines. This era splits the
/// lorem after "laborum." rather than after "explicabo.", and has no
/// "Nemo enim" paragraph at all.
static PARAGRAPHS: [&[&str]; 3] = [
    &[
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod",
        "tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim",
        "veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea com-",
        "modo consequat. Duis aute irure dolor in reprehenderit in voluptate velit",
        "esse cillum dolore eu fugiat nulla pariatur.",
    ],
    &[
        "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia de-",
        "serunt mollit anim id est laborum.",
    ],
    &[
        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium",
        "doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inven-",
        "tore veritatis et quasi architecto beatae vitae dicta sunt explicabo.",
    ],
];

// Inferred component-sheet row lift; both traced pieces retain their own trim.
const MAIL_GHOST: MailRowEcho = MailRowEcho {
    rings: 1, step: Frame::new(20.0, -20.0, 0.0, 0.0),
    fill: Some(Ink::Fixed(rgb(0x0f9f80))), fill_alpha: 0.58,
    ink: Ink::Fixed(rgb(0x6cc4bd)), width: 1.2, alpha: 0.80, fade: 0.0,
};

pub fn mailbox() -> Mailbox {
    Mailbox {
        text_baseline: 0.95,
        backdrop: MAIL_BACKDROP,
        chrome: &CHROME,
        overlay: &[],
        list: MailList {
            footer: &[],
            frame: None,
            frame_ink: Ink::Fg,
            frame_width: 0.0,
            // five rows on a 60px pitch inside the bracket; nothing is
            // drawn behind an unselected one
            row: Frame::new(154.0, 313.0, 338.0, 38.0),
            pitch: 59.8,
            rows: &ROWS,
            selected: 0,
            decor: RowDecor::Bare,
            feedback: Some(MailRowStates {
                hover: MailRowCoat {
                    fill: Some(Ink::Fixed(rgb(0x2c9798))),
                    outline: Some(Ink::Fixed(rgb(0xa9e6df))),
                    printing: Some(Ink::Fixed(rgb(0x123c38))),
                    sender: Some(Ink::Fixed(rgb(0x7fe0c8))),
                    spine: None, selection: true, echo: Some(MAIL_GHOST),
                },
                pressed: MailRowCoat {
                    fill: Some(Ink::Fixed(rgb(0xe8c21f))), outline: None,
                    printing: Some(Ink::Fixed(rgb(0x4a3a05))),
                    sender: Some(Ink::Fixed(rgb(0xf2c825))),
                    spine: None, selection: true, echo: None,
                },
                selected_hover: Some(MailRowCoat {
                    fill: None, outline: None, printing: None, sender: None,
                    spine: None, selection: true, echo: Some(MAIL_GHOST),
                }),
            }),
            row_fill: None,
            row_fills: &[],
            row_stroke: None,
            row_width: 1.8,
            row_trim: Trim::NONE,
            spine: None,
            rule: None,
            rule_ink: Ink::Fg,
            tab: None,
            tab_ink: Ink::Select,
            // the selection is solid yellow in two pieces split by a
            // 2px gap at x 196
            sel: Frame::new(197.0, 313.0, 294.0, 33.0),
            sel_trim: Trim::chamfer(BR, 20.0),
            sel_icon: Some(Frame::new(154.0, 313.0, 41.0, 38.0)),
            sel_icon_trim: Trim::chamfer(BR, 12.0),
            // the row's yellow (:224) is a stop deeper than the tab's
            // `#fbd42c` (:256) and `select`'s #fcc428; the DETAILS chevron
            // takes the row's (:280). All three drew `select` until 2026-09-04
            sel_fill: Ink::Fixed(rgb(0xe8c21f)),
            sel_notch: None,
            veneer: None,
            envelope: Some(MailEnvelope { normal: MAIL_CLOSED_ENVELOPE, open: MAIL_OPEN_ENVELOPE }),
            glyph_x: 165.0,
            glyph_dy: 12.0,
            glyph_offsets: &[],
            glyph_w: 20.0,
            text_x: 220.0,
            title_dy: 27.0,
            title_size: 18.0,
            row_type: &MAIL_ROW_TYPE,
            selected_row_type: Some(MAIL_SELECTED_ROW_TYPE),
            title_bold: false,
            title_ink: Ink::Fg,
            selected_ink: Ink::OnSelect,
            selected_printing: None,
            from_dy: 48.0,
            from_size: 13.5,
            from_ink: Ink::Fixed(rgb(0x87f9d8)),
            from_at: FromAt::Beneath,
            from_prefix: "from: ",
            title_upper: true,
            from_upper: false,
            new_pill: None,
            new_pill_selected: None,
            new_pill_art: &[],
            icons: None,
        },
        panel: MailPanel {
            // the body outline, bottom corners r~8, under a solid tab
            // whose top-right corner is chamfered
            frame: Some(Frame::new(576.0, 384.0, 551.0, 364.0)),
            frame_fill: None,
            frame_stroke: Some(Ink::Select),
            frame_width: 1.25,
            frame_trim: Trim::round(BL | BR, 8.0),
            head: Some(Frame::new(575.0, 313.0, 552.0, 36.0)),
            head_ink: Ink::Fixed(rgb(0xfbd42c)),
            head_trim: Trim::chamfer(TR, 22.0),
            // the tab reads the selected row's own subject, trace line
            // 257, and no sender under it
            message: 0,
            title: Run::new(590.0, 340.0, 17.0, Ink::OnSelect),
            title_upper: true,
            from: None,
            heading: None,
            sender: None,
            body: Run::new(592.0, 411.0, 16.8, Ink::Select),
            line: 19.0,
            para: 38.0,
            paragraphs: &PARAGRAPHS,
            paragraph_baselines: &[],
        },
        buttons: MailButtons {
            // four chevron tabs stacked down the right, where the other
            // eras put a row of buttons
            first: Frame::new(1216.0, 306.0, 161.0, 46.0),
            dx: 0.0,
            dy: 54.0,
            count: 4,
            filled: Some(0),
            fill: Ink::Fixed(rgb(0xe6c020)),
            idle_fill: None,
            joined: false,
            chevron: true,
            trim: Trim::NONE,
            width: 1.25,
            stroke: Ink::Fg,
            label: Run::new(27.0, 33.0, 16.0, Ink::Fg),
            tab: None,
            labels: &TABS,
        },
        badges: MailBadges {
            first: Frame::new(1215.5, 190.5, 56.0, 34.0),
            dx: 60.33,
            dy: 0.0,
            cols: 4,
            count: 4,
            selected: Some(1),
            trim: Trim::round(TL | TR | BR | BL, 2.0),
            width: 1.0,
            fill: None,
            stroke: Ink::Fg,
            label: Run::new(27.5, 24.5, 23.0, Ink::Fg).bold().centered(),
            label_runs: &[],
            caption: None,
            caption_text: "",
            labels: &LEVELS,
        },
        motions: MAILBOX_MOTIONS,
    }
}
// --- end mailbox ---

// --- store ---------------------------------------------------------------
//
// `docs/kitsch/store-trace.svg`, transcribed. Coordinates are the
// trace's own in the 1600x900 frame, measured off
// `images/kitsch-store.png`; each card is placed with `Prim::At` at the
// trace's own `<use x= y=>`, so a figure here reads against the SVG
// line it came from.
//
// What is not transcribed, and why: the two fading duplicate strokes of
// the nav bracket (`url(#fadeR)`, a gradient stroke -- the solid run
// under them is drawn), and the rose bloom's exact radial falloff,
// which is approximated by concentric bands because iced's canvas has
// only linear gradients. The bloom is ground rather than ink, but it is
// ground the extractor's palette split depends on -- the source spends
// five of its eight clusters on it -- so leaving the page flat is not
// the neutral choice it looks like.

use crate::style::{
    fill_path, fill_rect, line_path, line_rect, shut_path, txt, txt_bold, txt_end, tracked_mid, txt_mid,
    Group, Prim, Seg,
};

/// The gun drawing and the stat bar under the figures.
pub const GUN: iced::Color = rgb(0x93ffe4);
pub const MINT_BAR: iced::Color = rgb(0x81fee7);
pub const ON_MINT_BAR: iced::Color = rgb(0x123c38);
/// The grown card's own amber, its text, and its gun.
pub const GROWN_FILL: iced::Color = rgb(0xffc233);
pub const ON_GROWN: iced::Color = rgb(0x8a3f28);
pub const GROWN_GUN: iced::Color = rgb(0x3a2408);
pub const GROWN_OUTLINE: iced::Color = rgb(0xe2a408);
pub const GROWN_DETAIL: iced::Color = rgb(0xfabe29);
pub const GROWN_MICRO: iced::Color = rgb(0xe3bd20);
pub const BAND_RULE: iced::Color = rgb(0xc9931a);
/// The band, and the dark ink its marks and tag are set in.
pub const BAND: iced::Color = rgb(0xfec32f);
pub const ON_BAND: iced::Color = rgb(0x5a3a08);
/// The photographed warning print is lighter than the four main symbols.
pub const BAND_MICRO_INK: iced::Color = rgb(0xb38a39);
/// The solid teal the nav bracket ends in.
pub const WAVE_INK: iced::Color = rgb(0x1bb6a3);
/// The card's compliance micro-text, the footnote bodies, the marker
/// boxes and the bright line centred at the foot.
pub const MICRO: iced::Color = rgb(0x5fc9b5);
/// The two CC35 lines under a card, measured separately from footnotes.
pub const CARD_COMPLIANCE: iced::Color = rgb(0x65e5c8);
pub const GROWN_COMPLIANCE: iced::Color = rgb(0xe9a50d);
pub const MARK: iced::Color = rgb(0x7fd4cc);
pub const FOOT_MICRO: iced::Color = rgb(0x82f0d3);
/// The logotype's heavy mint.
pub const LOGO: iced::Color = rgb(0x8ff2dc);
/// Measured shared ground for native #49/#51/#52. The rose ellipse is
/// centred at (750,-331) with radii (1600,929); the edge wash is centred
/// at (0,393) with radii (535,400). See docs/kitsch/ground-fit.md.
pub const PAGE: iced::Color = rgb(0x0c0c0b);

const ROSE: &[(f32, iced::Color)] = &[
    (0.00, rgb(0xad465f)),
    (0.20, rgb(0xad465f)),
    (0.40, rgb(0xae4c60)),
    (0.60, rgb(0x8c334d)),
    (0.80, rgb(0x16100f)),
    (1.00, PAGE),
];
const MARGIN: &[(f32, iced::Color)] = &[
    (0.00, rgb(0x1f211c)),
    (0.40, iced::Color { a: 0.85, ..rgb(0x202620) }),
    (0.70, iced::Color { a: 0.45, ..rgb(0x15281e) }),
    (1.00, iced::Color { a: 0.0, ..rgb(0x26884f) }),
];

/// The backdrop. Ground rather than ink -- but ground the extractor's
/// palette split depends on: the source spends five of its eight
/// clusters on it, so leaving the page flat is not the neutral choice
/// it looks like. Composited since 2026-09-05, as the mailbox's and
/// the hub's already were; until then the store was the last kitsch
/// screen whose lobes the canvas drew as annuli, and `MARGIN`'s alpha
/// stops landed on the rose through the linear blend rather than the
/// trace's sRGB one.
const BACKDROP: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(PAGE)),
    Prim::Lobe { x: 750.0, y: -331.0, rx: 1600.0, ry: 929.0, stops: ROSE },
    Prim::Lobe { x: 0.0, y: 393.0, rx: 535.0, ry: 400.0, stops: MARGIN },
];

/// The nav chevron, 216x39 at its own origin: the left edge rises to a
/// peak, the right end chamfers back.
const CHEVRON: &[Seg] = &[
    Seg::Line(0.0, 19.0),
    Seg::Line(18.0, 0.0),
    Seg::Line(27.0, 3.0),
    Seg::Line(214.0, 3.0),
    Seg::Quad { cx: 216.0, cy: 3.0, x: 216.0, y: 5.0 },
    Seg::Line(216.0, 11.0),
    Seg::Line(190.0, 39.0),
];

const NAV_OUTLINE: &[Prim] = &[shut_path(0.0, 39.0, CHEVRON, Ink::Border, 1.5)];
const NAV_SOLID: &[Prim] = &[fill_path(0.0, 39.0, CHEVRON, Ink::Select)];

// `components.svg` #k-chevron-hover: inferred from the fan's first
// ghost step, not a cursor state photographed in the source. Keep the
// rest silhouette, paint its ghost (+20,-20), then the opaque idle slab.
// These foreground alphas use the canvas blend, not `Soft`'s sRGB blend.
const NAV_GHOST: &[Prim] = &[Prim::Path {
    x: 0.0, y: 39.0, segs: CHEVRON, close: true,
    fill: Some(Ink::Fixed(iced::Color { a: 0.58, ..rgb(0x0f9f80) })),
    stroke: Some(Ink::Fixed(iced::Color { a: 0.80, ..rgb(0x6cc4bd) })),
    width: 1.2,
}];
const NAV_LIFT: &[Prim] = &[
    Prim::At { x: 20.0, y: -20.0, prims: NAV_GHOST },
    Prim::Path {
        x: 0.0, y: 39.0, segs: CHEVRON, close: true,
        fill: Some(Ink::Fixed(rgb(0x2c9798))),
        stroke: Some(Ink::Fixed(rgb(0xa9e6df))),
        width: 1.8,
    },
];

/// The bracket's solid wave, and the single-stroke run of the bracket
/// itself: a top line, an S-bend around the customer block, the long
/// left edge, and the wave's own top and right side.
const WAVE_BODY: &[Seg] = &[
    Seg::Quad { cx: 106.5, cy: 617.0, x: 150.0, y: 617.0 },
    Seg::Line(312.0, 617.0),
    Seg::Quad { cx: 345.0, cy: 617.0, x: 345.0, y: 650.0 },
    Seg::Line(345.0, 685.0),
    Seg::Quad { cx: 345.0, cy: 716.0, x: 375.0, y: 718.0 },
    Seg::Line(310.0, 718.0),
    Seg::Quad { cx: 110.0, cy: 718.0, x: 106.5, y: 575.0 },
];
const BRACKET_PATH: &[Seg] = &[
    Seg::Line(372.0, 186.0),
    Seg::Quad { cx: 348.0, cy: 186.0, x: 348.0, y: 215.0 },
    Seg::Line(348.0, 235.0),
    Seg::Quad { cx: 348.0, cy: 268.0, x: 313.0, y: 268.0 },
    Seg::Line(136.0, 268.0),
    Seg::Quad { cx: 106.5, cy: 268.0, x: 106.5, y: 298.0 },
    Seg::Line(106.5, 575.0),
    Seg::Quad { cx: 106.5, cy: 617.0, x: 150.0, y: 617.0 },
    Seg::Line(312.0, 617.0),
    Seg::Quad { cx: 345.0, cy: 617.0, x: 345.0, y: 650.0 },
    Seg::Line(345.0, 685.0),
    Seg::Quad { cx: 345.0, cy: 716.0, x: 375.0, y: 718.0 },
];

/// The logotype's outlined T, x 280..318 with its stem at 292..306.
const TEE: &[Seg] = &[
    Seg::Line(318.0, 88.0),
    Seg::Line(318.0, 99.0),
    Seg::Line(306.0, 99.0),
    Seg::Line(306.0, 132.0),
    Seg::Line(292.0, 132.0),
    Seg::Line(292.0, 99.0),
    Seg::Line(280.0, 99.0),
];

/// The card outline: an r6 top-left, a 24px top-right chamfer, and
/// source-measured 10.5px quadratic lower corners at the socket row's foot.
const CARD_EDGE: &[Seg] = &[
    Seg::Line(237.0, 0.0),
    Seg::Line(261.0, 24.0),
    Seg::Line(261.0, 310.0),
    Seg::Quad { cx: 261.0, cy: 320.5, x: 250.5, y: 320.5 },
    Seg::Line(10.5, 320.5),
    Seg::Quad { cx: 0.0, cy: 320.5, x: 0.0, y: 310.0 },
    Seg::Line(0.0, 6.0),
    Seg::Quad { cx: 0.0, cy: 0.0, x: 6.0, y: 0.0 },
];
/// The yellow band with its left flag: it pokes 27px past the card's
/// edge, peaks at (-3,50) and chamfers its trailing corner back.
const BAND_SHAPE: &[Seg] = &[
    Seg::Line(-27.0, 72.0),
    Seg::Line(-3.0, 50.0),
    Seg::Line(-3.0, 59.0),
    Seg::Line(256.0, 59.0),
    Seg::Quad { cx: 258.0, cy: 59.0, x: 258.0, y: 61.0 },
    Seg::Line(258.0, 64.0),
    Seg::Line(233.0, 94.0),
];
/// The mint bar under the stat figures: its left 65px hang 8px lower
/// through a diagonal. This is the shape the extractor reads as a pair
/// of overlapping diamonds -- the class that carries 15% of the design.
const MINT_SHAPE: &[Seg] = &[
    Seg::Line(254.0, 232.0),
    Seg::Quad { cx: 258.0, cy: 232.0, x: 258.0, y: 236.0 },
    Seg::Line(258.0, 262.0),
    Seg::Line(72.0, 262.0),
    Seg::Line(65.0, 270.0),
    Seg::Line(8.0, 270.0),
    Seg::Quad { cx: 4.0, cy: 270.0, x: 4.0, y: 266.0 },
    Seg::Line(4.0, 236.0),
    Seg::Quad { cx: 4.0, cy: 232.0, x: 8.0, y: 232.0 },
];

#[path = "kitsch_store_art.rs"]
mod store_art;

// The first two paths reuse the Entropism MAGNUM source geometry with
// Kitsch's measured plain-card offset. A Kitsch-only dark mask restores
// source rail, receiver and grip seams that this photograph prints more
// sharply than the shared mint body.
const GUN_PLAIN_BASE: &[Seg] = &store_art::translated(RIFLE, -2.0, 26.0);
const GUN_PLAIN_BRIGHT: &[Seg] = &store_art::translated(RIFLE_BRIGHT, -2.0, 26.0);
const GUN_PLAIN: &[Prim] = &[
    Prim::Path { x: 235.833, y: 157.667, segs: GUN_PLAIN_BASE, close: true,
        fill: Some(Ink::Fixed(rgb(0x79cdb8))), stroke: None, width: 0.0 },
    Prim::Path { x: 76.25, y: 131.833, segs: GUN_PLAIN_BRIGHT, close: true,
        fill: Some(Ink::Fixed(GUN)), stroke: None, width: 0.0 },
];
const GUN_LIFT: &[Prim] = &[
    Prim::Path { x: 235.833, y: 157.667, segs: GUN_PLAIN_BASE, close: true,
        fill: Some(Ink::Fixed(ON_MINT_BAR)), stroke: None, width: 0.0 },
    Prim::Path { x: 76.25, y: 131.833, segs: GUN_PLAIN_BRIGHT, close: true,
        fill: Some(Ink::Fixed(ON_MINT_BAR)), stroke: None, width: 0.0 },
];
const GUN_FLAT: &[Prim] = &[
    Prim::Path { x: 235.833, y: 157.667, segs: GUN_PLAIN_BASE, close: true,
        fill: Some(Ink::Fixed(GROWN_GUN)), stroke: None, width: 0.0 },
    Prim::Path { x: 76.25, y: 131.833, segs: GUN_PLAIN_BRIGHT, close: true,
        fill: Some(Ink::Fixed(GROWN_GUN)), stroke: None, width: 0.0 },
];
const GUN_SELECTED: &[Prim] = &[
    Prim::Path { x: 77.667, y: 117.833, segs: store_art::SELECTED_DARK, close: true,
        fill: Some(Ink::Fixed(GROWN_GUN)), stroke: None, width: 0.0 },
    Prim::Path { x: 57.667, y: 115.75, segs: store_art::SELECTED_CORE, close: true,
        fill: Some(Ink::Fixed(rgb(0x302010))), stroke: None, width: 0.0 },
];
const GUN_SEAM: Prim = Prim::Path {
    x: 59.333, y: 129.083, segs: store_art::PLAIN_SEAM, close: true,
    fill: Some(Ink::Fixed(rgb(0x0e0e0d))), stroke: None, width: 0.0,
};

/// Source-native 9×9 scatter, 25 cells; it matches the repeated MAGNUM
/// socket glyph in Entropism (card 1, native x1185/y1201).
macro_rules! qr {
    ($ink:expr, $top:expr) => { &[
        // row 0: #..#.#..#
        fill_rect(9.60, $top + 0.00, 3.8, 3.8, $ink),
        fill_rect(20.07, $top + 0.00, 3.8, 3.8, $ink),
        fill_rect(27.05, $top + 0.00, 3.8, 3.8, $ink),
        fill_rect(37.52, $top + 0.00, 3.8, 3.8, $ink),
        // row 1: .#....#..
        fill_rect(13.09, $top + 3.49, 3.8, 3.8, $ink),
        fill_rect(30.54, $top + 3.49, 3.8, 3.8, $ink),
        // row 2: ..#.....#
        fill_rect(16.58, $top + 6.98, 3.8, 3.8, $ink),
        fill_rect(37.52, $top + 6.98, 3.8, 3.8, $ink),
        // row 3: #..#.#.#.
        fill_rect(9.60, $top + 10.47, 3.8, 3.8, $ink),
        fill_rect(20.07, $top + 10.47, 3.8, 3.8, $ink),
        fill_rect(27.05, $top + 10.47, 3.8, 3.8, $ink),
        fill_rect(34.03, $top + 10.47, 3.8, 3.8, $ink),
        // row 4: .........
        // row 5: .#.#..#.#
        fill_rect(13.09, $top + 17.45, 3.8, 3.8, $ink),
        fill_rect(20.07, $top + 17.45, 3.8, 3.8, $ink),
        fill_rect(30.54, $top + 17.45, 3.8, 3.8, $ink),
        fill_rect(37.52, $top + 17.45, 3.8, 3.8, $ink),
        // row 6: #.#..#...
        fill_rect(9.60, $top + 20.94, 3.8, 3.8, $ink),
        fill_rect(16.58, $top + 20.94, 3.8, 3.8, $ink),
        fill_rect(27.05, $top + 20.94, 3.8, 3.8, $ink),
        // row 7: .#.....#.
        fill_rect(13.09, $top + 24.43, 3.8, 3.8, $ink),
        fill_rect(34.03, $top + 24.43, 3.8, 3.8, $ink),
        // row 8: #..#.#..#
        fill_rect(9.60, $top + 27.92, 3.8, 3.8, $ink),
        fill_rect(20.07, $top + 27.92, 3.8, 3.8, $ink),
        fill_rect(27.05, $top + 27.92, 3.8, 3.8, $ink),
        fill_rect(37.52, $top + 27.92, 3.8, 3.8, $ink),
    ] };
}
const QR_STD: &[Prim] = qr!(Ink::Fixed(MINT_BAR), 282.5);
const QR_LIFT: &[Prim] = qr!(Ink::Fixed(ON_MINT_BAR), 282.5);
const QR_FLAT: &[Prim] = qr!(Ink::Fixed(ON_BAND), 282.5);

/// The source socket lettering has a 20-native-pixel cap, independently
/// measured in all three cells of both card states.
const fn socket_label(x: f32, y: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Text { x, y, size: 13.0, ink, face: Face::SemiBold, anchor: Anchor::Middle, content }
}

const fn card_stat(x: f32, y: f32, size: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Text { x, y, size, ink, face: Face::Medium, anchor: Anchor::Middle, content }
}

/// Four certification marks, measured from the band at native 2.4x.
/// The warning block contains a source-thresholded micro contour.
macro_rules! band_marks {
    ($ink:expr, $knock:expr) => {
        &[
            // Framed RG5 certification, then the filled square/disc SC mark.
            fill_rect(-16.5, 78.5, 10.8, 11.0, $ink),
            fill_rect(-15.0, 80.0, 7.8, 7.8, $knock),
            txt_bold(-14.5, 83.8, 3.2, $ink, "RG5"),
            fill_rect(-0.7, 78.5, 11.1, 11.0, $ink),
            Prim::Circle { x: 4.8, y: 84.0, r: 4.05, fill: Some($knock), stroke: None, width: 0.0 },
            txt_bold(2.35, 85.6, 4.5, $ink, "SC"),
            // The source C has short angular shoulders and a second inset C.
            line_path(24.5, 81.2, &[
                Seg::Line(22.6, 78.5), Seg::Line(17.6, 78.5), Seg::Line(15.5, 80.6),
                Seg::Line(15.5, 87.4), Seg::Line(17.6, 89.5), Seg::Line(22.6, 89.5),
                Seg::Line(24.5, 86.8),
            ], $ink, 1.3),
            line_path(22.0, 82.4, &[
                Seg::Line(19.5, 82.4), Seg::Line(18.5, 83.4), Seg::Line(18.5, 84.5),
                Seg::Line(19.5, 85.5), Seg::Line(22.0, 85.5),
            ], $ink, 0.8),
            Prim::Round { x: 28.5, y: 74.0, w: 64.0, h: 18.0, r: 1.5, fill: None, stroke: Some($ink), width: 0.5 },
            // Hollow warning triangle and positive exclamation on amber.
            fill_path(30.3, 90.0, &[
                Seg::Line(36.5, 77.6), Seg::Line(42.7, 90.0),
                Seg::Move(33.0, 88.7), Seg::Line(40.0, 88.7), Seg::Line(36.5, 81.0),
            ], $ink),
            fill_rect(36.0, 82.8, 1.0, 3.9, $ink),
            Prim::Circle { x: 36.5, y: 88.0, r: 0.55, fill: Some($ink), stroke: None, width: 0.0 },
            fill_path(0.0, 0.0, store_art::BAND_MICRO, Ink::Fixed(BAND_MICRO_INK)),
        ]
    };
}
const BAND_MARKS: &[Prim] = band_marks!(Ink::Fixed(ON_BAND), Ink::Fixed(BAND));
const BAND_MARKS_SEL: &[Prim] = band_marks!(Ink::Fixed(ON_BAND), Ink::Select);

/// A standard product card, at its outline's own origin.
const CARD: &[Prim] = &[
    shut_path(6.0, 0.0, CARD_EDGE, Ink::Border, 1.5),
    Prim::Tracked { x: 10.5, y: 29.5, size: 24.4, tracking: -0.75, ink: Ink::Fixed(GUN), face: Face::Medium, anchor: Anchor::Start, content: "MAGNUM 650" },
    Prim::Tracked { x: 10.0, y: 48.5, size: 19.5, tracking: 0.6, ink: Ink::Fixed(GUN), face: Face::Medium, anchor: Anchor::Start, content: "HAND GUN" },
    fill_path(-27.0, 94.0, BAND_SHAPE, Ink::Fixed(BAND)),
    Prim::At { x: 0.0, y: 0.0, prims: BAND_MARKS },
    fill_rect(160.0, 70.0, 60.0, 9.0, Ink::Fixed(ON_BAND)),
    txt_bold(163.0, 78.0, 8.0, Ink::Fixed(BAND), "PETROCHEM"),
    txt(160.0, 89.0, 8.0, Ink::Fixed(ON_BAND), "BETTERLIFE TEC"),
    Prim::At { x: 0.0, y: 0.0, prims: GUN_PLAIN },
    GUN_SEAM,
    card_stat(41.0, 225.0, 19.0, Ink::Fixed(MINT_BAR), "DPS"),
    card_stat(101.0, 225.0, 19.0, Ink::Fixed(MINT_BAR), "PNT"),
    card_stat(162.0, 225.0, 19.0, Ink::Fixed(MINT_BAR), "ACC"),
    card_stat(222.0, 225.0, 19.0, Ink::Fixed(MINT_BAR), "ROF"),
    fill_path(8.0, 232.0, MINT_SHAPE, Ink::Fixed(MINT_BAR)),
    card_stat(41.0, 254.7, 25.0, Ink::Fixed(ON_MINT_BAR), "86"),
    card_stat(101.0, 254.7, 25.0, Ink::Fixed(ON_MINT_BAR), "30"),
    card_stat(162.0, 254.7, 25.0, Ink::Fixed(ON_MINT_BAR), "5"),
    card_stat(222.0, 254.7, 25.0, Ink::Fixed(ON_MINT_BAR), "5"),
    fill_rect(0.0, 273.25, 261.0, 1.5, Ink::Border),
    fill_rect(51.25, 274.0, 1.5, 46.5, Ink::Border),
    fill_rect(118.25, 274.0, 1.5, 46.5, Ink::Border),
    fill_rect(189.25, 274.0, 1.5, 46.5, Ink::Border),
    Prim::At { x: 0.0, y: 0.0, prims: QR_STD },
    socket_label(82.75, 295.5, Ink::Fg, "EMPTY"),
    socket_label(82.75, 308.5, Ink::Fg, "SOCKET"),
    socket_label(153.5, 295.5, Ink::Fg, "EMPTY"),
    socket_label(153.5, 308.5, Ink::Fg, "SOCKET"),
    socket_label(224.5, 295.5, Ink::Fg, "EMPTY"),
    socket_label(224.5, 308.5, Ink::Fg, "SOCKET"),
    Prim::Tracked { x: 4.0, y: 341.0, size: 8.0, tracking: 0.3, ink: Ink::Fixed(CARD_COMPLIANCE), face: Face::Bold, anchor: Anchor::Start, content: "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO" },
    Prim::Tracked { x: 4.0, y: 349.0, size: 8.0, tracking: 0.3, ink: Ink::Fixed(CARD_COMPLIANCE), face: Face::Bold, anchor: Anchor::Start, content: "MANIPULATE, ACCESS OR DISABLE THIS DEVICE." },
];

/// The selection: the same layout filled amber to a stepped bottom,
/// with an amber-outlined lower body under it carrying the detail
/// block and a second socket row.
const GROWN_EDGE: &[Seg] = &[
    Seg::Line(237.0, 0.0),
    Seg::Line(261.0, 24.0),
    Seg::Line(261.0, 239.0),
    Seg::Line(79.0, 239.0),
    Seg::Line(64.0, 254.0),
    Seg::Line(0.0, 254.0),
    Seg::Line(0.0, 6.0),
    Seg::Quad { cx: 0.0, cy: 0.0, x: 6.0, y: 0.0 },
];
const GROWN_FLAG: &[Seg] = &[
    Seg::Line(-27.0, 72.0),
    Seg::Line(-3.0, 50.0),
    Seg::Line(-3.0, 59.0),
    Seg::Line(0.0, 59.0),
    Seg::Line(0.0, 94.0),
];
const QR_SEL: &[Prim] = qr!(Ink::Fixed(GROWN_DETAIL), 422.5);

const GROWN: &[Prim] = &[
    fill_path(6.0, 0.0, GROWN_EDGE, Ink::Select),
    fill_path(-27.0, 94.0, GROWN_FLAG, Ink::Select),
    fill_rect(0.0, 58.5, 256.0, 1.0, Ink::Fixed(BAND_RULE)),
    fill_rect(0.0, 93.5, 261.0, 1.0, Ink::Fixed(BAND_RULE)),
    Prim::Tracked { x: 10.5, y: 29.5, size: 24.4, tracking: -0.75, ink: Ink::Fixed(ON_GROWN), face: Face::Medium, anchor: Anchor::Start, content: "MAGNUM 650" },
    Prim::Tracked { x: 10.0, y: 48.5, size: 19.5, tracking: 0.6, ink: Ink::Fixed(ON_GROWN), face: Face::Medium, anchor: Anchor::Start, content: "HAND GUN" },
    Prim::At { x: 0.0, y: 0.0, prims: BAND_MARKS_SEL },
    fill_rect(160.0, 70.0, 60.0, 9.0, Ink::Fixed(ON_GROWN)),
    txt_bold(163.0, 78.0, 8.0, Ink::Select, "PETROCHEM"),
    txt(160.0, 89.0, 8.0, Ink::Fixed(ON_BAND), "BETTERLIFE TEC"),
    // The grown card has its own native dark print and light openings.
    Prim::At { x: 0.0, y: 0.0, prims: GUN_SELECTED },
    card_stat(41.0, 202.5, 18.5, Ink::Fixed(ON_BAND), "DPS"),
    card_stat(101.0, 202.5, 18.5, Ink::Fixed(ON_BAND), "PNT"),
    card_stat(162.0, 202.5, 18.5, Ink::Fixed(ON_BAND), "ACC"),
    card_stat(222.0, 202.5, 18.5, Ink::Fixed(ON_BAND), "ROF"),
    card_stat(41.0, 232.7, 25.0, Ink::Fixed(ON_BAND), "86"),
    card_stat(101.0, 232.7, 25.0, Ink::Fixed(ON_BAND), "30"),
    card_stat(162.0, 232.7, 25.0, Ink::Fixed(ON_BAND), "5"),
    card_stat(222.0, 232.7, 25.0, Ink::Fixed(ON_BAND), "5"),
    // the lower body, amber-outlined with the same measured lower feet
    line_path(0.0, 239.0, &[
        Seg::Line(0.0, 451.5),
        Seg::Quad { cx: 0.0, cy: 462.0, x: 10.5, y: 462.0 },
        Seg::Line(250.5, 462.0),
        Seg::Quad { cx: 261.0, cy: 462.0, x: 261.0, y: 451.5 },
        Seg::Line(261.0, 239.0),
        Seg::Move(0.0, 414.0), Seg::Line(261.0, 414.0),
        Seg::Move(52.0, 414.0), Seg::Line(52.0, 462.0),
        Seg::Move(118.0, 414.0), Seg::Line(118.0, 462.0),
        Seg::Move(190.0, 414.0), Seg::Line(190.0, 462.0),
    ], Ink::Fixed(GROWN_OUTLINE), 1.5),
    txt(16.0, 279.0, 16.0, Ink::Fixed(GROWN_DETAIL), "20"),
    txt(52.0, 279.0, 16.0, Ink::Fixed(GROWN_DETAIL), "Recoil"),
    txt(16.0, 300.0, 16.0, Ink::Fixed(GROWN_DETAIL), "22"),
    txt(52.0, 300.0, 16.0, Ink::Fixed(GROWN_DETAIL), "Sperad"),
    txt(16.0, 321.0, 16.0, Ink::Fixed(GROWN_DETAIL), "12"),
    txt(52.0, 321.0, 16.0, Ink::Fixed(GROWN_DETAIL), "Range"),
    txt(16.0, 350.0, 16.0, Ink::Fixed(GROWN_DETAIL), "Bonus"),
    txt(16.0, 371.0, 16.0, Ink::Fixed(GROWN_DETAIL), "+9 Reflexes"),
    txt(16.0, 392.0, 16.0, Ink::Fixed(GROWN_DETAIL), "+2 Modules Slots"),
    Prim::At { x: 0.0, y: 0.0, prims: QR_SEL },
    socket_label(82.75, 436.5, Ink::Fixed(GROWN_DETAIL), "EMPTY"),
    socket_label(82.75, 450.0, Ink::Fixed(GROWN_DETAIL), "SOCKET"),
    socket_label(153.5, 436.5, Ink::Fixed(GROWN_DETAIL), "EMPTY"),
    socket_label(153.5, 450.0, Ink::Fixed(GROWN_DETAIL), "SOCKET"),
    socket_label(224.5, 436.5, Ink::Fixed(GROWN_DETAIL), "EMPTY"),
    socket_label(224.5, 450.0, Ink::Fixed(GROWN_DETAIL), "SOCKET"),
    Prim::Tracked { x: 4.0, y: 483.0, size: 8.0, tracking: 0.3, ink: Ink::Fixed(GROWN_COMPLIANCE), face: Face::Bold, anchor: Anchor::Start, content: "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO" },
    Prim::Tracked { x: 4.0, y: 491.0, size: 8.0, tracking: 0.3, ink: Ink::Fixed(GROWN_COMPLIANCE), face: Face::Bold, anchor: Anchor::Start, content: "MANIPULATE, ACCESS OR DISABLE THIS DEVICE." },
];


// The nav's five chevrons and the shelf's four positions, as plates:
// hit box, the drawing worn when this one is the selection, and the
// drawing worn when it is not.
macro_rules! nav {
    ($top:expr, $base:expr, $label:expr) => {
        (
            &[
                Prim::At { x: 140.0, y: $top, prims: NAV_SOLID },
                txt(170.0, $base, 22.0, Ink::Fixed(ON_BAND), $label),
            ],
            &[
                Prim::At { x: 140.0, y: $top, prims: NAV_OUTLINE },
                txt(170.0, $base, 22.0, Ink::Fg, $label),
            ],
        )
    };
}
const NAV_ON_0: &[Prim] = nav!(297.0, 327.0, "RIFLES").0;
const NAV_OFF_0: &[Prim] = nav!(297.0, 327.0, "RIFLES").1;
const NAV_ON_1: &[Prim] = nav!(357.0, 387.0, "SMG").0;
const NAV_OFF_1: &[Prim] = nav!(357.0, 387.0, "SMG").1;
const NAV_ON_2: &[Prim] = nav!(417.0, 447.0, "SNIPER").0;
const NAV_OFF_2: &[Prim] = nav!(417.0, 447.0, "SNIPER").1;
const NAV_ON_3: &[Prim] = nav!(477.0, 507.0, "SHOTGUN").0;
const NAV_OFF_3: &[Prim] = nav!(477.0, 507.0, "SHOTGUN").1;
const NAV_ON_4: &[Prim] = nav!(537.0, 567.0, "PISTOL").0;
const NAV_OFF_4: &[Prim] = nav!(537.0, 567.0, "PISTOL").1;

macro_rules! nav_states {
    ($index:expr, $top:expr, $base:expr, $label:expr, $on:expr) => {
        PlateStates {
            group: Group::Category,
            index: $index,
            hover: &[
                Prim::At { x: 140.0, y: $top, prims: NAV_LIFT },
                txt(170.0, $base, 22.0, Ink::Fixed(ON_MINT_BAR), $label),
            ],
            pressed: $on,
            preserve_selected_hover: true,
            selected_away: None,
            selected_hover: None,
            selected_pressed: None,
        }
    };
}

// Product-card feedback extends the sheet's inferred lift/flat reading.
// Use the SAME card drawing at the idle size: amber material while held
// must not reveal GROWN's detail block or move its gun/stat/socket rows.
const fn card_face(pressed: bool) -> [Prim; CARD.len()] {
    let mut face = [CARD[0]; CARD.len()];
    let fill = if pressed { Ink::Select } else { Ink::Fixed(rgb(0x2c9798)) };
    let ink = if pressed { Ink::Fixed(ON_BAND) } else { Ink::Fixed(ON_MINT_BAR) };
    let mut i = 0;
    while i < CARD.len() {
        face[i] = CARD[i];
        match &mut face[i] {
            // Title, stat and socket printing sits on the feedback slab.
            // The source-fitted idle inks are fixed, so recolor by their
            // card positions for both Text and Tracked runs.
            Prim::Text { ink: color, .. } | Prim::Tracked { ink: color, .. }
                if i == 1 || i == 2 || (i >= 10 && i <= 13) || (i >= 24 && i <= 29) => *color = ink,
            Prim::Rect { fill: Some(color @ Ink::Border), .. } => *color = ink,
            _ => {}
        }
        i += 1;
    }
    face[0] = Prim::Path {
        x: 6.0, y: 0.0, segs: CARD_EDGE, close: true,
        fill: Some(fill),
        stroke: Some(if pressed { Ink::Select } else { Ink::Fixed(rgb(0xa9e6df)) }),
        width: 1.8,
    };
    // A filled slab needs the gun's dark-on-fill treatment, without
    // GROWN's translation. Both paths keep their original coordinates.
    face[8] = Prim::At { x: 0.0, y: 0.0, prims: if pressed { GUN_FLAT } else { GUN_LIFT } };
    face[9] = Prim::Path { x: 59.333, y: 129.083, segs: store_art::PLAIN_SEAM,
        close: true, fill: Some(fill), stroke: None, width: 0.0 };
    face[23] = Prim::At {
        x: 0.0, y: 0.0, prims: if pressed { QR_FLAT } else { QR_LIFT },
    };
    face
}

const fn card_ghost(segs: &'static [Seg]) -> Prim {
    Prim::Path {
        x: 6.0, y: 0.0, segs, close: true,
        fill: Some(Ink::Fixed(iced::Color { a: 0.58, ..rgb(0x0f9f80) })),
        stroke: Some(Ink::Fixed(iced::Color { a: 0.80, ..rgb(0x6cc4bd) })),
        width: 1.2,
    }
}
const CARD_LIFT: &[Prim] = &[
    Prim::At { x: 20.0, y: -20.0, prims: &[card_ghost(CARD_EDGE)] },
    Prim::At { x: 0.0, y: 0.0, prims: &card_face(false) },
];
const CARD_FLAT: &[Prim] = &card_face(true);
// The selected card already has a solid upper slab; lift that slab's
// ghost only, keeping the outlined detail body and all selected art.
const GROWN_LIFT: &[Prim] = &[
    Prim::At { x: 20.0, y: -20.0, prims: &[card_ghost(GROWN_EDGE)] },
    Prim::At { x: 0.0, y: 0.0, prims: GROWN },
];
// The fourth card is cropped by the photographed right edge. The main
// drawing stops at x1523 (local 80); five narrow translucent copies carry
// only its coloured residue through x1550. Applying this to each face
// keeps the single interactive Plate and all selection/feedback states.
macro_rules! fourth_face {
    ($face:expr) => {
        &[
            // Retain the projecting flag, half-strokes and lifted ghost;
            // only the right edge is meant to crop the artwork.
            Prim::Viewport { x: -32.0, y: -24.0, w: 112.0, h: 538.0, prims: $face },
            fourth_strip!($face, 80.0, 2.0, 0.85, "kitsch-card4-bleed-1"),
            fourth_strip!($face, 82.0, 3.0, 0.30, "kitsch-card4-bleed-2"),
            fourth_strip!($face, 85.0, 5.0, 0.20, "kitsch-card4-bleed-3"),
            fourth_strip!($face, 90.0, 7.0, 0.08, "kitsch-card4-bleed-4"),
            fourth_strip!($face, 97.0, 10.0, 0.02, "kitsch-card4-bleed-5"),
        ]
    };
}
macro_rules! fourth_strip {
    ($face:expr, $x:expr, $w:expr, $alpha:expr, $id:expr) => {
        Prim::Viewport {
            x: $x, y: -24.0, w: $w, h: 538.0,
            prims: &[Prim::Motion {
                motion: Motion { id: $id, begin: 0, dur: 1,
                    ease: Easing::Linear, change: Change::Opacity { alpha: ($alpha, $alpha) } },
                prims: $face,
            }],
        }
    };
}
const CARD_4: &[Prim] = fourth_face!(CARD);
const GROWN_4: &[Prim] = fourth_face!(GROWN);
const CARD_LIFT_4: &[Prim] = fourth_face!(CARD_LIFT);
const CARD_FLAT_4: &[Prim] = fourth_face!(CARD_FLAT);
const GROWN_LIFT_4: &[Prim] = fourth_face!(GROWN_LIFT);
macro_rules! card_states {
    ($index:expr) => {
        PlateStates {
            group: Group::Card,
            index: $index,
            hover: CARD_LIFT,
            pressed: CARD_FLAT,
            preserve_selected_hover: true,
            selected_away: None,
            selected_hover: Some(GROWN_LIFT),
            selected_pressed: Some(GROWN),
        }
    };
}

/// Categories keep their flat selection; product-card growth belongs
/// only to committed selection, independently of hover/held material.
pub(crate) const STORE_STATES: &[PlateStates] = &[
    nav_states!(0, 297.0, 327.0, "RIFLES", NAV_ON_0),
    nav_states!(1, 357.0, 387.0, "SMG", NAV_ON_1),
    nav_states!(2, 417.0, 447.0, "SNIPER", NAV_ON_2),
    nav_states!(3, 477.0, 507.0, "SHOTGUN", NAV_ON_3),
    nav_states!(4, 537.0, 567.0, "PISTOL", NAV_ON_4),
    card_states!(0),
    card_states!(1),
    card_states!(2),
    PlateStates {
        group: Group::Card, index: 3,
        hover: CARD_LIFT_4, pressed: CARD_FLAT_4,
        preserve_selected_hover: true, selected_away: None,
        selected_hover: Some(GROWN_LIFT_4), selected_pressed: Some(GROWN_4),
    },
];

macro_rules! shelf {
    ($i:expr) => {
        &[Prim::Pick {
            group: Group::Card,
            index: $i,
            on: &[Prim::Plate {
                group: Group::Card, index: $i,
                x: 0.0, y: 0.0, w: 261.0, h: 500.0,
                on: GROWN, off: CARD,
            }],
            off: &[Prim::Plate {
                group: Group::Card, index: $i,
                x: 0.0, y: 0.0, w: 261.0, h: 320.0,
                on: GROWN, off: CARD,
            }],
        }]
    };
}
const SHELF_0: &[Prim] = shelf!(0);
const SHELF_1: &[Prim] = shelf!(1);
const SHELF_2: &[Prim] = shelf!(2);
const SHELF_3: &[Prim] = &[Prim::Viewport {
    x: -32.0, y: -24.0, w: 139.0, h: 538.0,
    prims: &[Prim::Pick {
        group: Group::Card, index: 3,
        on: &[Prim::Plate {
            group: Group::Card, index: 3,
            x: 0.0, y: 0.0, w: 261.0, h: 500.0,
            on: GROWN_4, off: CARD_4,
        }],
        off: &[Prim::Plate {
            group: Group::Card, index: 3,
            x: 0.0, y: 0.0, w: 261.0, h: 320.0,
            on: GROWN_4, off: CARD_4,
        }],
    }],
}];

/// The four cards, tops at y 218.
const SHELF: &[Prim] = &[
    Prim::At { x: 484.0, y: 218.0, prims: SHELF_0 },
    Prim::At { x: 804.0, y: 218.0, prims: SHELF_1 },
    Prim::At { x: 1123.0, y: 218.0, prims: SHELF_2 },
    Prim::At { x: 1443.0, y: 218.0, prims: SHELF_3 },
];

pub const STORE: &[Prim] = &[
    Prim::Soft { prims: BACKDROP },
    // logotype: a heavy extended face, the T outline-only
    Prim::Wide { x: 155.0, y: 132.0, size: 60.0, stretch: 1.7, ink: Ink::Fixed(LOGO), face: Face::Bold, anchor: Anchor::Start, content: "4S" },
    shut_path(280.0, 88.0, TEE, Ink::Fixed(LOGO), 1.3),
    Prim::Spaced { x: 154.0, y: 155.0, size: 15.0, ink: Ink::Fg, face: Face::Medium, pitch: 32.0, content: "STORE" },
    // customer chip and account lines
    Prim::Round { x: 123.0, y: 178.0, w: 215.0, h: 22.0, r: 8.0, fill: None, stroke: Some(Ink::Border), width: 1.5 },
    txt(133.0, 193.0, 12.0, Ink::Fg, "customer"),
    txt(243.0, 193.0, 12.0, Ink::Fg, "#NC488402"),
    txt(133.0, 227.0, 12.0, Ink::Fg, "loyalty discount"),
    txt_end(306.0, 227.0, 12.0, Ink::Fg, "10%"),
    txt(133.0, 243.0, 12.0, Ink::Fg, "last update"),
    txt_end(306.0, 243.0, 12.0, Ink::Fg, "10/05/2077"),
    // the nav bracket: one swept line wrapping the customer block, and
    // the solid wave it ends in
    fill_path(106.5, 575.0, WAVE_BODY, Ink::Fixed(WAVE_INK)),
    line_path(440.0, 186.0, BRACKET_PATH, Ink::Border, 2.0),
    // nav chevrons, SMG solid
    Prim::Plate { group: Group::Category, index: 0, x: 140.0, y: 297.0, w: 216.0, h: 39.0, on: NAV_ON_0, off: NAV_OFF_0 },
    Prim::Plate { group: Group::Category, index: 1, x: 140.0, y: 357.0, w: 216.0, h: 39.0, on: NAV_ON_1, off: NAV_OFF_1 },
    Prim::Plate { group: Group::Category, index: 2, x: 140.0, y: 417.0, w: 216.0, h: 39.0, on: NAV_ON_2, off: NAV_OFF_2 },
    Prim::Plate { group: Group::Category, index: 3, x: 140.0, y: 477.0, w: 216.0, h: 39.0, on: NAV_ON_3, off: NAV_OFF_3 },
    Prim::Plate { group: Group::Category, index: 4, x: 140.0, y: 537.0, w: 216.0, h: 39.0, on: NAV_ON_4, off: NAV_OFF_4 },
    // the shelf, on a 320px pitch; the fourth runs off the frame edge.
    // Wiped on from the left over 0.45 s from 0, EaseOutCubic
    // (`#cards-extrude`, trace lines 303-309): the rect x 450 y 210
    // w 1150 h 505 takes the four cards with margin, to the frame edge
    Prim::Motion {
        motion: Motion {
            id: "cards-extrude",
            begin: 0,
            dur: 450,
            ease: Easing::EaseOutCubic,
            change: Change::Clip { x: 450.0, y: 210.0, w: (0.0, 1150.0), h: (505.0, 505.0) },
        },
        prims: SHELF,
    },
    // footer marks
    txt(181.0, 738.0, 7.5, Ink::Dim, "SPARE TIME MANAGER WAS DEVELO-"),
    txt(181.0, 747.0, 7.5, Ink::Dim, "PED BY SEOCHO. SERVING CUSTO-"),
    txt(181.0, 756.0, 7.5, Ink::Dim, "MERS SINCE 2006."),
    txt(1325.0, 746.0, 7.5, Ink::Dim, "MAPS ARE PROVIDED BY SEOCHO."),
    txt(1325.0, 755.0, 7.5, Ink::Dim, "SATELITE SERVICES SINCE 2006."),
    line_rect(350.0, 733.0, 28.0, 27.0, Ink::Fixed(MARK), 1.5),
    txt_mid(364.0, 752.0, 14.0, Ink::Fixed(MARK), "A"),
    line_rect(1500.0, 733.0, 29.0, 27.0, Ink::Fixed(MARK), 1.5),
    txt_mid(1514.5, 752.0, 14.0, Ink::Fixed(MARK), "C"),
    // one line of bright micro-text centred at the foot
    Prim::Tracked { x: 503.0, y: 870.0, size: 9.0, tracking: 0.25, ink: Ink::Fixed(FOOT_MICRO), face: Face::Bold, anchor: Anchor::Start, content: "ARASAKA CONSUMER TECHNOLOGY" },
    Prim::Tracked { x: 641.0, y: 870.0, size: 9.0, tracking: 0.24, ink: Ink::Fixed(FOOT_MICRO), face: Face::Bold, anchor: Anchor::Start, content: "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE." },
];
// --- end store -----------------------------------------------------------

#[cfg(test)]
mod store_interaction_tests {
    use super::*;

    #[test]
    fn card_material_feedback_keeps_idle_geometry_and_content_until_selection() {
        fn fourth_face_keeps_content(face: &[Prim], source: &[Prim]) {
            assert_eq!(face.len(), 6, "opaque slice plus five residue slices");
            let Prim::Viewport { x, y, w, h, prims } = face[0] else { panic!("opaque fourth-card slice") };
            assert!(x < -27.0 && y < -20.0 && y + h > 500.0,
                "retain the projecting flag, lifted ghost and selected footer");
            assert_eq!(prims, source);
            let mut right = x + w;
            let mut previous_alpha = 1.0;
            for slice in &face[1..] {
                let Prim::Viewport { x: sx, y, w: sw, h, prims } = *slice else { panic!("fourth-card residue slice") };
                assert_eq!(sx, right, "residue slices must not overlap or leave gaps");
                assert!(sw > 0.0 && y < -20.0 && y + h > 500.0);
                let [Prim::Motion { motion, prims }] = prims else { panic!("fourth-card residue opacity") };
                let Change::Opacity { alpha: (start, end) } = motion.change else { panic!("static residue opacity") };
                assert_eq!(start, end, "persistent fade is independent of opening time");
                assert!(start > 0.0 && start < previous_alpha);
                assert_eq!(*prims, source);
                right += sw;
                previous_alpha = start;
            }
            assert_eq!(right, 107.0, "source foreground ends at x1550");
        }
        fn geometry(mut prim: Prim) -> Prim {
            match &mut prim {
                Prim::Path { fill, stroke, width, .. }
                | Prim::Rect { fill, stroke, width, .. } => {
                    *fill = None;
                    *stroke = None;
                    *width = 0.0;
                }
                Prim::Text { ink, .. } | Prim::Tracked { ink, .. } => *ink = Ink::Fg,
                Prim::At { prims, .. } if *prims == QR_LIFT || *prims == QR_FLAT => {
                    *prims = QR_STD;
                }
                Prim::At { prims, .. } if *prims == GUN_LIFT || *prims == GUN_FLAT => {
                    *prims = GUN_PLAIN;
                }
                _ => {}
            }
            prim
        }
        let states: Vec<_> = STORE_STATES.iter().filter(|s| s.group == Group::Card).collect();
        assert_eq!(states.len(), 4);
        for (index, state) in states.into_iter().enumerate() {
            assert_eq!(state.index, index);
            if index == 3 {
                fourth_face_keeps_content(CARD_4, CARD);
                fourth_face_keeps_content(GROWN_4, GROWN);
                fourth_face_keeps_content(state.hover, CARD_LIFT);
                fourth_face_keeps_content(state.pressed, CARD_FLAT);
                fourth_face_keeps_content(state.selected_hover.unwrap(), GROWN_LIFT);
                fourth_face_keeps_content(state.selected_pressed.unwrap(), GROWN);
                assert_eq!(SHELF_3.len(), 1);
                let Prim::Viewport { x, y, w, h, prims } = SHELF_3[0] else { panic!("one fourth-card hit viewport") };
                assert_eq!(x + w, 107.0);
                assert!(x < -27.0 && y < -20.0 && y + h > 500.0);
                let [Prim::Pick { group, index, on, off }] = prims else { panic!("one fourth-card Pick") };
                assert_eq!((*group, *index), (Group::Card, 3));
                for (face, height) in [(*on, 500.0), (*off, 320.0)] {
                    let [Prim::Plate { group, index, w, h, on, off, .. }] = face else { panic!("one fourth-card Plate per selection") };
                    assert_eq!((*group, *index, *w, *h), (Group::Card, 3, 261.0, height));
                    assert_eq!((*on, *off), (GROWN_4, CARD_4));
                }
                continue;
            }
            let Prim::At { x, y, prims: hover } = state.hover[1] else { panic!("lifted face") };
            assert_eq!((x, y), (0.0, 0.0));
            for face in [hover, state.pressed] {
                assert_eq!(face.len(), CARD.len());
                assert_eq!(face.iter().copied().map(geometry).collect::<Vec<_>>(),
                    CARD.iter().copied().map(geometry).collect::<Vec<_>>());
            }
            assert_eq!(state.selected_pressed, Some(GROWN));
            let Prim::At { x, y, prims } = state.selected_hover.unwrap()[1] else { panic!("selected face") };
            assert_eq!((x, y, prims), (0.0, 0.0, GROWN));
        }
    }

    #[test]
    fn compact_card_qr_keeps_every_cell_and_prints_dark_on_filled_faces() {
        for (pressed, expected) in [(false, ON_MINT_BAR), (true, ON_BAND)] {
            let face = card_face(pressed);
            let Prim::At { x, y, prims } = face[23] else { panic!("compact QR") };
            assert_eq!((x, y), (0.0, 0.0));
            assert_eq!(prims.len(), QR_STD.len());
            for (&cell, &idle) in prims.iter().zip(QR_STD) {
                let Prim::Rect { x, y, w, h, fill, stroke, width } = cell else { panic!("QR cell") };
                assert_eq!(fill, Some(Ink::Fixed(expected)));
                assert_eq!(Prim::Rect { x, y, w, h, fill: Some(Ink::Fixed(MINT_BAR)), stroke, width }, idle);
            }
            // The only other nested content is the shelf band's dark
            // compliance printing, already drawn on its own yellow fill.
            assert_eq!(face[4], CARD[4]);
        }
    }

    #[test]
    fn card_lift_uses_measured_first_ghost_step_and_flat_amber_press() {
        for (lift, edge) in [(CARD_LIFT, CARD_EDGE), (GROWN_LIFT, GROWN_EDGE)] {
            let Prim::At { x, y, prims } = lift[0] else { panic!("ghost") };
            assert_eq!((x, y), (20.0, -20.0));
            assert_eq!(prims.len(), 1);
            let Prim::Path { x, y, segs, fill, stroke, width, close } = prims[0] else { panic!("ghost path") };
            assert_eq!((x, y, segs, close), (6.0, 0.0, edge, true));
            assert_eq!(fill, Some(Ink::Fixed(iced::Color { a: 0.58, ..rgb(0x0f9f80) })));
            assert_eq!(stroke, Some(Ink::Fixed(iced::Color { a: 0.80, ..rgb(0x6cc4bd) })));
            assert_eq!(width, 1.2);
        }
        for (face, expected) in [(card_face(false), Ink::Fixed(rgb(0x2c9798))), (card_face(true), Ink::Select)] {
            let Prim::Path { fill, .. } = face[0] else { panic!("slab") };
            assert_eq!(fill, Some(expected));
            let feedback_ink = if expected == Ink::Select { Ink::Fixed(ON_BAND) } else { Ink::Fixed(ON_MINT_BAR) };
            for index in [1, 2, 10, 11, 12, 13, 24, 25, 26, 27, 28, 29] {
                let ink = match face[index] {
                    Prim::Text { ink, .. } | Prim::Tracked { ink, .. } => ink,
                    _ => panic!("feedback printing at {index}"),
                };
                assert_eq!(ink, feedback_ink);
            }
        }
    }

    #[test]
    fn category_lift_preserves_hit_boxes_labels_and_flat_selection() {
        let plates: Vec<_> = STORE.iter().filter_map(|prim| match prim {
            Prim::Plate { group: Group::Category, index, x, y, w, h, on, off } =>
                Some((*index, *x, *y, *w, *h, *on, *off)),
            _ => None,
        }).collect();
        assert_eq!(plates.len(), 5);
        let states: Vec<_> = STORE_STATES.iter().filter(|state| state.group == Group::Category).collect();
        assert_eq!(states.len(), plates.len());
        for (state, (index, x, y, w, h, on, off)) in states.into_iter().zip(plates) {
            assert_eq!((state.group, state.index), (Group::Category, index));
            assert_eq!((w, h), (216.0, 39.0));
            assert_eq!(state.hover[0], Prim::At { x, y, prims: NAV_LIFT });
            let Prim::Text { x, y, size, face, anchor, content, .. } = off[1] else {
                panic!("category label is text");
            };
            assert_eq!(state.hover[1], Prim::Text {
                x, y, size, face, anchor, content, ink: Ink::Fixed(ON_MINT_BAR),
            });
            assert_eq!(state.pressed, on, "release leaves the same flat selected face");
            assert!(state.preserve_selected_hover);
            assert_eq!(state.selected_hover, None);
            assert_eq!(state.selected_pressed, None);
        }
    }

    #[test]
    fn category_ghost_uses_one_unscaled_first_fan_step() {
        assert_eq!(NAV_LIFT.len(), 2);
        assert_eq!(NAV_LIFT[0], Prim::At { x: 20.0, y: -20.0, prims: NAV_GHOST });
        for prim in [NAV_GHOST[0], NAV_LIFT[1], NAV_SOLID[0], NAV_OUTLINE[0]] {
            let Prim::Path { x, y, segs, close, .. } = prim else {
                panic!("same chevron path in every state");
            };
            assert_eq!((x, y, segs, close), (0.0, 39.0, CHEVRON, true));
        }
        let Prim::Path { fill, stroke, width, .. } = NAV_GHOST[0] else { unreachable!() };
        assert_eq!(fill, Some(Ink::Fixed(iced::Color { a: 0.58, ..rgb(0x0f9f80) })));
        assert_eq!(stroke, Some(Ink::Fixed(iced::Color { a: 0.80, ..rgb(0x6cc4bd) })));
        assert_eq!(width, 1.2);
    }
}

// --- dashboard -----------------------------------------------------------
//
// `docs/kitsch/dashboard-trace.svg`, transcribed: the module hub.
// Coordinates are the trace's own in the 1600x900 frame, measured off
// `images/kitsch-dashboard.png` (#49). Elements are in the trace's paint
// order -- ground and bloom, header, USER box, badges, the thirty-six
// ghosts, the six solid blades, the BRAINDANCE panel, the B mark, the
// foot line -- and every group cites the trace element it came from.
//
// What the `Prim` set cannot express here, and what stands in for it:
//
// - `rotate(30)` / `rotate(-30)` on the fan cards. `Prim::At` is a
//   translation only, so the rotation is *baked into the card path*:
//   `#card` (`x=-81 y=-25 width=162 height=50 rx=8`) is written as a
//   closed path with cubic corner arcs (handle 0.5523 * 8) and each of
//   its points rotated about the card's own origin, to two decimals
//   (`CARD_CW`, `CARD_CCW`). The `rotate(90)` cards need no path: a
//   162x50 r8 card turned a quarter is a 50x162 r8 `Prim::Round`.
// - the labels rotate with their blade in the trace (`components.svg`
//   line 575). Each label is a `Prim::Turn` at the blade's centre and
//   angle (+-30, or 90 for the two PRODUCTS cards) holding one
//   `txt_mid(0, 6.5, 19, ..)`, which is the trace's `<text y="6.5"
//   text-anchor="middle">` inside its `translate() rotate()` group, so
//   the label lies along the card. The card path itself stays
//   pre-rotated (above) rather than moving into the `Turn`, so the
//   hit boxes and the `BLADE_*` tables are unchanged. The group's
//   `letter-spacing="2"` is `Prim::Tracked`, drawn glyph by glyph off
//   measured advances since iced text has no tracking of its own.
// - `fill-opacity` / `stroke-opacity` on the ghosts are carried as the
//   alpha of an `Ink::Fixed` colour (`faded`); nothing is pre-mixed.
//   They composite onto the bloom the way the SVG does because the
//   ground/bloom and both ghost fans are successive `Prim::Soft`
//   groups, rasterised in sRGB by `screens/soft.rs` rather than blended in
//   linear light by wgpu -- the difference is 4-10 levels per channel
//   on the faint tails, and it is what G2i failed this screen on.
// - the `text-anchor="middle"` letters under a `scale(1.7 1)` (lines
//   130-134, 308) are `Prim::Wide` at `Anchor::Middle` on the trace's
//   `translate()` x, 176.7 and so on. Until 2026-09-07 `Wide` was
//   start-anchored and A / C / D / B sat at the box centreline minus
//   half the trace's stated cap *ink* width (15.8, line 124); the
//   shaped advance is about a pixel wider than the ink, so the anchor
//   moved each letter under a pixel left -- onto the columns rsvg
//   draws the trace's on (A / B ink boxes measured identical to the
//   trace render after, a pixel right before).
// - the shared rose and left-wash field is measured from clear native
//   patches across dashboard/mail/store; see ground-fit.md.

use crate::style::Anchor;

/// The hub uses the shared native #49/#51/#52 page.
pub const HUB_GROUND: iced::Color = PAGE;
/// The selection yellow the hub samples -- EVENTS, badge 02, the
/// BRAINDANCE tab and outlines -- and the inks set on it. A hair off
/// the palette's `YELLOW` (#fcc428) and the store's `BAND` (#fec32f).
pub const HUB_YELLOW: iced::Color = rgb(0xf5cb23);
pub const ON_HUB_YELLOW: iced::Color = rgb(0x4a3a05);
pub const ON_BADGE: iced::Color = rgb(0x6b4d08);
/// The selected blade's lit edge.
pub const SELECT_EDGE: iced::Color = rgb(0xfce89a);
/// An idle blade's fill; its edge is `NAME_INK`. The source idle label
/// is mint, distinct from the dark printing on the selected yellow face.
/// The core median on #49's vertical PRODUCTS is #7cffe5.
pub const BLADE_LABEL: iced::Color = rgb(0x7cffe5);
pub const BLADE: iced::Color = rgb(0x2c9798);
/// A ghost's fill and edge: a greener teal than the solid blade,
/// sampled mid-strip over the black ground (trace lines 189-193).
pub const GHOST: iced::Color = rgb(0x0f9f80);
pub const GHOST_EDGE: iced::Color = rgb(0x6cc4bd);
/// The warning tape's micro-text.
pub const TAPE_INK: iced::Color = rgb(0xd9b41f);

const fn faded(c: iced::Color, a: f32) -> iced::Color {
    iced::Color { a, ..c }
}

/// `#card` rotated 30 degrees clockwise (SVG `rotate(30)`), about its
/// own origin; opens at the rotated (-73,-25) and runs the top edge
/// first, as the unrotated rect would.
const CARD_CW: &[Seg] = &[
    Seg::Line(75.72, 14.85),
    Seg::Cubic { c1x: 79.55, c1y: 17.06, c2x: 80.86, c2y: 21.95, x: 78.65, y: 25.78 },
    Seg::Line(61.65, 55.22),
    Seg::Cubic { c1x: 59.44, c1y: 59.05, c2x: 54.55, c2y: 60.36, x: 50.72, y: 58.15 },
    Seg::Line(-75.72, -14.85),
    Seg::Cubic { c1x: -79.55, c1y: -17.06, c2x: -80.86, c2y: -21.95, x: -78.65, y: -25.78 },
    Seg::Line(-61.65, -55.22),
    Seg::Cubic { c1x: -59.44, c1y: -59.05, c2x: -54.55, c2y: -60.36, x: -50.72, y: -58.15 },
];
const CARD_CW_X: f32 = -50.72;
const CARD_CW_Y: f32 = -58.15;
/// `#card` under `rotate(-30)`, likewise.
const CARD_CCW: &[Seg] = &[
    Seg::Line(50.72, -58.15),
    Seg::Cubic { c1x: 54.55, c1y: -60.36, c2x: 59.44, c2y: -59.05, x: 61.65, y: -55.22 },
    Seg::Line(78.65, -25.78),
    Seg::Cubic { c1x: 80.86, c1y: -21.95, c2x: 79.55, c2y: -17.06, x: 75.72, y: -14.85 },
    Seg::Line(-50.72, 58.15),
    Seg::Cubic { c1x: -54.55, c1y: 60.36, c2x: -59.44, c2y: 59.05, x: -61.65, y: 55.22 },
    Seg::Line(-78.65, 25.78),
    Seg::Cubic { c1x: -80.86, c1y: 21.95, c2x: -79.55, c2y: 17.06, x: -75.72, y: 14.85 },
];
const CARD_CCW_X: f32 = -75.72;
const CARD_CCW_Y: f32 = 14.85;

/// One ghost at its own origin: `#card` in `#0f9f80` under a 1.2px
/// `#6cc4bd`, at one of the trace's seven opacity pairs (fill /
/// stroke). The ramp is the same for every blade, selected or not
/// (`components.svg` lines 560-564).
macro_rules! ghost {
    ($x0:expr, $y0:expr, $segs:expr, $fill:expr, $edge:expr) => {
        &[Prim::Path {
            x: $x0,
            y: $y0,
            segs: $segs,
            close: true,
            fill: Some(Ink::Fixed(faded(GHOST, $fill))),
            stroke: Some(Ink::Fixed(faded(GHOST_EDGE, $edge))),
            width: 1.2,
        }]
    };
}
macro_rules! ghost_v {
    ($fill:expr, $edge:expr) => {
        &[Prim::Round {
            x: -25.0,
            y: -81.0,
            w: 50.0,
            h: 162.0,
            r: 8.0,
            fill: Some(Ink::Fixed(faded(GHOST, $fill))),
            stroke: Some(Ink::Fixed(faded(GHOST_EDGE, $edge))),
            width: 1.2,
        }]
    };
}
/// Ghost dresses by depth, index 0 the faintest (WEAPONS' seventh) and
/// 6 the one nearest its solid card.
const GHOST_CW: [&[Prim]; 7] = [
    ghost!(CARD_CW_X, CARD_CW_Y, CARD_CW, 0.07, 0.16),
    ghost!(CARD_CW_X, CARD_CW_Y, CARD_CW, 0.12, 0.24),
    ghost!(CARD_CW_X, CARD_CW_Y, CARD_CW, 0.21, 0.34),
    ghost!(CARD_CW_X, CARD_CW_Y, CARD_CW, 0.30, 0.45),
    ghost!(CARD_CW_X, CARD_CW_Y, CARD_CW, 0.40, 0.56),
    ghost!(CARD_CW_X, CARD_CW_Y, CARD_CW, 0.48, 0.68),
    ghost!(CARD_CW_X, CARD_CW_Y, CARD_CW, 0.58, 0.80),
];
const GHOST_CCW: [&[Prim]; 7] = [
    ghost!(CARD_CCW_X, CARD_CCW_Y, CARD_CCW, 0.07, 0.16),
    ghost!(CARD_CCW_X, CARD_CCW_Y, CARD_CCW, 0.12, 0.24),
    ghost!(CARD_CCW_X, CARD_CCW_Y, CARD_CCW, 0.21, 0.34),
    ghost!(CARD_CCW_X, CARD_CCW_Y, CARD_CCW, 0.30, 0.45),
    ghost!(CARD_CCW_X, CARD_CCW_Y, CARD_CCW, 0.40, 0.56),
    ghost!(CARD_CCW_X, CARD_CCW_Y, CARD_CCW, 0.48, 0.68),
    ghost!(CARD_CCW_X, CARD_CCW_Y, CARD_CCW, 0.58, 0.80),
];
const GHOST_V: [&[Prim]; 7] = [
    ghost_v!(0.07, 0.16),
    ghost_v!(0.12, 0.24),
    ghost_v!(0.21, 0.34),
    ghost_v!(0.30, 0.45),
    ghost_v!(0.40, 0.56),
    ghost_v!(0.48, 0.68),
    ghost_v!(0.58, 0.80),
];

/// The solid blades at their own origin: idle `#2c9798` under 1.8px
/// `#a9e6df`, selected `#f5cb23` under 1.8px `#fce89a` (trace lines
/// 244 and 261; `components.svg` line 574).
const BLADE_CW_OFF: &[Prim] = &[shut_and_fill(CARD_CW_X, CARD_CW_Y, CARD_CW, Ink::Fixed(BLADE), Ink::Fixed(NAME_INK))];
const BLADE_CW_ON: &[Prim] = &[shut_and_fill(CARD_CW_X, CARD_CW_Y, CARD_CW, Ink::Fixed(HUB_YELLOW), Ink::Fixed(SELECT_EDGE))];
const BLADE_CCW_OFF: &[Prim] = &[shut_and_fill(CARD_CCW_X, CARD_CCW_Y, CARD_CCW, Ink::Fixed(BLADE), Ink::Fixed(NAME_INK))];
const BLADE_CCW_ON: &[Prim] = &[shut_and_fill(CARD_CCW_X, CARD_CCW_Y, CARD_CCW, Ink::Fixed(HUB_YELLOW), Ink::Fixed(SELECT_EDGE))];
const BLADE_V_OFF: &[Prim] = &[Prim::Round { x: -25.0, y: -81.0, w: 50.0, h: 162.0, r: 8.0, fill: Some(Ink::Fixed(BLADE)), stroke: Some(Ink::Fixed(NAME_INK)), width: 1.8 }];
const BLADE_V_ON: &[Prim] = &[Prim::Round { x: -25.0, y: -81.0, w: 50.0, h: 162.0, r: 8.0, fill: Some(Ink::Fixed(HUB_YELLOW)), stroke: Some(Ink::Fixed(SELECT_EDGE)), width: 1.8 }];

const fn shut_and_fill(x: f32, y: f32, segs: &'static [Seg], fill: Ink, stroke: Ink) -> Prim {
    Prim::Path { x, y, segs, close: true, fill: Some(fill), stroke: Some(stroke), width: 1.8 }
}

/// A +-30 blade as a plate: the hit box is the rotated card's bounding
/// box (half extents 81 cos 30 + 25 sin 30 = 82.65 by 81 sin 30 +
/// 25 cos 30 = 62.15), the label the trace's Rajdhani 19 centred at
/// baseline +6.5 inside a `Prim::Turn` at the blade's angle `$a`, so
/// it lies along the card as the trace's `rotate(a)` group has it,
/// tracked 2 as that group's `letter-spacing` says.
/// `$on` / `$off` carry the pre-rotated card path for the same angle.
macro_rules! blade {
    ($i:expr, $cx:expr, $cy:expr, $a:expr, $on:expr, $off:expr, $label:expr) => {
        Prim::Plate {
            group: Group::Module,
            index: $i,
            x: $cx - 82.65,
            y: $cy - 62.15,
            w: 165.3,
            h: 124.3,
            on: &[
                Prim::At { x: $cx, y: $cy, prims: $on },
                Prim::Turn { x: $cx, y: $cy, angle: $a, prims: &[tracked_mid(0.0, 6.5, 19.0, 2.0, Ink::Fixed(ON_HUB_YELLOW), $label)] },
            ],
            off: &[
                Prim::At { x: $cx, y: $cy, prims: $off },
                Prim::Turn { x: $cx, y: $cy, angle: $a, prims: &[tracked_mid(0.0, 6.5, 19.0, 2.0, Ink::Fixed(BLADE_LABEL), $label)] },
            ],
        }
    };
}
/// A 90-degree blade: hit box the 50x162 card itself, PRODUCTS set
/// once, centred, in a `Prim::Turn` at 90 so it reads down the card.
const PRODUCTS_ON: &[Prim] = &[
    Prim::At { x: 0.0, y: 0.0, prims: BLADE_V_ON },
    Prim::Turn { x: 0.0, y: 0.0, angle: 90.0, prims: &[tracked_mid(0.0, 6.5, 19.0, 2.0, Ink::Fixed(ON_HUB_YELLOW), "PRODUCTS")] },
];
const PRODUCTS_OFF: &[Prim] = &[
    Prim::At { x: 0.0, y: 0.0, prims: BLADE_V_OFF },
    Prim::Turn { x: 0.0, y: 0.0, angle: 90.0, prims: &[tracked_mid(0.0, 6.5, 19.0, 2.0, Ink::Fixed(BLADE_LABEL), "PRODUCTS")] },
];
macro_rules! blade_v {
    ($i:expr, $cx:expr, $cy:expr) => {
        Prim::Plate {
            group: Group::Module,
            index: $i,
            x: $cx - 25.0,
            y: $cy - 81.0,
            w: 50.0,
            h: 162.0,
            on: &[Prim::At { x: $cx, y: $cy, prims: PRODUCTS_ON }],
            off: &[Prim::At { x: $cx, y: $cy, prims: PRODUCTS_OFF }],
        }
    };
}

/// Source #49: rounded tab, connected diagonal ribbon, r10.5 body feet.
const TAB: &[Seg] = &[
    Seg::Line(1404.0, 260.5),
    Seg::Quad { cx: 1408.5, cy: 260.5, x: 1412.0, y: 264.0 },
    Seg::Line(1429.0, 281.0),
    Seg::Quad { cx: 1432.0, cy: 284.0, x: 1432.0, y: 288.0 },
    Seg::Line(1432.0, 300.0),
    Seg::Quad { cx: 1432.0, cy: 296.5, x: 1428.5, y: 296.5 },
    Seg::Line(1207.5, 296.5),
    Seg::Line(1207.5, 271.0),
    Seg::Quad { cx: 1207.5, cy: 260.5, x: 1218.0, y: 260.5 },
];
const HUB_RIBBON: &[Seg] = &[
    Seg::Quad { cx: 1172.5, cy: 313.0, x: 1174.0, y: 311.5 },
    Seg::Line(1200.5, 284.5),
    Seg::Quad { cx: 1205.0, cy: 281.0, x: 1205.0, y: 286.0 },
    Seg::Line(1205.0, 296.5),
    Seg::Line(1428.5, 296.5),
    Seg::Quad { cx: 1432.0, cy: 296.5, x: 1432.0, y: 300.0 },
    Seg::Line(1432.0, 305.0),
    Seg::Quad { cx: 1432.0, cy: 308.0, x: 1429.5, y: 310.5 },
    Seg::Line(1411.5, 328.5),
    Seg::Quad { cx: 1408.5, cy: 331.5, x: 1403.5, y: 331.5 },
    Seg::Line(1176.5, 331.5),
    Seg::Quad { cx: 1172.5, cy: 331.5, x: 1172.5, y: 327.5 },
];
const HUB_BODY: &[Seg] = &[
    Seg::Line(1432.0, 657.0),
    Seg::Quad { cx: 1432.0, cy: 667.5, x: 1421.5, y: 667.5 },
    Seg::Line(1218.0, 667.5),
    Seg::Quad { cx: 1207.5, cy: 667.5, x: 1207.5, y: 657.0 },
    Seg::Line(1207.5, 331.5),
];
/// The USER box: `M 155.5,189.5 H 349.5 V 232.5 H 220 L 208,240.5
/// H 155.5 Z` (trace line 148), the mailbox's stepped box.
const USER_STEP: &[Seg] = &[
    Seg::Line(349.5, 189.5),
    Seg::Line(349.5, 232.5),
    Seg::Line(220.0, 232.5),
    Seg::Line(208.0, 240.5),
    Seg::Line(155.5, 240.5),
];

/// The ground and bloom, composited in software:
/// up to seven translucent cards stack on the haze here, and wgpu's
/// linear blend cannot land rsvg's sRGB `fill-opacity` on a backdrop
/// that varies under the stack -- see `screens/soft.rs`. Everything in
/// this group sits under the header and the solid blades, so drawing
/// it first, ahead of the header text, changes no pixel's order.
const HUB_BACK: &[Prim] = &[
    // measured ground, rose and left wash (trace defs; ground-fit.md)
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(HUB_GROUND)),
    Prim::Lobe { x: 750.0, y: -331.0, rx: 1600.0, ry: 929.0, stops: ROSE },
    Prim::Lobe { x: 0.0, y: 393.0, rx: 535.0, ry: 400.0, stops: MARGIN },
];

// The ghosts, farthest first, every trail stepping (+20,-20) in screen
// space from its solid card (lines 196-239); they belong to no plate
// because they do not change with the selection (held feedback removes
// only its own trail), and the solid cards
// paint over them in one pass. Two groups, one per fan, because each
// fan's depth extrudes under its own clip (`#fan-left-extrude`,
// `#fan-right-extrude`, trace lines 123-135): a `Prim::Soft` under a
// `Prim::Motion` is composited over the ground the way the one group
// was (`scene::Backdrop`), so at rest the pixels are the old group's.
// The order across the two is the old group's, left then right.

/// The left fan's ghosts: VEHICLES, WEAPONS, the left PRODUCTS.
const FAN_LEFT: &[Prim] = &[
    // VEHICLES c(364,413) rot 30, 6 ghosts (lines 198-203)
    Prim::At { x: 484.0, y: 293.0, prims: GHOST_CW[1] },
    Prim::At { x: 464.0, y: 313.0, prims: GHOST_CW[2] },
    Prim::At { x: 444.0, y: 333.0, prims: GHOST_CW[3] },
    Prim::At { x: 424.0, y: 353.0, prims: GHOST_CW[4] },
    Prim::At { x: 404.0, y: 373.0, prims: GHOST_CW[5] },
    Prim::At { x: 384.0, y: 393.0, prims: GHOST_CW[6] },
    // WEAPONS c(551,414) rot -30, 7 ghosts (lines 205-211)
    Prim::At { x: 691.0, y: 274.0, prims: GHOST_CCW[0] },
    Prim::At { x: 671.0, y: 294.0, prims: GHOST_CCW[1] },
    Prim::At { x: 651.0, y: 314.0, prims: GHOST_CCW[2] },
    Prim::At { x: 631.0, y: 334.0, prims: GHOST_CCW[3] },
    Prim::At { x: 611.0, y: 354.0, prims: GHOST_CCW[4] },
    Prim::At { x: 591.0, y: 374.0, prims: GHOST_CCW[5] },
    Prim::At { x: 571.0, y: 394.0, prims: GHOST_CCW[6] },
    // left PRODUCTS c(458,575) rot 90, 6 ghosts (lines 213-218)
    Prim::At { x: 578.0, y: 455.0, prims: GHOST_V[1] },
    Prim::At { x: 558.0, y: 475.0, prims: GHOST_V[2] },
    Prim::At { x: 538.0, y: 495.0, prims: GHOST_V[3] },
    Prim::At { x: 518.0, y: 515.0, prims: GHOST_V[4] },
    Prim::At { x: 498.0, y: 535.0, prims: GHOST_V[5] },
    Prim::At { x: 478.0, y: 555.0, prims: GHOST_V[6] },
];

/// The right fan's ghosts: the right PRODUCTS, EVENTS, LOCATIONS.
const FAN_RIGHT: &[Prim] = &[
    // right PRODUCTS c(825,424) rot 90, 6 ghosts (lines 220-225)
    Prim::At { x: 945.0, y: 304.0, prims: GHOST_V[1] },
    Prim::At { x: 925.0, y: 324.0, prims: GHOST_V[2] },
    Prim::At { x: 905.0, y: 344.0, prims: GHOST_V[3] },
    Prim::At { x: 885.0, y: 364.0, prims: GHOST_V[4] },
    Prim::At { x: 865.0, y: 384.0, prims: GHOST_V[5] },
    Prim::At { x: 845.0, y: 404.0, prims: GHOST_V[6] },
    // EVENTS c(731,586) rot -30, 5 ghosts (lines 227-231)
    Prim::At { x: 831.0, y: 486.0, prims: GHOST_CCW[2] },
    Prim::At { x: 811.0, y: 506.0, prims: GHOST_CCW[3] },
    Prim::At { x: 791.0, y: 526.0, prims: GHOST_CCW[4] },
    Prim::At { x: 771.0, y: 546.0, prims: GHOST_CCW[5] },
    Prim::At { x: 751.0, y: 566.0, prims: GHOST_CCW[6] },
    // LOCATIONS c(919,586) rot 30, 6 ghosts (lines 233-238)
    Prim::At { x: 1039.0, y: 466.0, prims: GHOST_CW[1] },
    Prim::At { x: 1019.0, y: 486.0, prims: GHOST_CW[2] },
    Prim::At { x: 999.0, y: 506.0, prims: GHOST_CW[3] },
    Prim::At { x: 979.0, y: 526.0, prims: GHOST_CW[4] },
    Prim::At { x: 959.0, y: 546.0, prims: GHOST_CW[5] },
    Prim::At { x: 939.0, y: 566.0, prims: GHOST_CW[6] },
];

// Held feedback removes only the target trail. Keeping all remaining
// primitive order and the original fan clips preserves software compositing.
const fn without_trail<const N: usize>(source: &[Prim], begin: usize, count: usize) -> [Prim; N] {
    let mut out = [source[0]; N];
    let mut i = 0;
    while i < N { out[i] = source[if i < begin { i } else { i + count }]; i += 1; }
    out
}
const LEFT_HELD: [&[Prim]; 3] = [
    &without_trail::<13>(FAN_LEFT, 0, 6),
    &without_trail::<12>(FAN_LEFT, 6, 7),
    &without_trail::<13>(FAN_LEFT, 13, 6),
];
const RIGHT_HELD: [&[Prim]; 3] = [
    &without_trail::<11>(FAN_RIGHT, 0, 6),
    &without_trail::<12>(FAN_RIGHT, 6, 5),
    &without_trail::<11>(FAN_RIGHT, 11, 6),
];
const fn held_backdrop(index: usize, fan: &'static [Prim]) -> [Prim; 3] {
    let mut out = [DASHBOARD[0], DASHBOARD[1], DASHBOARD[2]];
    if let Prim::Motion { motion, .. } = out[index] {
        out[index] = Prim::Motion { motion, prims: fan };
    }
    out
}
const DASHBOARD_HELD_BACKDROPS: &[&[Prim]] = &[
    &held_backdrop(1, &[Prim::Soft { prims: LEFT_HELD[0] }]),
    &held_backdrop(1, &[Prim::Soft { prims: LEFT_HELD[1] }]),
    &held_backdrop(1, &[Prim::Soft { prims: LEFT_HELD[2] }]),
    &held_backdrop(2, &[Prim::Soft { prims: RIGHT_HELD[0] }]),
    &held_backdrop(2, &[Prim::Soft { prims: RIGHT_HELD[1] }]),
    &held_backdrop(2, &[Prim::Soft { prims: RIGHT_HELD[2] }]),
];
const fn blade_states(index: usize) -> PlateStates {
    let mut i = 0;
    while i < DASHBOARD.len() {
        if let Prim::Plate { index: target, on, off, .. } = DASHBOARD[i] {
            if target == index {
                return PlateStates { group: Group::Module, index, hover: off, pressed: on,
                    selected_hover: None, selected_pressed: Some(on), selected_away: None,
                    preserve_selected_hover: true };
            }
        }
        i += 1;
    }
    panic!("missing dashboard blade")
}
const DASHBOARD_STATES: &[PlateStates] = &[
    blade_states(0), blade_states(1), blade_states(2), blade_states(3), blade_states(4), blade_states(5),
];

pub const DASHBOARD: &[Prim] = &[
    Prim::Soft { prims: HUB_BACK },
    // the two fans' depth, each wiped on from the left over 0.45 s from
    // 0, `keySplines="0.33 1 0.68 1"` = EaseOutCubic (lines 123-135);
    // the rects are each fan's whole ghost box plus a 5px margin
    Prim::Motion {
        motion: Motion {
            id: "fan-left-extrude",
            begin: 0,
            dur: 450,
            ease: Easing::EaseOutCubic,
            change: Change::Clip { x: 296.0, y: 206.0, w: (0.0, 484.0), h: (436.0, 436.0) },
        },
        prims: &[Prim::Soft { prims: FAN_LEFT }],
    },
    Prim::Motion {
        motion: Motion {
            id: "fan-right-extrude",
            begin: 0,
            dur: 450,
            ease: Easing::EaseOutCubic,
            change: Change::Clip { x: 663.0, y: 217.0, w: (0.0, 465.0), h: (418.0, 418.0) },
        },
        prims: &[Prim::Soft { prims: FAN_RIGHT }],
    },
    // header notes: Rajdhani 600 8 stretched 1.3 (lines 107-122)
    Prim::Wide { x: 205.0, y: 113.6, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Start, content: "SPARE TIME MANAGER WAS DEVELO-" },
    Prim::Wide { x: 205.0, y: 122.8, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Start, content: "PED BY SEOCHO. SERVING CUSTO-" },
    Prim::Wide { x: 205.0, y: 131.9, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Start, content: "MERS SINCE 2006." },
    Prim::Wide { x: 715.3, y: 113.6, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Start, content: "SPARE TIME MANAGER WAS DEVELO-" },
    Prim::Wide { x: 715.3, y: 122.8, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Start, content: "PED BY SEOCHO. SERVING CUSTO-" },
    Prim::Wide { x: 715.3, y: 131.9, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Start, content: "MERS SINCE 2006." },
    Prim::Wide { x: 1253.8, y: 113.6, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Start, content: "MAPS ARE PROVIDED BY SEOCHO." },
    Prim::Wide { x: 1253.8, y: 122.8, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Start, content: "SATELITE SERVICES SINCE 2006." },
    // boxed A / C / D: 24x24 2px squares (lines 125-129) holding a
    // size-18 cap stretched 1.7, centred on the box (lines 130-134)
    line_rect(164.6, 109.8, 24.2, 24.2, Ink::Fixed(BRIGHT), 2.0),
    line_rect(673.3, 109.8, 24.2, 24.2, Ink::Fixed(BRIGHT), 2.0),
    line_rect(1215.2, 109.8, 24.0, 24.2, Ink::Fixed(BRIGHT), 2.0),
    Prim::Wide { x: 176.7, y: 129.2, size: 18.0, stretch: 1.7, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Middle, content: "A" },
    Prim::Wide { x: 685.4, y: 129.2, size: 18.0, stretch: 1.7, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Middle, content: "C" },
    Prim::Wide { x: 1227.2, y: 129.2, size: 18.0, stretch: 1.7, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Middle, content: "D" },
    // section labels: Rajdhani 600 12.3 stretched 1.37 (lines 137-141)
    Prim::Wide { x: 165.0, y: 163.0, size: 12.3, stretch: 1.37, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Start, content: "USER" },
    Prim::Wide { x: 674.0, y: 163.0, size: 12.3, stretch: 1.37, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Start, content: "SECURITY LEVEL" },
    Prim::Wide { x: 1216.0, y: 163.0, size: 12.3, stretch: 1.37, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Start, content: "DESCRIPTION" },
    // USER box and GUES 7702 (lines 148-151)
    shut_path(155.5, 189.5, USER_STEP, Ink::Fixed(MARK), 1.25),
    Prim::Text { x: 164.0, y: 218.0, size: 21.5, ink: Ink::Fixed(NAME_INK), face: Face::SemiBold, anchor: Anchor::Start, content: "GUES 7702" },
    // security badges, `#badge` 57x52, 02 filled (lines 155-164)
    line_rect(664.5, 189.5, 57.0, 52.0, Ink::Fixed(MARK), 1.0),
    Prim::Text { x: 692.0, y: 222.0, size: 22.0, ink: Ink::Fixed(MARK), face: Face::SemiBold, anchor: Anchor::Middle, content: "01" },
    fill_rect(725.0, 189.0, 57.0, 52.0, Ink::Fixed(HUB_YELLOW)),
    Prim::Text { x: 753.0, y: 222.0, size: 22.0, ink: Ink::Fixed(ON_BADGE), face: Face::SemiBold, anchor: Anchor::Middle, content: "02" },
    line_rect(785.5, 189.5, 57.0, 52.0, Ink::Fixed(MARK), 1.0),
    Prim::Text { x: 813.0, y: 222.0, size: 22.0, ink: Ink::Fixed(MARK), face: Face::SemiBold, anchor: Anchor::Middle, content: "03" },
    line_rect(844.5, 189.5, 57.0, 52.0, Ink::Fixed(MARK), 1.0),
    Prim::Text { x: 872.0, y: 222.0, size: 22.0, ink: Ink::Fixed(MARK), face: Face::SemiBold, anchor: Anchor::Middle, content: "04" },
    // the six solid blades in the trace's order (lines 243-267);
    // EVENTS is the selection
    blade!(0, 364.0, 413.0, 30.0, BLADE_CW_ON, BLADE_CW_OFF, "VEHICLES"),
    blade!(1, 551.0, 414.0, -30.0, BLADE_CCW_ON, BLADE_CCW_OFF, "WEAPONS"),
    blade_v!(2, 458.0, 575.0),
    blade_v!(3, 825.0, 424.0),
    blade!(4, 731.0, 586.0, -30.0, BLADE_CCW_ON, BLADE_CCW_OFF, "EVENTS"),
    blade!(5, 919.0, 586.0, 30.0, BLADE_CW_ON, BLADE_CW_OFF, "LOCATIONS"),
    // BRAINDANCE panel: tab, warning tape, outlined body, two
    // four-line paragraphs from source #49, wiped on from the
    // left once the fans have all but finished (`#panel-extrude`,
    // lines 151-158: 0.35 s from 0.25 s, EaseOutCubic)
    Prim::Motion {
        motion: Motion {
            id: "panel-extrude",
            begin: 250,
            dur: 350,
            ease: Easing::EaseOutCubic,
            change: Change::Clip { x: 1166.0, y: 255.0, w: (0.0, 272.0), h: (418.0, 418.0) },
        },
        prims: HUB_PANEL,
    },
    // B DEVICE SOFTWARE mark: notes Rajdhani 700 8 stretched 1.3, the
    // boxed B, the label (lines 302-308)
    Prim::Wide { x: 609.9, y: 746.0, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::Bold, anchor: Anchor::Start, content: "MAPS ARE PROVIDED BY SEOCHO." },
    Prim::Wide { x: 609.9, y: 755.0, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::Bold, anchor: Anchor::Start, content: "SATELITE SERVICES SINCE 2006." },
    line_rect(574.3, 741.0, 24.0, 24.2, Ink::Fixed(BRIGHT), 2.0),
    Prim::Wide { x: 586.3, y: 761.2, size: 18.0, stretch: 1.7, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Middle, content: "B" },
    Prim::Wide { x: 575.0, y: 794.0, size: 12.3, stretch: 1.37, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Start, content: "DEVICE SOFTWARE" },
    // the foot line, one bold weight, two runs (lines 318-321)
    txt_bold(503.0, 870.0, 9.0, Ink::Fixed(BRIGHT), "ARASAKA CONSUMER TECHNOLOGY"),
    txt_bold(641.0, 870.0, 9.0, Ink::Fixed(BRIGHT), "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE."),
];

/// The BRAINDANCE panel, the D DESCRIPTION of the selection.
const HUB_PANEL: &[Prim] = &[
    fill_path(1218.0, 260.5, TAB, Ink::Fixed(HUB_YELLOW)),
    Prim::Tracked { x: 1222.0, y: 288.0, size: 20.0, tracking: 1.0, ink: Ink::Fixed(ON_HUB_YELLOW), face: Face::SemiBold, anchor: Anchor::Start, content: "BRAINDANCE" },
    shut_path(1172.5, 315.0, HUB_RIBBON, Ink::Fixed(HUB_YELLOW), 1.25),
    Prim::Wide { x: 1179.5, y: 317.5, size: 8.0, stretch: 1.08, ink: Ink::Fixed(TAPE_INK), face: Face::SemiBold, anchor: Anchor::Start, content: "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE" },
    Prim::Wide { x: 1179.5, y: 325.0, size: 8.0, stretch: 1.08, ink: Ink::Fixed(TAPE_INK), face: Face::SemiBold, anchor: Anchor::Start, content: "ALLOWED TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE." },
    line_path(1432.0, 300.0, HUB_BODY, Ink::Fixed(HUB_YELLOW), 1.25),
    Prim::Wide { x: 1221.2, y: 356.25, size: 16.8, stretch: 0.999, ink: Ink::Fixed(rgb(0xffcf2f)), face: Face::Medium, anchor: Anchor::Start, content: "Ut enim ad minim veniam," },
    Prim::Wide { x: 1221.2, y: 375.25, size: 16.8, stretch: 0.99, ink: Ink::Fixed(rgb(0xffcf2f)), face: Face::Medium, anchor: Anchor::Start, content: "quis nostrud exercitation" },
    Prim::Wide { x: 1221.2, y: 394.25, size: 16.8, stretch: 0.994, ink: Ink::Fixed(rgb(0xffcf2f)), face: Face::Medium, anchor: Anchor::Start, content: "ullamco laboris nisi ut aliquip" },
    Prim::Wide { x: 1221.2, y: 413.25, size: 16.8, stretch: 1.009, ink: Ink::Fixed(rgb(0xffcf2f)), face: Face::Medium, anchor: Anchor::Start, content: "ex ea commodo consequat." },
    Prim::Wide { x: 1221.2, y: 451.25, size: 16.8, stretch: 0.997, ink: Ink::Fixed(rgb(0xffcf2f)), face: Face::Medium, anchor: Anchor::Start, content: "Duis aute irure dolor in repre-" },
    Prim::Wide { x: 1221.2, y: 470.25, size: 16.8, stretch: 0.99, ink: Ink::Fixed(rgb(0xffcf2f)), face: Face::Medium, anchor: Anchor::Start, content: "henderit in voluptate velit" },
    Prim::Wide { x: 1221.2, y: 489.25, size: 16.8, stretch: 0.99, ink: Ink::Fixed(rgb(0xffcf2f)), face: Face::Medium, anchor: Anchor::Start, content: "esse cillum dolore eu fugiat" },
    Prim::Wide { x: 1221.2, y: 508.25, size: 16.8, stretch: 0.972, ink: Ink::Fixed(rgb(0xffcf2f)), face: Face::Medium, anchor: Anchor::Start, content: "nulla pariatur." },
];
// --- end dashboard -------------------------------------------------------

#[cfg(test)]
mod dashboard_feedback_tests {
    use super::*;

    #[test]
    fn holding_a_blade_removes_only_its_trail_and_keeps_the_fan_clips() {
        for (index, (begin, count)) in [(0, 6), (6, 7), (13, 6), (0, 6), (6, 5), (11, 6)].into_iter().enumerate() {
            let changed = if index < 3 { 1 } else { 2 };
            let source = if index < 3 { FAN_LEFT } else { FAN_RIGHT };
            let variant = DASHBOARD_HELD_BACKDROPS[index];
            for slot in 0..3 {
                if slot != changed { assert_eq!(variant[slot], DASHBOARD[slot]); }
            }
            let Prim::Motion { motion, prims } = variant[changed] else { panic!("missing fan clip") };
            let Prim::Motion { motion: original, .. } = DASHBOARD[changed] else { unreachable!() };
            assert_eq!(motion, original);
            let [Prim::Soft { prims: trail }] = prims else { panic!("missing soft fan") };
            let expected: Vec<_> = source.iter().enumerate().filter(|(i, _)| *i < begin || *i >= begin + count).map(|(_, p)| *p).collect();
            assert_eq!(*trail, expected);
            let Prim::Plate { on, off, .. } = DASHBOARD.iter().find(|p| matches!(p, Prim::Plate { index: i, .. } if *i == index)).unwrap() else { unreachable!() };
            assert_eq!(DASHBOARD_STATES[index].hover, *off);
            assert_eq!(DASHBOARD_STATES[index].pressed, *on);
            assert!(DASHBOARD_STATES[index].preserve_selected_hover);
        }
    }

    // SoftCache's cut key omits the underlying groups. Removal-only
    // variants are safe because no right-fan pixel covers a left-fan
    // pixel: changing the left trail cannot alter the right composite.
    #[test]
    fn fans_have_disjoint_pixel_coverage() {
        let palette = crate::style::Era::Kitsch.style().palette;
        let left = crate::screens::soft::touched(FAN_LEFT, &palette, 1600, 900, 1.0);
        let right = crate::screens::soft::touched(FAN_RIGHT, &palette, 1600, 900, 1.0);
        assert!(left.iter().any(|p| *p) && right.iter().any(|p| *p));
        assert!(!left.iter().zip(right).any(|(a, b)| *a && b));
    }
}
