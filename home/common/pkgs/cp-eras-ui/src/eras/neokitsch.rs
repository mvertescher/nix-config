//! Neokitsch -- "substance and style".
//!
//! Gold line-work on true black under a violet haze. Sampled from
//! Behance Part 1, gallery positions 64-72 (title card 63) per
//! `docs/sources.md`; the "doc #54-62" this comment used to give came
//! from an earlier, smaller scrape and is shifted by ten.
//!
//! This comment used to add "with the device frame itself part of the
//! UI". No trace has a full-screen frame: the four
//! `docs/neokitsch/*-trace.svg` files draw the haze, the wire band and
//! per-widget outlines, never a double gold stroke around the screen
//! (`docs/neokitsch/README.md`, "There is no device frame"). The frame
//! was an invention of the deleted `target-app.svg` composite.
//! `chrome: Chrome::DeviceFrame` below stands because its only remaining
//! reader is the bar's mail example panel (`panels::mail` through
//! `widgets::chrome`); the dashboard no longer reads it, being a `Prim`
//! table transcribed from `dashboard-trace.svg` (the
//! `// --- dashboard ---` block at the foot of this file) since the
//! `Layout` fold of 2026-09-03. See `ERAS-DELTA.md`.
//!
//! Its defining rule is that selection is a *material*, not a colour:
//! the chosen tab, pill, card or mail row fills with wood veneer. That
//! is the one place the four eras genuinely stress the abstraction --
//! see [`crate::style::Selection::Veneer`] and `widgets::surface`, which
//! synthesises the grain rather than shipping a raster asset.

use crate::palette::{rgb, Ornaments, Palette};
use crate::style::{
    Banner, Bar, BarChrome, BarGround, BarMenu, BarOrnament, Chrome, Compliance, Corner, Dress,
    Era, Face, Footnotes, Ground, Ink, MenuMarker, MenuRule, Metrics, Nameplate,
    PanelEcho, Selection, Style, Tab, Ticket, WindowLabel,
};
use crate::widgets::surface::{Corners, Cut};
// --- login ---
use crate::style::{
    Access, Bevel, Colophon, Fixture, Legend, Masthead, Plate, Plot, Slot, Wash,
};
// --- end login ---

pub const BG: iced::Color = rgb(0x0a0a0a);
pub const BLOOM: iced::Color = rgb(0x34344c);
pub const FRAME: iced::Color = rgb(0x916424);
pub const FRAME_INNER: iced::Color = rgb(0x5e3414);
pub const GOLD_TEXT: iced::Color = rgb(0xe7c686);
pub const CHAMPAGNE: iced::Color = rgb(0xd3b279);
pub const VENEER: iced::Color = rgb(0xe3af5f);
pub const VENEER_LIGHT: iced::Color = rgb(0xf4c474);
pub const VENEER_DARK: iced::Color = rgb(0xd8a558);
pub const GRAIN: iced::Color = rgb(0xb98a44);
pub const AMBER: iced::Color = rgb(0xfcc474);
pub const FIELD: iced::Color = rgb(0x2c1c14);
pub const ON_VENEER: iced::Color = rgb(0x3a2410);
pub const DIM: iced::Color = rgb(0x8a7048);
/// Lit side of the relief bevel, against `FRAME_INNER` as its shaded
/// one. Originally "top stop of the device frame's outer stroke"; no
/// trace has a device frame (module doc), but this is the era const
/// nearest the traced outlines -- `store-trace.svg` card outline
/// `#c5965a`, `dashboard-trace.svg` strokes `#bd8951` / `#a97c48`,
/// `#dab176` and `#e0b67a` elsewhere -- and `docs/neokitsch/bar.svg`
/// draws its outlines in it (`stroke="#c69a55" stroke-width="1.6"`)
/// for that reason. `FRAME #916424` and `FRAME_INNER #5e3414` are
/// sampled by nothing in the traces.
pub const FRAME_LIT: iced::Color = rgb(0xc69a55);
/// Originally "the fine lines of a strata divider, bunching into a
/// wedge". No trace has a strata divider: the era's layered fine lines
/// are the wire band and the onion rings (`docs/neokitsch/README.md`,
/// "stacked-hairline wire band"), and `#634427` is sampled by nothing in
/// the four traces. Still consumed as `ornaments.ornament`: the one
/// reader this era reaches is `Chrome::DeviceFrame` in `widgets::chrome`
/// (dashboard top bar / footer; the bar's other reader is
/// `PanelEcho::Wave`, which is kitsch's echo, not this era's). The
/// gated renders override it from `home/themes/neokitsch/palettes.nix`.
pub const STRATA: iced::Color = rgb(0x634427);

/// The four stops of the violet haze every screen in the run sits in,
/// measured in `docs/neokitsch/dashboard-trace.svg` and copied verbatim
/// into `store-trace.svg`'s `<radialGradient id="haze">`. Separate
/// constants from `BLOOM` because they are a *measurement of the
/// material* and `BLOOM` is the single colour the page-ground widget
/// stacks discs of: the lobe is violet at the core and cold grey two
/// thirds out, which one colour at one alpha cannot say.
pub const HAZE_CORE: iced::Color = rgb(0x574568);
pub const HAZE_MID: iced::Color = rgb(0x3a3853);
pub const HAZE_EDGE: iced::Color = rgb(0x16121a);
pub const HAZE_OUT: iced::Color = rgb(0x0e0a0d);

pub fn palette() -> Palette {
    Palette {
        bg: BG,
        panel: BLOOM,
        border: FRAME,
        dim: DIM,
        fg: GOLD_TEXT,
        // The only strong call-to-action colour in the reference; used
        // for ENTER / LOGIN bars and nothing else.
        alert: AMBER,
        tape: VENEER,
        select: VENEER,
        on_select: ON_VENEER,
        // No highlight band anywhere in the references; that is a
        // kitsch device and this era does hierarchy by brightness.
        emphasis: None,
        // You cannot shade a *material*: a slightly darker champagne
        // band on a grained plank reads as a knot, so the era inverts
        // instead. The old `target-components.svg` (the by-eye sheet
        // replaced 2026-09-03 by `components.svg`, rebuilt from the traces;
        // sheet citations in this file are the old sheet's) filled the selected card's
        // footer nameplate `#3a2410` and prints the name on it in
        // `#e7c686` -- the era's `fg`, a stop brighter than the
        // champagne the unselected band is filled with.
        banner_selected: Some((ON_VENEER, GOLD_TEXT)),
        ornaments: Ornaments {
            // The card's footer nameplate, and the BASKET panel at the
            // top right of the store. Champagne, not veneer: selection
            // is a material here and the nameplate is not selected.
            banner: Some((CHAMPAGNE, ON_VENEER)),
            // The device frame is a double stroke, lit outside and
            // shaded in.
            relief: Some((FRAME_LIT, FRAME_INNER)),
            ornament: Some(STRATA),
            // The login field, and the socket wells on a card.
            inset: Some(FIELD),
        },
        cta: AMBER,
        bloom: BLOOM,
    }
}

pub fn style() -> Style {
    Style {
        era: Era::Neokitsch,
        palette: palette(),
        corner: Corner::ClipTopRight { cut: 30.0 },
        selection: Selection::Veneer,
        // Top-centre, softer and wider than the kitsch bloom.
        ground: Ground::Bloom {
            x: 0.5,
            y: 0.0,
            radius: 0.75,
        },
        chrome: Chrome::DeviceFrame,
        nameplate: Nameplate::Footer,
        // --- bar --- (docs/neokitsch/bar.svg, IMPLEMENTATION DELTA)
        //
        // The bar strip sits where the top of every neokitsch screen
        // sits, so it wears that region's chrome: the violet haze, the
        // header wire band, the store nav's tabbed buttons, and a
        // veneer plate for whatever is selected.
        bar: Bar {
            height: 31,
            host_tape: true,

            pad_left: 6.0,
            pad_right: 6.0,
            pad_y: 3.0,
            gap: 9.0,
            // The store nav button is 38 wide on a 45.2 pitch, but its
            // 1.6 stroke straddles that edge, so what lands on the
            // screen is 40. `Surface` draws a stroke *inside* the box
            // it is given, so the box is the footprint and the gap is
            // the remainder of the pitch.
            ws_gap: 5.2,
            ws_lead: 7.2,
            ws_width: 40.0,
            ws_corners: None,
            pad_x: 12.0,
            // The right end of a button is the tab's alone: 8 gap, 22
            // tab, 8 inset, and no label ever sits on one.
            trail: 38.0,
            // The one era whose per-character estimate is not 0.58:
            // its labels are set with tracking and the design measured
            // them, so this is the average the six readouts imply.
            em: 0.52,
            // Its designer measured, so `VOL  62%` is 97 wide there
            // and not the 109 a flat count gives.
            space_em: 0.2,
            // ENTER / LOGIN is spaced 3 at 14px; the CTA plate here is
            // spaced 2 and sized for it.
            alert_track: 2.0,
            // store-trace 1.3, mailbox-trace 1.7; the era's metric
            // stroke of 1.6 sits between and is kept.
            stroke: 2.0,
            // A 46-wide idle icon cell, per the delta.
            icon_pad: 27.0,
            // RIFLES at x+22 of 184; store nav at x+22 of 200. Labels
            // are set against the leading edge, never centred.
            label_left: true,
            // The trace sets the strip at weight 600 and the
            // annotation under the wire bridge at 400. `fonts.rs`
            // publishes no semibold face, and asking the shaper for
            // one resolved it to Bold -- which at 14px on a 25px cell
            // read as a smear rather than as the tracked face the
            // material has. Regular was tried and came out visibly
            // thinner than the design's dark-on-gold tab labels
            // side by side; Medium is the nearest published weight
            // below 600 and is what the strip is set in.
            face: Face::Medium,
            tape_extra: 10.0,
            tape_ticks: false,

            // The strip is the top 31px of the screen and every trace
            // is violet there and black at the ends: store-trace's
            // measured `#haze` lobe, centre (825,-120), r 1030,
            // y-scale 0.515, stops #574568 / #3a3853 / #16121a /
            // #0e0a0d.
            ground: BarGround::Haze {
                cx: 825.0,
                cy: -120.0,
                r: 1030.0,
                squash: 0.515,
                stops: [
                    (0.0, HAZE_CORE),
                    (0.258, HAZE_CORE),
                    (0.572, HAZE_MID),
                    (0.873, HAZE_EDGE),
                    (1.0, HAZE_OUT),
                ],
            },
            chrome: BarChrome::Loose,
            ornament: BarOrnament::Wire,

            // The store nav button: r3 corners, a bottom-LEFT cut 10
            // wide by 7 tall, outlined in the store card's own
            // sample -- brighter than FRAME, which the source never
            // uses for a button.
            idle: Dress {
                corners: Corners::all(Cut::Round { radius: 3.0 })
                    .with_bottom_left(Cut::Chamfer { x: 10.0, y: 7.0 }),
                fill: Ink::None,
                stroke: Ink::Relief,
                ink: Ink::Fg,
                tab: true,
                step: None,
            },
            // The SMG button: the same silhouette and tab, filled with
            // wood veneer. Selection is a material in this era, which
            // is `Selection::Veneer` and needs no colour here.
            selected: Dress {
                corners: Corners::all(Cut::Round { radius: 3.0 })
                    .with_bottom_left(Cut::Chamfer { x: 10.0, y: 7.0 }),
                fill: Ink::Select,
                stroke: Ink::None,
                ink: Ink::OnSelect,
                tab: true,
                step: None,
            },
            // The ENTER / LOGIN bar: solid amber, square corners, only
            // a bottom-left chamfer, no tab. The README's rule -- amber
            // is the one strong CTA.
            alert: Dress {
                corners: Corners::square().with_bottom_left(Cut::Chamfer { x: 10.0, y: 7.0 }),
                fill: Ink::Alert,
                stroke: Ink::None,
                ink: Ink::OnSelect,
                tab: false,
                step: None,
            },
            // The mailbox selection bar in miniature: chamfer 22/55 ->
            // 10 on the top-RIGHT, a Q4 bottom-right, veneer with a
            // book-match seam at the plate's midpoint.
            tape: Dress {
                corners: Corners::square()
                    .with_top_right(Cut::Chamfer { x: 10.0, y: 10.0 })
                    .with_bottom_right(Cut::Round { radius: 4.0 }),
                fill: Ink::Select,
                stroke: Ink::None,
                ink: Ink::OnSelect,
                tab: false,
                step: None,
            },
            // The filled trapezoid every neokitsch button and rule
            // carries on its bottom edge: mailbox RIFLES measures base
            // 37 / top 29 / 7 tall on a 39px button, which is 22/16/4
            // here, and 14/8 on the narrow cells.
            tab: Some(Tab {
                base: 22.0,
                top: 16.0,
                height: 4.0,
                inset: 8.0,
                narrow_base: 14.0,
                narrow_top: 8.0,
                narrow_below: 50.0,
                fill: Ink::Fixed(VENEER_LIGHT),
            }),
            // The annotation hanging under the wire bridge, in the
            // champagne the store header sets its annotations in.
            window: WindowLabel {
                dress: None,
                ink: Ink::Banner,
                leading: false,
                pad_x: 0.0,
            },

            alert_suffix: None,
            bold_tiers: false,
            // login-trace's 10:10 PM at (1293,87): the only clock in
            // the run, and it is unboxed.
            clock_plain: Some(18),

            menu: BarMenu {
                // The dashboard cascade card: chamfer 22 on the
                // top-right and bottom-left, Q6 on the other two.
                panel: Dress {
                    corners: Corners::square()
                        .with_top_left(Cut::Round { radius: 6.0 })
                        .with_top_right(Cut::Chamfer { x: 22.0, y: 22.0 })
                        .with_bottom_right(Cut::Round { radius: 6.0 })
                        .with_bottom_left(Cut::Chamfer { x: 22.0, y: 22.0 }),
                    fill: Ink::Bg,
                    stroke: Ink::Relief,
                    ink: Ink::Fg,
                    tab: false,
                    step: None,
                },
                air: 6.0,
                side: 0.0,
                row_air: 2.8,
                row_side: 10.0,
                icon_col: 16.0,
                icon_gap: 8.0,
                level_gap: 0.0,
                level_pad: 36.0,
                row_divider: false,
                // A mailbox list rule with its filled tab standing on
                // it.
                rule: MenuRule::Tabbed,
                // The mailbox selection bar again, at row scale.
                row: Dress {
                    corners: Corners::square()
                        .with_top_right(Cut::Chamfer { x: 10.0, y: 10.0 })
                        .with_bottom_right(Cut::Round { radius: 4.0 }),
                    fill: Ink::Select,
                    stroke: Ink::None,
                    ink: Ink::OnSelect,
                    tab: false,
                    step: None,
                },
                // The T2 badge: an outlined r3 mini-card in the bright
                // gold with a solid tab on its inside bottom edge.
                // Outlined is *current*; veneer is *chosen*.
                open: Dress {
                    corners: Corners::all(Cut::Round { radius: 3.0 }),
                    fill: Ink::None,
                    stroke: Ink::Fg,
                    ink: Ink::Fg,
                    tab: true,
                    step: None,
                },
                open_inset: (6.0, 6.0),
                row_split: None,
                disabled: Ink::Dim,
                rule_ink: Ink::Banner,
                row_inset: (0.0, 0.0),
                spine: 0.0,
                foot: 0.0,
                marker: MenuMarker::Text,
                // The card's onion rings: concentric offsets of the
                // silhouette at 3.5 pitch, fading out.
                echo: PanelEcho::Rings {
                    count: 4,
                    pitch: 3.5,
                },
                echo_pad: 14.0,
            },
        },
        // The footer nameplate hangs past the card by the same 12 as
        // kitsch, but does not step: `rect x=340 w=188` against a card
        // at `x=352 w=176` in target-components.svg.
        banner: Banner {
            overhang: 12.0,
            notch: 0.0,
        },
        // A and C along the top strata rail, B under the cards.
        footnotes: Footnotes::TopRail,
        // The footer nameplate is the card's last edge; the target
        // prints no notice under it.
        compliance: Compliance::None,
        // The era has a step-notch shape but spends it on the mailbox
        // footer; its nav pills are plain `rx="4"` rects.
        ticket: Ticket::default(),
        glyphs: false,
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
        dashboard_selection: 0,
        // --- end dashboard ---
        metrics: Metrics {
            stroke: 2.0,
            gap: 18.0,
            pad: 18.0,
            ..Metrics::default()
        },
    }
}

// --- login ---
//
// The access screen, transcribed from `docs/neokitsch/login-trace.svg`
// at 1600x900: the ARASAKA logotype and a clock, two identical entry
// groups 420 apart, and the wire band across the foot.
//
// Two entry groups and no locked one: this era's login offers A and B
// and lets you into either, where kitsch and neomil show one live
// account beside two you may not have. The trace is explicit that the
// right group is "the same at +420" but for its letter.
//
// The trace also draws a soft vertically-smeared halo under every glyph
// and bar, and this table does not carry it. That is a property of the
// photograph rather than of the design -- the trace's own note calls it
// "a blurred, darkened copy of the content group" -- and the pipeline
// rule is that photographic residue is not the spec.

/// The gold the entry bars are filled with and the brown the label on
/// them is printed in. `cta`/`AMBER` is the published role and is a
/// hair lighter; the dark brown is not a role anywhere.
pub const ON_BAR: iced::Color = rgb(0x6b3f1e);
/// The line-work of the header's caption box and of a letter box.
pub const HAIRLINE: iced::Color = rgb(0xc8914d);
/// The mid gold the captions, the letters and the clock are set in.
pub const CAPTION: iced::Color = rgb(0xd9a877);
/// The micro-text beside a letter box, a stop under the captions.
pub const MICRO: iced::Color = rgb(0xa97c48);
/// The wire band's brightest strand. The band steps from 30% of this
/// at the top strand to full at the bottom, and the floor between the
/// strands glows the same way.
pub const WIRE: iced::Color = rgb(0xeab15c);
pub const WIRE_GLOW: iced::Color = rgb(0x371c11);

const NOTE_1: &str = "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO.";
const NOTE_2: &str = "SERVING CUSTOMERS SINCE 2006.";

pub const ACCESS: Access = Access {
    wash: Wash::VioletHaze,
    masthead: Masthead::Logotype {
        cell: Plate::outlined(Plot::new(90.0, 100.0, 185.0, 25.0), Ink::Fixed(HAIRLINE), 1.0),
        divider: 195.0,
        labels: &[
            // Scaled onto the measured ink extent x 92..272. The
            // trace used to say `textLength="185"`, which librsvg
            // ignores; both it and this now carry the transform.
            Legend::new("ARASAKA", 91.0, 87.0, 36.0, Ink::Fg)
                .bold()
                .tracked(3.0)
                .stretched(1.155),
            Legend::new(
                "ARASAKA CONSUMER TECHNOLOGY",
                118.0,
                97.0,
                8.0,
                Ink::Fixed(CAPTION),
            ),
            Legend::new("57ASD4AV15AA", 96.0, 115.0, 9.0, Ink::Fixed(CAPTION)),
            Legend::new("COMBAT COLONIZATION", 200.0, 110.0, 8.0, Ink::Fixed(CAPTION)),
            Legend::new("DEFENCE PROGRAM", 200.0, 120.0, 8.0, Ink::Fixed(CAPTION)),
            Legend::new("10:10 PM", 1293.0, 87.0, 28.0, Ink::Fg).medium(),
            Legend::new("NIGHT CITY", 1295.0, 110.0, 13.0, Ink::Fg).medium(),
            Legend::new("AREA", 1295.0, 129.0, 13.0, Ink::Fg).medium(),
        ],
    },
    slots: &[
        Slot {
            name: Some(
                Legend::new("PRASE_6054012", 428.5, 350.0, 19.5, Ink::Fg)
                    .medium()
                    .tracked(0.15),
            ),
            field: Some(Plate::filled(
                Plot::new(417.0, 361.0, 345.0, 42.0),
                Ink::Inset,
            )),
            // Bottom-left chamfer only, 16 wide by 13 tall; the table
            // carries the one figure, so this is 16 square.
            action: Some(
                Plate::filled(Plot::new(417.0, 414.0, 344.0, 27.0), Ink::Cta)
                    .bevelled(Bevel::bl(16.0)),
            ),
            action_label: Some(
                Legend::new("ENTER / LOGIN", 582.5, 435.0, 14.0, Ink::Fixed(ON_BAR))
                    .centred()
                    .light()
                    .tracked(4.1),
            ),
            badge: Some(Plate::outlined(
                Plot::new(416.0, 465.0, 26.0, 26.0),
                Ink::Fixed(HAIRLINE),
                1.4,
            )),
            badge_letter: Some(
                Legend::new("A", 429.0, 483.0, 15.0, Ink::Fixed(CAPTION)).centred(),
            ),
            notes: &[
                Legend::new(NOTE_1, 454.0, 471.0, 7.0, Ink::Fixed(MICRO)),
                Legend::new(NOTE_2, 454.0, 479.0, 7.0, Ink::Fixed(MICRO)),
            ],
            ..Slot::EMPTY
        },
        Slot {
            name: Some(
                Legend::new("PRASE_6054012", 848.5, 350.0, 19.5, Ink::Fg)
                    .medium()
                    .tracked(0.15),
            ),
            field: Some(Plate::filled(
                Plot::new(837.0, 361.0, 345.0, 42.0),
                Ink::Inset,
            )),
            action: Some(
                Plate::filled(Plot::new(837.0, 414.0, 344.0, 27.0), Ink::Cta)
                    .bevelled(Bevel::bl(16.0)),
            ),
            action_label: Some(
                Legend::new("ENTER / LOGIN", 1002.5, 435.0, 14.0, Ink::Fixed(ON_BAR))
                    .centred()
                    .light()
                    .tracked(4.1),
            ),
            badge: Some(Plate::outlined(
                Plot::new(836.0, 465.0, 26.0, 26.0),
                Ink::Fixed(HAIRLINE),
                1.4,
            )),
            badge_letter: Some(
                Legend::new("B", 849.0, 483.0, 15.0, Ink::Fixed(CAPTION)).centred(),
            ),
            notes: &[
                Legend::new(NOTE_1, 874.0, 471.0, 7.0, Ink::Fixed(MICRO)),
                Legend::new(NOTE_2, 874.0, 479.0, 7.0, Ink::Fixed(MICRO)),
            ],
            ..Slot::EMPTY
        },
    ],
    // 22 strands: the outer plateaus at y 727.1 with a 3.9 spacing, the
    // centre plateau at 782.1 with the spacing tightened to 3.03, and
    // both ends curling down into a vertical that runs to y 812.
    fixture: Fixture::WireBand {
        outer: 727.1,
        inner: 782.1,
        end: 812.0,
        strands: 22,
    },
    colophon: Colophon::Notice {
        labels: &[Legend::new(
            "ARASAKA CONSUMER TECHNOLOGY  ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE.",
            800.0,
            875.0,
            9.5,
            Ink::Fixed(HAIRLINE),
        )
        .centred()
        .tracked(0.06)],
    },
};
// --- end login ---
// --- mailbox ---
//
// `docs/neokitsch/mailbox-trace.svg`, read at its 1600x900 frame. Two
// things in that trace are the photograph and not the design, and both
// are left out:
//
//   * the halo. Every glyph and stroke in the source sits on a soft
//     vertically-smeared glow (the #39281b family, 3.7% of the canvas),
//     which the trace reproduces with an feGaussianBlur copy of its own
//     content. That is how the material photographs, not a drawn
//     element, and README.md's "no glow" rule stands.
//   * the wood veneer's photographic figure -- the swirl at the bar's
//     left edge, the book-match seam at x~273, the arcs bending into
//     the top-right chamfer. `Selection::Veneer` synthesises grain
//     rather than shipping a raster, which is the era table's standing
//     answer to a material fill.
//
// The wire band's eight strands are the trace's beziers stepped into
// short segments; at 1.1px they read the same.
use crate::style::{
    Frame, MailBadges, MailButtons, MailList, MailPanel, Mailbox, Note, Piece, RowDecor,
    Run, Seg, Trim, Veneer, Lobe, FromAt, BL, TR,
};

/// The selection bar's own two tones, measured off the photograph at
/// 3840 rather than derived from the era's `VENEER` family: the flat
/// base the grain sits on, and the grain-line core the polished trace
/// recoloured to #cf975c. They are a stop brighter than `VENEER` /
/// `GRAIN`, which dress a synthesised plank rather than this one.
/// The three ground washes the trace measures on this photo: the
/// violet haze, the brighter lobe over its top left, and the blue band
/// that only reaches the right two thirds. Their centres sit well above
/// the frame, and each is an *ellipse* -- the trace's gradients carry a
/// y-scaling transform, so a disc stack would be the wrong shape.
const fn rgba(hex: u32, a: f32) -> iced::Color {
    iced::Color { a, ..rgb(hex) }
}

static HAZE_STOPS: [(f32, iced::Color); 5] = [
    (0.0, rgb(0x574568)),
    (0.35, rgb(0x574568)),
    (0.66, rgb(0x3a3853)),
    (0.85, rgb(0x16121a)),
    (1.0, rgb(0x0e0a0d)),
];
static LOBE_STOPS: [(f32, iced::Color); 3] = [
    (0.0, rgba(0x7a5288, 0.85)),
    (0.45, rgba(0x7a5288, 0.55)),
    (1.0, rgba(0x7a5288, 0.0)),
];
static BLUE_STOPS: [(f32, iced::Color); 5] = [
    (0.0, rgba(0x223350, 0.0)),
    (0.60, rgba(0x223350, 0.0)),
    (0.68, rgba(0x223350, 0.85)),
    (0.76, rgba(0x1a2c46, 0.80)),
    (0.84, rgba(0x101d30, 0.0)),
];
static HAZE: [Lobe; 3] = [
    Lobe {
        cx: 770.0,
        cy: -120.0,
        r: 1000.0,
        aspect: 0.49,
        stops: &HAZE_STOPS,
        fade: (0.0, 0.0),
    },
    Lobe {
        cx: 430.0,
        cy: -40.0,
        r: 560.0,
        aspect: 0.30,
        stops: &LOBE_STOPS,
        fade: (0.0, 0.0),
    },
    Lobe {
        // the blue is right-weighted: the photo shows only a thin arm
        // of it on the left and none at all before x~200, which the
        // trace does with a luminance mask and this does with a fade
        cx: 850.0,
        cy: -120.0,
        r: 1000.0,
        aspect: 0.47,
        stops: &BLUE_STOPS,
        fade: (100.0, 640.0),
    },
];

pub const BAR: iced::Color = rgb(0xf8c678);
pub const BAR_GRAIN: iced::Color = rgb(0xcf975c);

const fn text(x: f32, y: f32, size: f32, ink: Ink, s: &'static str) -> Piece {
    Piece::Label(Note {
        at: Run::new(x, y, size, ink),
        text: s,
    })
}

const fn strong(x: f32, y: f32, size: f32, s: &'static str) -> Piece {
    Piece::Label(Note {
        at: Run::new(x, y, size, Ink::Fg).bold(),
        text: s,
    })
}

// The section-letter plates: a 26x26 rounded plate whose bottom-right
// corner is *cut*, with the fold line drawn inside it -- the era's
// mini-SIM motif. `Trim` carries one kind for all four corners, so a
// plate that rounds three and chamfers the fourth is line art rather
// than a box.
static PLATE_A: [(f32, f32); 13] = [
    (241.0, 98.0),
    (261.0, 98.0),
    (263.1, 98.6),
    (264.0, 101.0),
    (264.0, 115.0),
    (257.0, 122.0),
    (255.4, 123.6),
    (252.0, 124.0),
    (241.0, 124.0),
    (238.9, 123.4),
    (238.0, 121.0),
    (238.0, 101.0),
    (238.6, 98.9),
];
static FOLD_A: [(f32, f32); 5] = [
    (264.0, 115.0),
    (259.0, 115.0),
    (257.6, 115.6),
    (257.0, 117.0),
    (257.0, 122.0),
];
static PLATE_B: [(f32, f32); 13] = [
    (1014.0, 98.0),
    (1034.0, 98.0),
    (1036.1, 98.6),
    (1037.0, 101.0),
    (1037.0, 115.0),
    (1030.0, 122.0),
    (1028.4, 123.6),
    (1025.0, 124.0),
    (1014.0, 124.0),
    (1011.9, 123.4),
    (1011.0, 121.0),
    (1011.0, 101.0),
    (1011.6, 98.9),
];
static FOLD_B: [(f32, f32); 5] = [
    (1037.0, 115.0),
    (1032.0, 115.0),
    (1030.6, 115.6),
    (1030.0, 117.0),
    (1030.0, 122.0),
];
static PLATE_C: [(f32, f32); 13] = [
    (142.0, 777.0),
    (162.0, 777.0),
    (164.1, 777.6),
    (165.0, 780.0),
    (165.0, 794.0),
    (158.0, 801.0),
    (156.4, 802.6),
    (153.0, 803.0),
    (142.0, 803.0),
    (139.9, 802.4),
    (139.0, 800.0),
    (139.0, 780.0),
    (139.6, 777.9),
];
static FOLD_C: [(f32, f32); 5] = [
    (165.0, 794.0),
    (160.0, 794.0),
    (158.6, 794.6),
    (158.0, 796.0),
    (158.0, 801.0),
];
static PLATE_D: [(f32, f32); 13] = [
    (738.0, 777.0),
    (758.0, 777.0),
    (760.1, 777.6),
    (761.0, 780.0),
    (761.0, 794.0),
    (754.0, 801.0),
    (752.4, 802.6),
    (749.0, 803.0),
    (738.0, 803.0),
    (735.9, 802.4),
    (735.0, 800.0),
    (735.0, 780.0),
    (735.6, 777.9),
];
static FOLD_D: [(f32, f32); 5] = [
    (761.0, 794.0),
    (756.0, 794.0),
    (754.6, 794.6),
    (754.0, 796.0),
    (754.0, 801.0),
];

/// The in-fiction micro-print. Both header blocks are left-aligned --
/// the re-cut trace found the right one flush at x 843.3, where an
/// earlier pass had it right-anchored at 1000 -- and all four runs are
/// 6.2px weight 500.
const fn micro(x: f32, y: f32, s: &'static str) -> Piece {
    Piece::Label(Note {
        at: Run::new(x, y, 6.2, Ink::Fg).medium(),
        text: s,
    })
}

const fn letter(x: f32, y: f32, s: &'static str) -> Piece {
    Piece::Label(Note {
        at: Run::new(x, y, 19.5, Ink::Fg).bold().centered(),
        text: s,
    })
}

// The wire band, transcribed segment for segment from the trace rather
// than sampled: each strand leaves the left margin flat, eases up into
// the tight line at y~87 with a cubic, runs to x 1040, eases back down
// with another, and curls off the right edge through a quadratic. A
// polyline through those endpoints draws a hard corner where the
// material shows an S-curve.
static WIRE0: [Seg; 7] = [
    Seg::Line(52.0, 122.0),
    Seg::Cubic { c1x: 84.0, c1y: 122.0, c2x: 100.0, c2y: 86.40, x: 130.0, y: 86.40 },
    Seg::Line(1040.0, 86.40),
    Seg::Cubic { c1x: 1075.0, c1y: 86.40, c2x: 1090.0, c2y: 123.0, x: 1125.0, y: 123.0 },
    Seg::Line(1535.0, 123.0),
    Seg::Quad { cx: 1562.0, cy: 123.0, x: 1568.0, y: 145.0 },
    Seg::Line(1570.0, 153.0),
];
static WIRE1: [Seg; 7] = [
    Seg::Line(52.0, 124.9),
    Seg::Cubic { c1x: 84.0, c1y: 124.9, c2x: 100.0, c2y: 86.56, x: 130.0, y: 86.56 },
    Seg::Line(1040.0, 86.56),
    Seg::Cubic { c1x: 1075.0, c1y: 86.56, c2x: 1090.0, c2y: 124.8, x: 1125.0, y: 124.8 },
    Seg::Line(1535.0, 124.8),
    Seg::Quad { cx: 1562.0, cy: 124.8, x: 1568.0, y: 146.8 },
    Seg::Line(1570.0, 154.8),
];
static WIRE2: [Seg; 7] = [
    Seg::Line(52.0, 127.8),
    Seg::Cubic { c1x: 84.0, c1y: 127.8, c2x: 100.0, c2y: 86.72, x: 130.0, y: 86.72 },
    Seg::Line(1040.0, 86.72),
    Seg::Cubic { c1x: 1075.0, c1y: 86.72, c2x: 1090.0, c2y: 126.6, x: 1125.0, y: 126.6 },
    Seg::Line(1535.0, 126.6),
    Seg::Quad { cx: 1562.0, cy: 126.6, x: 1568.0, y: 148.6 },
    Seg::Line(1570.0, 156.6),
];
static WIRE3: [Seg; 7] = [
    Seg::Line(52.0, 130.7),
    Seg::Cubic { c1x: 84.0, c1y: 130.7, c2x: 100.0, c2y: 86.88, x: 130.0, y: 86.88 },
    Seg::Line(1040.0, 86.88),
    Seg::Cubic { c1x: 1075.0, c1y: 86.88, c2x: 1090.0, c2y: 128.4, x: 1125.0, y: 128.4 },
    Seg::Line(1535.0, 128.4),
    Seg::Quad { cx: 1562.0, cy: 128.4, x: 1568.0, y: 150.4 },
    Seg::Line(1570.0, 158.4),
];
static WIRE4: [Seg; 7] = [
    Seg::Line(52.0, 133.6),
    Seg::Cubic { c1x: 84.0, c1y: 133.6, c2x: 100.0, c2y: 87.04, x: 130.0, y: 87.04 },
    Seg::Line(1040.0, 87.04),
    Seg::Cubic { c1x: 1075.0, c1y: 87.04, c2x: 1090.0, c2y: 130.2, x: 1125.0, y: 130.2 },
    Seg::Line(1535.0, 130.2),
    Seg::Quad { cx: 1562.0, cy: 130.2, x: 1568.0, y: 152.2 },
    Seg::Line(1570.0, 160.2),
];
static WIRE5: [Seg; 7] = [
    Seg::Line(52.0, 136.5),
    Seg::Cubic { c1x: 84.0, c1y: 136.5, c2x: 100.0, c2y: 87.20, x: 130.0, y: 87.20 },
    Seg::Line(1040.0, 87.20),
    Seg::Cubic { c1x: 1075.0, c1y: 87.20, c2x: 1090.0, c2y: 132.0, x: 1125.0, y: 132.0 },
    Seg::Line(1535.0, 132.0),
    Seg::Quad { cx: 1562.0, cy: 132.0, x: 1568.0, y: 154.0 },
    Seg::Line(1570.0, 162.0),
];
static WIRE6: [Seg; 7] = [
    Seg::Line(52.0, 139.4),
    Seg::Cubic { c1x: 84.0, c1y: 139.4, c2x: 100.0, c2y: 87.36, x: 130.0, y: 87.36 },
    Seg::Line(1040.0, 87.36),
    Seg::Cubic { c1x: 1075.0, c1y: 87.36, c2x: 1090.0, c2y: 133.8, x: 1125.0, y: 133.8 },
    Seg::Line(1535.0, 133.8),
    Seg::Quad { cx: 1562.0, cy: 133.8, x: 1568.0, y: 155.8 },
    Seg::Line(1570.0, 163.8),
];
static WIRE7: [Seg; 7] = [
    Seg::Line(52.0, 142.3),
    Seg::Cubic { c1x: 84.0, c1y: 142.3, c2x: 100.0, c2y: 87.52, x: 130.0, y: 87.52 },
    Seg::Line(1040.0, 87.52),
    Seg::Cubic { c1x: 1075.0, c1y: 87.52, c2x: 1090.0, c2y: 135.6, x: 1125.0, y: 135.6 },
    Seg::Line(1535.0, 135.6),
    Seg::Quad { cx: 1562.0, cy: 135.6, x: 1568.0, y: 157.6 },
    Seg::Line(1570.0, 165.6),
];

/// The T2 badge: a folder seen face on, its raised tab on the right.
/// Front outline measured at (1287,55)-(1338,104), tab top y~37.
static FOLDER: [(f32, f32); 14] = [
    (1291.0, 55.0),
    (1302.0, 55.0),
    (1309.0, 54.0),
    (1312.0, 50.0),
    (1315.0, 46.0),
    (1318.0, 38.0),
    (1324.0, 37.0),
    (1333.0, 37.0),
    (1338.0, 42.0),
    (1338.0, 100.0),
    (1334.0, 104.0),
    (1291.0, 104.0),
    (1287.0, 100.0),
    (1287.0, 59.0),
];
/// The folder's six receding hairline rings: the shelf tops fan widely,
/// the tab tops just enough to resolve as separate lines, the right
/// edges spill out and everything converges at the bottom. Measured
/// spread, stepped off the crisp outline above.
static RING1: [(f32, f32); 14] = [
    (1289.4, 52.9),
    (1301.2, 52.9),
    (1308.7, 51.9),
    (1312.0, 47.9),
    (1315.2, 43.9),
    (1318.4, 35.9),
    (1324.9, 34.9),
    (1334.5, 34.9),
    (1339.9, 39.9),
    (1339.9, 100.3),
    (1335.6, 104.3),
    (1289.4, 104.3),
    (1285.1, 100.3),
    (1285.1, 56.9),
];

static RING2: [(f32, f32); 14] = [
    (1287.8, 50.8),
    (1300.4, 50.8),
    (1308.5, 49.8),
    (1311.9, 45.8),
    (1315.4, 41.8),
    (1318.8, 33.8),
    (1325.7, 32.8),
    (1336.1, 32.8),
    (1341.8, 37.8),
    (1341.8, 100.6),
    (1337.2, 104.6),
    (1287.8, 104.6),
    (1283.2, 100.6),
    (1283.2, 54.8),
];

static RING3: [(f32, f32); 14] = [
    (1286.2, 48.7),
    (1299.7, 48.7),
    (1308.2, 47.7),
    (1311.9, 43.7),
    (1315.6, 39.7),
    (1319.2, 31.7),
    (1326.6, 30.7),
    (1337.6, 30.7),
    (1343.7, 35.7),
    (1343.7, 100.9),
    (1338.8, 104.9),
    (1286.2, 104.9),
    (1281.3, 100.9),
    (1281.3, 52.7),
];

static RING4: [(f32, f32); 14] = [
    (1284.6, 46.6),
    (1298.9, 46.6),
    (1308.0, 45.6),
    (1311.9, 41.6),
    (1315.7, 37.6),
    (1319.6, 29.6),
    (1327.4, 28.6),
    (1339.1, 28.6),
    (1345.6, 33.6),
    (1345.6, 101.2),
    (1340.4, 105.2),
    (1284.6, 105.2),
    (1279.4, 101.2),
    (1279.4, 50.6),
];

static RING5: [(f32, f32); 14] = [
    (1283.0, 44.5),
    (1298.1, 44.5),
    (1307.7, 43.5),
    (1311.8, 39.5),
    (1315.9, 35.5),
    (1320.0, 27.5),
    (1328.3, 26.5),
    (1340.6, 26.5),
    (1347.5, 31.5),
    (1347.5, 101.5),
    (1342.0, 105.5),
    (1283.0, 105.5),
    (1277.5, 101.5),
    (1277.5, 48.5),
];

static RING6: [(f32, f32); 14] = [
    (1281.4, 42.4),
    (1297.3, 42.4),
    (1307.4, 41.4),
    (1311.8, 37.4),
    (1316.1, 33.4),
    (1320.5, 25.4),
    (1329.1, 24.4),
    (1342.2, 24.4),
    (1349.4, 29.4),
    (1349.4, 101.8),
    (1343.6, 105.8),
    (1281.4, 105.8),
    (1275.6, 101.8),
    (1275.6, 46.4),
];

/// The solid gold tab pointing up from the folder's inside bottom edge.
static FOLDER_TAB: [(f32, f32); 4] = [
    (1293.0, 104.5),
    (1297.0, 100.0),
    (1322.0, 100.0),
    (1326.0, 104.5),
];

static CHROME: [Piece; 49] = [
    Piece::Label(Note {
        at: Run::new(118.3, 42.2, 13.0, Ink::Fg).medium(),
        text: "CUSTOMER #NC488402",
    }),
    text(120.0, 70.0, 12.0, Ink::Fg, "LEVEL"),
    strong(126.0, 90.0, 21.0, "T1"),
    text(1131.0, 68.0, 12.0, Ink::Fg, "SECURITY"),
    text(1131.0, 83.0, 12.0, Ink::Fg, "LEVEL"),
    text(1229.0, 63.0, 12.0, Ink::Fg, "LEVEL"),
    text(1354.0, 63.0, 12.0, Ink::Fg, "LEVEL"),
    text(1417.0, 63.0, 12.0, Ink::Fg, "LEVEL"),
    strong(1236.0, 86.0, 20.0, "T1"),
    strong(1361.0, 86.0, 20.0, "T3"),
    strong(1424.0, 86.0, 20.0, "T4"),
    Piece::Poly {
        points: &RING1,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 0.7,
        close: true,
    },
    Piece::Poly {
        points: &RING2,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 0.7,
        close: true,
    },
    Piece::Poly {
        points: &RING3,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 0.7,
        close: true,
    },
    Piece::Poly {
        points: &RING4,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 0.7,
        close: true,
    },
    Piece::Poly {
        points: &RING5,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 0.7,
        close: true,
    },
    Piece::Poly {
        points: &RING6,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 0.7,
        close: true,
    },
    Piece::Poly {
        points: &FOLDER,
        fill: None,
        stroke: Some(Ink::Fg),
        width: 1.1,
        close: true,
    },
    Piece::Poly {
        points: &FOLDER_TAB,
        fill: Some(Ink::Alert),
        stroke: None,
        width: 0.0,
        close: true,
    },
    text(1295.0, 71.0, 12.0, Ink::Fg, "LEVEL"),
    strong(1296.0, 95.0, 21.0, "T2"),
    Piece::Curve {
        start: (30.0, 122.0),
        steps: &WIRE0,
        fill: None,
        stroke: Some(Ink::Tape),
        width: 1.1,
        close: false,
    },
    Piece::Curve {
        start: (30.0, 124.9),
        steps: &WIRE1,
        fill: None,
        stroke: Some(Ink::Tape),
        width: 1.1,
        close: false,
    },
    Piece::Curve {
        start: (30.0, 127.8),
        steps: &WIRE2,
        fill: None,
        stroke: Some(Ink::Tape),
        width: 1.1,
        close: false,
    },
    Piece::Curve {
        start: (30.0, 130.7),
        steps: &WIRE3,
        fill: None,
        stroke: Some(Ink::Tape),
        width: 1.1,
        close: false,
    },
    Piece::Curve {
        start: (30.0, 133.6),
        steps: &WIRE4,
        fill: None,
        stroke: Some(Ink::Tape),
        width: 1.1,
        close: false,
    },
    Piece::Curve {
        start: (30.0, 136.5),
        steps: &WIRE5,
        fill: None,
        stroke: Some(Ink::Tape),
        width: 1.1,
        close: false,
    },
    Piece::Curve {
        start: (30.0, 139.4),
        steps: &WIRE6,
        fill: None,
        stroke: Some(Ink::Tape),
        width: 1.1,
        close: false,
    },
    Piece::Curve {
        start: (30.0, 142.3),
        steps: &WIRE7,
        fill: None,
        stroke: Some(Ink::Tape),
        width: 1.1,
        close: false,
    },    Piece::Poly {
        points: &PLATE_A,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 1.4,
        close: true,
    },
    Piece::Poly {
        points: &FOLD_A,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 1.0,
        close: false,
    },
    Piece::Poly {
        points: &PLATE_B,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 1.4,
        close: true,
    },
    Piece::Poly {
        points: &FOLD_B,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 1.0,
        close: false,
    },
    letter(249.7, 115.5, "A"),
    letter(1022.5, 115.5, "B"),
    micro(278.3, 103.3, "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    micro(278.3, 110.0, "SERVING CUSTOMERS SINCE 2006."),
    micro(843.3, 103.3, "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    micro(843.3, 110.0, "SERVING CUSTOMERS SINCE 2006."),
    Piece::Poly {
        points: &PLATE_C,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 1.4,
        close: true,
    },
    Piece::Poly {
        points: &FOLD_C,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 1.0,
        close: false,
    },
    Piece::Poly {
        points: &PLATE_D,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 1.4,
        close: true,
    },
    Piece::Poly {
        points: &FOLD_D,
        fill: None,
        stroke: Some(Ink::Dim),
        width: 1.0,
        close: false,
    },
    letter(151.3, 796.0, "C"),
    letter(750.7, 796.0, "D"),
    micro(183.3, 782.8, "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    micro(183.3, 789.8, "SERVING CUSTOMERS SINCE 2006."),
    micro(771.7, 782.8, "MAPS ARE PROVIDED BY SEOCHO. SATELITE SERVICES"),
    micro(771.7, 789.8, "SINCE 2006."),
];

static BUTTONS: [&str; 4] = ["RIFLES", "RIFLES", "RIFLES", "RIFLES"];
static LEVELS: [&str; 0] = [];

pub fn mailbox() -> Mailbox {
    Mailbox {
        haze: &HAZE,
        chrome: &CHROME,
        list: MailList {
            frame: None,
            frame_ink: Ink::Fg,
            frame_width: 0.0,
            // seven rows on a 60.2 pitch; a hairline x 35..483 under
            // each one, carrying a small filled trapezoid tab
            row: Frame::new(35.0, 248.8, 477.0, 60.2),
            pitch: 60.2,
            count: 7,
            selected: 1,
            decor: RowDecor::Ruled,
            row_fill: None,
            row_stroke: None,
            row_width: 0.0,
            row_trim: Trim::NONE,
            spine: None,
            rule: Some(Frame::new(0.0, 60.2, 448.0, 1.5)),
            rule_ink: Ink::Fg,
            tab: Some(Frame::new(386.5, 55.2, 31.0, 5.0)),
            tab_ink: Ink::Alert,
            // the selection: a gold bar with a 22px chamfer on its
            // top-right corner and nowhere else
            sel: Frame::new(35.0, 315.0, 477.0, 55.0),
            sel_trim: Trim::chamfer(TR, 22.0),
            sel_icon: None,
            sel_icon_trim: Trim::NONE,
            sel_notch: Some(Frame::new(420.0, 363.0, 37.0, 10.0)),
            veneer: Some(Veneer {
                // measured on the bar: a #f8c678 base carrying 32
                // hairlines of #cf975c on a 1.7 pitch
                base: BAR,
                grain: BAR_GRAIN,
                pitch: 1.7,
                width: 0.7,
                // vertices every 26px through a 2.4 sway, first at 100
                turn: 26.0,
                sway: 2.4,
                phase: 100.0,
            }),
            // the one era that puts the envelope on the right
            glyph_x: 429.0,
            glyph_dy: 22.9,
            glyph_w: 16.0,
            text_x: 193.0,
            title_dy: 27.2,
            title_size: 18.0,
            title_bold: true,
            from_dy: 48.2,
            from_size: 11.5,
            from_at: FromAt::Beneath,
            from_prefix: "FROM: ",
            title_upper: false,
            from_upper: true,
            new_pill: None,
            new_rows: 0,
            icons: None,
        },
        panel: MailPanel {
            // no panel outline at all: the message is plain text on the
            // ground, which no other era's mailbox does
            frame: None,
            frame_fill: None,
            frame_stroke: None,
            frame_width: 0.0,
            frame_trim: Trim::NONE,
            head: None,
            head_ink: Ink::Select,
            head_trim: Trim::NONE,
            message: 1,
            title: Run::new(738.0, 277.0, 15.0, Ink::Fg).bold(),
            title_upper: true,
            from: Some(Run::new(740.0, 297.0, 11.5, Ink::Fg)),
            body: Run::new(738.0, 333.0, 17.0, Ink::Fg),
            line: 21.5,
            para: 43.0,
            wrap: 565.0,
        },
        buttons: MailButtons {
            // four outlined RIFLES buttons, 184x39 on a 192 pitch, each
            // with a bottom-left chamfer and a filled tab on its bottom
            // edge
            first: Frame::new(735.0, 684.0, 184.0, 39.0),
            dx: 192.0,
            dy: 0.0,
            count: 4,
            filled: None,
            joined: false,
            chevron: false,
            trim: Trim::chamfer(BL, 13.0),
            width: 1.25,
            stroke: Ink::Fg,
            label: Run::new(22.0, 26.0, 15.0, Ink::Fg),
            tab: Some(Frame::new(131.0, 32.0, 37.0, 7.0)),
            labels: &BUTTONS,
        },
        badges: MailBadges {
            // the era spends its clearance display on the header's
            // T1/T2/T3/T4 marks and the folder badge, so the mailbox
            // draws no badge grid at all
            first: Frame::ZERO,
            dx: 0.0,
            dy: 0.0,
            cols: 1,
            count: 0,
            selected: None,
            trim: Trim::NONE,
            width: 0.0,
            fill: None,
            stroke: Ink::Fg,
            label: Run::new(0.0, 0.0, 0.0, Ink::Fg),
            caption: None,
            caption_text: "",
            labels: &LEVELS,
        },
    }
}
// --- end mailbox ---
// --- store ---------------------------------------------------------------
//
// `docs/neokitsch/store-trace.svg`, transcribed. Coordinates are the
// trace's own in the 1600x900 frame, measured off
// `images/neokitsch-store.png`; each card is placed with `Prim::At` at
// its own `x0`, so a figure here reads against the SVG line it came
// from.
//
// The source's soft `#38261a` glow under every stroke is *not* drawn:
// the trace tags its halo `<use>` elements `class="photo"` and G2i
// hides them, so the design side carries no halo family at all. Drawn
// anyway it does real damage -- widened strokes turn every card frame
// into a thick dark slab.
//
// What else is not transcribed, and why. The trace spends about four fifths
// of its bytes on **wood-veneer grain** -- hundreds of 0.7px polylines
// clipped to the three gold fills -- and its own comment says why they
// are there: "drawn here as clipped 0.7 strokes in a mid gold over the
// fill *so the average stays at the sampled mean*". A flat fill at that
// mean says the same thing to anything measuring the render, and the
// grain is a photographic texture rather than structure. The soft
// `#38261a` halo under every stroke is the photograph's glow and is out
// for the same reason (docs/PIPELINE.md). Everything structural -- the
// wire band, the echo strands, the BASKET plate, the card frames, the
// socket rows, the tabs -- is here.

use crate::style::{
    fill_path, fill_rect, line_path, shut_path, txt, txt_bold, txt_end, txt_mid, Anchor,
    Group, Prim,
};

/// The run's ink families, sampled by k-means over the photo: a bright
/// gold for the plate, the selection and the tabs; a mid gold for the
/// strands and card outlines; and the dark the gold carries text in.
pub const BRIGHT: iced::Color = rgb(0xf5c689);
pub const PLATE: iced::Color = rgb(0xfbb86c);
pub const PLATE_BAND: iced::Color = rgb(0xeea666);
pub const SMG_FILL: iced::Color = rgb(0xf4c078);
pub const BODY_FILL: iced::Color = rgb(0xf6c27a);
pub const TAB: iced::Color = rgb(0xfed08d);
pub const NAV_TAB: iced::Color = rgb(0xf7cc8c);
pub const OUTLINE: iced::Color = rgb(0xdab176);
pub const STRAND: iced::Color = rgb(0xc5965a);
pub const STORE_WIRE: iced::Color = rgb(0xe0b67a);
pub const LABEL: iced::Color = rgb(0xe9bd7a);
pub const STORE_MICRO: iced::Color = rgb(0xd9a877);
pub const ON_GOLD: iced::Color = rgb(0x3a2010);
pub const PLATE_INK: iced::Color = rgb(0x5a3418);
pub const GUN_FILL: iced::Color = rgb(0xf2c06e);
pub const GUN_LINE: iced::Color = rgb(0x2d2518);
/// The mid gold the source's veneer grain is drawn in.
pub const GRAIN_LINE: iced::Color = rgb(0xcd9553);
/// The four echo strands shadowing a card outline, with the trace's
/// stroke opacities (`0.55 / 0.40 / 0.28 / 0.18`) composited onto the
/// haze they sit on -- iced's canvas stroke has no opacity of its own.
///
/// The fade matters and is not cosmetic: drawn at one flat strand
/// colour the four strands form a closed ring that hole-fills into a
/// solid slab the size of a card, and the extractor then reports one
/// on each of cards 1, 3 and 4 that the design does not have.
pub const ECHO1: iced::Color = rgb(0x795d4a);
pub const ECHO2: iced::Color = rgb(0x604a3a);
pub const ECHO3: iced::Color = rgb(0x4b3b2f);
pub const ECHO4: iced::Color = rgb(0x3a2f28);

/// The run's violet-over-black haze, and the cold blue annulus the
/// right half carries. Both are `gradientUnits="userSpaceOnUse"`
/// radials scaled 0.515 in y, so both are ellipses roughly 2:1 --
/// drawn as concentric bands at the gradient's own stop offsets.
pub const PAGE: iced::Color = rgb(0x0e0a0d);

/// The run's violet haze and the cold blue annulus the right half
/// carries, as the trace's own stop tables. Both are
/// `gradientUnits="userSpaceOnUse"` radials scaled 0.49 in y, so both
/// are ellipses roughly 2:1.
const STORE_HAZE: &[(f32, iced::Color)] = &[
    (0.00, rgb(0x574568)),
    (0.35, rgb(0x574568)),
    (0.66, rgb(0x3a3853)),
    (0.85, rgb(0x16121a)),
    (1.00, PAGE),
];
/// The cold blue the right half carries. An *annulus* -- transparent
/// inside and out, and only briefly opaque across `t 0.60..0.84` -- so
/// its stops are opacities and it needs the same alpha treatment the
/// left margin does. Drawn as a thick flat stroke instead it has two
/// hard edges where the source has none.
const BLUE: &[(f32, iced::Color)] = &[
    (0.60, iced::Color { r: 0.133, g: 0.200, b: 0.314, a: 0.00 }),
    (0.68, iced::Color { r: 0.133, g: 0.200, b: 0.314, a: 0.85 }),
    (0.76, iced::Color { r: 0.102, g: 0.173, b: 0.275, a: 0.80 }),
    (0.84, iced::Color { r: 0.063, g: 0.114, b: 0.188, a: 0.00 }),
    (1.00, iced::Color { r: 0.063, g: 0.114, b: 0.188, a: 0.00 }),
];

const BACKDROP: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(PAGE)),
    Prim::Lobe { x: 770.0, y: -120.0, rx: 1000.0, ry: 490.0, stops: STORE_HAZE },
    Prim::Lobe { x: 850.0, y: -120.0, rx: 1000.0, ry: 470.0, stops: BLUE },
];

/// The 9x9 socket glyph, read off card 1. Not a lattice of equal cells:
/// the finder blocks are 13.8 and the rest 3.1 or 6.7, so it is spelled
/// out rather than gridded.
macro_rules! qr {
    ($ink:expr) => {
        &[
            fill_rect(0.0, 0.0, 13.8, 13.8, $ink),
            fill_rect(10.6, 0.0, 3.1, 3.1, $ink),
            fill_rect(17.8, 0.0, 6.7, 6.7, $ink),
            fill_rect(28.4, 0.0, 3.1, 3.1, $ink),
            fill_rect(24.8, 7.1, 6.7, 6.7, $ink),
            fill_rect(0.0, 10.6, 3.1, 3.1, $ink),
            fill_rect(17.8, 10.6, 3.1, 3.1, $ink),
            fill_rect(0.0, 17.8, 13.8, 13.8, $ink),
            fill_rect(17.8, 17.8, 6.7, 6.7, $ink),
            fill_rect(28.4, 17.8, 3.1, 3.1, $ink),
            fill_rect(24.8, 24.8, 6.7, 6.7, $ink),
            fill_rect(10.6, 28.4, 3.1, 3.1, $ink),
            fill_rect(17.8, 28.4, 3.1, 3.1, $ink),
        ]
    };
}
const QR_LIGHT: &[Prim] = qr!(Ink::Fixed(LABEL));
const QR_DARK: &[Prim] = qr!(Ink::Fixed(ON_GOLD));

/// The boxed section letter: a 26x26 rounded plate with a folded
/// bottom-right corner -- the era's mini-SIM motif.
const LETTERBOX: &[Seg] = &[
    Seg::Line(23.0, 0.0),
    Seg::Quad { cx: 26.0, cy: 0.0, x: 26.0, y: 3.0 },
    Seg::Line(26.0, 17.0),
    Seg::Line(19.0, 24.0),
    Seg::Quad { cx: 17.0, cy: 26.0, x: 14.0, y: 26.0 },
    Seg::Line(3.0, 26.0),
    Seg::Quad { cx: 0.0, cy: 26.0, x: 0.0, y: 23.0 },
    Seg::Line(0.0, 3.0),
    Seg::Quad { cx: 0.0, cy: 0.0, x: 3.0, y: 0.0 },
];
const LETTERBOX_FOLD: &[Seg] = &[
    Seg::Line(21.0, 17.0),
    Seg::Quad { cx: 19.0, cy: 17.0, x: 19.0, y: 19.0 },
    Seg::Line(19.0, 24.0),
];
macro_rules! letterbox {
    ($x:expr, $y:expr, $letter:expr) => {
        Prim::At {
            x: $x,
            y: $y,
            prims: &[
                shut_path(3.0, 0.0, LETTERBOX, Ink::Fixed(STORE_WIRE), 1.4),
                line_path(26.0, 17.0, LETTERBOX_FOLD, Ink::Fixed(STORE_WIRE), 1.0),
                Prim::Text { x: 8.0, y: 20.0, size: 17.0, ink: Ink::Fixed(STORE_WIRE), face: Face::Bold, anchor: Anchor::Start, content: $letter },
            ],
        }
    };
}

/// One strand of the header wire band: out of the end curl, along the
/// low run, through an S-bend onto the bridge at y 124.2, and mirrored
/// out the far side. `y` is the strand's own low run.
macro_rules! strand {
    ($y:expr, $ink:expr) => {
        line_path(35.0, $y + 10.0, &[
            Seg::Quad { cx: 35.0, cy: $y, x: 45.0, y: $y },
            Seg::Line(290.0, $y),
            Seg::Cubic { c1x: 322.0, c1y: $y, c2x: 316.0, c2y: 124.2, x: 349.0, y: 124.2 },
            Seg::Line(1230.0, 124.2),
            Seg::Cubic { c1x: 1264.0, c1y: 124.2, c2x: 1258.0, c2y: $y, x: 1290.0, y: $y },
            Seg::Line(1552.0, $y),
            Seg::Quad { cx: 1562.0, cy: $y, x: 1562.0, y: $y + 10.0 },
        ], $ink, 1.2)
    };
}

/// A nav button: an r4 plate with a bottom-left cut, and a small solid
/// tab under its bottom-right. `y` is the plate's top.
macro_rules! nav_plate {
    ($y:expr) => {
        &[
            Seg::Line(289.5, $y),
            Seg::Quad { cx: 293.5, cy: $y, x: 293.5, y: $y + 4.0 },
            Seg::Line(293.5, $y + 34.6),
            Seg::Quad { cx: 293.5, cy: $y + 38.6, x: 289.5, y: $y + 38.6 },
            Seg::Line(108.0, $y + 38.6),
            Seg::Line(92.9, $y + 27.0),
            Seg::Line(92.9, $y + 4.0),
            Seg::Quad { cx: 92.9, cy: $y, x: 97.0, y: $y },
        ]
    };
}
macro_rules! nav_tab {
    ($y:expr) => {
        &[Seg::Line(275.5, $y), Seg::Line(272.5, $y + 3.4), Seg::Line(244.7, $y + 3.4)]
    };
}

const NAV1: &[Seg] = nav_plate!(357.9);
const NAV2: &[Seg] = nav_plate!(418.6);
const NAV3: &[Seg] = nav_plate!(479.3);
const NAV4: &[Seg] = nav_plate!(540.0);
const NAV5: &[Seg] = nav_plate!(600.7);
const TAB1: &[Seg] = nav_tab!(394.1);
const TAB2: &[Seg] = nav_tab!(454.8);
const TAB3: &[Seg] = nav_tab!(515.5);
const TAB4: &[Seg] = nav_tab!(576.2);
const TAB5: &[Seg] = nav_tab!(636.9);

/// The rifle illustration: a slotted rail over a magazine block, a
/// trigger group and a stock. Card-local to its own `translate`.
const GUN_MAG: &[Seg] = &[
    Seg::Line(70.0, 9.0),
    Seg::Line(70.0, 43.0),
    Seg::Quad { cx: 70.0, cy: 47.0, x: 66.0, y: 47.0 },
    Seg::Line(4.0, 47.0),
    Seg::Quad { cx: 0.0, cy: 47.0, x: 0.0, y: 43.0 },
];
const GUN_TRIGGER: &[Seg] = &[
    Seg::Line(117.0, 9.0),
    Seg::Line(117.0, 28.0),
    Seg::Line(102.0, 28.0),
    Seg::Line(95.0, 43.0),
    Seg::Line(85.0, 43.0),
    Seg::Line(82.0, 28.0),
    Seg::Line(70.0, 28.0),
];
const GUN_STOCK: &[Seg] = &[
    Seg::Line(162.0, 9.0),
    Seg::Line(187.0, 20.0),
    Seg::Line(187.0, 53.0),
    Seg::Line(175.0, 53.0),
    Seg::Line(167.0, 28.0),
    Seg::Line(162.0, 26.0),
    Seg::Line(117.0, 26.0),
];
const GUN_SLOTS: &[Seg] = &[
    Seg::Line(64.0, 16.0),
    Seg::Move(6.0, 23.0), Seg::Line(64.0, 23.0),
    Seg::Move(6.0, 30.0), Seg::Line(64.0, 30.0),
    Seg::Move(6.0, 37.0), Seg::Line(64.0, 37.0),
];
const GUN: &[Prim] = &[
    Prim::Rect { x: 0.0, y: 0.0, w: 162.0, h: 9.0, fill: Some(Ink::Fixed(GUN_FILL)), stroke: Some(Ink::Fixed(GUN_LINE)), width: 0.8 },
    Prim::Path { x: 0.0, y: 9.0, segs: GUN_MAG, close: true, fill: Some(Ink::Fixed(GUN_FILL)), stroke: Some(Ink::Fixed(GUN_LINE)), width: 0.8 },
    Prim::Path { x: 70.0, y: 9.0, segs: GUN_TRIGGER, close: true, fill: Some(Ink::Fixed(GUN_FILL)), stroke: Some(Ink::Fixed(GUN_LINE)), width: 0.8 },
    Prim::Path { x: 117.0, y: 9.0, segs: GUN_STOCK, close: true, fill: Some(Ink::Fixed(GUN_FILL)), stroke: Some(Ink::Fixed(GUN_LINE)), width: 0.8 },
    line_path(6.0, 16.0, GUN_SLOTS, Ink::Fixed(GUN_LINE), 0.6),
];

/// A standard card's frame, card-local: an r13 top-left, a 37-degree
/// step up to the raised top edge, an r18 top-right, and a bottom-left
/// cut back to the left edge.
const CARD_EDGE: &[Seg] = &[
    Seg::Line(0.0, 358.0),
    Seg::Quad { cx: 0.0, cy: 345.0, x: 13.0, y: 345.0 },
    Seg::Line(136.2, 345.0),
    Seg::Line(176.2, 314.6),
    Seg::Line(244.1, 314.6),
    Seg::Quad { cx: 262.1, cy: 314.6, x: 262.1, y: 332.6 },
    Seg::Line(262.1, 632.4),
    Seg::Quad { cx: 262.1, cy: 640.4, x: 254.1, y: 640.4 },
    Seg::Line(19.2, 640.4),
];
macro_rules! echo {
    ($d:expr, $ink:expr) => {
        line_path(141.3 + 5.1 * $d, 345.0, &[
            Seg::Line(178.0 + 1.85 * $d, 317.0 + 2.45 * ($d - 1.0)),
            Seg::Line(244.1, 317.0 + 2.45 * ($d - 1.0)),
            Seg::Quad { cx: 259.1 - 3.05 * ($d - 1.0), cy: 317.0 + 2.45 * ($d - 1.0), x: 259.1 - 3.05 * ($d - 1.0), y: 332.0 - 0.6 * ($d - 1.0) },
            Seg::Line(259.1 - 3.05 * ($d - 1.0), 632.4 - 1.3 * ($d - 1.0) * ($d - 1.0)),
            Seg::Quad { cx: 259.1 - 3.05 * ($d - 1.0), cy: 637.4 - 1.55 * ($d - 1.0), x: 254.1 - 1.55 * ($d - 1.0), y: 637.4 - 1.55 * ($d - 1.0) },
            Seg::Line(16.6 - 2.6 * ($d - 1.0), 637.4 - 1.55 * ($d - 1.0)),
        ], $ink, 1.0)
    };
}
const ECHOES: &[Prim] = &[
    echo!(1.0, Ink::Fixed(ECHO1)),
    echo!(2.0, Ink::Fixed(ECHO2)),
    echo!(3.0, Ink::Fixed(ECHO3)),
    echo!(4.0, Ink::Fixed(ECHO4)),
];
/// The solid tab under a card's bottom edge.
const CARD_TAB: &[Seg] = &[
    Seg::Line(163.5, 632.9),
    Seg::Line(160.0, 641.4),
    Seg::Line(102.0, 641.4),
];

const CARD: &[Prim] = &[
    shut_path(0.0, 618.0, CARD_EDGE, Ink::Fixed(OUTLINE), 1.3),
    Prim::At { x: 0.0, y: 0.0, prims: ECHOES },
    fill_path(99.0, 632.9, CARD_TAB, Ink::Fixed(TAB)),
    Prim::At { x: 37.2, y: 384.0, prims: GUN },
    txt(22.0, 485.0, 16.5, Ink::Fixed(BRIGHT), "DPS"),
    txt(86.0, 485.0, 16.5, Ink::Fixed(BRIGHT), "PNT"),
    txt(137.0, 485.0, 16.5, Ink::Fixed(BRIGHT), "ACC"),
    txt(192.0, 485.0, 16.5, Ink::Fixed(BRIGHT), "ROF"),
    Prim::Text { x: 15.0, y: 522.0, size: 27.0, ink: Ink::Fixed(BRIGHT), face: Face::Medium, anchor: Anchor::Start, content: "620" },
    txt(88.0, 519.0, 21.0, Ink::Fixed(BRIGHT), "30"),
    txt(147.0, 519.0, 21.0, Ink::Fixed(BRIGHT), "5"),
    txt(200.0, 519.0, 21.0, Ink::Fixed(BRIGHT), "5"),
    fill_rect(0.0, 532.35, 262.1, 1.1, Ink::Fixed(STRAND)),
    fill_rect(0.0, 580.25, 262.1, 1.1, Ink::Fixed(STRAND)),
    fill_rect(50.75, 532.9, 1.1, 47.9, Ink::Fixed(STRAND)),
    fill_rect(118.25, 532.9, 1.1, 47.9, Ink::Fixed(STRAND)),
    fill_rect(189.85, 532.9, 1.1, 47.9, Ink::Fixed(STRAND)),
    Prim::At { x: 9.8, y: 542.2, prims: QR_LIGHT },
    txt_mid(85.1, 553.4, 11.5, Ink::Fixed(LABEL), "EMPTY"),
    txt_mid(85.1, 568.9, 11.5, Ink::Fixed(LABEL), "SOCKET"),
    txt_mid(154.6, 553.4, 11.5, Ink::Fixed(LABEL), "EMPTY"),
    txt_mid(154.6, 568.9, 11.5, Ink::Fixed(LABEL), "SOCKET"),
    txt_mid(226.3, 553.4, 11.5, Ink::Fixed(LABEL), "EMPTY"),
    txt_mid(226.3, 568.9, 11.5, Ink::Fixed(LABEL), "SOCKET"),
    txt_bold(24.0, 613.0, 19.0, Ink::Fixed(BRIGHT), "MAGNUM 650"),
    txt(133.0, 613.0, 19.0, Ink::Fixed(BRIGHT), "HAND GUN"),
];

/// The selected card: the same template stretched -- its top part
/// shifted up 81.7 and its bottom down 70.9 -- with the span
/// y 411.2..652.9 solid gold across the full width.
const GROWN_EDGE: &[Seg] = &[
    Seg::Line(0.0, 276.3),
    Seg::Quad { cx: 0.0, cy: 263.3, x: 13.0, y: 263.3 },
    Seg::Line(136.2, 263.3),
    Seg::Line(176.2, 232.9),
    Seg::Line(244.1, 232.9),
    Seg::Quad { cx: 262.1, cy: 232.9, x: 262.1, y: 250.9 },
    Seg::Line(262.1, 703.3),
    Seg::Quad { cx: 262.1, cy: 711.3, x: 254.1, y: 711.3 },
    Seg::Line(19.2, 711.3),
];
macro_rules! grown_echo {
    ($d:expr, $ink:expr) => {
        line_path(141.3 + 5.1 * $d, 263.3, &[
            Seg::Line(178.0 + 1.85 * $d, 235.3 + 2.45 * ($d - 1.0)),
            Seg::Line(244.1, 235.3 + 2.45 * ($d - 1.0)),
            Seg::Quad { cx: 259.1 - 3.05 * ($d - 1.0), cy: 235.3 + 2.45 * ($d - 1.0), x: 259.1 - 3.05 * ($d - 1.0), y: 250.3 - 0.6 * ($d - 1.0) },
            Seg::Line(259.1 - 3.05 * ($d - 1.0), 703.3 - 1.3 * ($d - 1.0) * ($d - 1.0)),
            Seg::Quad { cx: 259.1 - 3.05 * ($d - 1.0), cy: 708.2 - 1.55 * ($d - 1.0), x: 254.1 - 1.55 * ($d - 1.0), y: 708.2 - 1.55 * ($d - 1.0) },
            Seg::Line(16.6 - 2.6 * ($d - 1.0), 708.2 - 1.55 * ($d - 1.0)),
        ], $ink, 1.0)
    };
}
const GROWN_TAB: &[Seg] = &[
    Seg::Line(163.5, 703.8),
    Seg::Line(160.0, 712.3),
    Seg::Line(102.0, 712.3),
];

const GROWN: &[Prim] = &[
    shut_path(0.0, 688.9, GROWN_EDGE, Ink::Fixed(OUTLINE), 1.3),
    grown_echo!(1.0, Ink::Fixed(ECHO1)),
    grown_echo!(2.0, Ink::Fixed(ECHO2)),
    grown_echo!(3.0, Ink::Fixed(ECHO3)),
    grown_echo!(4.0, Ink::Fixed(ECHO4)),
    fill_path(99.0, 703.8, GROWN_TAB, Ink::Fixed(TAB)),
    // the gold body, and the veneer grain the source fills it with
    fill_rect(0.0, 411.2, 262.1, 241.7, Ink::Fixed(BODY_FILL)),
    Prim::Grain { x: 0.0, y: 411.2, w: 262.1, h: 241.7, pitch: 2.4, width: 0.7, ink: Ink::Fixed(GRAIN_LINE) },
    Prim::At { x: 37.2, y: 302.3, prims: GUN },
    txt(22.0, 403.3, 16.5, Ink::Fixed(BRIGHT), "DPS"),
    txt(86.0, 403.3, 16.5, Ink::Fixed(BRIGHT), "PNT"),
    txt(137.0, 403.3, 16.5, Ink::Fixed(BRIGHT), "ACC"),
    txt(192.0, 403.3, 16.5, Ink::Fixed(BRIGHT), "ROF"),
    Prim::Text { x: 15.0, y: 440.3, size: 27.0, ink: Ink::OnSelect, face: Face::Medium, anchor: Anchor::Start, content: "620" },
    txt(88.0, 437.3, 21.0, Ink::OnSelect, "30"),
    txt(147.0, 437.3, 21.0, Ink::OnSelect, "5"),
    txt(200.0, 437.3, 21.0, Ink::OnSelect, "5"),
    txt(5.9, 468.0, 19.0, Ink::OnSelect, "20"),
    txt(32.9, 468.0, 19.0, Ink::OnSelect, "Recoil"),
    txt(5.9, 488.0, 19.0, Ink::OnSelect, "22"),
    txt(32.9, 488.0, 19.0, Ink::OnSelect, "Sperad"),
    txt(5.9, 509.0, 19.0, Ink::OnSelect, "12"),
    txt(32.9, 509.0, 19.0, Ink::OnSelect, "Range"),
    txt(5.9, 539.0, 19.0, Ink::OnSelect, "Bonus"),
    txt(5.9, 560.0, 19.0, Ink::OnSelect, "+9 Reflexes"),
    txt(5.9, 580.0, 19.0, Ink::OnSelect, "+2 Modules Slots"),
    fill_rect(50.75, 603.8, 1.1, 47.9, Ink::OnSelect),
    fill_rect(118.25, 603.8, 1.1, 47.9, Ink::OnSelect),
    fill_rect(189.85, 603.8, 1.1, 47.9, Ink::OnSelect),
    Prim::At { x: 9.8, y: 613.1, prims: QR_DARK },
    txt_mid(85.1, 624.3, 11.5, Ink::OnSelect, "EMPTY"),
    txt_mid(85.1, 639.8, 11.5, Ink::OnSelect, "SOCKET"),
    txt_mid(154.6, 624.3, 11.5, Ink::OnSelect, "EMPTY"),
    txt_mid(154.6, 639.8, 11.5, Ink::OnSelect, "SOCKET"),
    txt_mid(226.3, 624.3, 11.5, Ink::OnSelect, "EMPTY"),
    txt_mid(226.3, 639.8, 11.5, Ink::OnSelect, "SOCKET"),
    txt_bold(24.0, 683.9, 19.0, Ink::Fixed(BRIGHT), "MAGNUM 650"),
    txt(133.0, 683.9, 19.0, Ink::Fixed(BRIGHT), "HAND GUN"),
];

/// The BASKET plate: a gold slab with its bottom-left corner cut, split
/// by a hairline into a title half and a strand-textured band.
const PLATE_EDGE: &[Seg] = &[
    Seg::Line(1496.3, 19.6),
    Seg::Line(1496.3, 105.0),
    Seg::Line(1307.0, 105.0),
    Seg::Line(1291.7, 90.0),
];
const PLATE_LOWER: &[Seg] = &[
    Seg::Line(1496.3, 60.0),
    Seg::Line(1496.3, 105.0),
    Seg::Line(1307.0, 105.0),
    Seg::Line(1291.7, 90.0),
];
const BASKET_QR: &[Prim] = &[
    fill_rect(1463.0, 28.0, 7.0, 7.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1472.0, 28.0, 4.0, 4.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1478.0, 28.0, 6.0, 6.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1486.0, 28.0, 6.0, 6.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1469.0, 35.0, 4.0, 4.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1483.0, 33.0, 4.0, 4.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1463.0, 38.0, 4.0, 4.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1472.0, 38.0, 6.0, 6.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1481.0, 39.0, 4.0, 4.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1488.0, 38.0, 4.0, 4.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1463.0, 45.0, 6.0, 6.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1471.0, 46.0, 4.0, 4.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1478.0, 45.0, 6.0, 6.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1486.0, 46.0, 6.0, 6.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1469.0, 53.0, 4.0, 4.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1476.0, 53.0, 7.0, 4.0, Ink::Fixed(PLATE_INK)),
    fill_rect(1486.0, 53.0, 4.0, 4.0, Ink::Fixed(PLATE_INK)),
];

/// The logotype's outlined T, x 237..277 with its stem centred.
const TEE: &[Seg] = &[
    Seg::Line(277.0, 66.0),
    Seg::Line(277.0, 79.0),
    Seg::Line(264.0, 79.0),
    Seg::Line(264.0, 117.0),
    Seg::Line(250.0, 117.0),
    Seg::Line(250.0, 79.0),
    Seg::Line(237.0, 79.0),
];


// The nav's five buttons and the shelf's four positions, as plates. The
// selected button is solid gold -- veneer in the source, so it carries
// the grain -- with a dark label; the rest are outlines.
macro_rules! nav {
    ($plate:expr, $tab:expr, $top:expr, $tabtop:expr, $base:expr, $label:expr) => {
        (
            &[
                fill_path(97.0, $top, $plate, Ink::Fixed(SMG_FILL)),
                Prim::Grain { x: 97.0, y: $top, w: 196.5, h: 34.0, pitch: 2.1, width: 0.7, ink: Ink::Fixed(GRAIN_LINE) },
                fill_path(241.7, $tabtop, $tab, Ink::Fixed(NAV_TAB)),
                txt(115.0, $base, 17.0, Ink::OnSelect, $label),
            ],
            &[
                shut_path(97.0, $top, $plate, Ink::Fixed(OUTLINE), 1.3),
                fill_path(241.7, $tabtop, $tab, Ink::Fixed(NAV_TAB)),
                txt(115.0, $base, 17.0, Ink::Fixed(BRIGHT), $label),
            ],
        )
    };
}
const NAV_ON_0: &[Prim] = nav!(NAV1, TAB1, 357.9, 394.1, 384.4, "RIFLES").0;
const NAV_OFF_0: &[Prim] = nav!(NAV1, TAB1, 357.9, 394.1, 384.4, "RIFLES").1;
const NAV_ON_1: &[Prim] = nav!(NAV2, TAB2, 418.6, 454.8, 445.1, "SMG").0;
const NAV_OFF_1: &[Prim] = nav!(NAV2, TAB2, 418.6, 454.8, 445.1, "SMG").1;
const NAV_ON_2: &[Prim] = nav!(NAV3, TAB3, 479.3, 515.5, 505.8, "SNIPER").0;
const NAV_OFF_2: &[Prim] = nav!(NAV3, TAB3, 479.3, 515.5, 505.8, "SNIPER").1;
const NAV_ON_3: &[Prim] = nav!(NAV4, TAB4, 540.0, 576.2, 566.5, "SHOTGUN").0;
const NAV_OFF_3: &[Prim] = nav!(NAV4, TAB4, 540.0, 576.2, 566.5, "SHOTGUN").1;
const NAV_ON_4: &[Prim] = nav!(NAV5, TAB5, 600.7, 636.9, 627.2, "PISTOL").0;
const NAV_OFF_4: &[Prim] = nav!(NAV5, TAB5, 600.7, 636.9, 627.2, "PISTOL").1;

macro_rules! shelf {
    ($i:expr) => {
        &[Prim::Plate {
            group: Group::Card,
            index: $i,
            x: 0.0,
            y: 314.6,
            w: 262.1,
            h: 325.8,
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
    Prim::At { x: 0.0, y: 0.0, prims: CONTENT },
];

const CONTENT: &[Prim] = &[
    // logotype: a very heavy face, "4S" solid and the "T" outline only
    Prim::Wide { x: 109.0, y: 117.0, size: 70.0, stretch: 1.73, ink: Ink::Fixed(BRIGHT), face: Face::Bold, content: "4S" },
    shut_path(237.0, 66.0, TEE, Ink::Fixed(BRIGHT), 1.4),
    Prim::Spaced { x: 113.0, y: 138.0, size: 16.5, ink: Ink::Fixed(LABEL), face: Face::Bold, pitch: 39.0, content: "STORE" },
    // BASKET plate
    fill_path(1291.7, 19.6, PLATE_EDGE, Ink::Fixed(PLATE)),
    fill_path(1291.7, 60.0, PLATE_LOWER, Ink::Fixed(PLATE_BAND)),
    Prim::Grain { x: 1291.7, y: 60.0, w: 204.6, h: 30.0, pitch: 2.1, width: 0.7, ink: Ink::Fixed(GRAIN_LINE) },
    fill_rect(1291.7, 59.6, 204.6, 0.8, Ink::Fixed(PLATE_INK)),
    fill_rect(1455.1, 19.6, 0.8, 40.4, Ink::Fixed(PLATE_INK)),
    txt_mid(1369.0, 50.0, 15.0, Ink::Fixed(PLATE_INK), "BASKET"),
    Prim::At { x: 0.0, y: 0.0, prims: BASKET_QR },
    txt(1307.0, 73.0, 8.0, Ink::Fixed(PLATE_INK), "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE"),
    txt(1307.0, 82.0, 8.0, Ink::Fixed(PLATE_INK), "ALLOWED TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE."),
    // header wire band: eight strands rising onto one bridge line
    strand!(160.4, Ink::Fixed(ECHO2)),
    strand!(163.6, Ink::Fixed(ECHO2)),
    strand!(166.8, Ink::Fixed(ECHO1)),
    strand!(170.0, Ink::Fixed(ECHO1)),
    strand!(173.2, Ink::Fixed(STRAND)),
    strand!(176.4, Ink::Fixed(STRAND)),
    strand!(179.6, Ink::Fixed(STORE_WIRE)),
    strand!(182.8, Ink::Fixed(STORE_WIRE)),
    letterbox!(360.0, 143.0, "A"),
    letterbox!(1178.0, 143.0, "C"),
    txt(401.0, 148.0, 6.5, Ink::Fixed(STORE_MICRO), "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    txt(401.0, 155.0, 6.5, Ink::Fixed(STORE_MICRO), "SERVING CUSTOMERS SINCE 2006."),
    txt(1012.0, 148.0, 6.5, Ink::Fixed(STORE_MICRO), "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    txt(1012.0, 155.0, 6.5, Ink::Fixed(STORE_MICRO), "SERVING CUSTOMERS SINCE 2006."),
    // nav column
    txt(96.0, 252.0, 11.5, Ink::Fixed(LABEL), "CUSTOMER"),
    txt_end(289.0, 252.0, 11.5, Ink::Fixed(LABEL), "#NC488402"),
    txt(96.0, 283.0, 11.5, Ink::Fixed(LABEL), "LOYALTY DISCOUNT"),
    txt_end(288.0, 283.0, 11.5, Ink::Fixed(LABEL), "10%"),
    txt(96.0, 299.0, 11.5, Ink::Fixed(LABEL), "LAST UPDATE"),
    txt_end(288.0, 299.0, 11.5, Ink::Fixed(LABEL), "10/05/2077"),
    Prim::Plate { group: Group::Category, index: 0, x: 92.9, y: 357.9, w: 200.6, h: 38.6, on: NAV_ON_0, off: NAV_OFF_0 },
    Prim::Plate { group: Group::Category, index: 1, x: 92.9, y: 418.6, w: 200.6, h: 38.6, on: NAV_ON_1, off: NAV_OFF_1 },
    Prim::Plate { group: Group::Category, index: 2, x: 92.9, y: 479.3, w: 200.6, h: 38.6, on: NAV_ON_2, off: NAV_OFF_2 },
    Prim::Plate { group: Group::Category, index: 3, x: 92.9, y: 540.0, w: 200.6, h: 38.6, on: NAV_ON_3, off: NAV_OFF_3 },
    Prim::Plate { group: Group::Category, index: 4, x: 92.9, y: 600.7, w: 200.6, h: 38.6, on: NAV_ON_4, off: NAV_OFF_4 },
    // the shelf
    Prim::At { x: 360.8, y: 0.0, prims: SHELF_0 },
    Prim::At { x: 667.1, y: 0.0, prims: SHELF_1 },
    Prim::At { x: 978.8, y: 0.0, prims: SHELF_2 },
    Prim::At { x: 1288.8, y: 0.0, prims: SHELF_3 },
    // foot
    letterbox!(675.0, 775.0, "B"),
    txt(715.0, 780.5, 6.5, Ink::Fixed(STORE_MICRO), "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    txt(715.0, 787.0, 6.5, Ink::Fixed(STORE_MICRO), "SERVING CUSTOMERS SINCE 2006."),
];
// --- end store -----------------------------------------------------------
// --- dashboard -----------------------------------------------------------
//
// `docs/neokitsch/dashboard-trace.svg` (revised 2026-09-03), transcribed
// the way the store block above is: coordinates are the trace's own in
// the 1600x900 frame, elements in the trace's paint order, every `<use>`
// expanded through `Prim::At` at the trace's `x`/`y` or `translate`, so
// a figure here reads against the SVG line it came from. Line numbers
// below are the trace's.
//
// What is not transcribed, and why:
//
//   * the halo (:252, `<use href="#content" filter="url(#halo)"
//     class="photo">`): the photograph's glow, hidden by G2i and never
//     drawn by any screen here (docs/PIPELINE.md).
//   * the blue annulus's left fade (`mask="url(#bluemask)"`, :250 and :128, a
//     luminance ramp over x 0..640): the `Prim` set has no mask, so the
//     annulus is drawn whole and shows a thin arm on the left the
//     source has only faintly.
//   * the haze's 1.3-degree rotation and the blue's 2 degrees
//     (`gradientTransform`, :133 and :110): `Prim::Lobe` is axis-aligned.
//   * `letter-spacing` on every text (1.5 on the header, 2 on LEVEL,
//     0.4 on the annotations): `Prim::Text` has no tracking. The store
//     block drops the same attribute; only `S T O R E` earned `Spaced`.
//   * stroke opacity. The onion rings are one hex (`#bd8951` on the
//     cards and panel, `#a97c48` on the T2 badge) at a per-ring
//     `stroke-opacity`, and iced's canvas stroke has none, so each ring
//     gets that hex composited onto its ground -- `PAGE` for the cards
//     and panel, `HAZE_MID` for the badge, which sits in the violet.
//   * the r4 foot fillet on the cards and their rings is an SVG arc
//     (`A 4 4 0 0 1`, :154 and :217-222); `Seg` has no arc, so each is
//     one cubic through the same two endpoints (k = 4/3 tan(135/4 deg)
//     = 0.891, within 0.02 px of the arc). The control points are the
//     only figures in this block that are derived rather than copied.

/// The run's dashboard ink families, the trace's hex values. None of
/// them is an existing era const (`GOLD_TEXT #e7c686` and `AMBER
/// #fcc474` are each a step off), so they are named here rather than
/// approximated; `MICRO #a97c48`, `CAPTION #d9a877`, `PAGE #0e0a0d` and
/// the `HAZE_*` stops are reused where the trace samples the same hex.
/// Mid gold: header text, onion rings, captions, the tape, letterbox strokes.
pub const HUB_MID: iced::Color = rgb(0xbd8951);
/// The front outline of every card and of the panel (:389, :451).
pub const HUB_EDGE: iced::Color = rgb(0xe8ab66);
/// The solid gold: EMAIL's card, the panel body, the labels, T2's tab.
pub const HUB_FILL: iced::Color = rgb(0xf2b463);
/// The tab plates on the cards' left edges (:400-406).
pub const HUB_PLATE: iced::Color = rgb(0xfcbe6d);
/// The dark paragraph bars on the panel body (:460).
pub const HUB_DARK: iced::Color = rgb(0x3b2416);
/// The T2 badge's front outline and its "T2" (:287, :290).
pub const BADGE_LIT: iced::Color = rgb(0xe8c186);
/// The interior of the A/B letterboxes where they mask the wire band (:312).
pub const BOX_FILL: iced::Color = rgb(0x4c3f5f);

/// `HUB_MID` at the trace's ring opacities over `PAGE`. The cards' six
/// rings run 0.85 0.73 0.61 0.49 0.37 0.25 outermost to innermost
/// (:344-349); the panel's four run 0.70 0.70 0.55 0.25 (:446-449).
pub const RING_85: iced::Color = rgb(0xa37647);
pub const RING_73: iced::Color = rgb(0x8e673f);
pub const RING_70: iced::Color = rgb(0x89633d);
pub const RING_61: iced::Color = rgb(0x795736);
pub const RING_55: iced::Color = rgb(0x6e5032);
pub const RING_49: iced::Color = rgb(0x64482e);
pub const RING_37: iced::Color = rgb(0x4f3926);
pub const RING_25: iced::Color = rgb(0x3a2a1e);
/// `MICRO` at the T2 badge's seven ring opacities 0.55..0.85 (:279-285)
/// over `HAZE_MID`, the haze stop nearest the badge's ground.
pub const BADGE_55: iced::Color = rgb(0x775d4d);
pub const BADGE_60: iced::Color = rgb(0x7d614c);
pub const BADGE_65: iced::Color = rgb(0x82644c);
pub const BADGE_70: iced::Color = rgb(0x88684b);
pub const BADGE_75: iced::Color = rgb(0x8d6b4b);
pub const BADGE_80: iced::Color = rgb(0x936e4a);
pub const BADGE_85: iced::Color = rgb(0x98724a);

/// The haze (`#haze`, :131-139): the same four colours the bar and store
/// use, at this trace's own stop offsets, centred (825,-120), r 1030,
/// y-scaled 0.515. The blue annulus (`#hazeblue`, :108-116) is the
/// store's `BLUE` table stop for stop, at (900,-120) and the same radii.
const HUB_HAZE: &[(f32, iced::Color)] = &[
    (0.0, HAZE_CORE),
    (0.258, HAZE_CORE),
    (0.572, HAZE_MID),
    (0.873, HAZE_EDGE),
    (1.0, HAZE_OUT),
];
const HUB_GROUND: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(PAGE)),
    Prim::Lobe { x: 825.0, y: -120.0, rx: 1030.0, ry: 530.45, stops: HUB_HAZE },
    Prim::Lobe { x: 900.0, y: -120.0, rx: 1030.0, ry: 530.45, stops: BLUE },
];

/// One cascade card (`#ncard`, :154): r6.5 top-left, the 45-degree
/// chamfer from (48,0) to the right edge at y 42.5, the right edge to
/// 322.3, the r4 fillet, and the foot diagonal back to the left edge at
/// y 241.5. Opens at (0,6.5).
const NCARD: &[Seg] = &[
    Seg::Quad { cx: 0.0, cy: 0.0, x: 6.5, y: 0.0 },
    Seg::Line(48.0, 0.0),
    Seg::Line(90.5, 42.5),
    Seg::Line(90.5, 322.3),
    Seg::Cubic { c1x: 90.54, c1y: 325.87, c2x: 86.25, c2y: 327.7, x: 83.7, y: 325.2 },
    Seg::Line(0.0, 241.5),
];
/// The selected card (`#ncardsel`, :161): the same silhouette with the
/// plate well cut 5 deep into the left edge over y 54.2..94.2.
const NCARDSEL: &[Seg] = &[
    Seg::Quad { cx: 0.0, cy: 0.0, x: 6.5, y: 0.0 },
    Seg::Line(48.0, 0.0),
    Seg::Line(90.5, 42.5),
    Seg::Line(90.5, 322.3),
    Seg::Cubic { c1x: 90.54, c1y: 325.87, c2x: 86.25, c2y: 327.7, x: 83.7, y: 325.2 },
    Seg::Line(0.0, 241.5),
    Seg::Line(0.0, 94.2),
    Seg::Line(5.0, 90.8),
    Seg::Line(5.0, 57.1),
    Seg::Line(0.0, 54.2),
];
/// The six echo outlines nested inside a card (`#nring1..6`, :217-222),
/// open: the top edge 3.7 lower and the right edge 2.4 further in per
/// ring, the chamfer keeping its start at x 48. Each opens on the left
/// edge at (0, 6.5 + 3.7 d).
const NRING1: &[Seg] = &[
    Seg::Quad { cx: 0.0, cy: 3.7, x: 6.5, y: 3.7 },
    Seg::Line(48.0, 3.7),
    Seg::Line(88.1, 43.8),
    Seg::Line(88.1, 319.9),
    Seg::Cubic { c1x: 88.14, c1y: 323.47, c2x: 83.85, c2y: 325.3, x: 81.3, y: 322.8 },
];
const NRING2: &[Seg] = &[
    Seg::Quad { cx: 0.0, cy: 7.4, x: 6.5, y: 7.4 },
    Seg::Line(48.0, 7.4),
    Seg::Line(85.7, 45.1),
    Seg::Line(85.7, 317.5),
    Seg::Cubic { c1x: 85.74, c1y: 321.07, c2x: 81.45, c2y: 322.9, x: 78.9, y: 320.4 },
];
const NRING3: &[Seg] = &[
    Seg::Quad { cx: 0.0, cy: 11.1, x: 6.5, y: 11.1 },
    Seg::Line(48.0, 11.1),
    Seg::Line(83.3, 46.4),
    Seg::Line(83.3, 315.1),
    Seg::Cubic { c1x: 83.34, c1y: 318.67, c2x: 79.05, c2y: 320.5, x: 76.5, y: 318.0 },
];
const NRING4: &[Seg] = &[
    Seg::Quad { cx: 0.0, cy: 14.8, x: 6.5, y: 14.8 },
    Seg::Line(48.0, 14.8),
    Seg::Line(80.9, 47.7),
    Seg::Line(80.9, 312.7),
    Seg::Cubic { c1x: 80.94, c1y: 316.27, c2x: 76.65, c2y: 318.1, x: 74.1, y: 315.6 },
];
const NRING5: &[Seg] = &[
    Seg::Quad { cx: 0.0, cy: 18.5, x: 6.5, y: 18.5 },
    Seg::Line(48.0, 18.5),
    Seg::Line(78.5, 49.0),
    Seg::Line(78.5, 310.3),
    Seg::Cubic { c1x: 78.54, c1y: 313.87, c2x: 74.25, c2y: 315.7, x: 71.7, y: 313.2 },
];
const NRING6: &[Seg] = &[
    Seg::Quad { cx: 0.0, cy: 22.2, x: 6.5, y: 22.2 },
    Seg::Line(48.0, 22.2),
    Seg::Line(76.1, 50.3),
    Seg::Line(76.1, 307.9),
    Seg::Cubic { c1x: 76.14, c1y: 311.47, c2x: 71.85, c2y: 313.3, x: 69.3, y: 310.8 },
];

/// A card's idle dress, card-local: the six rings innermost first
/// (:343-349, the trace's order), the front outline (:389-395) and the
/// 6x38.3 r1.5 plate on the left edge at local y 54.6 (:401: MATRIX's
/// is at 346.4 = 347 - 0.6, 338.6 = 284 + 54.6).
const CARD_IDLE: &[Prim] = &[
    line_path(0.0, 28.7, NRING6, Ink::Fixed(RING_25), 1.0),
    line_path(0.0, 25.0, NRING5, Ink::Fixed(RING_37), 1.0),
    line_path(0.0, 21.3, NRING4, Ink::Fixed(RING_49), 1.0),
    line_path(0.0, 17.6, NRING3, Ink::Fixed(RING_61), 1.0),
    line_path(0.0, 13.9, NRING2, Ink::Fixed(RING_73), 1.0),
    line_path(0.0, 10.2, NRING1, Ink::Fixed(RING_85), 1.0),
    shut_path(0.0, 6.5, NCARD, Ink::Fixed(HUB_EDGE), 1.2),
    Prim::Round { x: -0.6, y: 54.6, w: 6.0, h: 38.3, r: 1.5, fill: Some(Ink::Fixed(HUB_PLATE)), stroke: None, width: 0.0 },
];
/// A card's selected dress, from EMAIL (:413-414): the well silhouette
/// filled AND stroked `#f2b463` 1.2, no rings, and the smaller
/// 4.6x32.1 plate standing 1.25 proud of the edge inside the well
/// (244.75 = 246 - 1.25, 442.3 = 384 + 58.3).
const CARD_SELECTED: &[Prim] = &[
    Prim::Path { x: 0.0, y: 6.5, segs: NCARDSEL, close: true, fill: Some(Ink::Fixed(HUB_FILL)), stroke: Some(Ink::Fixed(HUB_FILL)), width: 1.2 },
    Prim::Round { x: -1.25, y: 58.3, w: 4.6, h: 32.1, r: 1.5, fill: Some(Ink::Fixed(HUB_PLATE)), stroke: None, width: 0.0 },
];
/// One menu unit: the plate's hit box is the stroke-centre silhouette
/// (90.5x327) at the trace's `<use x y>`, and both dresses are the
/// card-local consts placed there.
macro_rules! module {
    ($i:expr, $x:expr, $y:expr) => {
        Prim::Plate {
            group: Group::Module,
            index: $i,
            x: $x,
            y: $y,
            w: 90.5,
            h: 327.0,
            on: &[Prim::At { x: $x, y: $y, prims: CARD_SELECTED }],
            off: &[Prim::At { x: $x, y: $y, prims: CARD_IDLE }],
        }
    };
}

/// The five-line caption block under a card's foot (`#ncaption`, :231-237).
const NCAPTION: &[Prim] = &[
    fill_rect(0.0, 0.0, 84.0, 5.0, Ink::Fixed(HUB_MID)),
    fill_rect(0.0, 8.0, 90.0, 5.0, Ink::Fixed(HUB_MID)),
    fill_rect(0.0, 16.0, 82.0, 5.0, Ink::Fixed(HUB_MID)),
    fill_rect(0.0, 24.0, 88.0, 5.0, Ink::Fixed(HUB_MID)),
    fill_rect(0.0, 32.0, 38.0, 5.0, Ink::Fixed(HUB_MID)),
];

/// The detail panel (`#npanel`, :180): the shoulder at local y 30.3
/// from the r7.5 top-left corner to x 64, one cubic climbing to the
/// top line by x 110, r10 top-right and bottom corners. Opens (0,37.8).
const NPANEL: &[Seg] = &[
    Seg::Quad { cx: 0.0, cy: 30.3, x: 7.5, y: 30.3 },
    Seg::Line(64.0, 30.3),
    Seg::Cubic { c1x: 79.6, c1y: 30.3, c2x: 94.4, c2y: 0.0, x: 110.0, y: 0.0 },
    Seg::Line(220.4, 0.0),
    Seg::Quad { cx: 230.4, cy: 0.0, x: 230.4, y: 10.0 },
    Seg::Line(230.4, 455.3),
    Seg::Quad { cx: 230.4, cy: 465.3, x: 220.4, y: 465.3 },
    Seg::Line(9.0, 465.3),
    Seg::Quad { cx: 0.0, cy: 465.3, x: 0.0, y: 456.3 },
];
/// The panel's four rings nested inside it (`#npring1..4`, :200-203),
/// open, all leaving the shoulder at (64,30.3).
const NPRING1: &[Seg] = &[
    Seg::Cubic { c1x: 79.6, c1y: 30.3, c2x: 94.4, c2y: 3.2, x: 110.0, y: 3.2 },
    Seg::Line(220.4, 3.2),
    Seg::Quad { cx: 227.4, cy: 3.2, x: 227.4, y: 10.2 },
    Seg::Line(227.4, 455.1),
    Seg::Quad { cx: 227.4, cy: 462.1, x: 220.4, y: 462.1 },
    Seg::Line(7.0, 462.1),
    Seg::Quad { cx: 0.0, cy: 462.1, x: 0.0, y: 455.1 },
];
const NPRING2: &[Seg] = &[
    Seg::Cubic { c1x: 79.6, c1y: 30.3, c2x: 94.4, c2y: 6.4, x: 110.0, y: 6.4 },
    Seg::Line(217.4, 6.4),
    Seg::Quad { cx: 224.4, cy: 6.4, x: 224.4, y: 13.4 },
    Seg::Line(224.4, 451.9),
    Seg::Quad { cx: 224.4, cy: 458.9, x: 217.4, y: 458.9 },
    Seg::Line(7.0, 458.9),
    Seg::Quad { cx: 0.0, cy: 458.9, x: 0.0, y: 451.9 },
];
const NPRING3: &[Seg] = &[
    Seg::Cubic { c1x: 79.6, c1y: 30.3, c2x: 94.4, c2y: 9.6, x: 110.0, y: 9.6 },
    Seg::Line(214.4, 9.6),
    Seg::Quad { cx: 221.4, cy: 9.6, x: 221.4, y: 16.6 },
    Seg::Line(221.4, 448.7),
    Seg::Quad { cx: 221.4, cy: 455.7, x: 214.4, y: 455.7 },
    Seg::Line(7.0, 455.7),
    Seg::Quad { cx: 0.0, cy: 455.7, x: 0.0, y: 448.7 },
];
const NPRING4: &[Seg] = &[
    Seg::Cubic { c1x: 79.6, c1y: 30.3, c2x: 94.4, c2y: 12.8, x: 110.0, y: 12.8 },
    Seg::Line(211.4, 12.8),
    Seg::Quad { cx: 218.4, cy: 12.8, x: 218.4, y: 19.8 },
    Seg::Line(218.4, 445.5),
    Seg::Quad { cx: 218.4, cy: 452.5, x: 211.4, y: 452.5 },
    Seg::Line(7.0, 452.5),
    Seg::Quad { cx: 0.0, cy: 452.5, x: 0.0, y: 445.5 },
];
/// The panel's outline group, panel-local to `translate(1170.8 259.7)`
/// (:445-451): rings innermost first, then the front.
const PANEL_FRAME: &[Prim] = &[
    line_path(64.0, 30.3, NPRING4, Ink::Fixed(RING_25), 1.0),
    line_path(64.0, 30.3, NPRING3, Ink::Fixed(RING_55), 1.0),
    line_path(64.0, 30.3, NPRING2, Ink::Fixed(RING_70), 1.0),
    line_path(64.0, 30.3, NPRING1, Ink::Fixed(RING_70), 1.0),
    shut_path(0.0, 37.8, NPANEL, Ink::Fixed(HUB_EDGE), 1.2),
];

/// The T2 badge (:270-290): seven hairline rings of the folder outline
/// fading inward, the front, and the solid trapezoid tab pointing up
/// off the inside bottom edge. Each ring is closed (`Z`) and opens at
/// its own top-left, (1286.8,40.3) for the outermost.
const T2_1: &[Seg] = &[
    Seg::Line(1302.0, 40.3),
    Seg::Quad { cx: 1314.6, cy: 42.4, x: 1317.6, y: 37.4 },
    Seg::Line(1320.6, 41.1),
    Seg::Quad { cx: 1322.6, cy: 32.1, x: 1329.6, y: 32.1 },
    Seg::Line(1344.2, 32.1),
    Seg::Quad { cx: 1349.2, cy: 32.1, x: 1349.2, y: 37.1 },
    Seg::Line(1349.2, 101.8),
    Seg::Quad { cx: 1349.2, cy: 105.8, x: 1345.2, y: 105.8 },
    Seg::Line(1286.8, 105.8),
    Seg::Quad { cx: 1282.8, cy: 105.8, x: 1282.8, y: 101.8 },
    Seg::Line(1282.8, 46.4),
    Seg::Quad { cx: 1282.8, cy: 42.4, x: 1286.8, y: 42.4 },
];
const T2_2: &[Seg] = &[
    Seg::Line(1302.0, 42.4),
    Seg::Quad { cx: 1313.8, cy: 44.2, x: 1316.8, y: 39.2 },
    Seg::Line(1319.8, 41.8),
    Seg::Quad { cx: 1321.8, cy: 32.8, x: 1328.8, y: 32.8 },
    Seg::Line(1342.6, 32.8),
    Seg::Quad { cx: 1347.6, cy: 32.8, x: 1347.6, y: 37.8 },
    Seg::Line(1347.6, 101.5),
    Seg::Quad { cx: 1347.6, cy: 105.5, x: 1343.6, y: 105.5 },
    Seg::Line(1287.4, 105.5),
    Seg::Quad { cx: 1283.4, cy: 105.5, x: 1283.4, y: 101.5 },
    Seg::Line(1283.4, 48.2),
    Seg::Quad { cx: 1283.4, cy: 44.2, x: 1287.4, y: 44.2 },
];
const T2_3: &[Seg] = &[
    Seg::Line(1302.0, 44.5),
    Seg::Quad { cx: 1313.0, cy: 46.0, x: 1316.0, y: 41.0 },
    Seg::Line(1319.0, 42.5),
    Seg::Quad { cx: 1321.0, cy: 33.5, x: 1328.0, y: 33.5 },
    Seg::Line(1341.0, 33.5),
    Seg::Quad { cx: 1346.0, cy: 33.5, x: 1346.0, y: 38.5 },
    Seg::Line(1346.0, 101.2),
    Seg::Quad { cx: 1346.0, cy: 105.2, x: 1342.0, y: 105.2 },
    Seg::Line(1288.0, 105.2),
    Seg::Quad { cx: 1284.0, cy: 105.2, x: 1284.0, y: 101.2 },
    Seg::Line(1284.0, 50.0),
    Seg::Quad { cx: 1284.0, cy: 46.0, x: 1288.0, y: 46.0 },
];
const T2_4: &[Seg] = &[
    Seg::Line(1302.0, 46.6),
    Seg::Quad { cx: 1312.2, cy: 47.8, x: 1315.2, y: 42.8 },
    Seg::Line(1318.2, 43.2),
    Seg::Quad { cx: 1320.2, cy: 34.2, x: 1327.2, y: 34.2 },
    Seg::Line(1339.4, 34.2),
    Seg::Quad { cx: 1344.4, cy: 34.2, x: 1344.4, y: 39.2 },
    Seg::Line(1344.4, 101.0),
    Seg::Quad { cx: 1344.4, cy: 105.0, x: 1340.4, y: 105.0 },
    Seg::Line(1288.6, 105.0),
    Seg::Quad { cx: 1284.6, cy: 105.0, x: 1284.6, y: 101.0 },
    Seg::Line(1284.6, 51.8),
    Seg::Quad { cx: 1284.6, cy: 47.8, x: 1288.6, y: 47.8 },
];
const T2_5: &[Seg] = &[
    Seg::Line(1302.0, 48.7),
    Seg::Quad { cx: 1311.4, cy: 49.6, x: 1314.4, y: 44.6 },
    Seg::Line(1317.4, 43.9),
    Seg::Quad { cx: 1319.4, cy: 34.9, x: 1326.4, y: 34.9 },
    Seg::Line(1337.8, 34.9),
    Seg::Quad { cx: 1342.8, cy: 34.9, x: 1342.8, y: 39.9 },
    Seg::Line(1342.8, 100.8),
    Seg::Quad { cx: 1342.8, cy: 104.8, x: 1338.8, y: 104.8 },
    Seg::Line(1289.2, 104.8),
    Seg::Quad { cx: 1285.2, cy: 104.8, x: 1285.2, y: 100.8 },
    Seg::Line(1285.2, 53.6),
    Seg::Quad { cx: 1285.2, cy: 49.6, x: 1289.2, y: 49.6 },
];
const T2_6: &[Seg] = &[
    Seg::Line(1302.0, 50.8),
    Seg::Quad { cx: 1310.6, cy: 51.4, x: 1313.6, y: 46.4 },
    Seg::Line(1316.6, 44.6),
    Seg::Quad { cx: 1318.6, cy: 35.6, x: 1325.6, y: 35.6 },
    Seg::Line(1336.2, 35.6),
    Seg::Quad { cx: 1341.2, cy: 35.6, x: 1341.2, y: 40.6 },
    Seg::Line(1341.2, 100.5),
    Seg::Quad { cx: 1341.2, cy: 104.5, x: 1337.2, y: 104.5 },
    Seg::Line(1289.8, 104.5),
    Seg::Quad { cx: 1285.8, cy: 104.5, x: 1285.8, y: 100.5 },
    Seg::Line(1285.8, 55.4),
    Seg::Quad { cx: 1285.8, cy: 51.4, x: 1289.8, y: 51.4 },
];
const T2_7: &[Seg] = &[
    Seg::Line(1302.0, 52.9),
    Seg::Quad { cx: 1309.8, cy: 53.2, x: 1312.8, y: 48.2 },
    Seg::Line(1315.8, 45.3),
    Seg::Quad { cx: 1317.8, cy: 36.3, x: 1324.8, y: 36.3 },
    Seg::Line(1334.6, 36.3),
    Seg::Quad { cx: 1339.6, cy: 36.3, x: 1339.6, y: 41.3 },
    Seg::Line(1339.6, 100.2),
    Seg::Quad { cx: 1339.6, cy: 104.2, x: 1335.6, y: 104.2 },
    Seg::Line(1290.4, 104.2),
    Seg::Quad { cx: 1286.4, cy: 104.2, x: 1286.4, y: 100.2 },
    Seg::Line(1286.4, 57.2),
    Seg::Quad { cx: 1286.4, cy: 53.2, x: 1290.4, y: 53.2 },
];
/// The front (:287), opening at (1291,55).
const T2_FRONT: &[Seg] = &[
    Seg::Line(1302.0, 55.0),
    Seg::Quad { cx: 1309.0, cy: 55.0, x: 1312.0, y: 50.0 },
    Seg::Line(1315.0, 46.0),
    Seg::Quad { cx: 1317.0, cy: 37.0, x: 1324.0, y: 37.0 },
    Seg::Line(1333.0, 37.0),
    Seg::Quad { cx: 1338.0, cy: 37.0, x: 1338.0, y: 42.0 },
    Seg::Line(1338.0, 100.0),
    Seg::Quad { cx: 1338.0, cy: 104.0, x: 1334.0, y: 104.0 },
    Seg::Line(1291.0, 104.0),
    Seg::Quad { cx: 1287.0, cy: 104.0, x: 1287.0, y: 100.0 },
    Seg::Line(1287.0, 59.0),
    Seg::Quad { cx: 1287.0, cy: 55.0, x: 1291.0, y: 55.0 },
];
/// The tab (:288), opening at (1293,104.5).
const T2_TAB: &[Seg] = &[
    Seg::Line(1297.0, 100.0),
    Seg::Line(1322.0, 100.0),
    Seg::Line(1326.0, 104.5),
];
const T2_BADGE: &[Prim] = &[
    shut_path(1286.8, 40.3, T2_1, Ink::Fixed(BADGE_55), 0.7),
    shut_path(1287.4, 42.4, T2_2, Ink::Fixed(BADGE_60), 0.7),
    shut_path(1288.0, 44.5, T2_3, Ink::Fixed(BADGE_65), 0.7),
    shut_path(1288.6, 46.6, T2_4, Ink::Fixed(BADGE_70), 0.7),
    shut_path(1289.2, 48.7, T2_5, Ink::Fixed(BADGE_75), 0.7),
    shut_path(1289.8, 50.8, T2_6, Ink::Fixed(BADGE_80), 0.7),
    shut_path(1290.4, 52.9, T2_7, Ink::Fixed(BADGE_85), 0.7),
    shut_path(1291.0, 55.0, T2_FRONT, Ink::Fixed(BADGE_LIT), 1.1),
    fill_path(1293.0, 104.5, T2_TAB, Ink::Fixed(HUB_FILL)),
    txt(1295.0, 71.0, 12.0, Ink::Fixed(CAPTION), "LEVEL"),
    Prim::Text { x: 1296.0, y: 95.0, size: 21.0, ink: Ink::Fixed(BADGE_LIT), face: Face::SemiBold, anchor: Anchor::Start, content: "T2" },
];

/// One strand of the wire band (:299-306): in low at the left at `yl`,
/// a cubic up onto the tight line at `yb`, the long run to x 1040, a
/// cubic back down to `yr` under the badges, and the curl at x 1568.
macro_rules! wire {
    ($yl:expr, $yb:expr, $yr:expr) => {
        line_path(30.0, $yl, &[
            Seg::Line(52.0, $yl),
            Seg::Cubic { c1x: 84.0, c1y: $yl, c2x: 100.0, c2y: $yb, x: 130.0, y: $yb },
            Seg::Line(1040.0, $yb),
            Seg::Cubic { c1x: 1075.0, c1y: $yb, c2x: 1090.0, c2y: $yr, x: 1125.0, y: $yr },
            Seg::Line(1535.0, $yr),
            Seg::Quad { cx: 1562.0, cy: $yr, x: 1568.0, y: $yr + 22.0 },
            Seg::Line(1570.0, $yr + 30.0),
        ], Ink::Fixed(MICRO), 1.1)
    };
}

/// A boxed section letter on this screen (:308-335): the store's
/// `LETTERBOX` silhouette in `HUB_MID`, the 15px letter in `CAPTION`
/// centred on the plate at (+12, +19).
macro_rules! hub_box {
    ($x:expr, $y:expr, $letter:expr) => {
        Prim::At {
            x: $x,
            y: $y,
            prims: &[
                shut_path(3.0, 0.0, LETTERBOX, Ink::Fixed(HUB_MID), 1.4),
                line_path(26.0, 17.0, LETTERBOX_FOLD, Ink::Fixed(HUB_MID), 1.0),
                txt_mid(12.0, 19.0, 15.0, Ink::Fixed(CAPTION), $letter),
            ],
        }
    };
}

pub const DASHBOARD: &[Prim] = &[
    Prim::At { x: 0.0, y: 0.0, prims: HUB_GROUND },
    // ==== header (:254-291) ====
    txt(120.0, 42.0, 15.0, Ink::Fixed(HUB_MID), "CUSTOMER #NC488402"),
    txt(120.0, 70.0, 12.0, Ink::Fixed(HUB_MID), "LEVEL"),
    Prim::Text { x: 126.0, y: 90.0, size: 21.0, ink: Ink::Fixed(HUB_MID), face: Face::SemiBold, anchor: Anchor::Start, content: "T1" },
    txt(1131.0, 68.0, 12.0, Ink::Fixed(HUB_MID), "SECURITY"),
    txt(1131.0, 83.0, 12.0, Ink::Fixed(HUB_MID), "LEVEL"),
    txt(1229.0, 63.0, 12.0, Ink::Fixed(HUB_MID), "LEVEL"),
    txt(1354.0, 63.0, 12.0, Ink::Fixed(HUB_MID), "LEVEL"),
    txt(1417.0, 63.0, 12.0, Ink::Fixed(HUB_MID), "LEVEL"),
    Prim::Text { x: 1236.0, y: 86.0, size: 20.0, ink: Ink::Fixed(HUB_MID), face: Face::SemiBold, anchor: Anchor::Start, content: "T1" },
    Prim::Text { x: 1361.0, y: 86.0, size: 20.0, ink: Ink::Fixed(HUB_MID), face: Face::SemiBold, anchor: Anchor::Start, content: "T3" },
    Prim::Text { x: 1424.0, y: 86.0, size: 20.0, ink: Ink::Fixed(HUB_MID), face: Face::SemiBold, anchor: Anchor::Start, content: "T4" },
    Prim::At { x: 0.0, y: 0.0, prims: T2_BADGE },
    // the wire band (:299-306): eight strands, low runs 2.9 apart on the
    // left, 0.16 apart on the tight line, 1.8 apart on the right ribbon
    wire!(122.0, 86.40, 123.0),
    wire!(124.9, 86.56, 124.8),
    wire!(127.8, 86.72, 126.6),
    wire!(130.7, 86.88, 128.4),
    wire!(133.6, 87.04, 130.2),
    wire!(136.5, 87.20, 132.0),
    wire!(139.4, 87.36, 133.8),
    wire!(142.3, 87.52, 135.6),
    // boxed letters (:308-335): A/B mask the strands with an r3 interior
    Prim::Round { x: 238.0, y: 98.0, w: 26.0, h: 26.0, r: 3.0, fill: Some(Ink::Fixed(BOX_FILL)), stroke: None, width: 0.0 },
    Prim::Round { x: 1011.0, y: 98.0, w: 26.0, h: 26.0, r: 3.0, fill: Some(Ink::Fixed(BOX_FILL)), stroke: None, width: 0.0 },
    hub_box!(238.0, 98.0, "A"),
    hub_box!(1011.0, 98.0, "B"),
    hub_box!(585.0, 799.0, "C"),
    hub_box!(1172.0, 799.0, "D"),
    txt(288.0, 106.0, 8.0, Ink::Fixed(MICRO), "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    txt(288.0, 116.0, 8.0, Ink::Fixed(MICRO), "SERVING CUSTOMERS SINCE 2006."),
    txt_end(1000.0, 106.0, 8.0, Ink::Fixed(MICRO), "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    txt_end(1000.0, 116.0, 8.0, Ink::Fixed(MICRO), "SERVING CUSTOMERS SINCE 2006."),
    txt(620.0, 826.0, 8.0, Ink::Fixed(MICRO), "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    txt(620.0, 837.0, 8.0, Ink::Fixed(MICRO), "SERVING CUSTOMERS SINCE 2006."),
    txt(1208.0, 826.0, 8.0, Ink::Fixed(MICRO), "MAPS ARE PROVIDED BY SEOCHO. SATELITE SERVICES"),
    txt(1208.0, 837.0, 8.0, Ink::Fixed(MICRO), "SINCE 2006."),
    // ==== the six cascade cards (:338-414) ====
    // in the trace's reading order, at the `<use>` positions of :390-394
    // and :413; the trace paints all rings, then all fronts, then all
    // plates, which is the same picture since no two cards overlap
    module!(0, 246.0, 384.0),
    module!(1, 347.0, 284.0),
    module!(2, 449.0, 182.0),
    module!(3, 624.0, 384.0),
    module!(4, 724.0, 284.0),
    module!(5, 826.0, 182.0),
    // labels (:423-431), right-anchored beside each card
    txt_end(238.0, 466.7, 17.0, Ink::Fixed(HUB_FILL), "EMAIL"),
    txt_end(338.0, 366.3, 17.0, Ink::Fixed(HUB_FILL), "MATRIX"),
    txt_end(440.0, 264.6, 17.0, Ink::Fixed(HUB_FILL), "BRAINDANCE"),
    txt_end(615.0, 466.7, 17.0, Ink::Fixed(HUB_FILL), "PRIVATE"),
    txt_end(714.0, 356.3, 17.0, Ink::Fixed(HUB_FILL), "SECURITY"),
    txt_end(714.0, 377.9, 17.0, Ink::Fixed(HUB_FILL), "SYSTEMS"),
    txt_end(817.0, 264.6, 17.0, Ink::Fixed(HUB_FILL), "DEVICES"),
    // captions under each foot (:434-439)
    Prim::At { x: 253.0, y: 723.0, prims: NCAPTION },
    Prim::At { x: 352.0, y: 622.0, prims: NCAPTION },
    Prim::At { x: 455.0, y: 520.0, prims: NCAPTION },
    Prim::At { x: 630.0, y: 723.0, prims: NCAPTION },
    Prim::At { x: 730.0, y: 622.0, prims: NCAPTION },
    Prim::At { x: 832.0, y: 520.0, prims: NCAPTION },
    // ==== the detail panel (:441-480) ====
    Prim::At { x: 1170.8, y: 259.7, prims: PANEL_FRAME },
    // the solid body (:454)
    fill_rect(1170.8, 326.0, 230.4, 309.0, Ink::Fixed(HUB_FILL)),
    // two dark paragraphs, six lines and two, 11 tall at 19.5 pitch (:460-469)
    fill_rect(1181.5, 343.0, 182.0, 11.0, Ink::Fixed(HUB_DARK)),
    fill_rect(1181.5, 362.5, 200.0, 11.0, Ink::Fixed(HUB_DARK)),
    fill_rect(1181.5, 382.0, 195.0, 11.0, Ink::Fixed(HUB_DARK)),
    fill_rect(1181.5, 401.5, 170.0, 11.0, Ink::Fixed(HUB_DARK)),
    fill_rect(1181.5, 421.0, 202.0, 11.0, Ink::Fixed(HUB_DARK)),
    fill_rect(1181.5, 440.5, 101.0, 11.0, Ink::Fixed(HUB_DARK)),
    fill_rect(1181.5, 480.0, 202.0, 11.0, Ink::Fixed(HUB_DARK)),
    fill_rect(1181.5, 499.5, 207.0, 11.0, Ink::Fixed(HUB_DARK)),
    // the micro-text tape (:475-476) and the module name (:478)
    fill_rect(1194.0, 641.2, 174.0, 4.2, Ink::Fixed(HUB_MID)),
    fill_rect(1194.0, 647.5, 181.0, 4.2, Ink::Fixed(HUB_MID)),
    Prim::Text { x: 1286.7, y: 692.0, size: 20.0, ink: Ink::Fixed(HUB_FILL), face: Face::SemiBold, anchor: Anchor::Middle, content: "EMAIL" },
];
// --- end dashboard -------------------------------------------------------
