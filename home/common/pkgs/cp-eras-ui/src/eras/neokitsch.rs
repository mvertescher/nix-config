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
    Banner, Bar, BarChrome, BarGround, BarMenu, BarOrnament, Chrome, Coat, Compliance, Controls,
    Corner, Destination, Dress,
    Era, Face, Footnotes, Ground, Ink, MenuMarker, MenuRule, Metrics, Nameplate,
    PanelEcho, Selection, Style, Tab, Ticket, WindowLabel,
};
use crate::widgets::surface::{Corners, Cut};
// --- login ---
use crate::style::{
    Access, Bevel, Blink, Caret, Colophon, Entry, Fixture, Legend, Masthead, Plate, Plot, Slot,
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
/// reader this era reaches is `Chrome::DeviceFrame` in `widgets::chrome`,
/// and only from the bar's mail example panel (`panels::mail`) -- the
/// dashboard stopped reading it at the `Layout` fold of 2026-09-03.
/// (The bar's other reader is `PanelEcho::Wave`, which is kitsch's echo,
/// not this era's.) The gated renders override it from
/// `home/themes/neokitsch/palettes.nix`.
pub const STRATA: iced::Color = rgb(0x634427);

/// Broad-field fit shared by the four native screens. These are stops of
/// the fitted violet ellipse, separate from the generic `BLOOM` role.
/// The original photographic material recipe is unknown; see
/// `docs/neokitsch/ground-fit.md` for the clear-patch fit and holdout.
pub const HAZE_CORE: iced::Color = rgb(0x7a538b);
pub const HAZE_MID: iced::Color = rgb(0x3c3a57);
pub const HAZE_EDGE: iced::Color = rgb(0x131014);
pub const HAZE_OUT: iced::Color = rgb(0x0d0a0d);

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

/// The published reference palette differs from the standalone palette
/// only in its derived panel role (`home/themes/neokitsch/palettes.nix`).
/// Exact matching keeps the mailbox's photographed inks out of variants
/// and out of direct edits to any role, including `on_select`.
pub fn mailbox_reference_palette(actual: &Palette) -> bool {
    let built_in = palette();
    let mut published = built_in;
    published.panel = rgb(0x16161f);
    *actual == built_in || *actual == published
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
            // annotation under the wire bridge at 400. Until
            // 2026-09-04 this was Medium: `fonts.rs` published no
            // semibold face, and asking the shaper for one resolved
            // it to Bold, which at 14px on a 25px cell read as a
            // smear. The bar binaries now load the 600 file.
            face: Face::SemiBold,
            tape_extra: 10.0,
            tape_ticks: false,

            // The strip is violet at the centre and black at the ends.
            // `bar.svg` keeps the former dashboard haze as its own
            // design field, independent of the four fitted source
            // screens. Its ground is composited at the bar's pixels -- the
            // blue annulus included, which is what casts the bar's
            // last 150px (design #2b2e40 at x 1520). Until 2026-09-05
            // this carried the `#haze` lobe's numbers and the strip
            // stacked 64 discs from them, without the blue.
            ground: BarGround::Haze { prims: BAR_GROUND },
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
                stroke: None,
                // The trace's one label at 400 in a strip set at 600.
                face: Some(Face::Regular),
            },

            alert_suffix: None,
            bold_tiers: false,
            // login-trace's 10:10 PM at (1293,87): the only clock in
            // the run, and it is unboxed.
            clock_plain: Some((18, Face::Medium)),

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
                row_overshoot: 8.0,
                spine: 0.0,
                foot: 0.0,
                marker: MenuMarker::Text,
                // The onion rings, nested inside the panel as the
                // dashboard photo nests them inside its cards and detail
                // panel (dashboard-trace `#nring1..6`, `#npring1..4`):
                // four at the detail panel's 3.2 pitch, fading inward.
                // Outward at 3.5 until 2026-09-04 (`bar.svg` item 10).
                echo: PanelEcho::Rings {
                    count: 4,
                    pitch: 3.2,
                },
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
        // --- controls --- (components.svg LOGIN ENTRY GROUP, OUTLINED
        // BUTTON)
        controls: Controls {
            // Echo rings and veneer need custom drawing. Keep the
            // coats until the complete treatment is wired.
            primary_states: Default::default(),
            ghost_states: Default::default(),
            field_states: Default::default(),
            // ENTER / LOGIN: the amber bar with the chocolate ink
            // `ACCESS` uses on it; `on_select` is veneer's ink.
            primary: Coat::filled(Ink::Cta, Ink::Fixed(ON_BAR)),
            // RIFLES: outline 1.25 and label both in the gold text, as
            // `mailbox.buttons` reads them.
            ghost: Coat::outlined(Ink::Fg, 1.25, Ink::Fg),
            // No disabled control in the era; the outline one stop down.
            disabled: Coat::outlined(Ink::Dim, 1.25, Ink::Dim),
            // The entry field: solid chocolate, no outline, no rule.
            field: Coat::filled(Ink::Inset, Ink::Fg),
            placeholder: Ink::Dim,
            // The r5 the buttons carry (with a chamfer and a tab a
            // built-in cannot draw).
            radius: 5.0,
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
        store_states: STORE_STATES,
        store_selection: (1, 1),
        store_cursor: None,
        // --- end store ---
        // --- dashboard ---
        dashboard: DASHBOARD,
        dashboard_reference_fg: None,
        store_reference_fg: None,
        store_reference: None,
        dashboard_selection: 0,
        dashboard_cursor: false,
        mailbox_cursor: false,
        dashboard_states: HUB_STATES,
        dashboard_held_backdrops: &[],
        // EMAIL is card 0; nothing on this hub says "store", and the
        // store is `s` from the hub instead (`screens::hub`).
        dashboard_destinations: [Some(Destination::Mail), None, None, None, None, None],
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

// Native #70 logo foreground traced from the 3840x2160 source at its
// 1600x900 design coordinates. Outer contours run clockwise, and the four
// counters run counterclockwise so the canvas nonzero fill retains the
// background inside each lowercase a. The s cap and lower stroke share
// one contour. This is the source mark, not a stretched font run.
const LOGIN_LOGO_PATH: &[Seg] = &[
    Seg::Line(108.75, 63.33),
    Seg::Line(117.5, 69.17),
    Seg::Line(117.92, 75.42),
    Seg::Line(115.83, 76.25),
    Seg::Line(117.5, 77.5),
    Seg::Line(117.92, 79.17),
    Seg::Line(117.08, 86.67),
    Seg::Line(111.67, 86.67),
    Seg::Line(111.25, 85.0),
    Seg::Line(105.83, 86.67),
    Seg::Line(101.25, 86.25),
    Seg::Line(96.25, 83.75),
    Seg::Line(93.33, 80.42),
    Seg::Line(92.08, 77.5),
    Seg::Line(92.08, 72.92),
    Seg::Line(93.33, 69.58),
    Seg::Line(96.67, 65.83),
    Seg::Move(119.58, 63.33),
    Seg::Line(141.25, 63.75),
    Seg::Line(144.58, 68.75),
    Seg::Line(144.58, 74.17),
    Seg::Line(137.92, 74.17),
    Seg::Line(137.92, 71.67),
    Seg::Line(136.25, 69.58),
    Seg::Line(125.83, 69.58),
    Seg::Line(125.83, 74.17),
    Seg::Line(143.33, 86.25),
    Seg::Line(133.33, 86.67),
    Seg::Line(125.83, 82.08),
    Seg::Line(125.83, 86.25),
    Seg::Line(119.58, 86.67),
    Seg::Line(119.17, 77.92),
    Seg::Line(120.83, 77.92),
    Seg::Line(118.75, 75.42),
    Seg::Move(156.67, 63.33),
    Seg::Line(162.92, 63.33),
    Seg::Line(171.67, 69.58),
    Seg::Line(171.67, 75.83),
    Seg::Line(169.58, 76.25),
    Seg::Line(171.67, 77.92),
    Seg::Line(171.25, 86.67),
    Seg::Line(165.83, 86.67),
    Seg::Line(165.0, 85.0),
    Seg::Line(160.0, 86.67),
    Seg::Line(155.42, 86.25),
    Seg::Line(150.83, 84.17),
    Seg::Line(147.5, 80.83),
    Seg::Line(145.83, 75.83),
    Seg::Line(147.08, 70.0),
    Seg::Line(150.83, 65.83),
    Seg::Move(184.17, 63.33),
    Seg::Line(194.58, 63.33),
    Seg::Line(196.67, 65.42),
    Seg::Line(196.25, 66.25),
    Seg::Line(197.08, 66.25),
    Seg::Line(198.33, 68.75),
    Seg::Line(197.5, 70.0),
    Seg::Line(183.75, 70.0),
    Seg::Line(198.75, 79.58),
    Seg::Line(198.33, 86.67),
    Seg::Line(177.5, 86.67),
    Seg::Line(176.25, 85.83),
    Seg::Line(176.67, 84.58),
    Seg::Line(175.42, 85.0),
    Seg::Line(173.75, 82.92),
    Seg::Line(172.92, 80.0),
    Seg::Line(173.33, 75.83),
    Seg::Line(179.58, 75.83),
    Seg::Line(181.25, 80.42),
    Seg::Line(188.33, 80.42),
    Seg::Line(173.33, 70.42),
    Seg::Line(173.33, 69.58),
    Seg::Line(175.0, 69.17),
    Seg::Line(183.75, 69.58),
    Seg::Move(210.42, 63.33),
    Seg::Line(217.08, 63.75),
    Seg::Line(217.5, 65.0),
    Seg::Line(218.75, 65.0),
    Seg::Line(225.0, 69.58),
    Seg::Line(225.0, 75.83),
    Seg::Line(223.33, 76.25),
    Seg::Line(225.0, 77.92),
    Seg::Line(224.58, 86.67),
    Seg::Line(219.17, 86.67),
    Seg::Line(218.33, 85.0),
    Seg::Line(212.92, 86.67),
    Seg::Line(207.5, 85.83),
    Seg::Line(202.08, 82.5),
    Seg::Line(200.42, 80.0),
    Seg::Line(199.58, 77.5),
    Seg::Line(200.42, 70.0),
    Seg::Line(203.75, 66.25),
    Seg::Line(204.58, 66.67),
    Seg::Line(204.17, 65.83),
    Seg::Move(227.08, 63.33),
    Seg::Line(232.5, 63.33),
    Seg::Line(232.92, 68.33),
    Seg::Line(237.08, 65.83),
    Seg::Line(238.33, 66.25),
    Seg::Line(237.92, 65.42),
    Seg::Line(241.25, 63.33),
    Seg::Line(250.0, 63.33),
    Seg::Line(249.58, 65.42),
    Seg::Line(235.0, 74.58),
    Seg::Line(235.0, 75.42),
    Seg::Line(249.17, 84.17),
    Seg::Line(250.0, 85.0),
    Seg::Line(249.58, 86.67),
    Seg::Line(241.25, 86.67),
    Seg::Line(233.75, 81.67),
    Seg::Line(232.92, 81.67),
    Seg::Line(232.5, 86.67),
    Seg::Line(226.67, 86.25),
    Seg::Line(226.67, 77.92),
    Seg::Line(228.33, 77.92),
    Seg::Line(226.67, 76.25),
    Seg::Line(226.25, 68.75),
    Seg::Move(258.75, 63.33),
    Seg::Line(263.33, 63.33),
    Seg::Line(272.08, 69.58),
    Seg::Line(272.08, 75.83),
    Seg::Line(270.0, 76.25),
    Seg::Line(272.08, 77.92),
    Seg::Line(272.08, 86.25),
    Seg::Line(266.25, 86.67),
    Seg::Line(265.83, 85.0),
    Seg::Line(262.92, 86.25),
    Seg::Line(255.83, 86.25),
    Seg::Line(250.83, 83.75),
    Seg::Line(247.92, 80.83),
    Seg::Line(246.67, 77.92),
    Seg::Line(246.67, 72.5),
    Seg::Line(247.92, 69.58),
    Seg::Line(252.5, 65.0),
    Seg::Move(258.75, 69.17),
    Seg::Line(254.17, 71.25),
    Seg::Line(252.92, 73.75),
    Seg::Line(253.33, 77.5),
    Seg::Line(256.67, 80.42),
    Seg::Line(262.92, 80.0),
    Seg::Line(265.83, 84.17),
    Seg::Line(265.42, 72.5),
    Seg::Move(103.33, 69.58),
    Seg::Line(99.17, 72.08),
    Seg::Line(99.17, 77.92),
    Seg::Line(102.5, 80.42),
    Seg::Line(108.33, 80.0),
    Seg::Line(111.25, 84.17),
    Seg::Line(110.83, 72.5),
    Seg::Line(106.25, 69.58),
    Seg::Move(157.08, 69.58),
    Seg::Line(155.0, 70.42),
    Seg::Line(152.92, 72.92),
    Seg::Line(153.33, 77.92),
    Seg::Line(156.67, 80.42),
    Seg::Line(160.0, 80.83),
    Seg::Line(162.5, 80.0),
    Seg::Line(165.0, 83.75),
    Seg::Line(165.0, 72.5),
    Seg::Line(160.42, 69.58),
    Seg::Move(210.42, 69.58),
    Seg::Line(206.25, 72.5),
    Seg::Line(206.25, 77.5),
    Seg::Line(210.0, 80.42),
    Seg::Line(215.83, 80.0),
    Seg::Line(218.33, 83.75),
    Seg::Line(218.33, 72.5),
    Seg::Line(213.75, 69.58),
];
const LOGIN_LOGO: &[Plate] = &[Plate::filled(
    Plot::new(92.08, 63.33, 180.0, 23.34),
    Ink::Fg,
).outlined_path((102.5, 63.33), LOGIN_LOGO_PATH)];

// The photographed login badges have an external chamfer and a solid
// bottom tab. The shared mini-SIM badge adds an internal fold instead.
const LOGIN_BADGE_A_OUTLINE: &[Seg] = &[
    Seg::Line(438.0, 466.0),
    Seg::Quad { cx: 441.0, cy: 466.0, x: 441.0, y: 469.0 },
    Seg::Line(441.0, 482.5),
    Seg::Line(433.5, 490.5),
    Seg::Line(420.0, 490.5),
    Seg::Quad { cx: 417.0, cy: 490.5, x: 417.0, y: 487.5 },
    Seg::Line(417.0, 469.0),
    Seg::Quad { cx: 417.0, cy: 466.0, x: 420.0, y: 466.0 },
];
const LOGIN_BADGE_A_TAB: &[Seg] = &[
    Seg::Line(421.2, 488.0),
    Seg::Quad { cx: 421.6, cy: 487.5, x: 423.0, y: 487.5 },
    Seg::Line(429.9, 487.5),
    Seg::Quad { cx: 431.4, cy: 487.5, x: 431.8, y: 488.0 },
    Seg::Line(433.5, 490.8),
];
// The source A has a narrow apex, slim legs and an open triangular counter.
// The inner contour winds opposite the outer one so the fill leaves its hole.
const LOGIN_BADGE_A_GLYPH: &[Seg] = &[
    Seg::Line(426.6667, 470.0),
    Seg::Line(427.9167, 468.75),
    Seg::Line(429.5833, 468.75),
    Seg::Line(430.8333, 470.0),
    Seg::Line(437.5, 482.9167),
    Seg::Line(435.0, 482.9167),
    Seg::Line(433.3333, 479.5833),
    Seg::Line(423.75, 479.5833),
    Seg::Line(422.0833, 482.9167),
    Seg::Move(425.0, 477.5),
    Seg::Line(432.0833, 477.5),
    Seg::Line(429.1667, 471.25),
    Seg::Line(428.3333, 471.25),
];
const LOGIN_BADGE_B_OUTLINE: &[Seg] = &[
    Seg::Line(858.0, 466.0),
    Seg::Quad { cx: 861.0, cy: 466.0, x: 861.0, y: 469.0 },
    Seg::Line(861.0, 482.5),
    Seg::Line(853.5, 490.5),
    Seg::Line(840.0, 490.5),
    Seg::Quad { cx: 837.0, cy: 490.5, x: 837.0, y: 487.5 },
    Seg::Line(837.0, 469.0),
    Seg::Quad { cx: 837.0, cy: 466.0, x: 840.0, y: 466.0 },
];
const LOGIN_BADGE_B_TAB: &[Seg] = &[
    Seg::Line(841.2, 488.0),
    Seg::Quad { cx: 841.6, cy: 487.5, x: 843.0, y: 487.5 },
    Seg::Line(849.9, 487.5),
    Seg::Quad { cx: 851.4, cy: 487.5, x: 851.8, y: 488.0 },
    Seg::Line(853.5, 490.8),
];
const LOGIN_BADGE_A_ART: &[Plate] = &[
    Plate::outlined(Plot::new(416.0, 465.0, 26.0, 26.0), Ink::Fixed(HAIRLINE), 1.4)
        .outlined_path((420.0, 466.0), LOGIN_BADGE_A_OUTLINE),
    Plate::filled(Plot::new(416.0, 465.0, 26.0, 26.0), Ink::Fixed(HAIRLINE))
        .outlined_path((419.4, 490.8), LOGIN_BADGE_A_TAB),
    Plate::filled(Plot::new(416.0, 465.0, 26.0, 26.0), Ink::Fixed(CAPTION))
        .outlined_path((420.0, 482.9167), LOGIN_BADGE_A_GLYPH),
];
// The source B has a narrow spine and broad, flat counters. Both inner
// contours wind opposite the outer path to leave their openings clear.
const LOGIN_BADGE_B_GLYPH: &[Seg] = &[
    Seg::Line(851.6667, 468.9167),
    Seg::Quad { cx: 855.4167, cy: 468.9167, x: 855.4167, y: 472.5 },
    Seg::Quad { cx: 855.4167, cy: 474.5833, x: 852.9167, y: 475.4167 },
    Seg::Quad { cx: 855.8333, cy: 476.25, x: 855.8333, y: 479.1667 },
    Seg::Quad { cx: 855.8333, cy: 482.9167, x: 851.6667, y: 482.9167 },
    Seg::Line(842.5, 482.9167),
    Seg::Quad { cx: 841.25, cy: 482.9167, x: 841.25, y: 481.6667 },
    Seg::Line(841.25, 470.0),
    Seg::Quad { cx: 841.25, cy: 468.9167, x: 842.9167, y: 468.9167 },
    Seg::Move(844.1667, 470.8333),
    Seg::Line(844.1667, 474.5833),
    Seg::Line(852.0833, 474.5833),
    Seg::Quad { cx: 853.75, cy: 474.5833, x: 853.75, y: 472.5 },
    Seg::Quad { cx: 853.75, cy: 470.8333, x: 852.0833, y: 470.8333 },
    Seg::Move(843.9583, 476.4583),
    Seg::Line(843.9583, 481.0417),
    Seg::Line(852.5, 481.0417),
    Seg::Quad { cx: 853.75, cy: 481.0417, x: 853.75, y: 478.75 },
    Seg::Quad { cx: 853.75, cy: 476.4583, x: 852.5, y: 476.4583 },
];
const LOGIN_BADGE_B_ART: &[Plate] = &[
    Plate::outlined(Plot::new(836.0, 465.0, 26.0, 26.0), Ink::Fixed(HAIRLINE), 1.4)
        .outlined_path((840.0, 466.0), LOGIN_BADGE_B_OUTLINE),
    Plate::filled(Plot::new(836.0, 465.0, 26.0, 26.0), Ink::Fixed(HAIRLINE))
        .outlined_path((839.4, 490.8), LOGIN_BADGE_B_TAB),
    Plate::filled(Plot::new(836.0, 465.0, 26.0, 26.0), Ink::Fixed(CAPTION))
        .outlined_path((842.9167, 468.9167), LOGIN_BADGE_B_GLYPH),
];

pub const ACCESS: Access = Access {
    reference_fg: None,
    reference_backdrop: None,
    // The trace's haze is the store's to the number ("identical
    // backdrop to dashboard-trace.svg; re-verified here by column
    // profile"), so the ground is `PAGE_BACKDROP` from the store block:
    // page, haze, lobe, and the blue annulus through its fade. Not
    // `Ground::Bloom`, whose stacked translucent discs cap out around
    // 6% alpha and reach a tenth of this -- `#4f4262` at the top centre
    // where the bloom puts `#1f1f33`, the difference between the
    // backdrop holding two of the frame's palette clusters and none.
    backdrop: PAGE_BACKDROP,
    masthead: Masthead::Logotype {
        art: LOGIN_LOGO,
        cell: Plate::outlined(Plot::new(90.0, 100.0, 185.0, 25.0), Ink::Fixed(HAIRLINE), 1.0),
        divider: 195.0,
        labels: &[
            // The vector stencil above occupies x 92.08..272.08;
            // the two captions are independent measured type runs.
            Legend {
                weight: iced::font::Weight::Semibold,
                ..Legend::new(
                    "ARASAKA CONSUMER TECHNOLOGY", 118.5, 96.5, 9.0,
                    Ink::Fixed(CAPTION),
                ).tracked(0.8).stretched(0.90)
            },
            Legend {
                weight: iced::font::Weight::Semibold,
                ..Legend::new("57ASD4AV15AA", 95.0, 113.7, 8.4,
                    Ink::Fixed(CAPTION)).tracked(0.5).stretched(0.98)
            },
            Legend {
                weight: iced::font::Weight::Semibold,
                ..Legend::new("COMBAT COLONIZATION", 201.5, 112.0, 8.2,
                    Ink::Fixed(CAPTION)).stretched(0.85)
            },
            Legend {
                weight: iced::font::Weight::Semibold,
                ..Legend::new("DEFENCE PROGRAM", 201.5, 119.6, 8.2,
                    Ink::Fixed(CAPTION)).stretched(0.88)
            },
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
            // The trace shows the well empty: no run, no cursor. A typed
            // run goes in the name's face and ink, inset the way the name
            // is, on a baseline centred in the well.
            entry: Some(Entry {
                rest: Legend::new("", 429.0, 389.0, 19.5, Ink::Fg).medium().tracked(0.15),
                mask: '*',
                tail: "",
                caret: Caret::Fixed,
                blink: Blink::Still,
                busy: "VERIFYING",
                failed: "ACCESS DENIED",
            }),
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
            badge_art: LOGIN_BADGE_A_ART,
            badge_letter: None,
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
            entry: Some(Entry {
                rest: Legend::new("", 849.0, 389.0, 19.5, Ink::Fg).medium().tracked(0.15),
                mask: '*',
                tail: "",
                caret: Caret::Fixed,
                blink: Blink::Still,
                busy: "VERIFYING",
                failed: "ACCESS DENIED",
            }),
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
            badge_art: LOGIN_BADGE_B_ART,
            badge_letter: None,
            notes: &[
                Legend::new(NOTE_1, 874.0, 471.0, 7.0, Ink::Fixed(MICRO)),
                Legend::new(NOTE_2, 874.0, 479.0, 7.0, Ink::Fixed(MICRO)),
            ],
            ..Slot::EMPTY
        },
    ],
    // 22 strands: the outer plateaus at y 727.1 with a 3.9 spacing, the
    // centre plateau at 782.1 with the spacing tightened to 3.03, and
    // both feet descend from independent endpoints 17px above each run.
    fixture: Fixture::WireBand {
        outer: 727.1,
        inner: 782.1,
        end: 710.1,
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
// The wire band's eleven strands use the trace's measured cubic curves;
// source #71 has two more resolved strands than the hub.
use crate::style::{
    Frame, Mail, MailBadges, MailButtons, MailEnvelope, MailList, MailSelectedPrinting, MailRowCoat, MailRowStates, MailRowEcho, MailRowType, MailMotion, MailPanel, MailPart, Mailbox,
    Note, Piece, RowDecor, Run, Seg, Trim, Veneer, FromAt, BL, TR,
};

/// The selection bar's own two tones, measured off the photograph at
/// 3840 rather than derived from the era's `VENEER` family: the flat
/// base the grain sits on, and the grain-line core the polished trace
/// recoloured to #cf975c. They are a stop brighter than `VENEER` /
/// `GRAIN`, which dress a synthesised plank rather than this one.
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

// Source-specific mailbox frames and letters; other screens retain their
// independently fitted badge artwork.
#[path = "neokitsch_mailbox_badges.rs"]
mod mailbox_badges;

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

const fn letter(x: f32, y: f32, s: &'static str, art: &'static [Piece]) -> Piece {
    Piece::LabelArt {
        note: Note { at: Run::new(x, y, 19.5, Ink::Fg).bold().centered(), text: s },
        pieces: art,
    }
}

// Header wire geometry measured separately from source #69 and #71 (NK-02).
// The former common eight-strand table put the left rise through T1 and
// compressed only the right pitch. Source #69 resolves nine strands;
// source #71 eleven. Both have a 3.18 pitch and rounded downward feet.
const fn header_wire_steps(index: usize, mail: bool) -> [Seg; 9] {
    let i = index as f32;
    let y = if mail { 123.7 } else { 123.45 } + 3.18 * i;
    let bridge = if mail { 87.05 } else { 86.8 };
    let left = if mail { 34.7 } else { 35.5 };
    let right = if mail { 1564.2 } else { 1565.0 };
    let depart = if mail { 160.8 } else { 160.4 } - 1.6 * i;
    let control = if mail { 187.6 } else { 192.2 } - 0.7 * i;
    [
        Seg::Line(left, y + 16.0),
        Seg::Cubic { c1x: left, c1y: y + 7.16, c2x: left + 7.16, c2y: y, x: left + 16.0, y },
        Seg::Line(depart, y),
        Seg::Cubic { c1x: control, c1y: y, c2x: if mail { 195.3 } else { 195.05 }, c2y: bridge, x: 226.0, y: bridge },
        Seg::Line(1056.0, bridge),
        Seg::Cubic { c1x: 1078.5, c1y: bridge, c2x: 1092.3 + 0.7 * i, c2y: y, x: 1114.3 + 1.6 * i, y },
        Seg::Line(right - 16.0, y),
        Seg::Cubic { c1x: right - 7.16, c1y: y, c2x: right, c2y: y + 7.16, x: right, y: y + 16.0 },
        Seg::Line(right, y + 16.0),
    ]
}

const fn header_wire_ink(index: usize, mail: bool) -> Ink {
    Ink::Fixed(iced::Color {
        a: 1.0 - 0.8 * index as f32 / if mail { 10.0 } else { 8.0 },
        ..if mail { rgb(0xdfb47c) } else { rgb(0xedb778) }
    })
}

static WIRE0: [Seg; 9] = header_wire_steps(0, true);
static WIRE1: [Seg; 9] = header_wire_steps(1, true);
static WIRE2: [Seg; 9] = header_wire_steps(2, true);
static WIRE3: [Seg; 9] = header_wire_steps(3, true);
static WIRE4: [Seg; 9] = header_wire_steps(4, true);
static WIRE5: [Seg; 9] = header_wire_steps(5, true);
static WIRE6: [Seg; 9] = header_wire_steps(6, true);
static WIRE7: [Seg; 9] = header_wire_steps(7, true);
static WIRE8: [Seg; 9] = header_wire_steps(8, true);
static WIRE9: [Seg; 9] = header_wire_steps(9, true);
static WIRE10: [Seg; 9] = header_wire_steps(10, true);

// NK-03 source-specific badge curves share the header's sRGB backdrop.
mod badge;

// NK-10: closed folds measured in #71. The list's common glyph origin is
// the open flap apex; the closed box sits 3.5px below it. Read state and
// resolved printing ink are still supplied by the mail renderer.
const CLOSED_ENVELOPE: &[Piece] = &[
    Piece::Poly { points: &[(0.0,3.5),(16.0,3.5),(16.0,14.0),(0.0,14.0)], fill: None, stroke: Some(Ink::Fg), width: 1.0, close: true },
    Piece::Poly { points: &[(0.0,3.5),(8.0,10.7),(16.0,3.5)], fill: None, stroke: Some(Ink::Fg), width: 1.0, close: false },
    Piece::Poly { points: &[(0.0,14.0),(5.3,8.3)], fill: None, stroke: Some(Ink::Fg), width: 1.0, close: false },
    Piece::Poly { points: &[(16.0,14.0),(10.7,8.3)], fill: None, stroke: Some(Ink::Fg), width: 1.0, close: false },
];
const OPEN_ENVELOPE: &[Piece] = &[
    Piece::Poly { points: &[(0.0,4.0),(8.0,0.0),(16.0,4.0),(16.0,14.5),(0.0,14.5)], fill: None, stroke: Some(Ink::Fg), width: 1.0, close: true },
    Piece::Poly { points: &[(0.0,4.0),(8.0,9.5),(16.0,4.0)], fill: None, stroke: Some(Ink::Fg), width: 1.0, close: false },
];

static CHROME: [Piece; 33] = [
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
    text(1295.0, 71.0, 12.0, Ink::Fg, "LEVEL"),
    strong(1296.0, 95.0, 21.0, "T2"),
    mailbox_badges::MAIL_A_FRAME[0],
    mailbox_badges::MAIL_A_FRAME[1],
    mailbox_badges::MAIL_B_FRAME[0],
    mailbox_badges::MAIL_B_FRAME[1],
    letter(249.7, 115.5, "A", mailbox_badges::MAIL_A_CONTOUR),
    letter(1022.5, 115.5, "B", mailbox_badges::MAIL_B_CONTOUR),
    micro(278.3, 103.3, "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    micro(278.3, 110.0, "SERVING CUSTOMERS SINCE 2006."),
    micro(843.3, 103.3, "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    micro(843.3, 110.0, "SERVING CUSTOMERS SINCE 2006."),
    mailbox_badges::MAIL_C_FRAME[0],
    mailbox_badges::MAIL_C_FRAME[1],
    mailbox_badges::MAIL_D_FRAME[0],
    mailbox_badges::MAIL_D_FRAME[1],
    letter(151.3, 796.0, "C", mailbox_badges::MAIL_C_CONTOUR),
    letter(750.7, 796.0, "D", mailbox_badges::MAIL_D_CONTOUR),
    micro(183.3, 782.8, "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    micro(183.3, 789.8, "SERVING CUSTOMERS SINCE 2006."),
    micro(771.7, 782.8, "MAPS ARE PROVIDED BY SEOCHO. SATELITE SERVICES"),
    micro(771.7, 789.8, "SINCE 2006."),
];

static BUTTONS: [&str; 4] = ["RIFLES", "RIFLES", "RIFLES", "RIFLES"];

// Native cap/advance calibration of the four source action labels. Their
// small independent x offsets preserve the existing uniform button frames.
const MAIL_ACTION_TYPE: &[Run] = &[
    Run::new(21.0, 27.5, 16.75, Ink::Fg).medium().stretched(1.050746),
    Run::new(23.4667, 27.5, 16.75, Ink::Fg).medium().stretched(1.050746),
    Run::new(21.9833, 27.5, 16.75, Ink::Fg).medium().stretched(1.050746),
    Run::new(20.0833, 27.5, 16.75, Ink::Fg).medium().stretched(1.050746),
];
static LEVELS: [&str; 0] = [];

/// The seven rows, trace lines 442-455; the envelopes at lines 361-367
/// are open on rows 1, 3 and 7 and closed on the rest, the selected
/// row's included. Rows 6 and 7 are both from Rachel Ross here, where
/// entropism has Biala Robertson and Larix & Betula. The senders are
/// set in capitals; `from_upper` does that here.
static ROWS: [Mail; 7] = [
    Mail { subject: "You'll regret that", from: "Jackie", unread: true },
    Mail { subject: "Urgent information (!)", from: "Mom", unread: false },
    Mail { subject: "Heist data sent to you", from: "805000451", unread: true },
    Mail { subject: "I'm worried man", from: "Rachel Ross", unread: false },
    Mail { subject: "Special offer to you!", from: "JINX JINX STORE", unread: false },
    Mail { subject: "I'm worried man", from: "Rachel Ross", unread: false },
    Mail { subject: "Special offer to you!", from: "Rachel Ross", unread: true },
];

/// The body, trace lines 463-472: 2 + 5 + 3 lines. This era breaks its
/// first paragraph after "aliqua." and runs "Excepteur sint" into the
/// second, so the three paragraphs are the same words split in yet
/// another place; like kitsch it has no "Nemo enim" paragraph.
static PARAGRAPHS: [&[&str]; 3] = [
    &[
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod",
        "tempor incididunt ut labore et dolore magna aliqua.",
    ],
    &[
        "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut",
        "aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in vo-",
        "luptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat",
        "cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est la-",
        "borum.",
    ],
    &[
        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium do-",
        "loremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore",
        "veritatis et quasi architecto beatae vitae dicta sunt explicabo.",
    ],
];

// The source titles share one face and width. Their row-local baselines
// account for the native renderer's offset from the SVG text baselines;
// the row frames and 60.2 pitch stay fixed. Only row 2 starts at x 193.5.
const MAIL_ROW_TYPE: &[MailRowType] = &[
    MailRowType {
        title: Run::new(193.0, 27.825, 18.0, Ink::Fg).medium().stretched(0.975),
        from: Run::new(193.4167, 49.0333, 13.0, Ink::Mid).medium().stretched(1.15),
    },
    MailRowType {
        title: Run::new(193.5, 27.4083, 18.0, Ink::Fg).medium().stretched(0.975),
        from: Run::new(193.0, 49.0333, 13.0, Ink::Mid).medium().stretched(1.16),
    },
    MailRowType {
        title: Run::new(193.0, 27.2, 18.0, Ink::Fg).medium().stretched(0.975),
        from: Run::new(193.4167, 48.4083, 13.0, Ink::Mid).medium().stretched(1.13),
    },
    MailRowType {
        title: Run::new(193.0, 27.2, 18.0, Ink::Fg).medium().stretched(0.975),
        from: Run::new(193.4167, 48.2, 13.0, Ink::Mid).medium().stretched(1.15),
    },
    MailRowType {
        title: Run::new(193.0, 26.7833, 18.0, Ink::Fg).medium().stretched(0.975),
        from: Run::new(193.4167, 48.2, 13.0, Ink::Mid).medium().stretched(1.13),
    },
    MailRowType {
        title: Run::new(193.0, 26.575, 18.0, Ink::Fg).medium().stretched(0.975),
        from: Run::new(193.4167, 47.575, 13.0, Ink::Mid).medium().stretched(1.15),
    },
    MailRowType {
        title: Run::new(193.0, 26.1583, 18.0, Ink::Fg).medium().stretched(0.975),
        from: Run::new(193.4167, 47.3667, 13.0, Ink::Mid).medium().stretched(1.15),
    },
];

pub fn mailbox() -> Mailbox {
    Mailbox {
        text_baseline: 0.95,
        // The mailbox trace (`:211-213`) opens with the store's haze, lobe
        // and masked blue line for line, so it takes the store's ground.
        backdrop: MAIL_HEADER_BACKDROP,
        chrome: &CHROME,
        overlay: &[],
        list: MailList {
            envelope: Some(MailEnvelope { normal: CLOSED_ENVELOPE, open: OPEN_ENVELOPE }),
            row_type: MAIL_ROW_TYPE,
            selected_row_type: None,
            footer: &[],
            feedback: Some(MailRowStates {
                hover: MailRowCoat {
                    fill: None, outline: Some(Ink::Fixed(rgb(0xe8c186))),
                    printing: None, spine: None, selection: false,
                    sender: None,
                    echo: Some(MailRowEcho {
                        rings: 7, step: Frame::new(-0.6, -2.1, 2.2, 2.4),
                        ink: Ink::Fixed(rgb(0xa97c48)), width: 0.7,
                        alpha: 0.85, fade: 0.05,
                        fill: None, fill_alpha: 0.0,
                    }),
                },
                pressed: MailRowCoat {
                    fill: None, outline: None, printing: None, spine: None,
                    sender: None,
                    selection: true, echo: None,
                },
                selected_hover: None,
            }),
            frame: None,
            frame_ink: Ink::Fg,
            frame_width: 0.0,
            // The ruled rows and both text lines fit the source's 60.2
            // pitch. The envelopes alone sit progressively lower in the
            // photo; keep their independent, measured offsets below.
            row: Frame::new(35.0, 248.8, 477.0, 60.2),
            pitch: 60.2,
            rows: &ROWS,
            selected: 1,
            decor: RowDecor::Ruled,
            row_fill: None,
            row_fills: &[],
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
            sel_fill: Ink::Select,
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
            glyph_offsets: &[0.0, 0.0, 1.5, 1.5, 2.3, 3.2, 4.4],
            glyph_w: 16.0,
            text_x: 193.0,
            title_dy: 27.2,
            title_size: 18.0,
            title_bold: false,
            title_ink: Ink::Fg,
            selected_ink: Ink::OnSelect,
            // The photograph and trace distinguish all three marks over
            // the veneer; the general palette's on_select is much darker.
            selected_printing: Some(MailSelectedPrinting {
                title: Ink::Fixed(rgb(0x7b5438)),
                sender: Ink::Fixed(rgb(0x895f3b)),
                envelope: Ink::Fixed(rgb(0x865c39)),
            }),
            from_dy: 48.2,
            from_size: 13.0,
            from_ink: Ink::Mid,
            from_at: FromAt::Beneath,
            from_prefix: "FROM: ",
            title_upper: false,
            from_upper: true,
            new_pill: None,
            new_pill_selected: None,
            new_pill_art: &[],
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
            // the message is the selected row's, URGENT INFORMATION (!)
            // / FROM: MOM, trace lines 460-461
            message: 1,
            title: Run::new(736.0, 276.0, 17.5, Ink::Fg).semibold().stretched(1.01),
            title_upper: true,
            from: Some(Run::new(738.0, 297.0, 13.0, Ink::Fg).medium().stretched(1.16)),
            heading: None,
            sender: None,
            // Source body is a light, wide face; the photo's glow is
            // excluded from the font weight fit.
            body: Run::new(735.2, 333.0, 17.0, Ink::Fg).medium().stretched(1.054),
            line: 21.5,
            para: 43.0,
            paragraph_baselines: &[],
            paragraphs: &PARAGRAPHS,
        },
        buttons: MailButtons {
            // four outlined RIFLES buttons, 184x39 on a 192 pitch (the photo
            // steps 194, 191, 190: x 735, 929, 1120, 1310; the uniform pitch
            // lands within 2px of each), each
            // with a bottom-left chamfer and a filled tab on its bottom
            // edge
            first: Frame::new(735.0, 684.0, 184.0, 39.0),
            dx: 192.0,
            dy: 0.0,
            count: 4,
            filled: None,
            fill: Ink::Select,
            idle_fill: None,
            joined: false,
            chevron: false,
            trim: Trim::chamfer(BL, 13.0),
            width: 1.25,
            stroke: Ink::Fg,
            label: MAIL_ACTION_TYPE[0],
            tab: Some(Frame::new(131.0, 32.0, 37.0, 7.0)),
            label_runs: MAIL_ACTION_TYPE,
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
            label_runs: &[],
            label_art: &[],
            caption: None,
            caption_text: "",
            labels: &LEVELS,
        },
        motions: MAILBOX_MOTIONS,
    }
}

/// The mailbox's boot-in (mailbox-trace :208-250 and :322-348), the
/// neokitsch way -- the dashboard's left-to-right wipes and the hub
/// panel's fade: the rows scan on from the left, the buttons follow as
/// they finish, and the selection bar fades up last. The mailbox is a
/// layout, not a display list, so these name the sheet's regions
/// (`MailMotion`, `Mailbox::motions`; `screens/mail.rs` paints
/// them) where the store's `#shelf-open` wraps its prims:
///
///   * `#list-open` (:226-231): the rows' rules and tabs, their
///     envelopes and their titles, rect x 15 y 240 h 460, w 0 -> 520
///     over 0.5 s from 0, `keySplines="0.33 1 0.68 1"` = EaseOutCubic
///     -- `MailPart::List`. The one clipPath is referenced from the
///     rows' group in `#lines` and their glyphs' group in `#text`,
///     split only for the halo: one animation, one entry;
///   * `#bar-fade` + `#bar-fade-text` (:341-348, :545-548): the
///     selection bar with its grain, inverted tab, envelope and dark
///     printing, opacity 0 -> 1 over 0.3 s from 0.4 s,
///     `keySplines="0.61 1 0.88 1"` = EaseOut -- ONE motion, as the
///     trace says, over `MailPart::Fills` and `MailPart::Printing`;
///     `Fills` in this era is the bar alone (`buttons.filled` and `badges.selected` are `None`, the
///     panel has no head). The sheet draws a fill under its region's
///     cover as well, so the bar also sits under `#list-open`; that is
///     invisible, the wipe has cleared x 512 (EaseOutCubic at 0.4 of
///     0.5 s is 0.992 of 520) before the fade begins;
///   * `#buttons-open` (:244-250): the four RIFLES buttons, outlines,
///     tabs and labels, rect x 715 y 660 h 90, w 0 -> 810 over 0.4 s
///     from 0.3 s, EaseOutCubic -- `MailPart::Buttons`. The `<set>`
///     holding the width at 0 until 0.3 s needs nothing here:
///     `Motion::begin` holds at `from`.
///
/// The selected row's printing -- the inverted tab (`sel_notch`), its
/// envelope, its title and FROM: line -- fades in with the bar here
/// (:404-418, :538-550), which is what `MailPart::Printing` beside
/// `Fills` says; without it the dark glyphs and the tab's bright
/// outline would stand on the bare ground for 0.4 s. The fade is an
/// ink fade, so the grain over the fill and the envelope over the
/// grain show the stack for those 0.3 s, as the hub's panel does (the
/// `Change::Opacity` caveat, docs/PIPELINE.md).
pub const MAILBOX_MOTIONS: &[MailMotion] = &[
    MailMotion {
        motion: Motion {
            id: "list-open",
            begin: 0,
            dur: 500,
            ease: Easing::EaseOutCubic,
            change: Change::Clip { x: 15.0, y: 240.0, w: (0.0, 520.0), h: (460.0, 460.0) },
        },
        parts: &[MailPart::List],
    },
    MailMotion {
        motion: Motion {
            id: "bar-fade",
            begin: 400,
            dur: 300,
            ease: Easing::EaseOut,
            change: Change::Opacity { alpha: (0.0, 1.0) },
        },
        parts: &[MailPart::Fills, MailPart::Printing],
    },
    MailMotion {
        motion: Motion {
            id: "buttons-open",
            begin: 300,
            dur: 400,
            ease: Easing::EaseOutCubic,
            change: Change::Clip { x: 715.0, y: 660.0, w: (0.0, 810.0), h: (90.0, 90.0) },
        },
        parts: &[MailPart::Buttons],
    },
];
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
// The trace spends about four fifths of its bytes on **wood-veneer
// grain** -- hundreds of 0.7px polylines clipped to the three gold
// fills, "drawn as clipped 0.7 strokes in a mid gold over the fill *so
// the average stays at the sampled mean*". The selected store body
// uses longitudinal paths and a narrow seam in `store_grain`; inferred
// pressed feedback and the nav/BASKET bands still use `Prim::Grain`.
// (This comment said until 2026-09-07 that the grain was left
// out as photographic texture; the tables below had carried it since
// the store's first pass.) The soft `#38261a` halo under every stroke
// is the photograph's glow and is out (docs/PIPELINE.md). Everything
// structural -- the wire band, the echo strands, the BASKET plate, the
// card frames, the socket rows, the tabs -- is here.

use crate::style::{
    fill_path, fill_rect, line_path, shut_path, txt, txt_bold, txt_end, txt_mid, vline, Anchor,
    Change, Group, Motion, Prim,
};
use iced::animation::Easing;

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
pub const GUN_BASE: iced::Color = rgb(0xac9152);
pub const GUN_BRIGHT: iced::Color = rgb(0xffd779);
const STORE_LOGO_SOLID: iced::Color = rgb(0xfdcd9d);
const STORE_LOGO_OUTLINE: iced::Color = rgb(0xf0c48a);
/// The mid gold the source's veneer grain is drawn in.
pub const GRAIN_LINE: iced::Color = rgb(0xcd9553);
/// Dark convergence in the store card's selected-body veneer.
pub const GRAIN_SEAM: iced::Color = rgb(0xa86e33);
/// Low-contrast wire-band echoes. The card frames use the brighter
/// source-fitted `FRAME_ECHO*` inks below.
///
/// The fade matters and is not cosmetic: drawn at one flat strand
/// colour the five strands form a closed ring that hole-fills into a
/// solid slab the size of a card, and the extractor then reports one
/// on each of cards 1, 3 and 4 that the design does not have.
pub const ECHO1: iced::Color = rgb(0x795d4a);
pub const ECHO2: iced::Color = rgb(0x604a3a);
pub const ECHO3: iced::Color = rgb(0x4b3b2f);
pub const ECHO4: iced::Color = rgb(0x3a2f28);
pub const ECHO5: iced::Color = rgb(0x32251c);
// The frames cross both dark blue ground and gold body; their source
// opacity must blend with the ground at each point, especially echo 5.
const FRAME_ECHO1: iced::Color = rgba(0xebb57f, 0.84);
const FRAME_ECHO2: iced::Color = rgba(0xebb57f, 0.74);
const FRAME_ECHO3: iced::Color = rgba(0xebb57f, 0.46);
const FRAME_ECHO4: iced::Color = rgba(0xebb57f, 0.34);
const FRAME_ECHO5: iced::Color = rgba(0xebb57f, 0.08);

const fn rgba(hex: u32, a: f32) -> iced::Color {
    iced::Color { a, ..rgb(hex) }
}

/// Near-black field beyond the fitted haze.
pub const PAGE: iced::Color = rgb(0x0d090c);

/// Shared upper violet field, fitted to low-variation patches common to
/// all four 3840x2160 source frames. Coordinates use the 1600x900 trace
/// frame; the radial is turned about its own centre to match the SVG.
const STORE_HAZE: &[(f32, iced::Color)] = &[
    (0.00, HAZE_CORE),
    (0.20, rgb(0x5e4169)),
    (0.40, rgb(0x524064)),
    (0.60, HAZE_MID),
    (0.80, HAZE_EDGE),
    (1.00, HAZE_OUT),
];
const HAZE_LOBE: &[Prim] = &[Prim::Lobe { x: 0.0, y: 0.0, rx: 1015.8, ry: 393.6, stops: STORE_HAZE }];
/// The right-weighted blue transition. Its radial has a transparent
/// core and exterior; the horizontal mask suppresses the left edge.
const BLUE: &[(f32, iced::Color)] = &[
    (0.60, rgba(0x48537d, 0.00)),
    (0.68, rgba(0x21364e, 0.85)),
    (0.76, rgba(0x0d202f, 0.80)),
    (0.84, rgba(0x0e0d1a, 0.00)),
    (1.00, rgba(0x0e0d1a, 0.00)),
];
const BLUE_LOBE: &[Prim] = &[Prim::Lobe { x: 0.0, y: 0.0, rx: 1188.4, ry: 800.0, stops: BLUE }];
const BLUE_TURNED: &[Prim] = &[Prim::Turn { x: 765.3, y: -316.3, angle: 10.0, prims: BLUE_LOBE }];
/// `#hazebluefade` (:157-163), the luminance mask the blue annulus is
/// laid through (`mask="url(#bluemask)"`, :234): black at the left
/// edge, full from x 640, so the violet stays on top on the left where
/// the source shows only a thin blue arm. Greys, because a mask is
/// read as luminance.
const BLUE_FADE: &[(f32, iced::Color)] = &[
    (0.00, rgb(0x000000)),
    (0.12, rgb(0x1a1a1a)),
    (0.22, rgb(0x7a7a7a)),
    (0.40, rgb(0xffffff)),
    (1.00, rgb(0xffffff)),
];
const BLUE_MASK: &[Prim] = &[Prim::Ramp {
    x: 0.0,
    y: 0.0,
    w: 1600.0,
    h: 900.0,
    from: (0.0, 0.0),
    to: (1.0, 0.0),
    stops: BLUE_FADE,
}];
/// The top-left lift preserves the earlier trace's elliptical shape and
/// opacity falloff; the fitted violet is one step lighter.
const LOBE: &[(f32, iced::Color)] = &[
    (0.00, rgba(0x7a598a, 0.85)),
    (0.45, rgba(0x7a598a, 0.55)),
    (1.00, rgba(0x7a598a, 0.00)),
];

/// The store's ground in the trace's paint order (:231-234): page,
/// haze, lobe, then the blue through its mask.
const BACKDROP: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(PAGE)),
    Prim::Turn { x: 712.8, y: -61.2, angle: 1.76, prims: HAZE_LOBE },
    Prim::Lobe { x: 430.0, y: -40.0, rx: 560.0, ry: 168.0, stops: LOBE },
    Prim::Masked { prims: BLUE_TURNED, mask: BLUE_MASK },
];
/// The mailbox's and the login's ground: `BACKDROP` composited, the way
/// `STORE` opens.
const PAGE_BACKDROP: &[Prim] = &[Prim::Soft { prims: BACKDROP }];

/// The 9x9 socket glyph, read off card 1 as 25 separate 3.2 squares on
/// the source's 3.49 pitch. The same occupancy is used on both coats.
macro_rules! qr {
    ($ink:expr) => {
        &[
            fill_rect(0.00, 0.00, 3.2, 3.2, $ink),
            fill_rect(10.47, 0.00, 3.2, 3.2, $ink),
            fill_rect(17.45, 0.00, 3.2, 3.2, $ink),
            fill_rect(27.92, 0.00, 3.2, 3.2, $ink),
            fill_rect(3.49, 3.49, 3.2, 3.2, $ink),
            fill_rect(20.94, 3.49, 3.2, 3.2, $ink),
            fill_rect(6.98, 6.98, 3.2, 3.2, $ink),
            fill_rect(27.92, 6.98, 3.2, 3.2, $ink),
            fill_rect(0.00, 10.47, 3.2, 3.2, $ink),
            fill_rect(10.47, 10.47, 3.2, 3.2, $ink),
            fill_rect(17.45, 10.47, 3.2, 3.2, $ink),
            fill_rect(24.43, 10.47, 3.2, 3.2, $ink),
            fill_rect(3.49, 17.45, 3.2, 3.2, $ink),
            fill_rect(10.47, 17.45, 3.2, 3.2, $ink),
            fill_rect(20.94, 17.45, 3.2, 3.2, $ink),
            fill_rect(27.92, 17.45, 3.2, 3.2, $ink),
            fill_rect(0.00, 20.94, 3.2, 3.2, $ink),
            fill_rect(6.98, 20.94, 3.2, 3.2, $ink),
            fill_rect(17.45, 20.94, 3.2, 3.2, $ink),
            fill_rect(3.49, 24.43, 3.2, 3.2, $ink),
            fill_rect(24.43, 24.43, 3.2, 3.2, $ink),
            fill_rect(0.00, 27.92, 3.2, 3.2, $ink),
            fill_rect(10.47, 27.92, 3.2, 3.2, $ink),
            fill_rect(17.45, 27.92, 3.2, 3.2, $ink),
            fill_rect(27.92, 27.92, 3.2, 3.2, $ink),
        ]
    };
}
const QR_LIGHT: &[Prim] = qr!(Ink::Fixed(LABEL));
const QR_DARK: &[Prim] = qr!(Ink::Fixed(ON_GOLD));

#[path = "neokitsch_store_badges.rs"]
mod store_badges;

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
        &[
            Seg::Line(274.0, $y),
            Seg::Cubic { c1x: 275.2, c1y: $y, c2x: 274.5, c2y: $y + 5.1, x: 277.0, y: $y + 5.1 },
            Seg::Line(240.5, $y + 5.1),
            Seg::Cubic { c1x: 243.0, c1y: $y + 5.1, c2x: 241.8, c2y: $y, x: 243.0, y: $y },
        ]
    };
}

const NAV1: &[Seg] = nav_plate!(357.9);
const NAV2: &[Seg] = nav_plate!(418.6);
const NAV3: &[Seg] = nav_plate!(479.3);
const NAV4: &[Seg] = nav_plate!(540.0);
const NAV5: &[Seg] = nav_plate!(600.7);
const TAB1: &[Seg] = nav_tab!(391.4);
const TAB2: &[Seg] = nav_tab!(452.1);
const TAB3: &[Seg] = nav_tab!(512.8);
const TAB4: &[Seg] = nav_tab!(573.5);
const TAB5: &[Seg] = nav_tab!(634.2);

#[path = "neokitsch/weapon.rs"]
mod weapon;

/// A standard card's frame, card-local: an r13 top-left, a 37-degree
/// step up to the raised top edge, an r18 top-right, and a bottom-left
/// cut back to the left edge.
const CARD_EDGE: &[Seg] = &[
    Seg::Line(0.0, 358.0),
    Seg::Quad { cx: 0.0, cy: 345.0, x: 13.0, y: 345.0 },
    Seg::Line(136.2, 345.0),
    Seg::Cubic { c1x: 151.2, c1y: 345.0, c2x: 167.2, c2y: 314.6, x: 182.2, y: 314.6 },
    Seg::Line(244.1, 314.6),
    Seg::Quad { cx: 262.1, cy: 314.6, x: 262.1, y: 332.6 },
    Seg::Line(262.1, 590.0),
    Seg::Cubic { c1x: 262.1, c1y: 605.0, c2x: 262.5, c2y: 615.0, x: 262.5, y: 634.4 },
    Seg::Quad { cx: 262.5, cy: 641.2, x: 255.8, y: 641.2 },
    Seg::Line(19.2, 641.2),
    Seg::Line(19.2, 640.4),
];
// Native shoulder profiles share one low tangent. These controls fit the
// five resolved source ridges; high plateaus retain their measured spacing.
const fn store_shoulder(low: f32, high: f32, index: usize) -> Seg {
    const X: [(f32, f32, f32); 5] = [
        (151.14, 163.80, 182.2), (152.53, 162.25, 182.7),
        (153.00, 162.55, 183.2), (154.76, 160.70, 183.7),
        (155.63, 161.24, 184.2),
    ];
    let (c1x, c2x, x) = X[index];
    Seg::Cubic { c1x, c1y: low, c2x, c2y: high, x, y: high }
}

// The three inner right turns spread across the photographed corner before
// becoming vertical. Keep the outer two arcs and the common shoulder intact.
const fn store_corner_start(index: usize) -> f32 {
    match index {
        2 => 237.9,
        3 => 236.23,
        4 => 232.9,
        _ => 244.1,
    }
}
const fn store_corner_x(index: usize) -> f32 {
    match index {
        2 => 252.9,
        3 => 250.4,
        4 => 247.07,
        _ => 259.1 - 3.05 * index as f32,
    }
}
const fn store_corner_turn(high: f32, index: usize) -> Seg {
    match index {
        2 => Seg::Cubic { c1x: 250.35, c1y: high, c2x: 252.9, c2y: high + 10.01, x: 252.9, y: high + 11.58 },
        3 => Seg::Cubic { c1x: 239.3, c1y: high, c2x: 250.4, c2y: high + 3.04, x: 250.4, y: high + 13.8 },
        4 => Seg::Cubic { c1x: 237.17, c1y: high, c2x: 247.07, c2y: high + 4.4, x: 247.07, y: high + 11.02 },
        _ => {
            let x = store_corner_x(index);
            Seg::Quad { cx: x, cy: high, x, y: high + 15.0 - 3.05 * index as f32 }
        }
    }
}

// The photographed bottom has five distinct echo ridges. Their lower turns
// share one card-local shape; the selected card extends it by 70.9 in y.
// The upper turns and shoulder keep their separately measured controls.
const fn store_echo_stem(index: usize) -> f32 {
    [259.5, 256.4, 253.5, 250.4, 247.07][index]
}
const fn store_echo_start(index: usize, selected: bool) -> f32 {
    [632.4, 630.5, 627.4, 622.7, 621.0][index] + if selected { 70.9 } else { 0.0 }
}
const fn store_echo_bottom(index: usize, selected: bool) -> f32 {
    [637.8, 634.7, 631.2, 628.6, 625.5][index] + if selected { 70.9 } else { 0.0 }
}
const fn store_echo_transition(index: usize, selected: bool) -> Seg {
    Seg::Cubic {
        c1x: store_corner_x(index),
        c1y: if selected { 675.0 } else { 605.0 },
        c2x: store_echo_stem(index),
        c2y: if selected { 685.0 } else { 615.0 },
        x: store_echo_stem(index),
        y: store_echo_start(index, selected),
    }
}
const fn store_echo_bend(index: usize, selected: bool) -> Seg {
    Seg::Quad {
        cx: store_echo_stem(index),
        cy: store_echo_bottom(index, selected),
        x: [254.5, 253.1, 250.9, 245.9, 242.9][index],
        y: store_echo_bottom(index, selected),
    }
}
const fn store_echo_left(index: usize, selected: bool) -> Seg {
    Seg::Line(
        if index == 4 { 6.2 } else { 16.6 - 2.6 * index as f32 },
        store_echo_bottom(index, selected),
    )
}

macro_rules! echo {
    ($d:expr, $ink:expr) => {
        line_path(136.2, 345.0, &[
            store_shoulder(345.0, 318.3 + 3.2 * ($d - 1.0), $d as usize - 1),
            Seg::Line(store_corner_start($d as usize - 1), 318.3 + 3.2 * ($d - 1.0)),
            store_corner_turn(318.3 + 3.2 * ($d - 1.0), $d as usize - 1),
            Seg::Line(store_corner_x($d as usize - 1), 590.0),
            store_echo_transition($d as usize - 1, false),
            store_echo_bend($d as usize - 1, false),
            store_echo_left($d as usize - 1, false),
        ], $ink, 1.0)
    };
}
const ECHOES: &[Prim] = &[
    echo!(1.0, Ink::Fixed(FRAME_ECHO1)),
    echo!(2.0, Ink::Fixed(FRAME_ECHO2)),
    echo!(3.0, Ink::Fixed(FRAME_ECHO3)),
    echo!(4.0, Ink::Fixed(FRAME_ECHO4)),
    line_path(136.2, 345.0, &[
        store_shoulder(345.0, 331.1, 4),
        Seg::Line(store_corner_start(4), 331.1),
        store_corner_turn(331.1, 4),
        Seg::Line(store_corner_x(4), 590.0),
        store_echo_transition(4, false),
        store_echo_bend(4, false),
        store_echo_left(4, false),
    ], Ink::Fixed(FRAME_ECHO5), 1.0),
];
/// The solid tab under a card's bottom edge.
const CARD_TAB: &[Seg] = &[
    Seg::Line(160.0, 632.9),
    Seg::Cubic { c1x: 162.0, c1y: 632.9, c2x: 160.5, c2y: 641.4, x: 163.5, y: 641.4 },
    Seg::Line(99.0, 641.4),
    Seg::Cubic { c1x: 102.0, c1y: 641.4, c2x: 100.0, c2y: 632.9, x: 102.0, y: 632.9 },
];

// Canvas Rajdhani baselines land two native pixels below librsvg here.
const STORE_NATIVE_BASELINE_LIFT: f32 = 0.833_333;
const fn store_stat(x: f32, y: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Text { x, y: y - STORE_NATIVE_BASELINE_LIFT, size: 18.0, ink, face: Face::Medium, anchor: Anchor::Start, content }
}
const fn store_number(x: f32, y: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Text { x, y: y - STORE_NATIVE_BASELINE_LIFT, size: 22.5, ink, face: Face::SemiBold, anchor: Anchor::Start, content }
}
const fn store_socket(x: f32, y: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Text { x, y, size: 13.0, ink, face: Face::Medium, anchor: Anchor::Middle, content }
}

// Six repeated Store metadata lines share one native word-spacing model.
// The upper origins follow one copy-level calibration; B keeps its source origin.
const fn store_metadata(x: f32, y: f32, upper: bool, second: bool, content: &'static str) -> Prim {
    Prim::TrackedWords {
        x: if upper { x - 1.0 / 2.4 } else { x },
        y: if upper && second { y - 0.25 } else { y },
        size: 5.958333333333333,
        ink: Ink::Fixed(STORE_MICRO),
        face: Face::FreeSansBold,
        anchor: Anchor::Start,
        tracking: -0.25854214123006725,
        extra_space: 0.7602336207831586,
        content,
    }
}
const STORE_METADATA_FIRST: &str = "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO.";
const STORE_METADATA_SECOND: &str = "SERVING CUSTOMERS SINCE 2006.";

const CARD: &[Prim] = &[
    Prim::At { x: 0.0, y: 0.0, prims: ECHOES },
    shut_path(0.0, 618.0, CARD_EDGE, Ink::Fixed(OUTLINE), 1.3),
    fill_path(102.0, 632.9, CARD_TAB, Ink::Fixed(TAB)),
    Prim::At { x: 0.0, y: 345.0, prims: weapon::PATHS },
    store_stat(33.0, 483.3, Ink::Fixed(BRIGHT), "DPS"),
    store_stat(97.0, 483.3, Ink::Fixed(BRIGHT), "PNT"),
    store_stat(149.25, 483.3, Ink::Fixed(BRIGHT), "ACC"),
    store_stat(203.0, 483.3, Ink::Fixed(BRIGHT), "ROF"),
    Prim::Text { x: 26.5, y: 520.5 - STORE_NATIVE_BASELINE_LIFT, size: 28.0, ink: Ink::Fixed(BRIGHT), face: Face::SemiBold, anchor: Anchor::Start, content: "620" },
    store_number(99.5, 517.5, Ink::Fixed(BRIGHT), "30"),
    store_number(158.5, 517.5, Ink::Fixed(BRIGHT), "5"),
    store_number(211.5, 517.5, Ink::Fixed(BRIGHT), "5"),
    fill_rect(0.0, 532.35, 262.1, 1.1, Ink::Fixed(STRAND)),
    fill_rect(0.0, 580.25, 262.1, 1.1, Ink::Fixed(STRAND)),
    fill_rect(50.75, 532.9, 1.1, 47.9, Ink::Fixed(STRAND)),
    fill_rect(118.25, 532.9, 1.1, 47.9, Ink::Fixed(STRAND)),
    fill_rect(189.85, 532.9, 1.1, 47.9, Ink::Fixed(STRAND)),
    Prim::At { x: 9.8, y: 542.2, prims: QR_LIGHT },
    store_socket(83.1, 555.4, Ink::Fixed(LABEL), "EMPTY"),
    store_socket(83.1, 568.9 - STORE_NATIVE_BASELINE_LIFT, Ink::Fixed(LABEL), "SOCKET"),
    store_socket(154.6, 555.4, Ink::Fixed(LABEL), "EMPTY"),
    store_socket(154.6, 568.9 - STORE_NATIVE_BASELINE_LIFT, Ink::Fixed(LABEL), "SOCKET"),
    store_socket(226.3, 555.4, Ink::Fixed(LABEL), "EMPTY"),
    store_socket(226.3, 568.9 - STORE_NATIVE_BASELINE_LIFT, Ink::Fixed(LABEL), "SOCKET"),
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
    Seg::Cubic { c1x: 151.2, c1y: 263.3, c2x: 167.2, c2y: 232.9, x: 182.2, y: 232.9 },
    Seg::Line(244.1, 232.9),
    Seg::Quad { cx: 262.1, cy: 232.9, x: 262.1, y: 250.9 },
    Seg::Line(262.1, 660.0),
    Seg::Cubic { c1x: 262.1, c1y: 675.0, c2x: 262.9, c2y: 690.0, x: 262.9, y: 705.3 },
    Seg::Quad { cx: 262.9, cy: 712.55, x: 256.2, y: 712.55 },
    Seg::Line(19.2, 712.55),
    Seg::Line(19.2, 711.3),
];
macro_rules! grown_echo {
    ($d:expr, $ink:expr) => {
        line_path(136.2, 263.3, &[
            store_shoulder(263.3, 236.6 + 3.2 * ($d - 1.0), $d as usize - 1),
            Seg::Line(store_corner_start($d as usize - 1), 236.6 + 3.2 * ($d - 1.0)),
            store_corner_turn(236.6 + 3.2 * ($d - 1.0), $d as usize - 1),
            Seg::Line(store_corner_x($d as usize - 1), 660.0),
            store_echo_transition($d as usize - 1, true),
            store_echo_bend($d as usize - 1, true),
            store_echo_left($d as usize - 1, true),
        ], $ink, 1.0)
    };
}
const GROWN_TAB: &[Seg] = &[
    Seg::Line(160.0, 703.8),
    Seg::Cubic { c1x: 162.0, c1y: 703.8, c2x: 160.5, c2y: 712.3, x: 163.5, y: 712.3 },
    Seg::Line(99.0, 712.3),
    Seg::Cubic { c1x: 102.0, c1y: 712.3, c2x: 100.0, c2y: 703.8, x: 102.0, y: 703.8 },
];

const GROWN: &[Prim] = &[
    grown_echo!(1.0, Ink::Fixed(FRAME_ECHO1)),
    grown_echo!(2.0, Ink::Fixed(FRAME_ECHO2)),
    // The source's third flat is lower while its right bend is already aligned.
    // Preserve that bend; the short tail keeps native coverage at all three sizes.
    // SVG has a different phase and remains under separate material review.
    line_path(136.2, 263.3, &[
        store_shoulder(263.3, 236.6 + 3.2 * 2.0, 2),
        Seg::Line(store_corner_start(2), 236.6 + 3.2 * 2.0),
        store_corner_turn(236.6 + 3.2 * 2.0, 2),
        Seg::Line(store_corner_x(2), 660.0),
        store_echo_transition(2, true),
        store_echo_bend(2, true),
        Seg::Cubic {
            c1x: 249.9, c1y: store_echo_bottom(2, true),
            c2x: 248.5, c2y: store_echo_bottom(2, true) + 1.0,
            x: 247.5, y: store_echo_bottom(2, true) + 1.0,
        },
        Seg::Line(16.6 - 2.6 * 2.0, store_echo_bottom(2, true) + 1.0),
    ], Ink::Fixed(FRAME_ECHO3), 1.0),
    grown_echo!(4.0, Ink::Fixed(FRAME_ECHO4)),
    line_path(136.2, 263.3, &[
        store_shoulder(263.3, 249.4, 4),
        Seg::Line(store_corner_start(4), 249.4),
        store_corner_turn(249.4, 4),
        Seg::Line(store_corner_x(4), 660.0),
        store_echo_transition(4, true),
        store_echo_bend(4, true),
        store_echo_left(4, true),
    ], Ink::Fixed(FRAME_ECHO5), 1.0),
    shut_path(0.0, 688.9, GROWN_EDGE, Ink::Fixed(OUTLINE), 1.3),
    fill_path(102.0, 703.8, GROWN_TAB, Ink::Fixed(TAB)),
    Prim::At { x: 0.0, y: 263.3, prims: weapon::PATHS },
    store_stat(33.0, 401.6, Ink::Fixed(BRIGHT), "DPS"),
    store_stat(97.0, 401.6, Ink::Fixed(BRIGHT), "PNT"),
    store_stat(149.25, 401.6, Ink::Fixed(BRIGHT), "ACC"),
    store_stat(203.0, 401.6, Ink::Fixed(BRIGHT), "ROF"),
    txt_bold(24.0, 683.9, 19.0, Ink::Fixed(BRIGHT), "MAGNUM 650"),
    txt(133.0, 683.9, 19.0, Ink::Fixed(BRIGHT), "HAND GUN"),
    // the gold body, faded in after the shelf's wipe: `#body-fade`
    // (store-trace :516-666) and `#body-fade-text` (:756-794) take its
    // opacity from 0 to 1 over 0.3 s from 0.4 s, `keySplines="0.61 1
    // 0.88 1"` = EaseOut, and freeze -- two `<animate>`s because the
    // trace keeps glyphs apart from lines for the halo, one Motion
    // here; the `<set>` under each is the hold at 0 until then, which
    // `Motion::begin` already is. The trace paints the body *after* the
    // shelf's clip group so no fade nests in a wipe; here it stays on
    // the selected card because the card is a `Prim::Plate` and the
    // body must follow the selection, which no top-level group can do
    // without a second plate (and `every_era_offers_..._four_cards`
    // wants exactly four). The frames are the same either way: the
    // wipe has cleared the selected card's right edge (x 929) before
    // 0.25 s, so nothing of the fade is ever clipped. Moved from
    // between the tab and the gun to the card's end, as the trace
    // moved it; nothing painted between overlaps the body (the gun
    // ends at y 364, the labels' baselines are 403.3 and 683.9).
    Prim::Motion {
        motion: Motion {
            id: "body-fade",
            begin: 400,
            dur: 300,
            ease: Easing::EaseOut,
            change: Change::Opacity { alpha: (0.0, 1.0) },
        },
        prims: GROWN_BODY,
    },
];

/// The selected card's solid gold body (store-trace :516-666 and
/// :756-794): the veneer fill and its grain, the dark values and meta
/// lines, the dark socket rules and QR and the EMPTY / SOCKET pairs --
/// every dark run that sits on the gold. Its own table because `GROWN`
/// fades it in under `#body-fade`.
#[path = "neokitsch/store_grain.rs"]
mod store_grain;

const BODY_SURFACE: &[Prim] = &[
    fill_rect(0.0, 411.2, 262.1, 241.7, Ink::Fixed(BODY_FILL)),
    Prim::At { x: 0.0, y: 0.0, prims: store_grain::PATHS },
    Prim::Grain { x: 0.0, y: 603.8, w: 262.1, h: 49.1, pitch: 2.4, width: 0.7, ink: Ink::Fixed(GRAIN_LINE) },
];

const GROWN_BODY: &[Prim] = &[
    // the gold body, and the veneer grain the source fills it with
    Prim::Viewport { x: 0.0, y: 411.2, w: 262.1, h: 241.7, prims: BODY_SURFACE },
    Prim::Text { x: 26.5, y: 438.8 - STORE_NATIVE_BASELINE_LIFT, size: 28.0, ink: Ink::OnSelect, face: Face::SemiBold, anchor: Anchor::Start, content: "620" },
    store_number(99.5, 435.8, Ink::OnSelect, "30"),
    store_number(158.5, 435.8, Ink::OnSelect, "5"),
    store_number(211.5, 435.8, Ink::OnSelect, "5"),
    txt(5.9, 468.0, 19.0, Ink::OnSelect, "20"),
    txt(32.9, 468.0, 19.0, Ink::OnSelect, "Recoil"),
    txt(5.9, 488.0, 19.0, Ink::OnSelect, "22"),
    txt(32.9, 488.0, 19.0, Ink::OnSelect, "Sperad"),
    txt(5.9, 509.0, 19.0, Ink::OnSelect, "12"),
    txt(32.9, 509.0, 19.0, Ink::OnSelect, "Range"),
    txt(5.9, 539.0, 19.0, Ink::OnSelect, "Bonus"),
    txt(5.9, 560.0, 19.0, Ink::OnSelect, "+9 Reflexes"),
    txt(5.9, 580.0, 19.0, Ink::OnSelect, "+2 Modules Slots"),
    // the socket rows: two rules the width of the card, three dividers
    // between them (store-trace :662-663, 1.1 wide, centred on the line)
    fill_rect(0.0, 603.25, 262.1, 1.1, Ink::OnSelect),
    fill_rect(0.0, 651.15, 262.1, 1.1, Ink::OnSelect),
    fill_rect(50.75, 603.8, 1.1, 47.9, Ink::OnSelect),
    fill_rect(118.25, 603.8, 1.1, 47.9, Ink::OnSelect),
    fill_rect(189.85, 603.8, 1.1, 47.9, Ink::OnSelect),
    Prim::At { x: 9.8, y: 613.1, prims: QR_DARK },
    store_socket(83.1, 626.3, Ink::OnSelect, "EMPTY"),
    store_socket(83.1, 639.8 - STORE_NATIVE_BASELINE_LIFT, Ink::OnSelect, "SOCKET"),
    store_socket(154.6, 626.3, Ink::OnSelect, "EMPTY"),
    store_socket(154.6, 639.8 - STORE_NATIVE_BASELINE_LIFT, Ink::OnSelect, "SOCKET"),
    store_socket(226.3, 626.3, Ink::OnSelect, "EMPTY"),
    store_socket(226.3, 639.8 - STORE_NATIVE_BASELINE_LIFT, Ink::OnSelect, "SOCKET"),
];

// Product hover borrows #nk-button-hover's T2 fan, adapted to the
// sourced product silhouette at its CURRENT size. Expanded geometry
// belongs to selection, never pointer feedback.
macro_rules! product_echo {
    ($top:expr, $bottom:expr, $n:expr, $alpha:expr) => {
        shut_path(-0.6 * $n, $bottom - 22.4 + 0.3 * $n, &[
            Seg::Line(-0.6 * $n, $top + 43.4 - 2.1 * $n),
            Seg::Quad { cx: -0.6 * $n, cy: $top + 30.4 - 2.1 * $n, x: 13.0 - 0.6 * $n, y: $top + 30.4 - 2.1 * $n },
            Seg::Line(136.2, $top + 30.4 - 2.1 * $n),
            Seg::Cubic { c1x: 151.2, c1y: $top + 30.4 - 2.1 * $n, c2x: 167.2, c2y: $top - 2.1 * $n, x: 182.2, y: $top - 2.1 * $n },
            Seg::Line(244.1 + 1.6 * $n, $top - 2.1 * $n),
            Seg::Quad { cx: 262.1 + 1.6 * $n, cy: $top - 2.1 * $n, x: 262.1 + 1.6 * $n, y: $top + 18.0 - 2.1 * $n },
            Seg::Line(262.1 + 1.6 * $n, $bottom - 8.0 + 0.3 * $n),
            Seg::Quad { cx: 262.1 + 1.6 * $n, cy: $bottom + 0.3 * $n, x: 254.1 + 1.6 * $n, y: $bottom + 0.3 * $n },
            Seg::Line(19.2 - 0.6 * $n, $bottom + 0.3 * $n),
        ], Ink::Fixed(iced::Color { a: $alpha, ..rgb(0xa97c48) }), 0.7)
    };
}
macro_rules! product_hover {
    ($top:expr, $bottom:expr, $face:expr) => {
        &[
            product_echo!($top, $bottom, 7.0, 0.55),
            product_echo!($top, $bottom, 6.0, 0.60),
            product_echo!($top, $bottom, 5.0, 0.65),
            product_echo!($top, $bottom, 4.0, 0.70),
            product_echo!($top, $bottom, 3.0, 0.75),
            product_echo!($top, $bottom, 2.0, 0.80),
            product_echo!($top, $bottom, 1.0, 0.85),
            Prim::At { x: 0.0, y: 0.0, prims: $face },
        ]
    };
}
const PRODUCT_HOVER: &[Prim] = product_hover!(314.6, 640.4, CARD);
const PRODUCT_SELECTED_HOVER: &[Prim] = product_hover!(232.9, 711.3, GROWN);

// The selected card's veneer material applied to the compact card's
// values/socket band. This compression is inferred: the source only
// shows veneer on the expanded selection. Keep every existing glyph,
// illustration, tab and frame in place; do not reveal expanded details.
const fn compact_product_press() -> [Prim; 28] {
    let mut prims = [CARD[0]; 28];
    prims[0] = fill_rect(0.0, 492.9, 262.1, 88.45, Ink::Fixed(BODY_FILL));
    prims[1] = Prim::Grain { x: 0.0, y: 492.9, w: 262.1, h: 88.45, pitch: 2.4, width: 0.7, ink: Ink::Fixed(GRAIN_LINE) };
    let mut i = 0;
    while i < CARD.len() {
        let mut prim = CARD[i];
        if i >= 8 && i <= 23 {
            match &mut prim {
                Prim::Text { ink, .. } => *ink = Ink::OnSelect,
                Prim::Rect { fill, .. } => *fill = Some(Ink::OnSelect),
                Prim::At { prims, .. } => *prims = QR_DARK,
                _ => (),
            }
        }
        prims[i + 2] = prim;
        i += 1;
    }
    prims
}
const PRODUCT_PRESSED: &[Prim] = &compact_product_press();

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
    fill_rect(1464.25, 29.80, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1472.38, 29.80, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1477.80, 29.80, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1485.93, 29.80, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1469.67, 32.51, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1483.22, 32.51, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1464.25, 35.22, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1480.51, 35.22, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1466.96, 37.93, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1472.38, 37.93, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1477.80, 37.93, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1485.93, 37.93, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1464.25, 43.35, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1469.67, 43.35, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1477.80, 43.35, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1483.22, 43.35, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1472.38, 46.06, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1480.51, 46.06, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1485.93, 46.06, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1466.96, 48.77, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1483.22, 48.77, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1464.25, 51.48, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1472.38, 51.48, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1477.80, 51.48, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
    fill_rect(1485.93, 51.48, 2.3, 2.3, Ink::Fixed(PLATE_INK)),
];

// The photographed 4ST mark repeats the source-native contours used by the
// Entropism store, shifted (-28.333, -35) in design coordinates here.
const STORE_FOUR: &[Seg] = &[
    Seg::Line(188.0, 105.0),
    Seg::Line(190.0, 106.0),
    Seg::Line(190.0, 132.0),
    Seg::Line(198.0, 132.0),
    Seg::Line(198.0, 144.0),
    Seg::Line(190.0, 145.0),
    Seg::Line(190.0, 152.0),
    Seg::Line(173.0, 152.0),
    Seg::Line(172.0, 145.0),
    Seg::Line(138.0, 145.0),
    Seg::Line(137.0, 133.0),
    Seg::Line(157.0, 114.0),
    Seg::Line(164.0, 105.0),
    Seg::Move(158.0, 130.0),
    Seg::Line(162.0, 128.0),
    Seg::Line(172.0, 118.0),
    Seg::Line(172.0, 132.0),
    Seg::Line(158.0, 132.0),
];
const STORE_ESS: &[Seg] = &[
    Seg::Cubic { c1x: 263.0, c1y: 146.0, c2x: 256.0, c2y: 151.0, x: 246.0, y: 153.0 },
    Seg::Cubic { c1x: 232.0, c1y: 156.0, c2x: 218.0, c2y: 154.0, x: 211.0, y: 151.0 },
    Seg::Cubic { c1x: 205.0, c1y: 148.0, c2x: 201.0, c2y: 143.0, x: 201.0, y: 138.0 },
    Seg::Line(205.0, 136.0),
    Seg::Line(218.0, 136.0),
    Seg::Cubic { c1x: 223.0, c1y: 136.0, c2x: 226.0, c2y: 138.0, x: 226.0, y: 140.0 },
    Seg::Cubic { c1x: 228.0, c1y: 142.0, c2x: 236.0, c2y: 142.0, x: 240.0, y: 140.0 },
    Seg::Cubic { c1x: 242.0, c1y: 137.0, c2x: 238.0, c2y: 135.0, x: 230.0, y: 134.0 },
    Seg::Line(211.0, 131.0),
    Seg::Cubic { c1x: 204.0, c1y: 129.0, c2x: 202.0, c2y: 125.0, x: 202.0, y: 118.0 },
    Seg::Cubic { c1x: 201.0, c1y: 109.0, c2x: 208.0, c2y: 103.0, x: 221.0, y: 101.0 },
    Seg::Cubic { c1x: 234.0, c1y: 99.0, c2x: 249.0, c2y: 102.0, x: 255.0, y: 106.0 },
    Seg::Cubic { c1x: 259.0, c1y: 109.0, c2x: 261.0, c2y: 113.0, x: 261.0, y: 118.0 },
    Seg::Line(239.0, 118.0),
    Seg::Cubic { c1x: 238.0, c1y: 114.0, c2x: 233.0, c2y: 112.0, x: 227.0, y: 113.0 },
    Seg::Cubic { c1x: 223.0, c1y: 113.0, c2x: 222.0, c2y: 116.0, x: 223.0, y: 118.0 },
    Seg::Cubic { c1x: 227.0, c1y: 120.0, c2x: 241.0, c2y: 121.0, x: 253.0, y: 122.0 },
    Seg::Cubic { c1x: 261.0, c1y: 125.0, c2x: 264.0, c2y: 131.0, x: 264.0, y: 138.0 },
];
const STORE_SOLID_MARK: &[Prim] = &[
    fill_path(164.0, 105.0, STORE_FOUR, Ink::Fixed(STORE_LOGO_SOLID)),
    fill_path(264.0, 138.0, STORE_ESS, Ink::Fixed(STORE_LOGO_SOLID)),
];

/// The outlined T follows its photographed bar and narrower stem.
const TEE: &[Seg] = &[
    Seg::Line(276.2, 68.5),
    Seg::Line(276.2, 77.0),
    Seg::Line(262.1, 77.0),
    Seg::Line(262.1, 115.0),
    Seg::Line(251.7, 115.0),
    Seg::Line(251.7, 77.0),
    Seg::Line(237.5, 77.0),
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
                fill_path(243.0, $tabtop, $tab, Ink::Fixed(NAV_TAB)),
                txt(115.0, $base, 17.0, Ink::OnSelect, $label),
            ],
            &[
                shut_path(97.0, $top, $plate, Ink::Fixed(OUTLINE), 1.3),
                fill_path(243.0, $tabtop, $tab, Ink::Fixed(NAV_TAB)),
                txt(115.0, $base, 17.0, Ink::Fixed(BRIGHT), $label),
            ],
        )
    };
}
const NAV_ON_0: &[Prim] = nav!(NAV1, TAB1, 357.9, 391.4, 384.4, "RIFLES").0;
const NAV_OFF_0: &[Prim] = nav!(NAV1, TAB1, 357.9, 391.4, 384.4, "RIFLES").1;
const NAV_ON_1: &[Prim] = nav!(NAV2, TAB2, 418.6, 452.1, 445.1, "SMG").0;
const NAV_OFF_1: &[Prim] = nav!(NAV2, TAB2, 418.6, 452.1, 445.1, "SMG").1;
const NAV_ON_2: &[Prim] = nav!(NAV3, TAB3, 479.3, 512.8, 505.8, "SNIPER").0;
const NAV_OFF_2: &[Prim] = nav!(NAV3, TAB3, 479.3, 512.8, 505.8, "SNIPER").1;
const NAV_ON_3: &[Prim] = nav!(NAV4, TAB4, 540.0, 573.5, 566.5, "SHOTGUN").0;
const NAV_OFF_3: &[Prim] = nav!(NAV4, TAB4, 540.0, 573.5, 566.5, "SHOTGUN").1;
const NAV_ON_4: &[Prim] = nav!(NAV5, TAB5, 600.7, 634.2, 627.2, "PISTOL").0;
const NAV_OFF_4: &[Prim] = nav!(NAV5, TAB5, 600.7, 634.2, 627.2, "PISTOL").1;

// components.svg #nk-button-hover retains the inferred seven-ring outward echo;
// it is not a literal copy of the source-corrected T2 badge fan.
// Apply its asymmetric expansion to the store's own r4/cut silhouette;
// the plate, tab and label are the untouched resting drawing on top.
macro_rules! nav_echo {
    ($y:expr, $n:expr, $alpha:expr) => {
        shut_path(97.0 - 0.6 * $n, $y - 2.1 * $n, &[
            Seg::Line(289.5 + 1.6 * $n, $y - 2.1 * $n),
            Seg::Quad { cx: 293.5 + 1.6 * $n, cy: $y - 2.1 * $n, x: 293.5 + 1.6 * $n, y: $y + 4.0 - 2.1 * $n },
            Seg::Line(293.5 + 1.6 * $n, $y + 34.6 + 0.3 * $n),
            Seg::Quad { cx: 293.5 + 1.6 * $n, cy: $y + 38.6 + 0.3 * $n, x: 289.5 + 1.6 * $n, y: $y + 38.6 + 0.3 * $n },
            Seg::Line(108.0 - 0.6 * $n, $y + 38.6 + 0.3 * $n),
            Seg::Line(92.9 - 0.6 * $n, $y + 27.0 + 0.3 * $n),
            Seg::Line(92.9 - 0.6 * $n, $y + 4.0 - 2.1 * $n),
            Seg::Quad { cx: 92.9 - 0.6 * $n, cy: $y - 2.1 * $n, x: 97.0 - 0.6 * $n, y: $y - 2.1 * $n },
        ], Ink::Fixed(iced::Color { a: $alpha, ..rgb(0xa97c48) }), 0.7)
    };
}
macro_rules! nav_states {
    ($i:expr, $y:expr, $off:expr, $on:expr) => {
        crate::style::PlateStates {
            group: Group::Category,
            index: $i,
            hover: &[
                nav_echo!($y, 7.0, 0.55),
                nav_echo!($y, 6.0, 0.60),
                nav_echo!($y, 5.0, 0.65),
                nav_echo!($y, 4.0, 0.70),
                nav_echo!($y, 3.0, 0.75),
                nav_echo!($y, 2.0, 0.80),
                nav_echo!($y, 1.0, 0.85),
                Prim::At { x: 0.0, y: 0.0, prims: $off },
            ],
            pressed: $on,
            selected_away: None,
            preserve_selected_hover: true,
            selected_hover: None,
            selected_pressed: None,
        }
    };
}
macro_rules! product_states {
    ($i:expr) => {
        crate::style::PlateStates {
            group: Group::Card,
            index: $i,
            hover: PRODUCT_HOVER,
            pressed: PRODUCT_PRESSED,
            selected_hover: Some(PRODUCT_SELECTED_HOVER),
            selected_pressed: Some(GROWN),
            selected_away: None,
            preserve_selected_hover: false,
        }
    };
}
pub(crate) const STORE_STATES: &[crate::style::PlateStates] = &[
    nav_states!(0, 357.9, NAV_OFF_0, NAV_ON_0),
    nav_states!(1, 418.6, NAV_OFF_1, NAV_ON_1),
    nav_states!(2, 479.3, NAV_OFF_2, NAV_ON_2),
    nav_states!(3, 540.0, NAV_OFF_3, NAV_ON_3),
    nav_states!(4, 600.7, NAV_OFF_4, NAV_ON_4),
    product_states!(0),
    product_states!(1),
    product_states!(2),
    product_states!(3),
];

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
/// The four cards at their columns (:394-514): its own table because
/// `CONTENT` wipes it on under `#shelf-open`.
const SHELF: &[Prim] = &[
    Prim::At { x: 360.8, y: 0.0, prims: SHELF_0 },
    Prim::At { x: 667.1, y: 0.0, prims: SHELF_1 },
    Prim::At { x: 978.8, y: 0.0, prims: SHELF_2 },
    Prim::At { x: 1288.8, y: 0.0, prims: SHELF_3 },
];

pub const STORE: &[Prim] = &[
    // Composited in software: the blue lobe is translucent over the
    // haze and both over the page, and wgpu's linear blend lands the
    // top edge 20 levels dark (design `64 54 83`, painted `40 43 66`).
    // See `screens/soft.rs`.
    Prim::Soft { prims: BACKDROP },
    Prim::At { x: 0.0, y: 0.0, prims: CONTENT },
];

const CONTENT: &[Prim] = &[
    // solid source contours for 4S; the T remains an outline
    Prim::At { x: -28.333, y: -35.0, prims: STORE_SOLID_MARK },
    shut_path(237.5, 68.5, TEE, Ink::Fixed(STORE_LOGO_OUTLINE), 2.0),
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
    Prim::At { x: 360.0, y: 143.0, prims: &store_badges::STORE_A_BADGE },
    Prim::At { x: 1178.0, y: 143.0, prims: &store_badges::STORE_C_BADGE },
    store_metadata(401.0, 148.0, true, false, STORE_METADATA_FIRST),
    store_metadata(401.0, 155.0, true, true, STORE_METADATA_SECOND),
    store_metadata(1012.0, 148.0, true, false, STORE_METADATA_FIRST),
    store_metadata(1012.0, 155.0, true, true, STORE_METADATA_SECOND),
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
    // the shelf, wiped on from the left at boot: `#shelf-open`
    // (:246-252) grows one clip over all four cards from no width to
    // 1240 over 0.5 s from 0, `keySplines="0.33 1 0.68 1"` =
    // EaseOutCubic, and freezes, so the cards arrive in reading order
    // as the hub's cascade does under `#cards-open`; at rest it is the
    // trace's own groups (:393-515 for the lines, :704-755 for the
    // labels -- split for the halo, one animation). The selected
    // card's gold body is not under it: it fades in on its own
    // (`#body-fade`, in `GROWN`).
    Prim::Motion {
        motion: Motion {
            id: "shelf-open",
            begin: 0,
            dur: 500,
            ease: Easing::EaseOutCubic,
            change: Change::Clip { x: 340.0, y: 205.0, w: (0.0, 1240.0), h: (530.0, 530.0) },
        },
        prims: SHELF,
    },
    // foot
    Prim::At { x: 675.0, y: 775.0, prims: &store_badges::STORE_B_BADGE },
    store_metadata(715.0, 780.5, false, false, STORE_METADATA_FIRST),
    store_metadata(715.0, 787.0, false, true, STORE_METADATA_SECOND),
];
// --- end store -----------------------------------------------------------
// --- dashboard -----------------------------------------------------------
//
// `docs/neokitsch/dashboard-trace.svg` (revised 2026-09-07), transcribed
// the way the store block above is: coordinates are the trace's own in
// the 1600x900 frame, elements in the trace's paint order, every `<use>`
// expanded through `Prim::At` at the trace's `x`/`y` or `translate`, so
// a figure here reads against the SVG line it came from. Line numbers
// below are the trace's.
//
// What is not transcribed, and why:
//
//   * the halo (:269, `<use href="#content" filter="url(#halo)"
//     class="photo">`): the photograph's glow, hidden by G2i and never
//     drawn by any screen here (docs/PIPELINE.md).
//   * some `letter-spacing` on text (2 on LEVEL, 0.4 on annotations)
//     remains approximate. Corrected labels use `Prim::Tracked`; the
//     Store metadata uses `TrackedWords` for its additional word spacing,
//     while `S T O R E` uses `Spaced`.
//   * stroke opacity. The onion rings are one hex (`#bd8951` on the
//     cards and panel, `#a97c48` on the T2 badge) at a per-ring
//     `stroke-opacity`, and iced's canvas stroke has none, so each ring
//     gets that hex composited onto its ground -- `PAGE` for the cards
//     and panel, `HAZE_MID` for the badge, which sits in the violet.
//   * the r4 foot fillet on the cards and their rings is an SVG arc
//     (`A 4 4 0 0 1`, :154 and :217-222); `Seg` has no arc, so each is
//     one cubic through the same two endpoints (k = 4/3 tan(135/4 deg)
//     = 0.891, within 0.02 px of the arc). The control points and the
//     EMAIL grain's strand ends (below) are the only figures in this
//     block that are derived rather than copied.
//   * the veneer grain's wander. Both solid gold fills carry it since
//     2026-09-07 (EMAIL's card :503-546, the panel body :619-705): the
//     trace's strands wave, lean into the card's top and swing into its
//     two book-match seams, and `Prim::Grain` paints straight strands
//     (its doc says so). The pitch, width, ink and coverage are the
//     trace's, so the average over each fill is the sampled mean the
//     trace holds itself to. Two more things `Grain` cannot say, and how
//     this block says them instead: the strands run VERTICAL (the
//     trace's `l 0,5` steps; `Grain` paints horizontal strands only),
//     and EMAIL's are clipped to a chamfered, footed silhouette (`Grain`
//     fills a rectangle). The panel body is a rectangle, so its grain
//     is one `Grain` under a `Prim::Turn` of -90 about the body's
//     bottom-left corner, which lays the strands on the body's x pitch
//     from the bottom edge up; EMAIL's is one `vline` per strand with
//     its ends on the silhouette, computed by `email_grain` from the
//     `#ncardsel` geometry. The strands' wander would need a polyline
//     grain prim, or `Grain` with a `wave`; neither exists, and neither
//     is added here (the style vocabulary is another agent's).

/// The run's dashboard ink families, the trace's hex values. None of
/// them is an existing era const (`GOLD_TEXT #e7c686` and `AMBER
/// #fcc474` are each a step off), so they are named here rather than
/// approximated; `MICRO #a97c48` and `CAPTION #d9a877` are reused where
/// the trace samples the same hex.
/// Mid gold: header text, onion rings, captions, the tape, letterbox strokes.
pub const HUB_MID: iced::Color = rgb(0xbd8951);
/// The front outline of every card and of the panel (:407, :485).
pub const HUB_EDGE: iced::Color = rgb(0xe8ab66);
/// The solid gold: EMAIL's card, the panel body, the labels, T2's tab.
pub const HUB_FILL: iced::Color = rgb(0xf2b463);
/// The tab plates on the cards' left edges (:418-424).
pub const HUB_PLATE: iced::Color = rgb(0xfcbe6d);
/// The panel paragraphs' glyph ink (:724), the darkest 5% of the text
/// block in the photo. Until 2026-09-07 this was `#3b2416`, the k-means
/// family of the bars the paragraphs were drawn as, which is the halo
/// blend of glyph and gold rather than the glyph.
pub const HUB_DARK: iced::Color = rgb(0x4b341f);
/// The captions' glyph ink (`#ncaption`, :289): brighter than `HUB_MID`,
/// which was the bars' average of glyph and ground.
pub const HUB_CAPTION: iced::Color = rgb(0xce9754);
/// The panel tape's glyph ink (:742), a step brighter again.
pub const HUB_TAPE: iced::Color = rgb(0xdea45c);
/// The T2 badge's front outline and its "T2" (:304, :307).
pub const BADGE_LIT: iced::Color = rgb(0xe8c186);
/// The interior of the A/B letterboxes where they mask the wire band (:329).
pub const BOX_FILL: iced::Color = rgb(0x4c3f5f);

/// `HUB_MID` at the trace's ring opacities over the original `#0e0a0d`
/// base. The cards' six
/// rings run 0.85 0.73 0.61 0.49 0.37 0.25 outermost to innermost
/// (:362-367); the panel's four run 0.70 0.70 0.55 0.25 (:480-483).
pub const RING_85: iced::Color = rgb(0xa37647);
pub const RING_73: iced::Color = rgb(0x8e673f);
pub const RING_70: iced::Color = rgb(0x89633d);
pub const RING_61: iced::Color = rgb(0x795736);
pub const RING_55: iced::Color = rgb(0x6e5032);
pub const RING_49: iced::Color = rgb(0x64482e);
pub const RING_37: iced::Color = rgb(0x4f3926);
pub const RING_25: iced::Color = rgb(0x3a2a1e);
/// The four native screens share the same clear-patch field. The hub
/// uses the fitted backdrop, including the left lift and blue transition.
const HUB_GROUND: &[Prim] = BACKDROP;

/// The standalone bar is an original design, not one of the four native
/// screens. Keep its previously cited dashboard-era haze unchanged.
/// `docs/neokitsch/bar.svg` uses these legacy colors and geometry.
const BAR_HAZE: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x574568)),
    (0.258, rgb(0x574568)),
    (0.572, rgb(0x3a3853)),
    (0.873, rgb(0x16121a)),
    (1.0, rgb(0x0e0a0d)),
];
const BAR_HAZE_LOBE: &[Prim] =
    &[Prim::Lobe { x: 0.0, y: 0.0, rx: 1030.0, ry: 530.45, stops: BAR_HAZE }];
const BAR_BLUE: &[(f32, iced::Color)] = &[
    (0.60, rgba(0x223350, 0.00)),
    (0.68, rgba(0x223350, 0.85)),
    (0.76, rgba(0x1a2c46, 0.80)),
    (0.84, rgba(0x101d30, 0.00)),
    (1.00, rgba(0x101d30, 0.00)),
];
const BAR_BLUE_LOBE: &[Prim] =
    &[Prim::Lobe { x: 0.0, y: 0.0, rx: 1030.0, ry: 530.45, stops: BAR_BLUE }];
const BAR_BLUE_TURNED: &[Prim] =
    &[Prim::Turn { x: 900.0, y: -120.0, angle: 2.0, prims: BAR_BLUE_LOBE }];
const BAR_GROUND: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(rgb(0x0e0a0d))),
    Prim::Turn { x: 825.0, y: -120.0, angle: 1.3, prims: BAR_HAZE_LOBE },
    Prim::Masked { prims: BAR_BLUE_TURNED, mask: BLUE_MASK },
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
/// (:361-367, the trace's order), the front outline (:407-413) and the
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
/// A card's selected dress, from EMAIL (:489-547): the well silhouette
/// filled AND stroked `#f2b463` 1.2, no rings, the veneer grain clipped
/// to it, and the smaller 4.6x32.1 plate standing 1.25 proud of the
/// edge inside the well (244.75 = 246 - 1.25, 442.3 = 384 + 58.3).
const CARD_SELECTED: &[Prim] = &[
    Prim::Path { x: 0.0, y: 6.5, segs: NCARDSEL, close: true, fill: Some(Ink::Fixed(HUB_FILL)), stroke: Some(Ink::Fixed(HUB_FILL)), width: 1.2 },
    Prim::At { x: 0.0, y: 0.0, prims: &EMAIL_GRAIN },
    Prim::Round { x: -1.25, y: 58.3, w: 4.6, h: 32.1, r: 1.5, fill: Some(Ink::Fixed(HUB_PLATE)), stroke: None, width: 0.0 },
];

/// components.svg #nk-card-hover: lift the six existing rings from
/// HUB_MID to HUB_EDGE and the front to HUB_FILL, with geometry and
/// opacity unchanged. Like the resting rings, preblend over the legacy
/// `#0e0a0d` base to preserve the existing inferred feedback inks.
const RING_BASE: iced::Color = rgb(0x0e0a0d);
const fn hover_ring(alpha: f32) -> Ink {
    Ink::Fixed(iced::Color {
        r: RING_BASE.r + (HUB_EDGE.r - RING_BASE.r) * alpha,
        g: RING_BASE.g + (HUB_EDGE.g - RING_BASE.g) * alpha,
        b: RING_BASE.b + (HUB_EDGE.b - RING_BASE.b) * alpha,
        a: 1.0,
    })
}
const CARD_HOVER: &[Prim] = &[
    line_path(0.0, 28.7, NRING6, hover_ring(0.25), 1.0),
    line_path(0.0, 25.0, NRING5, hover_ring(0.37), 1.0),
    line_path(0.0, 21.3, NRING4, hover_ring(0.49), 1.0),
    line_path(0.0, 17.6, NRING3, hover_ring(0.61), 1.0),
    line_path(0.0, 13.9, NRING2, hover_ring(0.73), 1.0),
    line_path(0.0, 10.2, NRING1, hover_ring(0.85), 1.0),
    shut_path(0.0, 6.5, NCARD, Ink::Fixed(HUB_FILL), 1.2),
    CARD_IDLE[7],
];

macro_rules! module_states {
    ($i:expr, $x:expr, $y:expr) => {
        crate::style::PlateStates {
            group: Group::Module,
            index: $i,
            hover: &[Prim::At { x: $x, y: $y, prims: CARD_HOVER }],
            pressed: &[Prim::At { x: $x, y: $y, prims: CARD_SELECTED }],
            selected_away: None,
            preserve_selected_hover: true,
            selected_hover: None,
            selected_pressed: None,
        }
    };
}
const HUB_STATES: &[crate::style::PlateStates] = &[
    module_states!(0, 246.0, 384.0),
    module_states!(1, 347.0, 284.0),
    module_states!(2, 449.0, 182.0),
    module_states!(3, 624.0, 384.0),
    module_states!(4, 724.0, 284.0),
    module_states!(5, 826.0, 182.0),
];

/// The strand count of [`EMAIL_GRAIN`]: the trace's 42 strands at 2.1
/// (:504-545), plus the two the plate well cuts in two.
const EMAIL_STRANDS: usize = 44;
/// EMAIL's veneer grain (:503-546): 42 vertical strands on the trace's
/// 2.1 pitch, 0.7 wide in `GRAIN_LINE`, each clipped to `#ncardsel`
/// (see the block comment above on why these are `vline`s and not a
/// `Grain`). Card-local, like `CARD_SELECTED`.
static EMAIL_GRAIN: [Prim; EMAIL_STRANDS] = email_grain();
/// Each strand's ends on the `NCARDSEL` silhouette, at local x = 2.1 k:
/// the top edge, or the 45-degree chamfer (`L 90.5,42.5` from (48,0))
/// past x 48; the foot diagonal (`L 0,241.5` from (83.7,325.2), also
/// slope 1) to x 83.7, then the chord of the r4 fillet to (90.5,322.3),
/// which lies inside the arc so no strand leaves the fill. The two
/// strands inside the well (x < 5) break over it, between its slants
/// (`L 5,90.8` from (0,94.2), `L 0,54.2` from (5,57.1)). The trace's
/// `clip-path` does all of this at once; here it is arithmetic, the
/// one place in this block a figure is computed rather than read.
const fn email_grain() -> [Prim; EMAIL_STRANDS] {
    const PITCH: f32 = 2.1;
    const WIDTH: f32 = 0.7;
    const INK: Ink = Ink::Fixed(GRAIN_LINE);
    let mut out = [fill_rect(0.0, 0.0, 0.0, 0.0, INK); EMAIL_STRANDS];
    let mut n = 0;
    let mut k = 1;
    while k <= 42 {
        let x = PITCH * k as f32;
        let top = if x <= 48.0 { 0.0 } else { x - 48.0 };
        let bottom = if x <= 83.7 {
            241.5 + x
        } else {
            325.2 - (x - 83.7) * (2.9 / 6.8)
        };
        if x < 5.0 {
            // the well: its top slant runs (0,54.2) to (5,57.1), its
            // bottom slant (5,90.8) to (0,94.2)
            out[n] = vline(x, top, 54.2 + x * (2.9 / 5.0), INK, WIDTH);
            out[n + 1] = vline(x, 94.2 - x * (3.4 / 5.0), bottom, INK, WIDTH);
            n += 2;
        } else {
            out[n] = vline(x, top, bottom, INK, WIDTH);
            n += 1;
        }
        k += 1;
    }
    out
}
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

/// One line of the caption micro-text, or of the panel's tape: Rajdhani
/// 7.5 at weight 600, `Start`-anchored at `x` on the baseline `y`.
macro_rules! micro {
    ($x:expr, $y:expr, $ink:expr, $s:expr) => {
        Prim::Text { x: $x, y: $y, size: 7.5, ink: Ink::Fixed($ink), face: Face::SemiBold, anchor: Anchor::Start, content: $s }
    };
}
macro_rules! caption {
    ($y:expr, $s:expr) => {
        micro!(0.0, $y, HUB_CAPTION, $s)
    };
}
/// The five-line caption block under a card's foot (`#ncaption`,
/// :289-295): micro-text, 7.5 semibold on a 6.77 pitch from the first
/// baseline at the origin. Bars until 2026-09-07.
const NCAPTION: &[Prim] = &[
    caption!(0.0, "ONLY CC35 CERTIFIED AND"),
    caption!(6.77, "DHSF 5TH CLASS OFFICERS"),
    caption!(13.54, "ARE ALLOWED TO MANIPU-"),
    caption!(20.31, "LATE, ACCESS OR DISABLE"),
    caption!(27.08, "THIS DEVICE."),
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
/// (:479-485): rings innermost first, then the front.
const PANEL_FRAME: &[Prim] = &[
    line_path(64.0, 30.3, NPRING4, Ink::Fixed(RING_25), 1.0),
    line_path(64.0, 30.3, NPRING3, Ink::Fixed(RING_55), 1.0),
    line_path(64.0, 30.3, NPRING2, Ink::Fixed(RING_70), 1.0),
    line_path(64.0, 30.3, NPRING1, Ink::Fixed(RING_70), 1.0),
    shut_path(0.0, 37.8, NPANEL, Ink::Fixed(HUB_EDGE), 1.2),
];

// The badge's ink-only silhouette is in the existing header Soft group;
// text remains native alongside the other security-level labels.
const T2_BADGE: &[Prim] = &[
    Prim::Tracked { x: 1294.2, y: 69.6, size: 12.5, tracking: 0.3, ink: Ink::Fixed(CAPTION), face: Face::Regular, anchor: Anchor::Start, content: "LEVEL" },
    Prim::Wide { x: 1296.0, y: 91.6, size: 21.0, stretch: 1.45, ink: Ink::Fixed(BADGE_LIT), face: Face::SemiBold, anchor: Anchor::Start, content: "T2" },
];

/// Dashboard source #69 header strand; mailbox #71 uses the separately
/// measured variant of the same local geometry helper above.
macro_rules! wire {
    ($i:expr) => {
        line_path(35.5, 139.45 + 3.18 * $i as f32,
            &header_wire_steps($i, false), header_wire_ink($i, false), 1.1)
    };
}

// The measured fading strands overlap a varying haze. Composite them
// together with that ground in sRGB, as the SVG does. Drawing alpha over
// the already-uploaded backdrop instead blends in linear light, noticeably
// lifting the lower strands and changing their material. These wrappers
// leave the shared bar/store/login ground tables unchanged.
const HUB_HEADER_BACKDROP: &[Prim] = &[
    Prim::At { x: 0.0, y: 0.0, prims: HUB_GROUND },
    wire!(0),
    wire!(1),
    wire!(2),
    wire!(3),
    wire!(4),
    wire!(5),
    wire!(6),
    wire!(7),
    wire!(8),
    Prim::At { x: 0.0, y: 0.0, prims: badge::DASHBOARD },
];
const MAIL_HEADER_GROUND: &[Prim] = &[
    Prim::At { x: 0.0, y: 0.0, prims: BACKDROP },
    line_path(34.7, 139.70, &WIRE0, header_wire_ink(0, true), 1.1),
    line_path(34.7, 142.88, &WIRE1, header_wire_ink(1, true), 1.1),
    line_path(34.7, 146.06, &WIRE2, header_wire_ink(2, true), 1.1),
    line_path(34.7, 149.24, &WIRE3, header_wire_ink(3, true), 1.1),
    line_path(34.7, 152.42, &WIRE4, header_wire_ink(4, true), 1.1),
    line_path(34.7, 155.60, &WIRE5, header_wire_ink(5, true), 1.1),
    line_path(34.7, 158.78, &WIRE6, header_wire_ink(6, true), 1.1),
    line_path(34.7, 161.96, &WIRE7, header_wire_ink(7, true), 1.1),
    line_path(34.7, 165.14, &WIRE8, header_wire_ink(8, true), 1.1),
    line_path(34.7, 168.32, &WIRE9, header_wire_ink(9, true), 1.1),
    line_path(34.7, 185.50, &WIRE10, header_wire_ink(10, true), 1.1),
    Prim::At { x: 0.0, y: 0.0, prims: badge::MAILBOX },
];
const MAIL_HEADER_BACKDROP: &[Prim] = &[Prim::Soft { prims: MAIL_HEADER_GROUND }];

#[path = "neokitsch_dashboard_badges.rs"]
mod dashboard_badges;

/// The six cascade cards, their labels and captions (:356-432): its own
/// table because `DASHBOARD` wipes it on under `#cards-open`.
const CASCADE: &[Prim] = &[
    // ==== the six cascade cards (:356-432) ====
    // in the trace's reading order, at the `<use>` positions of :408-412
    // and :431; the trace paints all rings, then all fronts, then all
    // plates, which is the same picture since no two cards overlap
    module!(0, 246.0, 384.0),
    module!(1, 347.0, 284.0),
    module!(2, 449.0, 182.0),
    module!(3, 624.0, 384.0),
    module!(4, 724.0, 284.0),
    module!(5, 826.0, 182.0),
    // labels (:441-449), right-anchored beside each card
    Prim::Tracked { x: 237.0, y: 466.7, size: 17.0, tracking: 0.69, ink: Ink::Fixed(HUB_FILL), face: Face::Regular, anchor: Anchor::End, content: "EMAIL" },
    Prim::Tracked { x: 337.375, y: 366.3, size: 17.0, tracking: 0.77, ink: Ink::Fixed(HUB_FILL), face: Face::Regular, anchor: Anchor::End, content: "MATRIX" },
    Prim::Tracked { x: 439.375, y: 264.6, size: 17.0, tracking: 0.74, ink: Ink::Fixed(HUB_FILL), face: Face::Regular, anchor: Anchor::End, content: "BRAINDANCE" },
    Prim::Tracked { x: 614.375, y: 466.7, size: 17.0, tracking: 0.68, ink: Ink::Fixed(HUB_FILL), face: Face::Regular, anchor: Anchor::End, content: "PRIVATE" },
    Prim::Tracked { x: 714.625, y: 356.3, size: 17.0, tracking: 0.67, ink: Ink::Fixed(HUB_FILL), face: Face::Regular, anchor: Anchor::End, content: "SECURITY" },
    Prim::Tracked { x: 714.625, y: 377.9, size: 17.0, tracking: 0.76, ink: Ink::Fixed(HUB_FILL), face: Face::Regular, anchor: Anchor::End, content: "SYSTEMS" },
    Prim::Tracked { x: 816.792, y: 264.6, size: 17.0, tracking: 0.63, ink: Ink::Fixed(HUB_FILL), face: Face::Regular, anchor: Anchor::End, content: "DEVICES" },
    // captions under each foot (:571-576), at the first line's text
    // origin: 4.2..5.2 in from the card's left edge, 347.2 below its top
    Prim::At { x: 250.7, y: 731.25, prims: NCAPTION },
    Prim::At { x: 351.2, y: 630.8, prims: NCAPTION },
    Prim::At { x: 453.2, y: 529.6, prims: NCAPTION },
    Prim::At { x: 628.2, y: 731.25, prims: NCAPTION },
    Prim::At { x: 728.7, y: 630.8, prims: NCAPTION },
    Prim::At { x: 830.75, y: 529.2, prims: NCAPTION },
];

// Exact SVG grain paths retain the existing panel-fade animation. The
// scene cannot fade a Soft composite; ordinary paths remain foreground
// artwork and the persistent viewport applies the SVG's #panelclip.
#[path = "neokitsch/panel_grain.rs"]
mod panel_grain;

// Iced pastes clipped meshes underneath the frame's pending geometry.
// Keep the base and grain in the same clipped layer, or the un-clipped
// base would cover every strand when the parent frame is finished.
const PANEL_BODY: &[Prim] = &[
    fill_rect(1170.8, 326.0, 230.4, 309.0, Ink::Fixed(HUB_PLATE)),
    Prim::At { x: 0.0, y: 0.0, prims: panel_grain::PATHS },
];

/// The panel's paragraph text: Rajdhani 16.3 regular, `Start`-anchored
/// at the trace's x 1180.4 so the L's stem lands on the measured ink
/// left, in `HUB_DARK`.
macro_rules! para {
    ($y:expr, $s:expr) => {
        txt(1180.4, $y, 16.3, Ink::Fixed(HUB_DARK), $s)
    };
}
/// Literal source #69 detail-panel text (NK-06), six lines and two.
/// The photographed hub and mailbox deliberately carry different copy;
/// do not silently substitute `PARAGRAPHS` into this source reference.
const PANEL_COPY: [&str; 8] = [
    "Lorem ipsum dolor sit amet,",
    "consectetur adipiscing elit, sed",
    "do eiusmod tempor incididunt",
    "ut labore et dolore magna",
    "aliqua. Quis ipsum suspendisse",
    "ultrices gravida.",
    "Risus commodo viverra maece-",
    "nas accumsan lacus vel facilisis.",
];

/// The detail panel (:460-748): the rings and front outline, the gold
/// body and its grain, the paragraphs, the tape and the module name.
/// Its own table because `DASHBOARD` fades it in under `#panel-fade`.
const PANEL: &[Prim] = &[
    // ==== the detail panel (:460-748) ====
    Prim::At { x: 1170.8, y: 259.7, prims: PANEL_FRAME },
    // the body (:612): `HUB_PLATE`, not `HUB_FILL`, because `#f2b463`
    // is the body's sampled AVERAGE and the grain takes 26% of it in
    // `GRAIN_LINE`; `#fcbe6d` under that averages back to it
    Prim::Viewport { x: 1170.8, y: 326.0, w: 230.4, h: 309.0, prims: PANEL_BODY },
    // two paragraphs on the 19.5 pitch from the first baseline 354.2,
    // one slot blank between them (:725-732)
    para!(354.2, PANEL_COPY[0]),
    para!(373.7, PANEL_COPY[1]),
    para!(393.2, PANEL_COPY[2]),
    para!(412.7, PANEL_COPY[3]),
    para!(432.2, PANEL_COPY[4]),
    para!(451.7, PANEL_COPY[5]),
    para!(491.2, PANEL_COPY[6]),
    para!(510.7, PANEL_COPY[7]),
    // the micro-text tape (:743-744), the caption sentence on two
    // lines, and the module name (:747)
    micro!(1193.7, 645.8, HUB_TAPE, "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE"),
    micro!(1193.7, 652.1, HUB_TAPE, "ALLOWED TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE."),
    Prim::Text { x: 1286.7, y: 692.0, size: 20.0, ink: Ink::Fixed(HUB_FILL), face: Face::SemiBold, anchor: Anchor::Middle, content: "EMAIL" },
];

pub const DASHBOARD: &[Prim] = &[
    // Composited in software, as the store's backdrop is: the two
    // lobes carry opacities and stack.
    Prim::Soft { prims: HUB_HEADER_BACKDROP },
    // ==== header (:271-308) ====
    // Iced native cap alignment: one pixel above the SVG baseline at 4K.
    Prim::Text { x: 118.3, y: 41.7833, size: 13.0, ink: Ink::Fixed(HUB_MID),
        face: Face::SemiBold, anchor: Anchor::Start, content: "CUSTOMER #NC488402" },
    Prim::Wide { x: 118.7667, y: 69.6, size: 12.0, stretch: 1.6, ink: Ink::Fixed(HUB_MID), face: Face::Regular, anchor: Anchor::Start, content: "LEVEL" },
    Prim::Wide { x: 131.5, y: 92.1, size: 21.0, stretch: 1.45, ink: Ink::Fixed(HUB_MID), face: Face::SemiBold, anchor: Anchor::Start, content: "T1" },
    Prim::Tracked { x: 1137.7, y: 71.4, size: 12.0, tracking: 0.75, ink: Ink::Fixed(HUB_MID), face: Face::Regular, anchor: Anchor::Start, content: "SECURITY" },
    Prim::Tracked { x: 1137.7, y: 87.2, size: 12.0, tracking: 0.75, ink: Ink::Fixed(HUB_MID), face: Face::Regular, anchor: Anchor::Start, content: "LEVEL" },
    Prim::Tracked { x: 1232.75, y: 68.8, size: 12.0, tracking: 0.75, ink: Ink::Fixed(HUB_MID), face: Face::Regular, anchor: Anchor::Start, content: "LEVEL" },
    Prim::Tracked { x: 1354.8, y: 69.6, size: 12.0, tracking: 0.75, ink: Ink::Fixed(HUB_MID), face: Face::Regular, anchor: Anchor::Start, content: "LEVEL" },
    Prim::Tracked { x: 1415.75, y: 69.6, size: 12.0, tracking: 0.75, ink: Ink::Fixed(HUB_MID), face: Face::Regular, anchor: Anchor::Start, content: "LEVEL" },
    Prim::Wide { x: 1238.1, y: 90.9, size: 20.0, stretch: 1.45, ink: Ink::Fixed(HUB_MID), face: Face::SemiBold, anchor: Anchor::Start, content: "T1" },
    Prim::Wide { x: 1356.8, y: 90.9, size: 20.0, stretch: 1.45, ink: Ink::Fixed(HUB_MID), face: Face::SemiBold, anchor: Anchor::Start, content: "T3" },
    Prim::Wide { x: 1417.75, y: 91.8, size: 20.0, stretch: 1.45, ink: Ink::Fixed(HUB_MID), face: Face::SemiBold, anchor: Anchor::Start, content: "T4" },
    Prim::At { x: 0.0, y: 0.0, prims: T2_BADGE },
    // Source-fitted section badges. A/B retain their existing wire masks;
    // backing color/profile and photographic softness remain separate work.
    Prim::Round { x: 238.0, y: 98.0, w: 26.0, h: 26.0, r: 3.0, fill: Some(Ink::Fixed(BOX_FILL)), stroke: None, width: 0.0 },
    Prim::Round { x: 1011.0, y: 98.0, w: 26.0, h: 26.0, r: 3.0, fill: Some(Ink::Fixed(BOX_FILL)), stroke: None, width: 0.0 },
    Prim::At { x: 0.0, y: 0.0, prims: dashboard_badges::DASH_HEADER_A },
    Prim::At { x: 0.0, y: 0.0, prims: dashboard_badges::DASH_HEADER_B },
    Prim::At { x: 574.58333, y: 821.66667, prims: dashboard_badges::HUB_FOOT_C_ART },
    Prim::At { x: 1170.41667, y: 821.66667, prims: dashboard_badges::HUB_FOOT_D_ART },
    txt(288.0, 106.0, 8.0, Ink::Fixed(MICRO), "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    txt(288.0, 116.0, 8.0, Ink::Fixed(MICRO), "SERVING CUSTOMERS SINCE 2006."),
    txt_end(1000.0, 106.0, 8.0, Ink::Fixed(MICRO), "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    txt_end(1000.0, 116.0, 8.0, Ink::Fixed(MICRO), "SERVING CUSTOMERS SINCE 2006."),
    txt(618.0, 827.6, 7.5, Ink::Fixed(MICRO), "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO."),
    txt(618.0, 834.0, 7.5, Ink::Fixed(MICRO), "SERVING CUSTOMERS SINCE 2006."),
    txt(1205.5, 827.6, 7.5, Ink::Fixed(MICRO), "MAPS ARE PROVIDED BY SEOCHO. SATELITE SERVICES"),
    txt(1205.5, 834.0, 7.5, Ink::Fixed(MICRO), "SINCE 2006."),
    // the six cascade cards, wiped on from the left at boot: `#cards-open`
    // (:256-262) grows the block's clip from no width to 760 over 0.5 s
    // from 0, `keySplines="0.33 1 0.68 1"` = EaseOutCubic, and freezes;
    // at rest it is the trace's own group (:355-433)
    Prim::Motion {
        motion: Motion {
            id: "cards-open",
            begin: 0,
            dur: 500,
            ease: Easing::EaseOutCubic,
            change: Change::Clip { x: 180.0, y: 160.0, w: (0.0, 760.0), h: (630.0, 630.0) },
        },
        prims: CASCADE,
    },
    // the detail panel, faded in after the cascade: `#panel-fade`
    // (:471-473) takes the group's opacity from 0 to 1 over 0.3 s from
    // 0.4 s, `keySplines="0.61 1 0.88 1"` = EaseOut, and freezes; the
    // `<set>` under it (:478) is the hold at 0 until then, which
    // `Motion::begin` already is
    Prim::Motion {
        motion: Motion {
            id: "panel-fade",
            begin: 400,
            dur: 300,
            ease: Easing::EaseOut,
            change: Change::Opacity { alpha: (0.0, 1.0) },
        },
        prims: PANEL,
    },
];
// --- end dashboard -------------------------------------------------------

#[cfg(test)]
mod dashboard_tests {
    use super::PANEL_COPY;

    #[test]
    fn store_categories_echo_outward_without_changing_faces_or_selection_material() {
        use super::*;
        let categories: Vec<_> = CONTENT.iter().filter_map(|prim| match prim {
            Prim::Plate { group: Group::Category, index, y, on, off, .. } => Some((*index, *y, *on, *off)),
            _ => None,
        }).collect();
        let states: Vec<_> = STORE_STATES.iter().filter(|s| s.group == Group::Category).collect();
        assert_eq!(states.len(), categories.len());
        for state in states {
            let (_, top, on, off) = categories.iter().find(|(i, ..)| *i == state.index).unwrap();
            assert_eq!(state.group, Group::Category);
            assert_eq!(state.pressed, *on, "press preserves existing grain, tab and dark label");
            assert!(state.preserve_selected_hover);
            assert_eq!(state.hover.len(), 8);
            assert_eq!(state.hover[7], Prim::At { x: 0.0, y: 0.0, prims: off });
            for (i, ring) in state.hover[..7].iter().enumerate() {
                let n = (7 - i) as f32;
                let Prim::Path { x, y, segs, stroke: Some(Ink::Fixed(ink)), width, fill, .. } = ring else {
                    panic!("hover ring must be an unfilled outline");
                };
                assert!(fill.is_none());
                assert_eq!(*width, 0.7);
                assert_eq!((ink.r, ink.g, ink.b), (rgb(0xa97c48).r, rgb(0xa97c48).g, rgb(0xa97c48).b));
                assert!((ink.a - (0.55 + 0.05 * i as f32)).abs() < 0.00001);
                assert_eq!((*x, *y), (97.0 - 0.6 * n, top - 2.1 * n));
                assert_eq!(segs[0], Seg::Line(289.5 + 1.6 * n, top - 2.1 * n));
                assert_eq!(segs[4], Seg::Line(108.0 - 0.6 * n, top + 38.6 + 0.3 * n));
                assert_eq!(segs[5], Seg::Line(92.9 - 0.6 * n, top + 27.0 + 0.3 * n));
            }
        }
    }

    #[test]
    fn product_feedback_keeps_compact_content_and_selected_growth_separate() {
        use super::*;
        let states: Vec<_> = STORE_STATES.iter().filter(|s| s.group == Group::Card).collect();
        assert_eq!(states.len(), 4);
        for (i, state) in states.iter().enumerate() {
            assert_eq!(state.index, i);
            assert_eq!(state.hover[7], Prim::At { x: 0.0, y: 0.0, prims: CARD });
            assert_eq!(state.selected_hover.unwrap()[7], Prim::At { x: 0.0, y: 0.0, prims: GROWN });
            assert_eq!(state.selected_pressed, Some(GROWN));
            assert_eq!(state.pressed, PRODUCT_PRESSED);
        }
        assert_eq!(PRODUCT_PRESSED.len(), CARD.len() + 2);
        // Same compact content/positions, with only ink and QR polarity
        // changed inside the new veneer strip. No expanded-only text.
        for (i, (rest, held)) in CARD.iter().zip(&PRODUCT_PRESSED[2..]).enumerate() {
            let mut expected = *rest;
            if (8..=23).contains(&i) {
                match &mut expected {
                    Prim::Text { ink, .. } => *ink = Ink::OnSelect,
                    Prim::Rect { fill, .. } => *fill = Some(Ink::OnSelect),
                    Prim::At { prims, .. } => *prims = QR_DARK,
                    _ => panic!("unexpected compact veneer content"),
                }
            }
            assert_eq!(*held, expected);
        }
        let Prim::Grain { y, h, pitch, ink, .. } = PRODUCT_PRESSED[1] else { panic!("veneer needs grain") };
        assert_eq!((y, h, pitch, ink), (492.9, 88.45, 2.4, Ink::Fixed(GRAIN_LINE)));
        for (hover, top, bottom) in [(PRODUCT_HOVER, 314.6, 640.4), (PRODUCT_SELECTED_HOVER, 232.9, 711.3)] {
            for (i, prim) in hover[..7].iter().enumerate() {
                let n = (7 - i) as f32;
                let Prim::Path { x, segs, stroke: Some(Ink::Fixed(ink)), width, fill, .. } = prim else { panic!("echo needs outline") };
                assert!(fill.is_none());
                assert_eq!(*width, 0.7);
                assert_eq!(*x, -0.6 * n);
                assert_eq!(segs[3], Seg::Cubic {
                    c1x: 151.2, c1y: top + 30.4 - 2.1 * n,
                    c2x: 167.2, c2y: top - 2.1 * n,
                    x: 182.2, y: top - 2.1 * n,
                });
                assert_eq!(segs[6], Seg::Line(262.1 + 1.6 * n, bottom - 8.0 + 0.3 * n));
                assert!((ink.a - (0.55 + 0.05 * i as f32)).abs() < 0.00001);
            }
        }
    }

    #[test]
    fn hover_preserves_card_geometry_and_press_reuses_veneer_at_each_origin() {
        use super::*;
        for (idle, hover) in CARD_IDLE.iter().zip(CARD_HOVER) {
            match (*idle, *hover) {
                (Prim::Path { stroke: Some(_), .. }, Prim::Path { stroke: Some(ink), .. }) => {
                    let mut expected = *idle;
                    if let Prim::Path { stroke, .. } = &mut expected { *stroke = Some(ink); }
                    assert_eq!(expected, *hover);
                }
                _ => assert_eq!(idle, hover),
            }
        }
        let mut centres = Vec::new();
        crate::screens::scene::plates(DASHBOARD, 0.0, 0.0, &mut centres);
        assert_eq!(HUB_STATES.len(), centres.len());
        for state in HUB_STATES {
            let (_, _, centre) = centres.iter().find(|(g, i, _)| (*g, *i) == (state.group, state.index)).unwrap();
            for (drawing, face) in [(state.hover, CARD_HOVER), (state.pressed, CARD_SELECTED)] {
                let [Prim::At { x, y, prims }] = drawing else { panic!("card origin missing") };
                assert_eq!((*x + 90.5 / 2.0, *y + 327.0 / 2.0), (centre.x, centre.y));
                assert_eq!(*prims, face);
            }
        }
    }

    /// Source #69 has6+2 lines and a visibly hyphenated last paragraph;
    /// the mailbox's different copy must not become the hub reference.
    #[test]
    fn panel_copy_preserves_source_words_and_line_breaks() {
        assert_eq!(PANEL_COPY[1], "consectetur adipiscing elit, sed");
        assert_eq!(&PANEL_COPY[4..], &[
            "aliqua. Quis ipsum suspendisse", "ultrices gravida.",
            "Risus commodo viverra maece-", "nas accumsan lacus vel facilisis.",
        ]);
        assert!(!PANEL_COPY.iter().any(|line| line.contains("Ut enim")));
    }
}
