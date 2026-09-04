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
    Banner, Bar, BarChrome, BarGround, BarMenu, BarOrnament, Chrome, Compliance, Corner, Dress,
    Era, Face, Footnotes, Ground, Ink, MenuMarker, MenuRule, Metrics, Nameplate,
    PanelEcho, Selection, Style, Ticket, WindowLabel,
};
use crate::widgets::surface::{Corners, Cut};
// --- login ---
use crate::style::{
    Access, Colophon, Emblem, Fixture, Legend, Masthead, Plate, Plot, Slot, Wash,
};
// --- end login ---

pub const BG: iced::Color = rgb(0x0b0b07);
pub const BLOOM: iced::Color = rgb(0xa63355);
pub const TEAL: iced::Color = rgb(0x7ddec8);
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
/// (below), which reaches `Palette::banner_on_select` -> the
/// `widgets::banner` / `widgets::card` helpers, none of which any screen
/// or bar calls. Trace value would be `#ffc233` (grown-card fill) if the
/// band-on-selected pair is ever wired up.
pub const YELLOW_SHADE: iced::Color = rgb(0xf0a80a);

pub fn palette() -> Palette {
    Palette {
        bg: BG,
        panel: BLOOM,
        border: TEAL,
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
                    .with_top_left(Cut::Chamfer { x: 12.0, y: 13.0 })
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
                spine: 0.0,
                // The 20px foot the wave lives in.
                foot: 20.0,
                marker: MenuMarker::Text,
                echo: PanelEcho::Wave,
                echo_pad: 0.0,
            },
        },
        // The shelf band hangs 12px past the card and steps its
        // trailing corner down 8; measured off the target-app.svg
        // composite (deleted 2026-09-03).
        //
        // Unconsumed as of 2026-09-03: `style.banner` is read only by
        // `widgets::banner::banner` and `widgets::card::product_card`,
        // which nothing calls (the store is a canvas program). Trace
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
        // caller -- `bar.rs`, `screens::mail`, `widgets::bracket`,
        // `Surface::{outlined, filled, selected}` -- passes
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
        // --- login ---
        access: ACCESS,
        // --- end login ---
        // --- mailbox ---
        mailbox: mailbox(),
        // --- end mailbox ---
        // --- store ---
        store: STORE,
        store_selection: (1, 1),
        // --- end store ---
        // --- dashboard ---
        dashboard: DASHBOARD,
        // EVENTS, the fifth solid card in `dashboard-trace.svg` (group at
        // `translate(731 586) rotate(-30)`, the one `<use href="#card"
        // fill="#f5cb23" stroke="#fce89a">` under the comment "the
        // selection"); the other five fill `#2c9798`.
        dashboard_selection: 4,
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
// their right two fifths standing some 8px taller than their left, and
// that is why [`crate::style::Step`] exists. An earlier reading that
// made them plain rounded rectangles cannot be squared with
// `M257,470 H418 L430,463 H591.5 V497.5 H257 Z`.

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

pub const ACCESS: Access = Access {
    wash: Wash::RoseBloom,
    masthead: Masthead::Clock {
        labels: &[Legend::new("10:20 PM", 781.0, 74.0, 18.0, Ink::Fixed(rgb(0xb4ece3)))],
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
            field: Some(Plate::filled(
                Plot::new(257.0, 413.0, 335.0, 51.0),
                Ink::Fixed(WELL),
            )),
            caret: Some(Plate::filled(
                Plot::new(266.0, 421.0, 2.0, 22.0),
                Ink::Fixed(BARCODE),
            )),
            action: Some(
                Plate::filled(Plot::new(257.0, 463.0, 334.5, 34.5), Ink::Fixed(LIT))
                    .stepped(418.0, 7.0, 12.0),
            ),
            action_label: Some(
                Legend::new("ENTER", 510.0, 486.0, 14.0, Ink::Fixed(ON_LIT))
                    .centred()
                    .medium(),
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
                Plate::filled(Plot::new(649.0, 462.0, 335.5, 35.5), Ink::Fixed(LOCKED))
                    .stepped(812.0, 9.0, 12.0)
                    .edged(Ink::Fixed(LOCKED_EDGE), 1.0),
            ),
            action_label: Some(
                Legend::new("PROTECTED", 904.0, 486.0, 13.0, Ink::Fixed(ANNOTATION))
                    .centred()
                    .medium(),
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
                Plate::filled(Plot::new(1042.0, 462.0, 335.0, 35.5), Ink::Fixed(LOCKED))
                    .stepped(1205.0, 9.0, 12.0)
                    .edged(Ink::Fixed(LOCKED_EDGE), 1.0),
            ),
            action_label: Some(
                Legend::new("PROTECTED", 1297.0, 486.0, 13.0, Ink::Fixed(ANNOTATION))
                    .centred()
                    .medium(),
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
    Frame, Mail, MailBadges, MailButtons, MailList, MailPanel, Mailbox, Note, Piece,
    RowDecor, Run, Trim, FromAt, BL, BR, TL, TR,
};

const fn text(x: f32, y: f32, size: f32, ink: Ink, s: &'static str) -> Piece {
    Piece::Label(Note {
        at: Run::new(x, y, size, ink),
        text: s,
    })
}

/// The page, `<rect fill="#0e0d0c">` (:145): a step darker in B than
/// the store's `PAGE`, and what `#bloom` fades to.
const MAIL_PAGE: iced::Color = rgb(0x0e0d0c);
/// `#bloom` (:2-8): `cx=0.5 cy=-0.25 r=0.9` over the 1600x620 rect at
/// :146, so centre (800,-155), rx 1440, ry 558 -- the store's `ROSE`
/// stops, but a smaller radius and this page's black at the rim.
const MAIL_ROSE: &[(f32, iced::Color)] = &[
    (0.00, rgb(0xb05064)),
    (0.35, rgb(0x933b53)),
    (0.60, rgb(0x5c2236)),
    (0.85, rgb(0x1e0f14)),
    (1.00, MAIL_PAGE),
];
/// The ground, composited: page, bloom, and `#leftwash` (:9-13, :147),
/// which is the store's `MARGIN` lobe to the number -- `cx=0 cy=0.5
/// r=0.5` over the 600x780 rect at (0,60).
const MAIL_GROUND: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(MAIL_PAGE)),
    Prim::Lobe { x: 800.0, y: -155.0, rx: 1440.0, ry: 558.0, stops: MAIL_ROSE },
    Prim::Lobe { x: 0.0, y: 450.0, rx: 300.0, ry: 390.0, stops: MARGIN },
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

static CHROME: [Piece; 29] = [
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
    Piece::Poly {
        points: &FLAG,
        fill: None,
        stroke: Some(Ink::Select),
        width: 1.25,
        close: false,
    },
    text(
        578.0,
        365.0,
        6.0,
        Ink::Select,
        "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE",
    ),
    text(
        578.0,
        373.0,
        6.0,
        Ink::Select,
        "ALLOWED TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE.",
    ),
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

static TABS: [&str; 4] = ["DETAILS", "MODS", "PRICE", "DAMAGE"];
static LEVELS: [&str; 4] = ["01", "02", "03", "04"];

/// The five rows, trace lines 229-230 and 241-250. Every envelope is
/// the closed `#env` (lines 228 / 234-237; the trace defines no open
/// one), so nothing here is unread. The subjects are set in capitals
/// there; `title_upper` does that here.
static ROWS: [Mail; 5] = [
    Mail { subject: "You'll regret that", from: "Jackie", unread: false },
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

pub fn mailbox() -> Mailbox {
    Mailbox {
        backdrop: MAIL_BACKDROP,
        haze: &[],
        chrome: &CHROME,
        list: MailList {
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
            row_fill: None,
            row_stroke: None,
            row_width: 0.0,
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
            sel_notch: None,
            veneer: None,
            glyph_x: 165.0,
            glyph_dy: 12.0,
            glyph_w: 20.0,
            text_x: 220.0,
            title_dy: 27.0,
            title_size: 18.0,
            title_bold: false,
            from_dy: 49.0,
            from_size: 11.0,
            from_at: FromAt::Beneath,
            from_prefix: "from: ",
            title_upper: true,
            from_upper: false,
            new_pill: None,
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
            head_ink: Ink::Select,
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
        },
        buttons: MailButtons {
            // four chevron tabs stacked down the right, where the other
            // eras put a row of buttons
            first: Frame::new(1216.0, 306.0, 161.0, 46.0),
            dx: 0.0,
            dy: 54.0,
            count: 4,
            filled: Some(0),
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
            caption: None,
            caption_text: "",
            labels: &LEVELS,
        },
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

/// The outlines: a stop dimmer than the era's body teal, and its own
/// sampled tone rather than the `border` role -- the published kitsch
/// theme resolves `border` to `#2e5f57`, a third of this brightness,
/// and drawn in it the card frames drop out of the teal ink family
/// altogether.
pub const OUTLINE: iced::Color = rgb(0x5fd6c2);
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
/// The solid teal the nav bracket ends in.
pub const WAVE_INK: iced::Color = rgb(0x1bb6a3);
/// The card's compliance micro-text, the footnote bodies, the marker
/// boxes and the bright line centred at the foot.
pub const MICRO: iced::Color = rgb(0x5fc9b5);
pub const MARK: iced::Color = rgb(0x7fd4cc);
pub const FOOT_MICRO: iced::Color = rgb(0x82f0d3);
/// The logotype's heavy mint.
pub const LOGO: iced::Color = rgb(0x8ff2dc);
/// The rose bloom over the top of the page, and the grey-green left
/// margin under it: the trace's two `radialGradient`s, as their own
/// stop tables.
///
/// Both are bounding-box radials and so *elliptical*. The bloom is
/// `cx=0.5 cy=-0.25 r=0.95` over a 1600x620 rect, which is rx 1520 by
/// ry 589 -- a 2.6:1 lobe centred 155px above the page; getting the
/// aspect wrong puts the haze 300px down. The left wash is `cx=0
/// cy=0.5 r=0.5` over a 600x780 rect at x 0, y 60, and its stops are
/// *opacities* over `#2a2e2a`, composited here onto the page ground the
/// bloom has already faded to at that distance.
pub const PAGE: iced::Color = rgb(0x0e0e0d);

const ROSE: &[(f32, iced::Color)] = &[
    (0.00, rgb(0xb05064)),
    (0.35, rgb(0x933b53)),
    (0.60, rgb(0x5c2236)),
    (0.85, rgb(0x1e0f14)),
    (1.00, PAGE),
];
/// The left margin's stops are *opacities* over `#2a2e2a`, not
/// colours, so they are spelled with their alpha: painted opaque they
/// black out the rose bloom they are supposed to sit on.
const MARGIN_TONE: (f32, f32, f32) = (42.0 / 255.0, 46.0 / 255.0, 42.0 / 255.0);
const MARGIN: &[(f32, iced::Color)] = &[
    (0.00, iced::Color { r: MARGIN_TONE.0, g: MARGIN_TONE.1, b: MARGIN_TONE.2, a: 1.0 }),
    (0.60, iced::Color { r: MARGIN_TONE.0, g: MARGIN_TONE.1, b: MARGIN_TONE.2, a: 0.5 }),
    (1.00, iced::Color { r: MARGIN_TONE.0, g: MARGIN_TONE.1, b: MARGIN_TONE.2, a: 0.0 }),
];

/// The backdrop. Ground rather than ink -- but ground the extractor's
/// palette split depends on: the source spends five of its eight
/// clusters on it, so leaving the page flat is not the neutral choice
/// it looks like.
const BACKDROP: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(PAGE)),
    Prim::Lobe { x: 800.0, y: -155.0, rx: 1520.0, ry: 589.0, stops: ROSE },
    Prim::Lobe { x: 0.0, y: 450.0, rx: 300.0, ry: 390.0, stops: MARGIN },
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

const NAV_OUTLINE: &[Prim] = &[shut_path(0.0, 39.0, CHEVRON, Ink::Fixed(OUTLINE), 1.5)];
const NAV_SOLID: &[Prim] = &[fill_path(0.0, 39.0, CHEVRON, Ink::Select)];

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

/// The card outline: an r6 top-left, a 24px top-right chamfer, down to
/// the socket row's foot at y 320.
const CARD_EDGE: &[Seg] = &[
    Seg::Line(237.0, 0.0),
    Seg::Line(261.0, 24.0),
    Seg::Line(261.0, 320.0),
    Seg::Line(0.0, 320.0),
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

/// The gun silhouette, card-local, with its three dark openings as
/// even-odd subpaths: the ejection port, the magazine well and the
/// trigger guard. Measured off the photo's mint mask.
const GUN_BODY: &[Seg] = &[
    Seg::Line(76.0, 131.0), Seg::Line(76.0, 133.7), Seg::Line(136.0, 133.7),
    Seg::Line(137.7, 132.0), Seg::Line(146.0, 132.0), Seg::Line(169.3, 140.3),
    Seg::Line(196.0, 145.8), Seg::Line(197.7, 148.7), Seg::Line(202.7, 149.5),
    Seg::Line(205.2, 147.4), Seg::Line(235.2, 154.5), Seg::Line(235.2, 180.3),
    Seg::Line(231.0, 179.9), Seg::Line(228.5, 182.0), Seg::Line(228.5, 157.4),
    Seg::Line(223.5, 157.4), Seg::Line(223.5, 182.0), Seg::Line(221.0, 182.8),
    Seg::Line(214.3, 180.8), Seg::Line(211.8, 171.6), Seg::Line(201.8, 159.1),
    Seg::Line(184.3, 159.1), Seg::Line(183.5, 161.2), Seg::Line(180.2, 161.2),
    Seg::Line(157.7, 170.8), Seg::Line(135.2, 170.3), Seg::Line(131.8, 172.4),
    Seg::Line(121.0, 172.4), Seg::Line(116.8, 170.8), Seg::Line(115.2, 175.8),
    Seg::Line(111.8, 176.2), Seg::Line(86.0, 176.2), Seg::Line(85.2, 173.2),
    Seg::Line(59.3, 173.2), Seg::Line(56.8, 173.2), Seg::Line(56.0, 176.6),
    Seg::Line(55.2, 175.3), Seg::Line(45.2, 175.3), Seg::Line(43.5, 172.0),
    Seg::Line(38.5, 174.1), Seg::Line(36.8, 172.0), Seg::Line(36.8, 148.7),
    Seg::Line(40.2, 144.5), Seg::Line(41.0, 130.8),
    // the ejection port
    Seg::Move(127.0, 144.5), Seg::Line(146.0, 144.5), Seg::Line(146.0, 146.0),
    Seg::Line(148.0, 149.0), Seg::Line(147.0, 151.0), Seg::Line(135.0, 152.5),
    Seg::Line(131.0, 150.0), Seg::Line(124.0, 149.0), Seg::Line(124.0, 146.5),
    // the magazine well
    Seg::Move(135.0, 152.5), Seg::Line(153.0, 152.5), Seg::Line(153.0, 167.0),
    Seg::Line(135.0, 167.0),
    // the trigger guard
    Seg::Move(159.0, 147.0), Seg::Line(171.0, 147.0), Seg::Line(174.0, 150.0),
    Seg::Line(174.0, 156.0), Seg::Line(170.0, 158.5), Seg::Line(164.0, 159.5),
    Seg::Line(164.0, 152.0),
];

/// The gun's inner detail, drawn in the ground colour over the
/// silhouette: the rail rule and its ticks, the panel lines, the 3x4
/// grid, the magazine tubes and the row of squares along the foot.
const GUN_DETAIL: &[Seg] = &[
    Seg::Line(136.0, 137.0),
    Seg::Move(50.0, 131.0), Seg::Line(50.0, 137.0),
    Seg::Move(58.0, 131.0), Seg::Line(58.0, 137.0),
    Seg::Move(66.0, 131.0), Seg::Line(66.0, 137.0),
    Seg::Move(74.0, 131.0), Seg::Line(74.0, 137.0),
    Seg::Move(61.0, 140.0), Seg::Line(61.0, 172.0),
    Seg::Move(84.0, 140.0), Seg::Line(84.0, 172.0),
    Seg::Move(66.0, 145.0), Seg::Line(88.0, 167.0),
    Seg::Move(41.0, 147.0), Seg::Line(61.0, 147.0),
    Seg::Move(88.0, 147.0), Seg::Line(118.0, 147.0),
    Seg::Move(94.0, 152.0), Seg::Line(116.0, 152.0),
    Seg::Move(94.0, 157.0), Seg::Line(116.0, 157.0),
    Seg::Move(94.0, 162.0), Seg::Line(116.0, 162.0),
    Seg::Move(94.0, 167.0), Seg::Line(116.0, 167.0),
    Seg::Move(101.0, 152.0), Seg::Line(101.0, 167.0),
    Seg::Move(109.0, 152.0), Seg::Line(109.0, 167.0),
    Seg::Move(49.0, 149.0), Seg::Line(49.0, 172.0),
    Seg::Move(37.0, 152.0), Seg::Line(49.0, 152.0),
    Seg::Move(37.0, 158.0), Seg::Line(49.0, 158.0),
    Seg::Move(37.0, 164.0), Seg::Line(49.0, 164.0),
    Seg::Move(37.0, 170.0), Seg::Line(49.0, 170.0),
    Seg::Move(59.0, 171.0), Seg::Line(83.0, 171.0),
    Seg::Move(63.0, 171.0), Seg::Line(63.0, 175.0),
    Seg::Move(67.0, 171.0), Seg::Line(67.0, 175.0),
    Seg::Move(71.0, 171.0), Seg::Line(71.0, 175.0),
    Seg::Move(75.0, 171.0), Seg::Line(75.0, 175.0),
    Seg::Move(79.0, 171.0), Seg::Line(79.0, 175.0),
    Seg::Move(150.0, 138.0), Seg::Line(190.0, 146.0),
    Seg::Move(214.0, 166.0), Seg::Line(223.0, 166.0),
    Seg::Move(228.5, 166.0), Seg::Line(235.0, 166.0),
];

/// The socket row's QR glyph: fifteen 5px cells on a 6px row pitch and
/// no fixed column pitch, so it is spelled out rather than gridded.
macro_rules! qr {
    ($ink:expr) => {
        &[
            fill_rect(12.0, 283.0, 5.0, 5.0, $ink), fill_rect(22.0, 283.0, 5.0, 5.0, $ink), fill_rect(32.0, 283.0, 5.0, 5.0, $ink),
            fill_rect(17.0, 289.0, 5.0, 5.0, $ink), fill_rect(37.0, 289.0, 5.0, 5.0, $ink),
            fill_rect(12.0, 295.0, 5.0, 5.0, $ink), fill_rect(24.0, 295.0, 5.0, 5.0, $ink), fill_rect(32.0, 295.0, 5.0, 5.0, $ink),
            fill_rect(18.0, 301.0, 5.0, 5.0, $ink), fill_rect(27.0, 301.0, 5.0, 5.0, $ink), fill_rect(37.0, 301.0, 5.0, 5.0, $ink),
            fill_rect(12.0, 307.0, 5.0, 5.0, $ink), fill_rect(22.0, 307.0, 5.0, 5.0, $ink), fill_rect(32.0, 307.0, 5.0, 5.0, $ink),
        ]
    };
}
const QR_STD: &[Prim] = qr!(Ink::Fixed(MINT_BAR));

/// The band's compliance marks, in the band's LOWER half: a
/// certification square, a disc-in-square, a C-in-C mark and a rounded
/// text block holding a warning triangle.
macro_rules! band_marks {
    ($ink:expr, $knock:expr) => {
        &[
            line_path(-16.5, 78.5, &[Seg::Line(-6.0, 78.5), Seg::Line(-6.0, 89.5), Seg::Line(-16.5, 89.5), Seg::Line(-16.5, 78.5)], $ink, 1.0),
            line_path(-0.5, 78.5, &[Seg::Line(10.0, 78.5), Seg::Line(10.0, 89.5), Seg::Line(-0.5, 89.5), Seg::Line(-0.5, 78.5)], $ink, 1.0),
            Prim::Circle { x: 4.75, y: 84.0, r: 4.2, fill: Some($ink), stroke: None, width: 0.0 },
            line_path(24.5, 78.5, &[Seg::Line(17.5, 78.5), Seg::Quad { cx: 15.5, cy: 78.5, x: 15.5, y: 80.5 }, Seg::Line(15.5, 87.5), Seg::Quad { cx: 15.5, cy: 89.5, x: 17.5, y: 89.5 }, Seg::Line(24.5, 89.5)], $ink, 1.4),
            Prim::Round { x: 28.5, y: 74.0, w: 64.0, h: 18.0, r: 1.5, fill: None, stroke: Some($ink), width: 0.5 },
            fill_path(30.5, 90.0, &[Seg::Line(36.5, 78.0), Seg::Line(42.5, 90.0)], $ink),
            fill_rect(45.0, 77.0, 46.0, 1.0, $ink),
            fill_rect(45.0, 81.2, 45.0, 1.0, $ink),
            fill_rect(45.0, 85.4, 46.0, 1.0, $ink),
            fill_rect(45.0, 89.5, 42.0, 1.0, $ink),
            fill_rect(36.0, 82.5, 1.1, 4.2, $knock),
        ]
    };
}
const BAND_MARKS: &[Prim] = band_marks!(Ink::Fixed(ON_BAND), Ink::Fixed(BAND));
const BAND_MARKS_SEL: &[Prim] = band_marks!(Ink::Fixed(ON_BAND), Ink::Select);

/// A standard product card, at its outline's own origin.
const CARD: &[Prim] = &[
    shut_path(6.0, 0.0, CARD_EDGE, Ink::Fixed(OUTLINE), 1.5),
    txt(12.0, 29.0, 19.0, Ink::Fg, "MAGNUM 650"),
    txt(12.0, 49.0, 17.0, Ink::Fg, "HAND GUN"),
    fill_path(-27.0, 94.0, BAND_SHAPE, Ink::Fixed(BAND)),
    Prim::At { x: 0.0, y: 0.0, prims: BAND_MARKS },
    fill_rect(160.0, 70.0, 60.0, 9.0, Ink::Fixed(ON_BAND)),
    txt_bold(163.0, 78.0, 8.0, Ink::Fixed(BAND), "PETROCHEM"),
    txt(160.0, 89.0, 8.0, Ink::Fixed(ON_BAND), "BETTERLIFE TEC"),
    fill_path(41.0, 131.0, GUN_BODY, Ink::Fixed(GUN)),
    line_path(41.0, 137.0, GUN_DETAIL, Ink::Fixed(PAGE), 1.0),
    txt_mid(41.0, 225.0, 17.0, Ink::Fg, "DPS"),
    txt_mid(101.0, 225.0, 17.0, Ink::Fg, "PNT"),
    txt_mid(162.0, 225.0, 17.0, Ink::Fg, "ACC"),
    txt_mid(222.0, 225.0, 17.0, Ink::Fg, "ROF"),
    fill_path(8.0, 232.0, MINT_SHAPE, Ink::Fixed(MINT_BAR)),
    txt_mid(41.0, 255.0, 20.0, Ink::Fixed(ON_MINT_BAR), "86"),
    txt_mid(101.0, 255.0, 20.0, Ink::Fixed(ON_MINT_BAR), "30"),
    txt_mid(162.0, 255.0, 20.0, Ink::Fixed(ON_MINT_BAR), "5"),
    txt_mid(222.0, 255.0, 20.0, Ink::Fixed(ON_MINT_BAR), "5"),
    fill_rect(0.0, 273.25, 261.0, 1.5, Ink::Fixed(OUTLINE)),
    fill_rect(51.25, 274.0, 1.5, 46.0, Ink::Fixed(OUTLINE)),
    fill_rect(118.25, 274.0, 1.5, 46.0, Ink::Fixed(OUTLINE)),
    fill_rect(189.25, 274.0, 1.5, 46.0, Ink::Fixed(OUTLINE)),
    Prim::At { x: 0.0, y: 0.0, prims: QR_STD },
    txt_mid(85.0, 295.0, 9.0, Ink::Fg, "EMPTY"),
    txt_mid(85.0, 307.0, 9.0, Ink::Fg, "SOCKET"),
    txt_mid(154.0, 295.0, 9.0, Ink::Fg, "EMPTY"),
    txt_mid(154.0, 307.0, 9.0, Ink::Fg, "SOCKET"),
    txt_mid(225.0, 295.0, 9.0, Ink::Fg, "EMPTY"),
    txt_mid(225.0, 307.0, 9.0, Ink::Fg, "SOCKET"),
    txt(4.0, 341.0, 6.5, Ink::Fixed(MICRO), "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO"),
    txt(4.0, 349.0, 6.5, Ink::Fixed(MICRO), "MANIPULATE, ACCESS OR DISABLE THIS DEVICE."),
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
const QR_SEL: &[Prim] = &[
    fill_rect(12.0, 423.0, 5.0, 5.0, Ink::Fixed(GROWN_DETAIL)), fill_rect(22.0, 423.0, 5.0, 5.0, Ink::Fixed(GROWN_DETAIL)), fill_rect(32.0, 423.0, 5.0, 5.0, Ink::Fixed(GROWN_DETAIL)),
    fill_rect(17.0, 429.0, 5.0, 5.0, Ink::Fixed(GROWN_DETAIL)), fill_rect(37.0, 429.0, 5.0, 5.0, Ink::Fixed(GROWN_DETAIL)),
    fill_rect(12.0, 435.0, 5.0, 5.0, Ink::Fixed(GROWN_DETAIL)), fill_rect(24.0, 435.0, 5.0, 5.0, Ink::Fixed(GROWN_DETAIL)), fill_rect(32.0, 435.0, 5.0, 5.0, Ink::Fixed(GROWN_DETAIL)),
    fill_rect(18.0, 441.0, 5.0, 5.0, Ink::Fixed(GROWN_DETAIL)), fill_rect(27.0, 441.0, 5.0, 5.0, Ink::Fixed(GROWN_DETAIL)), fill_rect(37.0, 441.0, 5.0, 5.0, Ink::Fixed(GROWN_DETAIL)),
    fill_rect(12.0, 447.0, 5.0, 5.0, Ink::Fixed(GROWN_DETAIL)), fill_rect(22.0, 447.0, 5.0, 5.0, Ink::Fixed(GROWN_DETAIL)), fill_rect(32.0, 447.0, 5.0, 5.0, Ink::Fixed(GROWN_DETAIL)),
];

const GROWN: &[Prim] = &[
    fill_path(6.0, 0.0, GROWN_EDGE, Ink::Select),
    fill_path(-27.0, 94.0, GROWN_FLAG, Ink::Select),
    fill_rect(0.0, 58.5, 256.0, 1.0, Ink::Fixed(BAND_RULE)),
    fill_rect(0.0, 93.5, 261.0, 1.0, Ink::Fixed(BAND_RULE)),
    txt(12.0, 29.0, 19.0, Ink::Fixed(ON_GROWN), "MAGNUM 650"),
    txt(12.0, 49.0, 17.0, Ink::Fixed(ON_GROWN), "HAND GUN"),
    Prim::At { x: 0.0, y: 0.0, prims: BAND_MARKS_SEL },
    fill_rect(160.0, 70.0, 60.0, 9.0, Ink::Fixed(ON_GROWN)),
    txt_bold(163.0, 78.0, 8.0, Ink::Select, "PETROCHEM"),
    txt(160.0, 89.0, 8.0, Ink::Fixed(ON_BAND), "BETTERLIFE TEC"),
    // the gun, dark on amber, 14px higher than on a standard card
    Prim::At { x: 1.5, y: -13.7, prims: &[
        fill_path(41.0, 131.0, GUN_BODY, Ink::Fixed(GROWN_GUN)),
        line_path(41.0, 137.0, GUN_DETAIL, Ink::Select, 1.0),
    ] },
    txt_mid(41.0, 204.0, 17.0, Ink::Fixed(ON_BAND), "DPS"),
    txt_mid(101.0, 204.0, 17.0, Ink::Fixed(ON_BAND), "PNT"),
    txt_mid(162.0, 204.0, 17.0, Ink::Fixed(ON_BAND), "ACC"),
    txt_mid(222.0, 204.0, 17.0, Ink::Fixed(ON_BAND), "ROF"),
    txt_mid(41.0, 233.0, 20.0, Ink::Fixed(ON_BAND), "86"),
    txt_mid(101.0, 233.0, 20.0, Ink::Fixed(ON_BAND), "30"),
    txt_mid(162.0, 233.0, 20.0, Ink::Fixed(ON_BAND), "5"),
    txt_mid(222.0, 233.0, 20.0, Ink::Fixed(ON_BAND), "5"),
    // the lower body, amber-outlined
    line_path(0.0, 239.0, &[
        Seg::Line(0.0, 462.0), Seg::Line(261.0, 462.0), Seg::Line(261.0, 239.0),
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
    txt_mid(85.0, 436.0, 9.0, Ink::Fixed(GROWN_DETAIL), "EMPTY"),
    txt_mid(85.0, 448.0, 9.0, Ink::Fixed(GROWN_DETAIL), "SOCKET"),
    txt_mid(154.0, 436.0, 9.0, Ink::Fixed(GROWN_DETAIL), "EMPTY"),
    txt_mid(154.0, 448.0, 9.0, Ink::Fixed(GROWN_DETAIL), "SOCKET"),
    txt_mid(225.0, 436.0, 9.0, Ink::Fixed(GROWN_DETAIL), "EMPTY"),
    txt_mid(225.0, 448.0, 9.0, Ink::Fixed(GROWN_DETAIL), "SOCKET"),
    txt(4.0, 481.0, 6.5, Ink::Fixed(GROWN_MICRO), "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO"),
    txt(4.0, 489.0, 6.5, Ink::Fixed(GROWN_MICRO), "MANIPULATE, ACCESS OR DISABLE THIS DEVICE."),
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

macro_rules! shelf {
    ($i:expr) => {
        &[Prim::Plate {
            group: Group::Card,
            index: $i,
            x: 0.0,
            y: 0.0,
            w: 261.0,
            h: 320.0,
            on: GROWN,
            off: CARD,
        }]
    };
}
const SHELF_0: &[Prim] = shelf!(0);
const SHELF_1: &[Prim] = shelf!(1);
const SHELF_2: &[Prim] = shelf!(2);
const SHELF_3: &[Prim] = shelf!(3);

pub const STORE: &[Prim] = &[
    Prim::At { x: 0.0, y: 0.0, prims: BACKDROP },
    // logotype: a heavy extended face, the T outline-only
    Prim::Wide { x: 155.0, y: 132.0, size: 60.0, stretch: 1.7, ink: Ink::Fixed(LOGO), face: Face::Bold, content: "4S" },
    shut_path(280.0, 88.0, TEE, Ink::Fixed(LOGO), 1.3),
    Prim::Spaced { x: 154.0, y: 155.0, size: 15.0, ink: Ink::Fg, face: Face::Medium, pitch: 32.0, content: "STORE" },
    // customer chip and account lines
    Prim::Round { x: 123.0, y: 178.0, w: 215.0, h: 22.0, r: 8.0, fill: None, stroke: Some(Ink::Fixed(OUTLINE)), width: 1.5 },
    txt(133.0, 193.0, 12.0, Ink::Fg, "customer"),
    txt(243.0, 193.0, 12.0, Ink::Fg, "#NC488402"),
    txt(133.0, 227.0, 12.0, Ink::Fg, "loyalty discount"),
    txt_end(306.0, 227.0, 12.0, Ink::Fg, "10%"),
    txt(133.0, 243.0, 12.0, Ink::Fg, "last update"),
    txt_end(306.0, 243.0, 12.0, Ink::Fg, "10/05/2077"),
    // the nav bracket: one swept line wrapping the customer block, and
    // the solid wave it ends in
    fill_path(106.5, 575.0, WAVE_BODY, Ink::Fixed(WAVE_INK)),
    line_path(440.0, 186.0, BRACKET_PATH, Ink::Fixed(OUTLINE), 2.0),
    // nav chevrons, SMG solid
    Prim::Plate { group: Group::Category, index: 0, x: 140.0, y: 297.0, w: 216.0, h: 39.0, on: NAV_ON_0, off: NAV_OFF_0 },
    Prim::Plate { group: Group::Category, index: 1, x: 140.0, y: 357.0, w: 216.0, h: 39.0, on: NAV_ON_1, off: NAV_OFF_1 },
    Prim::Plate { group: Group::Category, index: 2, x: 140.0, y: 417.0, w: 216.0, h: 39.0, on: NAV_ON_2, off: NAV_OFF_2 },
    Prim::Plate { group: Group::Category, index: 3, x: 140.0, y: 477.0, w: 216.0, h: 39.0, on: NAV_ON_3, off: NAV_OFF_3 },
    Prim::Plate { group: Group::Category, index: 4, x: 140.0, y: 537.0, w: 216.0, h: 39.0, on: NAV_ON_4, off: NAV_OFF_4 },
    // the shelf, on a 320px pitch; the fourth runs off the frame edge
    Prim::At { x: 484.0, y: 218.0, prims: SHELF_0 },
    Prim::At { x: 804.0, y: 218.0, prims: SHELF_1 },
    Prim::At { x: 1123.0, y: 218.0, prims: SHELF_2 },
    Prim::At { x: 1443.0, y: 218.0, prims: SHELF_3 },
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
    txt_bold(503.0, 870.0, 9.0, Ink::Fixed(FOOT_MICRO), "ARASAKA CONSUMER TECHNOLOGY"),
    txt(640.0, 870.0, 9.0, Ink::Fixed(FOOT_MICRO), "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE."),
];
// --- end store -----------------------------------------------------------

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
//   ground, bloom and ghosts are one `Prim::Soft` group (`HUB_BACK`),
//   rasterised in sRGB by `screens/soft.rs` rather than blended in
//   linear light by wgpu -- the difference is 4-10 levels per channel
//   on the faint tails, and it is what G2i failed this screen on.
// - the two `text-anchor="middle"` letters under a `scale(1.7 1)` --
//   `Prim::Wide` is start-anchored, so A / C / D / B are placed at the
//   box centreline minus half the trace's stated cap width (15.8, line
//   124): 176.7 - 7.9 and so on.
// - the bloom: `radialGradient cx=0.52 cy=-0.05 r=0.85` over the
//   1600x620 rect is an ellipse centred (832, -31) with radii 1360 x
//   527, which the `Lobe` draws as rings; its foot stop is the page
//   ground, so the rect's clip edge at y 620 (beyond ry) never shows.

use crate::style::Anchor;

/// The hub's page ground and bloom stops: its own samples, a shade
/// off the store's `PAGE` / `ROSE`.
pub const HUB_GROUND: iced::Color = rgb(0x0d0d0c);
const HUB_ROSE: &[(f32, iced::Color)] = &[
    (0.00, rgb(0xa84a5e)),
    (0.45, rgb(0x7a3346)),
    (0.75, rgb(0x3a1c24)),
    (1.00, HUB_GROUND),
];
/// The selection yellow the hub samples -- EVENTS, badge 02, the
/// BRAINDANCE tab and outlines -- and the inks set on it. A hair off
/// the palette's `YELLOW` (#fcc428) and the store's `BAND` (#fec32f).
pub const HUB_YELLOW: iced::Color = rgb(0xf5cb23);
pub const ON_HUB_YELLOW: iced::Color = rgb(0x4a3a05);
pub const ON_BADGE: iced::Color = rgb(0x6b4d08);
/// The selected blade's lit edge.
pub const SELECT_EDGE: iced::Color = rgb(0xfce89a);
/// An idle blade's fill; its edge is `NAME_INK` and its label
/// `ON_MINT_BAR`, both already sampled on other screens.
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
                Prim::Turn { x: $cx, y: $cy, angle: $a, prims: &[tracked_mid(0.0, 6.5, 19.0, 2.0, Ink::Fixed(ON_MINT_BAR), $label)] },
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
    Prim::Turn { x: 0.0, y: 0.0, angle: 90.0, prims: &[tracked_mid(0.0, 6.5, 19.0, 2.0, Ink::Fixed(ON_MINT_BAR), "PRODUCTS")] },
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

/// The BRAINDANCE header tab, chamfered top-left, top-right and
/// bottom-left: `M 1213,261 H 1419 L 1432,274 V 299 H 1213 L 1200,286
/// V 274 Z` (trace line 273).
const TAB: &[Seg] = &[
    Seg::Line(1419.0, 261.0),
    Seg::Line(1432.0, 274.0),
    Seg::Line(1432.0, 299.0),
    Seg::Line(1213.0, 299.0),
    Seg::Line(1200.0, 286.0),
    Seg::Line(1200.0, 274.0),
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

/// The ground, the bloom and the ghost trails, composited in software:
/// up to seven translucent cards stack on the haze here, and wgpu's
/// linear blend cannot land rsvg's sRGB `fill-opacity` on a backdrop
/// that varies under the stack -- see `screens/soft.rs`. Everything in
/// this group sits under the header and the solid blades, so drawing
/// it first, ahead of the header text, changes no pixel's order.
const HUB_BACK: &[Prim] = &[
    // ground and bloom (trace lines 89-94, 100-101)
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(HUB_GROUND)),
    Prim::Lobe { x: 832.0, y: -31.0, rx: 1360.0, ry: 527.0, stops: HUB_ROSE },
    // the ghosts, farthest first, every trail stepping (+20,-20) in
    // screen space from its solid card (lines 196-239); they belong to
    // no plate because they do not change with the selection and the
    // solid cards paint over them in one pass
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

pub const DASHBOARD: &[Prim] = &[
    Prim::Soft { prims: HUB_BACK },
    // header notes: Rajdhani 600 8 stretched 1.3 (lines 107-122)
    Prim::Wide { x: 205.0, y: 113.6, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "SPARE TIME MANAGER WAS DEVELO-" },
    Prim::Wide { x: 205.0, y: 122.8, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "PED BY SEOCHO. SERVING CUSTO-" },
    Prim::Wide { x: 205.0, y: 131.9, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "MERS SINCE 2006." },
    Prim::Wide { x: 715.3, y: 113.6, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "SPARE TIME MANAGER WAS DEVELO-" },
    Prim::Wide { x: 715.3, y: 122.8, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "PED BY SEOCHO. SERVING CUSTO-" },
    Prim::Wide { x: 715.3, y: 131.9, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "MERS SINCE 2006." },
    Prim::Wide { x: 1253.8, y: 113.6, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "MAPS ARE PROVIDED BY SEOCHO." },
    Prim::Wide { x: 1253.8, y: 122.8, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "SATELITE SERVICES SINCE 2006." },
    // boxed A / C / D: 24x24 2px squares (lines 125-129) holding a
    // size-18 cap stretched 1.7, centred on the box (lines 130-134)
    line_rect(164.6, 109.8, 24.2, 24.2, Ink::Fixed(BRIGHT), 2.0),
    line_rect(673.3, 109.8, 24.2, 24.2, Ink::Fixed(BRIGHT), 2.0),
    line_rect(1215.2, 109.8, 24.0, 24.2, Ink::Fixed(BRIGHT), 2.0),
    Prim::Wide { x: 168.8, y: 129.2, size: 18.0, stretch: 1.7, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "A" },
    Prim::Wide { x: 677.5, y: 129.2, size: 18.0, stretch: 1.7, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "C" },
    Prim::Wide { x: 1219.3, y: 129.2, size: 18.0, stretch: 1.7, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "D" },
    // section labels: Rajdhani 600 12.3 stretched 1.37 (lines 137-141)
    Prim::Wide { x: 165.0, y: 163.0, size: 12.3, stretch: 1.37, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "USER" },
    Prim::Wide { x: 674.0, y: 163.0, size: 12.3, stretch: 1.37, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "SECURITY LEVEL" },
    Prim::Wide { x: 1216.0, y: 163.0, size: 12.3, stretch: 1.37, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "DESCRIPTION" },
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
    // paragraphs of yellow bars (lines 273-295)
    fill_path(1213.0, 261.0, TAB, Ink::Fixed(HUB_YELLOW)),
    Prim::Text { x: 1222.0, y: 288.0, size: 20.0, ink: Ink::Fixed(ON_HUB_YELLOW), face: Face::SemiBold, anchor: Anchor::Start, content: "BRAINDANCE" },
    line_rect(1172.0, 306.0, 180.0, 24.0, Ink::Fixed(HUB_YELLOW), 1.0),
    txt(1178.0, 316.0, 7.0, Ink::Fixed(TAPE_INK), "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE"),
    txt(1178.0, 325.0, 7.0, Ink::Fixed(TAPE_INK), "ALLOWED TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE."),
    line_rect(1207.5, 299.5, 224.5, 367.0, Ink::Fixed(HUB_YELLOW), 1.25),
    fill_rect(1224.0, 345.0, 196.0, 11.0, Ink::Fixed(GROWN_MICRO)),
    fill_rect(1224.0, 365.0, 188.0, 11.0, Ink::Fixed(GROWN_MICRO)),
    fill_rect(1224.0, 385.0, 192.0, 11.0, Ink::Fixed(GROWN_MICRO)),
    fill_rect(1224.0, 405.0, 176.0, 11.0, Ink::Fixed(GROWN_MICRO)),
    fill_rect(1224.0, 425.0, 120.0, 11.0, Ink::Fixed(GROWN_MICRO)),
    fill_rect(1224.0, 448.0, 194.0, 11.0, Ink::Fixed(GROWN_MICRO)),
    fill_rect(1224.0, 468.0, 184.0, 11.0, Ink::Fixed(GROWN_MICRO)),
    fill_rect(1224.0, 488.0, 188.0, 11.0, Ink::Fixed(GROWN_MICRO)),
    fill_rect(1224.0, 508.0, 98.0, 11.0, Ink::Fixed(GROWN_MICRO)),
    // B DEVICE SOFTWARE mark: notes Rajdhani 700 8 stretched 1.3, the
    // boxed B, the label (lines 302-308)
    Prim::Wide { x: 609.9, y: 746.0, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::Bold, content: "MAPS ARE PROVIDED BY SEOCHO." },
    Prim::Wide { x: 609.9, y: 755.0, size: 8.0, stretch: 1.3, ink: Ink::Fixed(BRIGHT), face: Face::Bold, content: "SATELITE SERVICES SINCE 2006." },
    line_rect(574.3, 741.0, 24.0, 24.2, Ink::Fixed(BRIGHT), 2.0),
    Prim::Wide { x: 578.4, y: 761.2, size: 18.0, stretch: 1.7, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "B" },
    Prim::Wide { x: 575.0, y: 794.0, size: 12.3, stretch: 1.37, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, content: "DEVICE SOFTWARE" },
    // the foot line, one bold weight, two runs (lines 318-321)
    txt_bold(503.0, 870.0, 9.0, Ink::Fixed(BRIGHT), "ARASAKA CONSUMER TECHNOLOGY"),
    txt_bold(641.0, 870.0, 9.0, Ink::Fixed(BRIGHT), "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE."),
];
// --- end dashboard -------------------------------------------------------
