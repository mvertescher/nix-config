//! Entropism -- "necessity over style".
//!
//! One hue. Sage green on a warm dark olive-brown ground, square
//! everything, no glow. Selection is a solid sage fill. Sampled from
//! Behance Part 1, gallery positions 34-42 (title card 33) per
//! `docs/sources.md`; the "doc #24-32" this comment used to give came
//! from an earlier, smaller scrape and is shifted by ten.
//!
//! Strokes: the traces measure 1.25px on login, dashboard and store
//! (`docs/entropism/login-trace.svg` header rect at 49,43 1498x26,
//! `stroke-width="1.25"`; `dashboard-trace.svg` same rect;
//! `store-trace.svg` card frame 265x237) and 2px on the mailbox
//! (`mailbox-trace.svg` `#hdr-chrome` / `#a-chrome`, `stroke-width="2"`).
//! `metrics.stroke` below is still 1.0 -- the canvas arms pass their
//! widths explicitly, so the metric only reaches a widget that inherits
//! it (surface, menu, chrome, bracket, ornament), and those are bar /
//! dashboard consumers; see `ERAS-DELTA.md`.
//!
//! Ground: the native photos have a broad upper light field, fitted from
//! clear patches in `docs/entropism/ground-fit.md`. Hub, mail and store
//! share one field; login has a dimmer one. `Ground::Flat` remains the
//! fallback for surfaces and the original bar composition.
//!
//! The predecessor crate (`entropism-ui`) carried twelve colours --
//! including cybr's red, cyan, mint, violet, orange and gold -- and a
//! radial glow module. None of that is in the reference; see
//! `docs/entropism/README.md`. This is the one-hue system.

use crate::palette::{rgb, Ornaments, Palette};
use crate::style::{
    Banner, Bar, BarChrome, BarGround, BarMenu, BarOrnament, Chrome, Coat, Compliance, ControlStates, Controls,
    Corner, Destination, Dress,
    Era, Face, Footnotes, Ground, Ink, MenuMarker, MenuRule, Metrics, Nameplate,
    PanelEcho, Selection, Style, Ticket, WindowLabel,
};
use crate::widgets::surface::Corners;
// --- login ---
use crate::style::{
    Access, Blink, Caret, Colophon, Entry, Fixture, Legend, Masthead, Plate, Plot, Slot,
};
// --- end login ---

pub const BG: iced::Color = rgb(0x110c07);
/// The selection fill and the footer band: the value every trace's
/// k-means gives the solid sage (store #a8d4a2, mailbox #a6d2a8, hub
/// #a6d3a7). #9cb795 until 2026-09-05, when `border` took the frames'
/// bright #8fba97 and the two were 13 levels apart: a selected row no
/// longer stood off its own outline, and the G2i extractor merged the
/// families (store 32/32 -> 19/32). A fill does not dilute in the 1600
/// rescale the way a line does, so the traces' number is the photo's.
pub const SAGE_SOLID: iced::Color = rgb(0xa6d3a7);
pub const SAGE_TEXT: iced::Color = rgb(0x94bb94);
pub const MID: iced::Color = rgb(0x728f76);
/// The frames' ink: the 1.25px core of every outlined box on the hub
/// and store sheets at full resolution (hub trace header "Stroke
/// profile"). #5d7752 until 2026-09-05, a stop darker than any frame
/// in the material.
pub const OUTLINE: iced::Color = rgb(0x8fba97);
pub const DIM: iced::Color = rgb(0x3d4d38);
pub const ON_SOLID: iced::Color = rgb(0x1f2a1c);

pub fn palette() -> Palette {
    Palette {
        bg: BG,
        // No lifted panel in the reference: surfaces are outlines on the
        // page ground, so panel and bg are the same colour on purpose.
        panel: BG,
        border: OUTLINE,
        dim: DIM,
        fg: SAGE_TEXT,
        // The one place the reference departs from monochrome is the
        // "(!)" urgency marker, and even that is the same sage. Alert
        // reads as mid rather than inventing a second hue.
        alert: MID,
        tape: MID,
        select: SAGE_SOLID,
        on_select: ON_SOLID,
        emphasis: None,
        // No band anywhere, so nothing to restate for the selected
        // state either; `banner()` degrades to a tape label and
        // `banner_on_select()` swaps it.
        banner_selected: None,
        // A minimalist era declares no ornament: the vocabulary is
        // additive and nothing here wants it.
        ornaments: Ornaments::default(),
        cta: SAGE_SOLID,
        bloom: BG,
    }
}

/// The published reference palette differs from the standalone palette
/// only in its derived panel role (`home/themes/entropism/palettes.nix`).
/// Exact matching confines photographed access inks to those two palettes.
pub fn access_reference_palette(actual: &Palette) -> bool {
    let built_in = palette();
    let mut published = built_in;
    published.panel = rgb(0x181109);
    *actual == built_in || *actual == published
}

pub fn style() -> Style {
    Style {
        era: Era::Entropism,
        palette: palette(),
        corner: Corner::Square,
        selection: Selection::Solid,
        ground: Ground::Flat,
        chrome: Chrome::Segmented,
        nameplate: Nameplate::Header,
        // --- bar --- (docs/entropism/bar.svg, IMPLEMENTATION DELTA)
        //
        // The bar is not a row of cells at all: it is the era's header
        // strip, one 2px outlined frame from x 6 to 1594 cut by
        // dividers on every module boundary, exactly as
        // mailbox-trace.svg's `#hdr-chrome` cuts its own. Nothing is
        // rounded, nothing floats, and alarm is a word rather than an
        // ink -- "necessity over style", drawn.
        bar: Bar {
            height: 31,
            host_tape: true,

            pad_left: 6.0,
            pad_right: 6.0,
            pad_y: 3.0,
            // Segments of one frame, so no air anywhere between them.
            gap: 0.0,
            ws_gap: 0.0,
            ws_lead: 0.0,
            // A run of near-square numbered cells, like the boxed
            // [A] [B] [C] of mailbox-trace at 26x26.
            ws_width: 28.0,
            ws_corners: None,
            // The header strings start 12px inside their segment
            // (mailbox/login x 61 in a frame at 49).
            pad_x: 12.0,
            trail: 12.0,
            em: 0.58,
            // The design sized its cells by counting characters flat.
            space_em: 0.58,
            alert_track: 0.0,
            // mailbox-trace measures the designed stroke of every
            // outlined frame at 2px, not the era's screen-wide 1.
            stroke: 2.0,
            icon_pad: 18.0,
            label_left: false,
            face: Face::Regular,
            tape_extra: 0.0,
            tape_ticks: false,

            ground: BarGround::Plain,
            chrome: BarChrome::Frame,
            ornament: BarOrnament::None,

            // Idle is the ground showing through the frame: a segment
            // has no outline of its own, only the dividers either side.
            idle: Dress {
                corners: Corners::square(),
                fill: Ink::None,
                stroke: Ink::None,
                ink: Ink::Fg,
                tab: false,
                step: None,
            },
            selected: Dress {
                fill: Ink::Select,
                ink: Ink::OnSelect,
                ..Dress::default()
            },
            // No stroke and no ink moves: see `alert_suffix`.
            alert: Dress::default(),
            // The login band's dark-on-sage, in the dimmer of the two
            // sage fills so it does not read as a selection next to
            // workspace 3.
            tape: Dress {
                fill: Ink::Tape,
                ink: Ink::OnSelect,
                ..Dress::default()
            },
            tab: None,
            // The long open centre string, left-aligned after the left
            // run's closing divider exactly as STORE ACCESS SCREEN is.
            window: WindowLabel {
                dress: None,
                ink: Ink::Tape,
                leading: true,
                pad_x: 12.0,
                stroke: None,
                face: None,
            },

            // The only urgency mark in the material is a literal
            // " (!)" suffix in the same ink as its neighbours
            // (mailbox-trace "URGENT INFORMATION (!)").
            alert_suffix: Some(" (!)"),
            bold_tiers: false,
            clock_plain: None,

            menu: BarMenu {
                panel: Dress {
                    fill: Ink::Bg,
                    stroke: Ink::Border,
                    ..Dress::default()
                },
                // Rows start at the frame's inner edge; the only air
                // is the stroke's own half.
                air: 1.0,
                side: 1.0,
                // 24px rows: 14px text with 5px above and below.
                row_air: 2.0,
                row_side: 12.0,
                icon_col: 16.0,
                icon_gap: 8.0,
                level_gap: 0.0,
                level_pad: 24.0,
                // store-trace's nav and mailbox-trace's list both
                // separate every row from the next with a 2px rule.
                row_divider: true,
                // An 8px EMPTY CELL between two dividers, not a
                // floating rule -- the era has no floating rules.
                rule: MenuRule::Empty { height: 8.0 },
                row: Dress {
                    fill: Ink::Select,
                    ink: Ink::OnSelect,
                    ..Dress::default()
                },
                open: Dress {
                    fill: Ink::Select,
                    ink: Ink::OnSelect,
                    ..Dress::default()
                },
                open_inset: (0.0, 0.0),
                row_split: None,
                // The quietest legible ink in the material is the
                // dashboard caption strip's; DIM is the faint-rule
                // tone and is not legible at 14px. Was `Ink::Border`
                // while that was the dim #5d7752; since `border` took
                // the frames' bright sage (2026-09-05) the quiet ink
                // is MID, and this follows the quietness, not the role.
                disabled: Ink::Mid,
                rule_ink: Ink::Border,
                row_inset: (0.0, 0.0),
                row_overshoot: 0.0,
                spine: 0.0,
                foot: 0.0,
                marker: MenuMarker::Text,
                echo: PanelEcho::None,
            },
        },
        banner: Banner::default(),
        // A and B under the nav, and a dead lower third the reference
        // is content with.
        footnotes: Footnotes::UnderNav,
        // The reference sets it inside the outline, under the sockets
        // of every unselected card.
        compliance: Compliance::Inside,
        // No wedge: entropism cuts nothing, anywhere.
        ticket: Ticket::default(),
        glyphs: false,
        // --- controls --- (components.svg BUTTON ROW, LOGIN FORM)
        controls: Controls {
            // components.svg band C: reverse video on hover, outline
            // while held. Resolve through the same desktop roles as
            // the existing rest coats (the field colours are derived).
            primary_states: ControlStates {
                hover: Some(Coat::filled(Ink::Cta, Ink::OnSelect)),
                pressed: Some(Coat::outlined(Ink::Border, 2.0, Ink::Fg)),
            },
            ghost_states: ControlStates {
                hover: Some(Coat::filled(Ink::Cta, Ink::OnSelect)),
                pressed: Some(Coat::outlined(Ink::Border, 2.0, Ink::Fg)),
            },
            field_states: ControlStates {
                hover: Some(Coat::filled(Ink::Cta, Ink::OnSelect)),
                pressed: Some(Coat::outlined(Ink::Border, 1.25, Ink::Fg)),
            },
            // REPORT SPAM and NEXT: the solid sage, dark ink on it.
            primary: Coat::filled(Ink::Cta, Ink::OnSelect),
            // REPLY / FORWARD / DELETE: the bare strip, stroke 2, same
            // readings as `mailbox.buttons`.
            ghost: Coat::outlined(Ink::Border, 2.0, Ink::Fg),
            // No disabled control anywhere in the era; the outline one
            // stop down.
            disabled: Coat::outlined(Ink::Dim, 1.25, Ink::Dim),
            // The USERNAME field: outlined 1.25, no fill, bold value.
            field: Coat::outlined(Ink::Border, 1.25, Ink::Fg),
            placeholder: Ink::Dim,
            radius: 0.0,
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
        // Entropism is the era that grows its *first* card; the
        // other three grow their second.
        store_selection: (1, 0),
        store_cursor: Some(Group::Category),
        store_states: STORE_STATES,
        // --- end store ---
        // --- dashboard ---
        dashboard: DASHBOARD,
        dashboard_reference_fg: None,
        store_reference_fg: None,
        store_reference: None,
        // BRAINDANCE, row 1 tile 3: the one tile dashboard-trace.svg
        // fills solid (`<rect x="716" y="227" ... fill="#a6d3a7"/>`
        // under "the selection: solid sage fill, dark caption box and
        // text").
        dashboard_selection: 2,
        dashboard_cursor: true,
        mailbox_cursor: true,
        dashboard_states: &[],
        dashboard_held_backdrops: &[],
        // EMAILS is tile 0; nothing on this hub says "store", and the
        // store is `s` from the hub instead (`screens::hub`).
        dashboard_destinations: [Some(Destination::Mail), None, None, None, None, None],
        // --- end dashboard ---
        metrics: Metrics {
            // Traces measure 1.25 (login/dashboard/store) and 2.0
            // (mailbox); see the module doc. Left at 1.0 because the
            // only readers are bar/dashboard widgets and the fold is
            // undecided -- ERAS-DELTA.md.
            stroke: 1.0,
            gap: 14.0,
            pad: 14.0,
            ..Metrics::default()
        },
    }
}

// --- login ---
//
// The access screen, transcribed from `docs/entropism/login-trace.svg`
// at 1600x900. The trace's own summary of the screen is "sparse by
// design: one big fill, one small fill, two outline boxes and the
// header strip", and that is the whole table below -- there is no
// margin marker, no badge and no caption anywhere in the photo, so
// there is none here either.
//
// The one thing worth flagging because it looks like a mistake: the
// login screen carries the caption STORE ACCESS SCREEN in its middle
// header cell. The trace records that as material, not an error.
//
// Colours are the era's published roles rather than the trace's spot
// samples, so the theme still reaches the screen. The pairs, for the
// record: band and button `select`/`cta` #a6d3a7 against the trace's
// #8aac8c, outlines `border` #8fba97 against #739479, text `fg`
// #94bb94 against #8aac8c, dark ink on the band `on_select` #1f2a1c
// against #20281c.
//
// From 2026-09-03 to 2026-09-05 the four were `Ink::Fixed` at the
// values a full-resolution probe of this photo gave: line #75967b (a
// 3px line with a peak core of #6c8a77..#819b82, no dark ring), band
// and button #8aac8c, ink on the band #20281c, header strings
// #799d81. This screen photographs a stop under the hub, mail and
// store sheets the palette was sampled from, and the fixed values
// bought a G2i point (on the published `select` the k-means spends a
// cluster on the antialiasing ramp and has none left for the ground's
// warm lift). They went back to the roles when `border` took the
// frames' bright sage, because four screens of one era disagreeing
// about their outline ink was the larger fault; the probe stays here
// so nobody re-measures it.

/// Login's separate, dimmer upper field; see ground-fit.md. The photo's
/// solid footer band is drawn later and is unchanged.
const LOGIN_LIFT: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x1a150d)),
    (0.25, rgb(0x1a1a14)),
    (0.5, rgb(0x1c1a12)),
    (0.75, rgb(0x0f0b04)),
    (1.0, rgb(0x0d0804)),
];
const LOGIN_GROUND: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(rgb(0x0d0804))),
    Prim::Lobe { x: 950.0, y: -139.0, rx: 1172.0, ry: 767.0, stops: LOGIN_LIFT },
];
const LOGIN_BACKDROP: &[Prim] = &[Prim::Soft { prims: LOGIN_GROUND }];

/// The footer band is the one solid fill on the screen and it is 12.1%
/// of the frame; the header strip and the field are hairline outlines.
///
/// Retracked 2026-09-03 against the trace's polish pass: header strings
/// 15/ls 1 -> 17 medium at natural tracking (the photo's runs are
/// x 62..305 / 519..672 / 1383..1494, which 15px missed by a fifth),
/// middle cell x 521 -> 518 and baseline 61 -> 60; outlines 1.5 ->
/// 1.25px; USERNAME: medium ls 0.75 at x 576 baseline 402; the masked
/// run was bold 22 until the ten thin masks were remeasured 2026-09-21;
/// NEXT medium ls 2 at x 940 baseline
/// 433; the footer strings to baseline 863 and x 61/519/1383.
pub const ACCESS: Access = Access {
    reference_fg: None,
    reference_backdrop: None,
    backdrop: LOGIN_BACKDROP,
    // Header strip, x 49..1547, y 43..69, dividers at 465 and 1353.
    masthead: Masthead::Strip {
        plate: Plate::outlined(Plot::new(49.0, 43.0, 1498.0, 26.0), Ink::Border, 1.25),
        dividers: &[465.0, 1353.0],
        labels: &[
            Legend::new("RIPPERDOC SURGICAL SOFTWAREV2", 61.0, 60.0, 17.0, Ink::Fg)
                .medium(),
            Legend::new("STORE ACCESS SCREEN", 518.0, 60.0, 17.0, Ink::Fg).medium(),
            Legend::new("FLAIR TRS 5MMP", 1382.0, 60.0, 17.0, Ink::Fg).medium(),
        ],
    },
    // One slot, alone in the upper two thirds of an empty frame.
    slots: &[Slot {
        prompt: Some(
            Legend::new("USERNAME:", 576.0, 402.0, 23.0, Ink::Fg)
                .medium()
                .tracked(0.75),
        ),
        field: Some(Plate::outlined(
            Plot::new(563.0, 414.0, 359.0, 33.0),
            Ink::Border,
            1.25,
        )),
        // Ten thin stars at ~9.54px pitch, native bright-core extent
        // x577.9..670.4 / y425..431.7 (2026-09-21 source correction).
        // The resting count is only a mock. Typed masks still follow
        // the full secret, and the underline trails the visible run.
        entry: Some(Entry {
            rest: Legend::new("**********", 576.4, 441.25, 25.0, Ink::Fg)
                .stretched(0.94045),
            mask: '*',
            tail: "",
            caret: Caret::Trails,
            blink: Blink::Caret,
            busy: "VERIFYING:",
            failed: "ACCESS DENIED:",
        }),
        // The short caret underline under the first pair of characters.
        caret: Some(Plate::filled(Plot::new(576.5, 439.125, 19.0, 0.5), Ink::Border)),
        action: Some(Plate::filled(Plot::new(932.0, 413.0, 105.0, 33.0), Ink::Cta)),
        action_label: Some(
            Legend::new("NEXT", 940.0, 433.0, 22.0, Ink::OnSelect)
                .medium()
                .tracked(2.0),
        ),
        ..Slot::EMPTY
    }],
    fixture: Fixture::None,
    // The band IS the footer on this screen: no outline box and no
    // dividers, unlike the thin outlined strip the hub, mail and store
    // screens use.
    colophon: Colophon::Band {
        plate: Plate::filled(Plot::new(36.0, 765.0, 1529.0, 115.0), Ink::Select)
            .reference_fill(rgb(0x8aac8c)),
        labels: &[
            Legend::new("INTERFACE LOADED", 61.0, 863.0, 15.0, Ink::OnSelect).tracked(1.0),
            Legend::new("PROVIDED BY NEXUS NETWORK V10.8", 519.0, 863.0, 15.0, Ink::OnSelect)
                .tracked(1.0),
            Legend::new("BUILD 6.47.48441.R15", 1383.0, 863.0, 15.0, Ink::OnSelect).tracked(1.0),
        ],
    },
};
// --- end login ---
// --- mailbox ---
//
// `docs/entropism/mailbox-trace.svg`, read at its 1600x900 frame. The
// trace's own numbers, with two deliberate departures, both of them the
// trace's instruction rather than mine:
//
//   * the three-stroke edge profile (7px #25281d halo, 4px #0a0a02
//     undershoot, 2px #709174 stroke) is how the *photograph* renders a
//     bright edge. The trace says so in as many words -- "it is not a
//     designed glow -- README.md's 'no glow' rule stands for the iced
//     implementation, which should draw the 2px stroke only" -- so every
//     outline here is the single 2px stroke.
//   * the upper field is the shared native-photo ground, now painted
//     through the mailbox backdrop like the hub and store scenes.
//
// Outline ink is `Ink::Border`. It was `Ink::Mid` until 2026-09-05
// because the trace samples every frame at #709174, next to the era's
// `MID` #728f76 and a stop above the `OUTLINE` of the time (#5d7752);
// the #709174 is the 1600 rescale's dilution of a bright 2px line,
// which is what `border` now is (#8fba97), so the frames read the role
// like the hub's and the store's. The two 9px captions under A MAIL
// BOX stay `Ink::Mid`: they are the screen's faintest text, not frames.

use crate::style::{
    Change, Frame, Mail, MailBadgeArt, MailBadges, MailButtons, MailList, MailMotion, MailPanel, MailPart, MailRowType,
    Mailbox, Motion, Note, Piece, RowDecor, Run, Trim, FromAt,
};
use iced::animation::Easing;

/// Header strip and footer strip: the chrome that stands from frame 0.
/// The section headings are the body's (`BODY_HEADINGS`, under
/// `#body-scan`) and the footer strings type on last (`FOOTER_STRINGS`,
/// under `#footer-type`); see `MAILBOX_MOTIONS`.
static CHROME: [Piece; 7] = [
    // header strip x 49..1547, y 43..69, dividers at x 465 and 1353
    Piece::Box {
        at: Frame::new(49.0, 43.0, 1498.0, 26.0),
        fill: None,
        stroke: Some(Ink::Border),
        width: 2.0,
        trim: Trim::NONE,
    },
    Piece::Box {
        at: Frame::new(465.0, 43.0, 2.0, 26.0),
        fill: Some(Ink::Border),
        stroke: None,
        width: 0.0,
        trim: Trim::NONE,
    },
    Piece::Box {
        at: Frame::new(1353.0, 43.0, 2.0, 26.0),
        fill: Some(Ink::Border),
        stroke: None,
        width: 0.0,
        trim: Trim::NONE,
    },
    Piece::Label(Note {
        at: Run::new(61.0, 60.0, 17.0, Ink::Fg).medium(),
        text: "RIPPERDOC SURGICAL SOFTWAREV2",
    }),
    Piece::Label(Note {
        at: Run::new(518.0, 60.0, 17.0, Ink::Fg).medium(),
        text: "STORE ACCESS SCREEN",
    }),
    Piece::Label(Note {
        at: Run::new(1382.0, 60.0, 17.0, Ink::Fg).medium(),
        text: "FLAIR TRS 5MMP",
    }),
    // footer strip x 49..1547, y 847..873, no dividers
    Piece::Box {
        at: Frame::new(49.0, 847.0, 1498.0, 26.0),
        fill: None,
        stroke: Some(Ink::Border),
        width: 2.0,
        trim: Trim::NONE,
    },
];

/// The three boxed section letters and their strings: the top of the
/// body, scanned in with it.
static BODY_HEADINGS: [Piece; 12] = [
    // A MAIL BOX, with the two lines of micro-print under it
    Piece::Box {
        at: Frame::new(100.0, 98.0, 26.0, 26.0),
        fill: None,
        stroke: Some(Ink::Fg),
        width: 1.5,
        trim: Trim::NONE,
    },
    Piece::Label(Note {
        at: Run::new(104.3, 119.25, 22.0, Ink::Fg).medium().stretched(1.65),
        text: "A",
    }),
    Piece::Label(Note {
        at: Run::new(137.03, 117.92, 22.5, Ink::Fg).medium().stretched(1.008),
        text: "MAIL BOX",
    }),
    Piece::Label(Note {
        at: Run::new(103.1, 145.0, 9.0, Ink::Mid).medium().stretched(0.955),
        text: "SPARE TIME MANAGER WAS DEVELOPED BY",
    }),
    Piece::Label(Note {
        at: Run::new(103.1, 154.17, 9.0, Ink::Mid).medium().stretched(0.948),
        text: "SEOCHO. SERVING CUSTOMERS SINCE 2006.",
    }),
    // B MESSAGE
    Piece::Box {
        at: Frame::new(556.0, 98.0, 26.0, 26.0),
        fill: None,
        stroke: Some(Ink::Fg),
        width: 1.5,
        trim: Trim::NONE,
    },
    Piece::Label(Note {
        at: Run::new(560.2, 119.25, 22.0, Ink::Fg).medium().stretched(1.65),
        text: "B",
    }),
    Piece::Label(Note {
        at: Run::new(590.79, 117.92, 22.5, Ink::Fg).medium().stretched(1.003),
        text: "MESSAGE",
    }),
    // C ENCRIPTION LEVEL -- the source really does spell it that way,
    // and a trace is not the place to correct a sign painter.
    Piece::Box {
        at: Frame::new(1347.0, 98.0, 26.0, 26.0),
        fill: None,
        stroke: Some(Ink::Fg),
        width: 1.5,
        trim: Trim::NONE,
    },
    Piece::Label(Note {
        at: Run::new(1351.6, 119.25, 22.0, Ink::Fg).medium().stretched(1.6),
        text: "C",
    }),
    Piece::Label(Note {
        at: Run::new(1381.89, 117.92, 24.0, Ink::Fg).medium().stretched(1.024),
        text: "ENCRIPTION",
    }),
    Piece::Label(Note {
        at: Run::new(1381.9, 142.92, 22.5, Ink::Fg).medium().stretched(1.084),
        text: "LEVEL",
    }),
];

/// The footer strip's three strings, typed on from the left.
static FOOTER_STRINGS: [Piece; 3] = [
    Piece::Label(Note {
        at: Run::new(61.0, 865.0, 17.0, Ink::Fg).medium(),
        text: "INTERFACE LOADED",
    }),
    Piece::Label(Note {
        at: Run::new(518.0, 865.0, 17.0, Ink::Fg).medium(),
        text: "PROVIDED BY NEXUS NETWORK V10.8",
    }),
    Piece::Label(Note {
        at: Run::new(1382.0, 865.0, 17.0, Ink::Fg).medium(),
        text: "BUILD 6.47.48441.R15",
    }),
];

// --- motion ---
//
// The three boot-in animations of every entropism screen (the traces'
// `<defs>`, and `docs/entropism/README.md`): the body scans down, the
// selection lights, the footer types. The three screens share the
// ids, the timing and the easing, and differ only in the rectangles,
// so the two shared ones are consts and each screen sets its own clip.

/// `#select-lit`: the solid selection fills come up as one group
/// opacity 0 -> 1 over 0.15 s from 0.45 s, `keySplines="0.61 1 0.88 1"`
/// = EaseOut. Only the fills: the dark ink on them is painted after
/// and stands from frame 0, as the traces draw it.
pub const SELECT_LIT: Motion = Motion {
    id: "select-lit",
    begin: 450,
    dur: 150,
    ease: Easing::EaseOut,
    change: Change::Opacity { alpha: (0.0, 1.0) },
};

/// `#body-scan` for a given body rectangle: a clip on `height`, 0 to
/// the whole, over 0.45 s from 0, `keySplines="0.45 0 0.55 1"` =
/// EaseInOutQuad -- a scan, not a wipe.
pub const fn body_scan(x: f32, y: f32, w: f32, h: f32) -> Motion {
    Motion {
        id: "body-scan",
        begin: 0,
        dur: 450,
        ease: Easing::EaseInOutQuad,
        change: Change::Clip { x, y, w: (w, w), h: (0.0, h) },
    }
}

/// `#footer-type` for the footer strip's text box: a clip on `width`,
/// 0 to the whole, over 0.35 s from 0.6 s, EaseInOutQuad again (a
/// teletype). The strip's frame is chrome and does not move.
pub const fn footer_type(x: f32, y: f32, w: f32, h: f32) -> Motion {
    Motion {
        id: "footer-type",
        begin: 600,
        dur: 350,
        ease: Easing::EaseInOutQuad,
        change: Change::Clip { x, y, w: (0.0, w), h: (h, h) },
    }
}

/// The mailbox's three, over the sheet's regions
/// (`Mailbox::motions`; `screens/mail.rs` paints them):
///
///   * `#body-scan` over the body rect x 70 y 90 w 1490 h 670 -- the
///     section headings, the list, the message panel with its heading,
///     the buttons and the badges, which is everything between the
///     strips;
///   * `#select-lit` over the four solid fills (row 1's plate, the
///     message's title bar, REPORT SPAM, the T2 badge) -- the sheet's
///     `MailPart::Fills`, each inside its region's own cover as the
///     trace nests the group inside the clip;
///   * `#footer-type` over the three footer strings, x 49 y 835 w 1498
///     h 50.
pub const MAILBOX_MOTIONS: &[MailMotion] = &[
    MailMotion {
        motion: body_scan(70.0, 90.0, 1490.0, 670.0),
        parts: &[
            MailPart::Pieces(&BODY_HEADINGS),
            MailPart::List,
            MailPart::Panel,
            MailPart::Title,
            MailPart::Buttons,
            MailPart::Badges,
        ],
    },
    MailMotion {
        motion: SELECT_LIT,
        parts: &[MailPart::Fills],
    },
    MailMotion {
        motion: footer_type(49.0, 835.0, 1498.0, 50.0),
        parts: &[MailPart::Pieces(&FOOTER_STRINGS)],
    },
];
// --- end motion ---

static BUTTONS: [&str; 4] = ["REPLY", "FORWARD", "DELETE", "REPORT SPAM"];
const MAILBOX_ACTION_LABELS: &[Run] = &[
                Run::new(21.50000, 24.5, 23.5, Ink::Fg).medium().stretched(0.978500),
                Run::new(20.55862, 24.5, 23.5, Ink::Fg).medium().stretched(1.043857),
                Run::new(20.07832, 24.5, 23.5, Ink::Fg).medium().stretched(1.075346),
                Run::new(20.19425, 24.5, 23.5, Ink::Fg).medium().stretched(1.013856),
            ];
/// The trace reads them T1 T3 over T2 T4, and T2 -- bottom left -- is
/// the filled one.
static LEVELS: [&str; 4] = ["T1", "T3", "T2", "T4"];

// Relative to each mailbox badge; the source's four glyph widths and
// centers differ. Their words remain supplied by LEVELS.
const MAILBOX_BADGE_LABELS: &[Run] = &[
    Run::new(36.0, 43.5, 27.0, Ink::Fg).bold().centered().stretched(1.44),
    Run::new(36.0, 43.5, 27.0, Ink::Fg).bold().centered().stretched(1.48),
    Run::new(36.0, 43.5, 27.0, Ink::Fg).bold().centered().stretched(1.47),
    Run::new(35.5, 43.5, 27.0, Ink::Fg).bold().centered().stretched(1.42),
];

// Source T silhouettes and all four digits; see docs/entropism/mailbox-badges.md.
const MAILBOX_BADGE_ART: &[MailBadgeArt] = &[
    MailBadgeArt { text: "T1", replace_prefix: 2, pieces: &[
        Piece::Poly { points: &[
            (-14.500000, -16.333000),
            (3.417000, -16.333000),
            (3.417000, -13.833000),
            (-4.083000, -13.833000),
            (-4.083000, 0.333000),
            (-7.000000, 0.333000),
            (-7.000000, -13.833000),
            (-14.500000, -13.833000),
        ], fill: Some(Ink::Fg), stroke: None, width: 0.0, close: true },
        Piece::Poly { points: &[
            (10.500000, -17.167000),
            (12.167000, -17.167000),
            (12.167000, 0.333000),
            (9.667000, 0.333000),
            (9.667000, -12.583000),
            (5.917000, -12.583000),
            (5.917000, -14.667000),
            (8.417000, -15.083000),
            (9.250000, -15.917000),
        ], fill: Some(Ink::Fg), stroke: None, width: 0.0, close: true },
    ] },
    MailBadgeArt { text: "T3", replace_prefix: 2, pieces: &[
        Piece::Poly { points: &[
            (-18.583333, -17.583000),
            (-0.666333, -17.583000),
            (-0.666333, -15.083000),
            (-8.166333, -15.083000),
            (-8.166333, -0.917000),
            (-11.083333, -0.917000),
            (-11.083333, -15.083000),
            (-18.583333, -15.083000),
        ], fill: Some(Ink::Fg), stroke: None, width: 0.0, close: true },
        Piece::Poly { points: &[
            (7.250000, -18.000000),
            (13.083333, -17.583333),
            (14.750000, -16.750000),
            (16.833333, -14.250000),
            (16.416667, -11.333333),
            (15.166667, -10.083333),
            (13.500000, -9.666667),
            (13.500000, -8.833333),
            (15.166667, -8.416667),
            (16.416667, -7.166667),
            (16.833333, -4.250000),
            (14.333333, -1.333333),
            (12.666667, -0.500000),
            (8.083333, -0.083333),
            (1.416667, -2.166667),
            (2.250000, -4.666667),
            (7.666667, -3.000000),
            (11.833333, -3.416667),
            (13.500000, -4.666667),
            (13.500000, -6.333333),
            (12.666667, -7.166667),
            (10.583333, -8.000000),
            (6.833333, -8.000000),
            (6.833333, -10.500000),
            (11.833333, -10.500000),
            (13.083333, -11.333333),
            (13.500000, -13.833333),
            (12.666667, -14.666667),
            (6.833333, -15.083333),
            (3.083333, -13.833333),
            (2.250000, -15.083333),
            (2.250000, -16.333333),
        ], fill: Some(Ink::Fg), stroke: None, width: 0.0, close: true },
    ] },
    MailBadgeArt { text: "T2", replace_prefix: 2, pieces: &[
        Piece::Poly { points: &[
            (-18.250000, -16.208000),
            (-0.333000, -16.208000),
            (-0.333000, -13.708000),
            (-8.250000, -13.708000),
            (-8.250000, 0.458000),
            (-11.583000, 0.458000),
            (-11.583000, -13.708000),
            (-18.250000, -13.708000),
        ], fill: Some(Ink::Fg), stroke: None, width: 0.0, close: true },
        // Source contour; the foot keeps medium coverage at the lower edge.
        Piece::Poly { points: &[
            (6.750000, -16.416667),
            (12.166667, -16.416667),
            (13.000000, -15.583333),
            (14.250000, -15.583333),
            (16.333333, -13.083333),
            (16.333333, -10.166667),
            (14.666667, -8.500000),
            (11.333333, -6.833333),
            (5.916667, -5.166667),
            (4.666667, -3.916667),
            (4.250000, -2.458333),
            (16.750000, -2.458333),
            (16.750000, 0.187500),
            (1.750000, 0.187500),
            (1.750000, -3.083333),
            (3.416667, -6.000000),
            (6.750000, -8.083333),
            (11.750000, -9.333333),
            (13.416667, -10.583333),
            (13.416667, -12.666667),
            (12.166667, -13.916667),
            (10.500000, -14.333333),
            (7.583333, -14.333333),
            (2.583333, -12.666667),
            (2.583333, -13.500000),
            (1.750000, -13.916667),
            (2.166667, -14.750000),
        ], fill: Some(Ink::Fg), stroke: None, width: 0.0, close: true },
    ] },
    MailBadgeArt { text: "T4", replace_prefix: 2, pieces: &[
        Piece::Poly { points: &[
            (-18.500000, -17.249667),
            (-0.583000, -17.249667),
            (-0.583000, -14.749667),
            (-8.083000, -14.749667),
            (-8.083000, -0.583667),
            (-11.000000, -0.583667),
            (-11.000000, -14.749667),
            (-18.500000, -14.749667),
        ], fill: Some(Ink::Fg), stroke: None, width: 0.0, close: true },
        // One fill with a reversed counter keeps the ink continuous without
        // compositing overlapping diagonal, stem and crossbar polygons.
        Piece::Curve {
            start: (11.500000, -17.250000),
            steps: &[
                Seg::Line(14.416667, -17.250000),
                Seg::Line(14.416667, -7.666667),
                Seg::Line(17.750000, -7.666667),
                Seg::Line(17.750000, -5.166667),
                Seg::Line(14.416667, -5.166667),
                Seg::Line(14.416667, -0.583333),
                Seg::Line(11.083333, -0.583333),
                Seg::Line(11.083333, -5.166667),
                Seg::Line(1.500000, -5.166667),
                Seg::Line(1.500000, -7.666667),
                Seg::Line(11.083333, -16.833333),
                Seg::Line(11.500000, -17.250000),
                Seg::Move(5.666667, -7.666667),
                Seg::Line(11.083333, -7.666667),
                Seg::Line(11.083333, -14.166667),
                Seg::Line(5.666667, -7.666667),
            ],
            fill: Some(Ink::Fg), stroke: None, width: 0.0, close: true,
        },
    ] },
];

/// The seven rows, trace lines 188-207 (text) and 186 / 210-216 (the
/// envelopes: only row 2's is `#env-open`). The trace sets them in
/// capitals; `title_upper` / `from_upper` do that here.
static ROWS: [Mail; 7] = [
    Mail { subject: "You'll regret that", from: "Jackie", unread: false },
    Mail { subject: "Urgent information (!)", from: "Mom", unread: true },
    Mail { subject: "Heist data sent to you", from: "805000451", unread: false },
    Mail { subject: "I'm worried man", from: "Rachel Ross", unread: false },
    Mail { subject: "Special offer to you!", from: "JINX JINX STORE", unread: false },
    Mail { subject: "I'm worried man", from: "Biala Robertson", unread: false },
    Mail { subject: "Special offer to you!", from: "Larix & Betula", unread: false },
];

// Native source fit, measured in the 1600px design frame. Only the
// typography drifts within the existing 61.95px row pitch; hit frames
// and selection geometry are unchanged. Runtime supplies the row text
// and feedback ink, so these templates carry no duplicate content.
static ROW_TYPE: [MailRowType; 7] = [
    MailRowType {
        title: Run::new(137.08, 29.83, 22.25, Ink::Fg).stretched(0.991),
        from: Run::new(137.15, 48.58, 16.25, Ink::Mid).stretched(0.963),
    },
    MailRowType {
        title: Run::new(136.64, 29.55, 22.25, Ink::Fg).medium().stretched(1.006),
        from: Run::new(136.61, 48.72, 16.25, Ink::Mid).medium().stretched(1.058),
    },
    MailRowType {
        title: Run::new(137.12, 28.85, 22.25, Ink::Fg).medium().stretched(0.966),
        from: Run::new(136.67, 48.02, 16.25, Ink::Mid).medium().stretched(1.015),
    },
    MailRowType {
        title: Run::new(137.08, 28.57, 22.25, Ink::Fg).medium().stretched(0.991),
        from: Run::new(136.69, 47.32, 16.25, Ink::Mid).medium().stretched(0.994),
    },
    MailRowType {
        title: Run::new(136.29, 27.87, 22.25, Ink::Fg).medium().stretched(0.966),
        from: Run::new(136.73, 46.62, 16.25, Ink::Mid).medium().stretched(0.965),
    },
    MailRowType {
        title: Run::new(137.08, 27.58, 22.25, Ink::Fg).medium().stretched(0.991),
        from: Run::new(136.67, 46.33, 16.25, Ink::Mid).medium().stretched(1.014),
    },
    MailRowType {
        title: Run::new(136.29, 26.88, 22.25, Ink::Fg).medium().stretched(0.966),
        from: Run::new(136.66, 46.05, 16.25, Ink::Mid).medium().stretched(1.023),
    },
];

/// The source body: 4 + 4 + 2 lines with the original words and
/// breaks. A supported horizontal transform replaces the trace's
/// formerly ineffective `textLength` declarations.
static PARAGRAPHS: [&[&str]; 3] = [
    &[
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incidi-",
        "dunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitati-",
        "on ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in re-",
        "prehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.",
    ],
    &[
        "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit",
        "anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem ac-",
        "cusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore",
        "veritatis et quasi architecto beatae vitae dicta sunt explicabo.",
    ],
    &[
        "Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia",
        "consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt.",
    ],
];

pub fn mailbox() -> Mailbox {
    Mailbox {
        // Native Iced paints the fitted Rajdhani runs about 1.2 design pixels
        // above their SVG baselines at 3840x2160. This shared factor
        // moves all mailbox text down without changing source-fitted Runs.
        text_baseline: 0.89,
        backdrop: MAIL_GROUND,
        chrome: &CHROME,
        overlay: &[],
        list: MailList {
            footer: &[],
            feedback: None,
            // frame x 84..451, y 205..686
            frame: Some(Frame::new(84.0, 205.0, 367.0, 481.0)),
            frame_ink: Ink::Border,
            frame_width: 2.0,
            // seven rows on a 62px pitch starting 21px below the top
            // edge; the dividers at 349 / 411 / 473 / 535 / 596 / 658
            // are each row's own foot.
            row: Frame::new(85.0, 226.0, 366.0, 62.0),
            pitch: 61.95,
            rows: &ROWS,
            row_type: &ROW_TYPE,
            selected_row_type: None,
            selected: 0,
            decor: RowDecor::Framed,
            row_fill: None,
            row_fills: &[],
            row_stroke: None,
            row_width: 0.0,
            row_trim: Trim::NONE,
            spine: None,
            rule: Some(Frame::new(0.0, 61.95, 366.0, 2.0)),
            rule_ink: Ink::Border,
            tab: None,
            tab_ink: Ink::Fg,
            sel: Frame::new(85.0, 226.0, 366.0, 62.0),
            sel_trim: Trim::NONE,
            sel_icon: None,
            sel_icon_trim: Trim::NONE,
            sel_fill: Ink::Select,
            sel_notch: None,
            veneer: None,
            glyph_x: 102.0,
            glyph_dy: 18.0,
            glyph_offsets: &[],
            glyph_w: 17.0,
            text_x: 138.0,
            title_dy: 30.0,
            title_size: 20.0,
            title_bold: false,
            title_ink: Ink::Fg,
            selected_ink: Ink::OnSelect,
            selected_printing: None,
            from_dy: 48.0,
            from_size: 14.0,
            from_ink: Ink::Mid,
            from_at: FromAt::Beneath,
            from_prefix: "FROM: ",
            title_upper: true,
            from_upper: true,
            new_pill: None,
            new_pill_selected: None,
            new_pill_art: &[],
            icons: None,
            envelope: None,
        },
        panel: MailPanel {
            frame: Some(Frame::new(529.0, 205.0, 750.0, 481.0)),
            frame_fill: None,
            frame_stroke: Some(Ink::Border),
            frame_width: 2.0,
            frame_trim: Trim::NONE,
            head: Some(Frame::new(531.0, 227.0, 746.0, 61.0)),
            head_ink: Ink::Select,
            head_trim: Trim::NONE,
            // the panel reads row 2 (URGENT INFORMATION (!)) while row
            // 1 is selected, trace lines 185-188 / 228
            message: 1,
            title: Run::new(554.79, 252.5, 23.3, Ink::OnSelect).stretched(1.063),
            title_upper: true,
            from: Some(Run::new(554.65, 278.75, 18.5, Ink::OnSelect).stretched(1.098)),
            heading: None,
            // trace line 229: the list shouts "FROM: MOM", the panel
            // does not
            sender: Some("from: Mom"),
            body: Run::new(556.0, 325.0, 17.0, Ink::Fg).stretched(1.14),
            line: 21.7,
            para: 39.0,
            // The first two paragraph starts follow the common pitch;
            // the source puts the third at y535 rather than y533.2.
            paragraph_baselines: &[325.0, 429.1, 535.0],
            paragraphs: &PARAGRAPHS,
        },
        buttons: MailButtons {
            // one outlined strip y 694..746 split at x 716 / 903 /
            // 1091, the last cell filled solid with dark text
            first: Frame::new(529.0, 694.0, 187.33, 52.0),
            dx: 187.33,
            dy: 0.0,
            count: 4,
            filled: Some(3),
            fill: Ink::Select,
            idle_fill: None,
            joined: true,
            chevron: false,
            trim: Trim::NONE,
            width: 2.0,
            stroke: Ink::Border,
            label: Run::new(21.0, 24.5, 23.5, Ink::Fg).medium().stretched(1.03),
            tab: None,
            label_runs: MAILBOX_ACTION_LABELS,
            labels: &BUTTONS,
        },
        badges: MailBadges {
            // 2x2 of 69x69 at columns x 1341 / 1418, rows y 227 / 305
            first: Frame::new(1341.0, 227.0, 69.0, 69.0),
            dx: 77.0,
            dy: 78.0,
            cols: 2,
            count: 4,
            selected: Some(2),
            trim: Trim::NONE,
            width: 2.0,
            fill: None,
            stroke: Ink::Border,
            label: Run::new(35.0, 43.5, 27.0, Ink::Fg).bold().centered().stretched(1.42),
            label_runs: MAILBOX_BADGE_LABELS,
            label_art: MAILBOX_BADGE_ART,
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
// `docs/entropism/store-trace.svg`, transcribed. Every figure below is
// the trace's own coordinate in the 1600x900 frame; the trace's header
// records how each was measured off `images/entropism-dashboard.png`
// (which is the store -- the two entropism source files are named the
// wrong way round, see `docs/sources.md`).

use crate::style::{
    fill_path, fill_rect, line_rect, shut_path, txt, txt_end, txt_mid, vline, Group,
    Prim, Seg,
};

/// The yellow band. The one place entropism leaves its single hue, and
/// the only store ink the era's role table has no name for.
pub const STORE_BAND: iced::Color = rgb(0xeebf09);
pub const STORE_ON_BAND: iced::Color = rgb(0x35462e);

/// The socket-row glyph: a 9x9 dot matrix with the middle row and
/// column empty, not a QR. Sampled at every dot centre on cards 1 and 2
/// of the photo; identical on both.
const QR: &[&str] = &[
    "#..#.#..#",
    ".#....#..",
    "..#.....#",
    "#..#.#.#.",
    ".........",
    ".#.#..#.#",
    "#.#..#...",
    ".#.....#.",
    "#..#.#..#",
];

#[path = "entropism_store_art.rs"]
mod store_art;

/// Source-native nested tone contours for the repeated MAGNUM weapon.
/// The two thresholds retain rail holes, receiver fasteners and grip folds
/// without strokes that can escape the weapon body (store-art.md).
const RIFLE_PLAIN: Prim = Prim::Path {
    x: 237.833, y: 131.667, segs: super::magnum_art::RIFLE, close: true,
    fill: Some(Ink::Mid), stroke: None, width: 0.0,
};
const RIFLE_INVERSE: Prim = Prim::Path {
    x: 72.333, y: 105.833, segs: store_art::SELECTED_DARK, close: true,
    fill: Some(Ink::Dim), stroke: None, width: 0.0,
};
const RIFLE_PLAIN_DETAIL: Prim = Prim::Path {
    x: 78.25, y: 105.833, segs: super::magnum_art::RIFLE_BRIGHT, close: true,
    fill: Some(Ink::Select), stroke: None, width: 0.0,
};
const RIFLE_INVERSE_DETAIL: Prim = Prim::Path {
    x: 72.333, y: 105.833, segs: store_art::SELECTED_CORE, close: true,
    fill: Some(Ink::OnSelect), stroke: None, width: 0.0,
};

/// Store-local printing calibrated against card 1 at 1600x900.
const fn store_text(x: f32, y: f32, size: f32, stretch: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Wide { x, y, size, stretch, ink, face: Face::Medium,
        anchor: crate::style::Anchor::Start, content }
}
const fn store_mid(x: f32, y: f32, size: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Wide { x, y, size, stretch: 1.0, ink, face: Face::Medium,
        anchor: crate::style::Anchor::Middle, content }
}

/// Store values keep the fitted SVG baseline 229.1667. Native stretched
/// runs land one pixel below the unstretched values, so lift only those.
const fn store_value(x: f32, stretch: f32, ink: Ink, content: &'static str) -> Prim {
    let y = if stretch == 1.0 { 228.3334 } else { 228.3334 - 0.416_667 };
    Prim::Wide { x, y, size: 24.5, stretch, ink,
        face: Face::SemiBold, anchor: crate::style::Anchor::Middle, content }
}

/// Selected-card manufacturer printing follows the measured source glyph spans.
/// Stretched native glyphs need a one-physical-pixel lift at 4K versus SVG.
/// The ordinary copies retain their current type pending independent gap fitting.
const BETTERLIFE_TEC_GLYPHS: &[Prim] = &[
    store_text(185.875004, 71.383333, 9.5, 1.250000, Ink::Fixed(STORE_ON_BAND), "B"),
    store_text(191.500000, 71.383333, 9.5, 1.500000, Ink::Fixed(STORE_ON_BAND), "E"),
    store_text(197.571348, 71.383333, 9.5, 1.428571, Ink::Fixed(STORE_ON_BAND), "T"),
    store_text(202.988003, 71.383333, 9.5, 1.428571, Ink::Fixed(STORE_ON_BAND), "T"),
    store_text(208.722147, 71.383333, 9.5, 1.333333, Ink::Fixed(STORE_ON_BAND), "E"),
    store_text(214.208333, 71.383333, 9.5, 1.250000, Ink::Fixed(STORE_ON_BAND), "R"),
    store_text(220.388816, 71.383333, 9.5, 1.333333, Ink::Fixed(STORE_ON_BAND), "L"),
    store_text(226.083337, 71.8, 9.5, 1.000000, Ink::Fixed(STORE_ON_BAND), "I"),
    store_text(228.305484, 71.383333, 9.5, 1.333333, Ink::Fixed(STORE_ON_BAND), "F"),
    store_text(233.305482, 71.383333, 9.5, 1.333333, Ink::Fixed(STORE_ON_BAND), "E"),
    store_text(241.321329, 71.383333, 9.5, 1.428571, Ink::Fixed(STORE_ON_BAND), "T"),
    store_text(247.055478, 71.383333, 9.5, 1.333333, Ink::Fixed(STORE_ON_BAND), "E"),
    store_text(252.125004, 71.383333, 9.5, 1.250000, Ink::Fixed(STORE_ON_BAND), "C"),
];
const BETTERLIFE_TEC: Prim = Prim::At {
    x: 0.0, y: 0.0, prims: BETTERLIFE_TEC_GLYPHS,
};

/// Ordinary-card manufacturer printing uses one shared fit to both
/// photographed ordinary copies. Stretched native glyphs lift one 4K pixel.
/// Keep these Fixed inks through cursor inversions.
const ORDINARY_BETTERLIFE_TEC_GLYPHS: &[Prim] = &[
    store_text(185.486111, 71.383333, 9.5, 1.122222, Ink::Fixed(STORE_ON_BAND), "B"),
    store_text(191.714286, 71.383333, 9.5, 1.157143, Ink::Fixed(STORE_ON_BAND), "E"),
    store_text(197.174479, 71.383333, 9.5, 1.262500, Ink::Fixed(STORE_ON_BAND), "T"),
    store_text(203.450521, 71.383333, 9.5, 1.137500, Ink::Fixed(STORE_ON_BAND), "T"),
    store_text(208.886905, 71.383333, 9.5, 1.014286, Ink::Fixed(STORE_ON_BAND), "E"),
    store_text(214.305556, 71.383333, 9.5, 1.011111, Ink::Fixed(STORE_ON_BAND), "R"),
    store_text(220.553571, 71.383333, 9.5, 1.014286, Ink::Fixed(STORE_ON_BAND), "L"),
    store_text(226.158333, 71.383333, 9.5, 0.560000, Ink::Fixed(STORE_ON_BAND), "I"),
    store_text(227.964286, 71.383333, 9.5, 1.157143, Ink::Fixed(STORE_ON_BAND), "F"),
    store_text(233.380952, 71.383333, 9.5, 1.157143, Ink::Fixed(STORE_ON_BAND), "E"),
    store_text(241.361979, 71.383333, 9.5, 1.162500, Ink::Fixed(STORE_ON_BAND), "T"),
    store_text(246.714286, 71.383333, 9.5, 1.157143, Ink::Fixed(STORE_ON_BAND), "E"),
    store_text(252.152778, 71.383333, 9.5, 1.122222, Ink::Fixed(STORE_ON_BAND), "C"),
];
const ORDINARY_BETTERLIFE_TEC: Prim = Prim::At {
    x: 0.0, y: 0.0, prims: ORDINARY_BETTERLIFE_TEC_GLYPHS,
};

/// An unselected product card, at its own origin. 265 wide, outlined,
/// with the socket row hung off its foot so the two frames share an
/// edge -- which is why the extractor reads them as one component.
const CARD: &[Prim] = &[
    line_rect(0.0, 0.0, 265.0, 237.0, Ink::Border, 2.0),
    txt(12.0, 24.5, 24.0, Ink::Select, "MAGNUM 650"),
    txt(12.0, 43.5, 20.0, Ink::Select, "HAND GUN"),
    fill_rect(3.0, 55.0, 259.0, 20.0, Ink::Fixed(STORE_BAND)),
    fill_rect(3.0, 62.0, 58.0, 11.0, Ink::Fixed(STORE_ON_BAND)),
    txt(6.0, 71.0, 9.5, Ink::Fixed(STORE_BAND), "PETROCHEM"),
    ORDINARY_BETTERLIFE_TEC,
    RIFLE_PLAIN,
    RIFLE_PLAIN_DETAIL,
    txt_mid(41.0, 196.7, 17.5, Ink::Select, "DPS"),
    txt_mid(102.0, 196.7, 17.5, Ink::Select, "PNT"),
    txt_mid(164.0, 196.7, 17.5, Ink::Select, "ACC"),
    txt_mid(225.0, 196.7, 17.5, Ink::Select, "ROF"),
    fill_rect(5.0, 209.0, 255.0, 25.0, Ink::Select),
    store_value(41.4167, 0.97, Ink::OnSelect, "86"),
    store_value(102.0, 1.0, Ink::OnSelect, "30"),
    store_value(164.0, 1.0, Ink::OnSelect, "5"),
    store_value(225.0, 1.0, Ink::OnSelect, "5"),
    // socket row y 237..286, dividers at 52 / 119 / 191
    line_rect(0.0, 237.0, 265.0, 49.0, Ink::Border, 2.0),
    vline(52.0, 237.0, 286.0, Ink::Border, 1.5),
    vline(119.0, 237.0, 286.0, Ink::Border, 1.5),
    vline(191.0, 237.0, 286.0, Ink::Border, 1.5),
    Prim::Dots { x: 10.5, y: 246.5, cell: 3.8, pitch: 3.5, ink: Ink::Select, rows: QR },
    store_mid(83.5, 259.5, 13.0, Ink::Select, "EMPTY"),
    store_mid(83.5, 273.0, 13.0, Ink::Select, "SOCKET"),
    store_mid(153.5, 259.5, 13.0, Ink::Select, "EMPTY"),
    store_mid(153.5, 273.0, 13.0, Ink::Select, "SOCKET"),
    store_mid(226.5, 259.5, 13.0, Ink::Select, "EMPTY"),
    store_mid(226.5, 273.0, 13.0, Ink::Select, "SOCKET"),
    store_text(5.0, 306.0, 8.5, 1.04, Ink::Fg, "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO"),
    store_text(5.0, 314.0, 8.5, 1.025, Ink::Fg, "MANIPULATE, ACCESS OR DISABLE THIS DEVICE."),
];

// A clipped canvas draft is pasted below direct frame meshes. Keep the
// selected fill in the same draft as its title text, before the text.
// The two drafts meet in the empty M-A gap at card-local x29.
// Exact point-for-point transcription of the selected parent/component
// SVG first-M approximation. Native text suffix remains the full run.
const SELECTED_M: &[Seg] = &[
    Seg::Line(27.17912000, 10.56800000),
    Seg::Quad { cx: 27.44792000, cy: 10.56800000, x: 27.44792000, y: 10.85600000 },
    Seg::Line(27.55544000, 17.60000000),
    Seg::Line(27.87800000, 22.88000000),
    Seg::Line(27.87800000, 24.36800000),
    Seg::Quad { cx: 27.87800000, cy: 24.65600000, x: 27.60920000, y: 24.65600000 },
    Seg::Line(26.69528000, 24.65600000),
    Seg::Quad { cx: 26.42648000, cy: 24.65600000, x: 26.42648000, y: 24.36800000 },
    Seg::Line(26.42648000, 22.88000000),
    Seg::Line(26.07704000, 17.60000000),
    Seg::Line(26.07704000, 12.41600000),
    Seg::Line(25.96952000, 12.41600000),
    Seg::Line(21.96440000, 22.37600000),
    Seg::Quad { cx: 21.80312000, cy: 22.64000000, x: 21.56120000, y: 22.64000000 },
    Seg::Line(20.67416000, 22.64000000),
    Seg::Quad { cx: 20.40536000, cy: 22.64000000, x: 20.29784000, y: 22.37600000 },
    Seg::Line(15.13688000, 12.39200000),
    Seg::Line(15.02936000, 12.39200000),
    Seg::Line(14.94872000, 17.60000000),
    Seg::Line(14.57240000, 22.88000000),
    Seg::Line(14.57240000, 24.32000000),
    Seg::Quad { cx: 14.57240000, cy: 24.60800000, x: 14.30360000, y: 24.60800000 },
    Seg::Line(13.41656000, 24.60800000),
    Seg::Quad { cx: 13.14776000, cy: 24.60800000, x: 13.14776000, y: 24.32000000 },
    Seg::Line(13.14776000, 22.88000000),
    Seg::Line(13.52408000, 17.60000000),
    Seg::Line(13.57784000, 10.85600000),
    Seg::Quad { cx: 13.57784000, cy: 10.56800000, x: 13.84664000, y: 10.56800000 },
    Seg::Line(15.40568000, 10.56800000),
    Seg::Quad { cx: 15.59384000, cy: 10.56800000, x: 15.64760000, y: 10.71200000 },
    Seg::Line(21.05048000, 21.24800000),
    Seg::Line(21.15800000, 21.24800000),
    Seg::Line(25.53944000, 10.71200000),
    Seg::Quad { cx: 25.59320000, cy: 10.56800000, x: 25.78136000, y: 10.56800000 },
];

const GROWN_TITLE_ON_LEFT: &[Prim] = &[
    Prim::Motion { motion: SELECT_LIT, prims: &[fill_rect(0.0, 0.0, 265.0, 234.0, Ink::Select)] },
    fill_path(25.78136000, 10.56800000, SELECTED_M, Ink::OnSelect),
];
const GROWN_TITLE_ON_RIGHT: &[Prim] = &[
    Prim::Motion { motion: SELECT_LIT, prims: &[fill_rect(0.0, 0.0, 265.0, 234.0, Ink::Select)] },
    txt(13.0, 24.5, 24.0, Ink::OnSelect, "MAGNUM 650"),
];
const GROWN_TITLE_OFF_LEFT: &[Prim] = &[fill_path(25.78136000, 10.56800000, SELECTED_M, Ink::Select)];
const GROWN_TITLE_OFF_RIGHT: &[Prim] = &[txt(13.0, 24.5, 24.0, Ink::Select, "MAGNUM 650")];

/// The grown card: the header block down through the values row is one
/// solid, the outline runs 412 tall, and the detail block takes the
/// room the unselected card spends on its compliance notice.
const GROWN: &[Prim] = &[
    // the header fill lights with the SMG row (`#select-lit`, :285);
    // the dark ink on it below stands from frame 0
    line_rect(0.0, 0.0, 265.0, 412.0, Ink::Border, 2.0),
    Prim::Viewport { x: 0.0, y: 0.0, w: 29.0, h: 412.0, prims: GROWN_TITLE_ON_LEFT },
    Prim::Viewport { x: 29.0, y: 0.0, w: 236.0, h: 412.0, prims: GROWN_TITLE_ON_RIGHT },
    txt(13.0, 43.5, 20.0, Ink::OnSelect, "HAND GUN"),
    fill_rect(3.0, 55.0, 259.0, 20.0, Ink::Fixed(STORE_BAND)),
    fill_rect(3.0, 62.0, 58.0, 11.0, Ink::Fixed(STORE_ON_BAND)),
    txt(6.0, 71.0, 9.5, Ink::Fixed(STORE_BAND), "PETROCHEM"),
    BETTERLIFE_TEC,
    RIFLE_INVERSE,
    RIFLE_INVERSE_DETAIL,
    txt_mid(41.0, 196.7, 17.5, Ink::OnSelect, "DPS"),
    txt_mid(102.0, 196.7, 17.5, Ink::OnSelect, "PNT"),
    txt_mid(164.0, 196.7, 17.5, Ink::OnSelect, "ACC"),
    txt_mid(225.0, 196.7, 17.5, Ink::OnSelect, "ROF"),
    fill_rect(0.0, 207.25, 265.0, 1.5, Ink::OnSelect),
    store_value(41.4167, 1.07, Ink::OnSelect, "86"),
    store_value(102.4167, 1.07, Ink::OnSelect, "30"),
    store_value(164.0, 1.0, Ink::OnSelect, "5"),
    store_value(225.0, 1.0, Ink::OnSelect, "5"),
    // detail block, y 494..672 on the page
    store_text(17.0, 270.0, 19.75, 1.01, Ink::Select, "20"),
    store_text(51.0, 270.0, 19.75, 1.01, Ink::Select, "Recoil"),
    store_text(17.0, 291.0, 19.75, 1.01, Ink::Select, "22"),
    store_text(51.0, 291.0, 19.75, 1.01, Ink::Select, "Sperad"),
    store_text(17.0, 312.0, 19.75, 1.01, Ink::Select, "12"),
    store_text(51.0, 312.0, 19.75, 1.01, Ink::Select, "Range"),
    store_text(17.0, 348.0, 19.75, 1.01, Ink::Select, "Bonus"),
    store_text(17.0, 368.0, 19.75, 1.01, Ink::Select, "+9 Reflexes"),
    store_text(17.0, 390.0, 19.75, 1.01, Ink::Select, "+2 Modules Slots"),
    line_rect(0.0, 412.0, 265.0, 48.0, Ink::Border, 2.0),
    vline(52.0, 412.0, 460.0, Ink::Border, 1.5),
    vline(120.0, 412.0, 460.0, Ink::Border, 1.5),
    vline(192.0, 412.0, 460.0, Ink::Border, 1.5),
    Prim::Dots { x: 10.5, y: 420.5, cell: 3.8, pitch: 3.5, ink: Ink::Select, rows: QR },
    store_mid(84.5, 434.5, 13.0, Ink::Select, "EMPTY"),
    store_mid(84.5, 448.0, 13.0, Ink::Select, "SOCKET"),
    store_mid(154.5, 434.5, 13.0, Ink::Select, "EMPTY"),
    store_mid(154.5, 448.0, 13.0, Ink::Select, "SOCKET"),
    store_mid(226.5, 434.5, 13.0, Ink::Select, "EMPTY"),
    store_mid(226.5, 448.0, 13.0, Ink::Select, "SOCKET"),
    store_text(5.0, 480.0, 8.5, 1.04, Ink::Fg, "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO"),
    store_text(5.0, 488.0, 8.5, 1.025, Ink::Fg, "MANIPULATE, ACCESS OR DISABLE THIS DEVICE."),
];

// Transpose the sourced grown header's reverse video onto the compact
// card. The compact adaptation and held blink are inferred; growth and
// socket/detail positions remain selection. Brand inks stay unchanged.
const fn compact_cursor() -> [Prim; CARD.len() + 1] {
    let mut out = [CARD[0]; CARD.len() + 1];
    out[0] = fill_rect(0.0, 0.0, 265.0, 234.0, Ink::Select);
    let mut i = 0;
    while i < CARD.len() {
        let mut prim = CARD[i];
        match &mut prim {
            Prim::Text { y, ink, .. } | Prim::Wide { y, ink, .. } if *y <= 234.0 => {
                *ink = match *ink {
                    Ink::Select => Ink::OnSelect,
                    Ink::OnSelect => Ink::Select,
                    other => other,
                };
            }
            Prim::Rect { y, fill, stroke, .. } if *y >= 102.0 && *y < 157.0 => {
                *fill = Some(Ink::OnSelect);
                *stroke = Some(Ink::Dim);
            }
            Prim::Path { fill, .. } => {
                *fill = match *fill {
                    Some(Ink::Select) => Some(Ink::OnSelect),
                    Some(Ink::Mid) => Some(Ink::Dim),
                    other => other,
                };
            }
            // Invert the existing values strip, not the grown divider.
            Prim::Rect { y, fill, .. } if *y == 209.0 => *fill = Some(Ink::OnSelect),
            _ => {}
        }
        out[i + 1] = prim;
        i += 1;
    }
    out
}
const CARD_CURSOR: &[Prim] = &compact_cursor();

// Remove only the grown header fill, then restore sage printing on the
// exposed ground. The fill lives inside each selected-title viewport,
// so the away/pressed version swaps its nested subscene to text-only.
const fn grown_outline() -> [Prim; GROWN.len()] {
    let mut out = [GROWN[0]; GROWN.len()];
    let mut i = 0;
    while i < GROWN.len() {
        let mut prim = GROWN[i];
        match &mut prim {
            Prim::Text { y, ink, .. } | Prim::Wide { y, ink, .. } if *y <= 234.0 => {
                if let Ink::OnSelect = *ink { *ink = Ink::Select; }
            }
            // These are the only top-level viewports in GROWN. Their
            // selected fill must disappear and nested text must invert.
            Prim::Viewport { x, prims, .. } if *x == 0.0 => *prims = GROWN_TITLE_OFF_LEFT,
            Prim::Viewport { x, prims, .. } if *x == 29.0 => *prims = GROWN_TITLE_OFF_RIGHT,
            Prim::Rect { y, fill, stroke, .. } if *y >= 102.0 && *y < 157.0 => {
                *fill = Some(Ink::Fg);
                *stroke = Some(Ink::Select);
            }
            Prim::Path { fill, .. } => {
                *fill = match *fill {
                    Some(Ink::OnSelect) => Some(Ink::Select),
                    Some(Ink::Dim) => Some(Ink::Mid),
                    other => other,
                };
            }
            Prim::Rect { y, fill, .. } if *y == 207.25 => *fill = Some(Ink::Select),
            _ => {}
        }
        out[i] = prim;
        i += 1;
    }
    out
}
const GROWN_OUTLINE: &[Prim] = &grown_outline();

macro_rules! product_states {
    ($index:expr) => {
        crate::style::PlateStates {
            group: Group::Card, index: $index,
            hover: CARD_CURSOR,
            pressed: CARD,
            selected_hover: Some(GROWN),
            selected_pressed: Some(GROWN_OUTLINE),
            selected_away: Some(GROWN_OUTLINE),
            preserve_selected_hover: false,
        }
    };
}
const STORE_STATES: &[crate::style::PlateStates] = &[
    product_states!(0), product_states!(1), product_states!(2), product_states!(3),
];

#[cfg(test)]
mod store_interaction_tests {
    use super::*;

    #[test]
    fn fourth_card_crop_limits_pointer_and_keyboard_targets_at_every_scale() {
        use crate::screens::scene::{hit_selected, plates_selected, Picked};
        use iced::Point;

        for card in [0, 3] {
            let picked = Picked { card, ..Picked::default() };
            for k in [0.75, 1.0, 1.25, 2.4] {
                let point = |x, y| Point::new(x * k, y * k);
                assert_eq!(hit_selected(STORE, picked, k, point(1564.0, 350.0)), Some((Group::Card, 3)));
                assert_eq!(hit_selected(STORE, picked, k, point(1565.0, 350.0)), None);
                assert_eq!(hit_selected(STORE, picked, k, point(1580.0, 600.0)), None);
            }
            let mut targets = Vec::new();
            plates_selected(STORE, picked, 0.0, 0.0, &mut targets);
            let (_, _, center) = targets.iter().find(|(g, i, _)| (*g, *i) == (Group::Card, 3)).unwrap();
            assert!((center.x - 1496.8).abs() < 0.001);
            assert_eq!(hit_selected(STORE, picked, 1.0, *center), Some((Group::Card, 3)));
        }
    }

    fn without_ink(mut prim: Prim) -> Prim {
        match &mut prim {
            Prim::Rect { fill, stroke, .. } | Prim::Path { fill, stroke, .. } => {
                *fill = None;
                *stroke = None;
            }
            Prim::Text { ink, .. } | Prim::Wide { ink, .. } => *ink = Ink::Fg,
            // Compare viewport placement independently of title ink/fill.
            Prim::Viewport { x, prims, .. } if *x == 0.0 => *prims = GROWN_TITLE_OFF_LEFT,
            Prim::Viewport { x, prims, .. } if *x == 29.0 => *prims = GROWN_TITLE_OFF_RIGHT,
            _ => {}
        }
        prim
    }

    #[test]
    fn cursor_keeps_compact_and_grown_geometry_content_and_socket_rows() {
        let geometry = |prims: &[Prim]| prims.iter().copied().map(without_ink).collect::<Vec<_>>();
        assert_eq!(geometry(&CARD_CURSOR[1..]), geometry(CARD));
        assert_eq!(geometry(GROWN_OUTLINE), geometry(GROWN));
        // Details, sockets, QR and compliance are outside the header;
        // none follow hover or press, including their original inks.
        assert_eq!(&CARD_CURSOR[19..], &CARD[18..]);
        assert_eq!(&GROWN_OUTLINE[19..], &GROWN[19..]);
        for (index, states) in STORE_STATES.iter().enumerate() {
            assert_eq!((states.group, states.index), (Group::Card, index));
            assert_eq!(states.hover, CARD_CURSOR);
            assert_eq!(states.pressed, CARD);
            assert_eq!(states.selected_hover, Some(GROWN));
            assert_eq!(states.selected_pressed, Some(GROWN_OUTLINE));
            assert_eq!(states.selected_away, Some(GROWN_OUTLINE));
        }
    }

    #[test]
    fn reverse_video_changes_only_header_material_and_keeps_brand_inks() {
        assert_eq!(CARD_CURSOR[0], fill_rect(0.0, 0.0, 265.0, 234.0, Ink::Select));
        assert_eq!(&CARD_CURSOR[4..8], &CARD[3..7]);
        assert_eq!(&GROWN_OUTLINE[4..8], &GROWN[4..8]);

        for prim in GROWN_OUTLINE {
            assert!(!matches!(prim, Prim::Motion { .. }));
            if let Prim::Viewport { prims, .. } = prim {
                assert!(!prims.iter().any(|nested| matches!(nested, Prim::Motion { .. })),
                    "away/pressed title must have no selected fill");
                assert!(prims.iter().all(|nested| match nested {
                    Prim::Text { ink: Ink::Select, .. } => true,
                    Prim::Path { fill: Some(Ink::Select), .. } => true,
                    _ => false,
                }), "away/pressed nested title must use selected ink");
            }
            if let Prim::Text { y, ink, .. } | Prim::Wide { y, ink, .. } = prim {
                if *y <= 234.0 { assert_ne!(*ink, Ink::OnSelect); }
            }
        }
        assert!(matches!(CARD_CURSOR[2], Prim::Text { ink: Ink::OnSelect, .. }));
        assert!(matches!(CARD_CURSOR[15],
            Prim::Text { ink: Ink::Select, .. } | Prim::Wide { ink: Ink::Select, .. }));
    }
}

/// The source 4 has its left corner at (137,133), a triangular counter
/// and a lower descender ending at y152. The S follows source-measured
/// rounded lobes and the thin y136–141 waist (store-art.md).
const FOUR: &[Seg] = &[
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

const ESS: &[Seg] = &[
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

/// The T is outline-only in the source; the 4 and the S are solid.
const TEE: &[Seg] = &[
    Seg::Line(308.0, 102.0),
    Seg::Line(308.0, 114.0),
    Seg::Line(292.0, 114.0),
    Seg::Line(292.0, 151.0),
    Seg::Line(278.0, 151.0),
    Seg::Line(278.0, 114.0),
    Seg::Line(262.0, 114.0),
];


// The nav's five rows and the shelf's four positions, as plates: hit
// box, the drawing worn when this one is the selection, and the drawing
// worn when it is not. `screens::store` picks between them; nothing
// here or there names an era.
macro_rules! nav {
    ($top:expr, $h:expr, $base:expr, $label:expr) => {
        (
            &[
                // the row's fill lights (`#select-lit`); its label does not
                Prim::Motion { motion: SELECT_LIT, prims: &[fill_rect(112.0, $top, 218.0, $h, Ink::Select)] },
                txt(140.0, $base, 24.0, Ink::OnSelect, $label),
            ],
            &[txt(140.0, $base, 24.0, Ink::Select, $label)],
        )
    };
}
const NAV_ON_0: &[Prim] = nav!(301.0, 57.0, 337.0, "RIFLES").0;
const NAV_OFF_0: &[Prim] = nav!(301.0, 57.0, 337.0, "RIFLES").1;
const NAV_ON_1: &[Prim] = nav!(358.0, 63.0, 397.0, "SMG").0;
const NAV_OFF_1: &[Prim] = nav!(358.0, 63.0, 397.0, "SMG").1;
const NAV_ON_2: &[Prim] = nav!(421.0, 61.0, 457.0, "SNIPER").0;
const NAV_OFF_2: &[Prim] = nav!(421.0, 61.0, 457.0, "SNIPER").1;
const NAV_ON_3: &[Prim] = nav!(482.0, 62.0, 516.0, "SHOTGUN").0;
const NAV_OFF_3: &[Prim] = nav!(482.0, 62.0, 516.0, "SHOTGUN").1;
const NAV_ON_4: &[Prim] = nav!(544.0, 62.0, 576.0, "PISTOL").0;
const NAV_OFF_4: &[Prim] = nav!(544.0, 62.0, 576.0, "PISTOL").1;

macro_rules! shelf {
    ($i:expr) => {
        &[Prim::Plate {
            group: Group::Card,
            index: $i,
            x: 0.0,
            y: 0.0,
            w: 265.0,
            h: 412.0,
            on: GROWN,
            off: CARD,
        }]
    };
}
const SHELF_0: &[Prim] = shelf!(0);
const SHELF_1: &[Prim] = shelf!(1);
const SHELF_2: &[Prim] = shelf!(2);
// Clip the complete fourth card, including its feedback and captions.
// Hit collection intersects this same viewport; no invisible target
// survives to the right of the source's open cut at x1564.6.
const SHELF_3: &[Prim] = &[Prim::Viewport {
    x: -2.0, y: -2.0, w: 137.6, h: 492.0, prims: shelf!(3),
}];

/// Hub, mailbox and store are frames of the same photographic field.
/// Its side margins stay near black while the upper centre receives a
/// broad warm illumination. The Rust table matches the SVG `#lift`.
const SHARED_LIFT: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x1f1e15)),
    (0.25, rgb(0x1f1f15)),
    (0.5, rgb(0x1f1e14)),
    (0.75, rgb(0x18140a)),
    (1.0, rgb(0x0f0903)),
];
const SHARED_GROUND: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(rgb(0x0f0903))),
    Prim::Lobe { x: 935.0, y: 5.0, rx: 943.0, ry: 420.0, stops: SHARED_LIFT },
];
const MAIL_GROUND: &[Prim] = &[Prim::Soft { prims: SHARED_GROUND }];

pub const STORE: &[Prim] = &[
    // ground (:168), composited
    Prim::Soft { prims: SHARED_GROUND },
    // header strip, y 43..69, dividers at x 467 and 1357: chrome,
    // standing from frame 0
    line_rect(49.0, 43.0, 1502.0, 26.0, Ink::Border, 1.5),
    vline(467.0, 43.0, 69.0, Ink::Border, 1.5),
    vline(1357.0, 43.0, 69.0, Ink::Border, 1.5),
    txt(61.0, 61.0, 15.0, Ink::Fg, "DIGITAL DISTRIBUTION SOFTWAREV2"),
    txt(506.0, 61.0, 15.0, Ink::Fg, "STORE ACCESS SCREEN"),
    txt(1372.0, 61.0, 15.0, Ink::Fg, "FLAIR TRS 5MMP"),
    // the body: everything between the strips, scanned down at boot
    // (`#body-scan`, :193, rect x 100 y 90 w 1500 h 730)
    Prim::Motion { motion: body_scan(100.0, 90.0, 1500.0, 730.0), prims: STORE_BODY },
    // footer strip, y 847..873, no dividers: the frame is chrome, the
    // strings type on (`#footer-type`, :206, x 52 y 835 w 1497 h 50)
    line_rect(52.0, 847.0, 1497.0, 26.0, Ink::Border, 1.5),
    Prim::Motion { motion: footer_type(52.0, 835.0, 1497.0, 50.0), prims: STORE_FOOTER },
];

/// The footer's three strings.
const STORE_FOOTER: &[Prim] = &[
    txt(61.0, 865.0, 15.0, Ink::Fg, "INTERFACE LOADED"),
    txt(506.0, 865.0, 15.0, Ink::Fg, "PROVIDED BY NEXUS NETWORK V10.8"),
    txt(1400.0, 865.0, 15.0, Ink::Fg, "BUILD 6.47.48441.R15"),
];

/// The store's body (:235-365), in the trace's paint order.
const STORE_BODY: &[Prim] = &[
    // 4ST logotype
    fill_path(164.0, 105.0, FOUR, Ink::Select),
    fill_path(264.0, 138.0, ESS, Ink::Select),
    shut_path(262.0, 102.0, TEE, Ink::Select, 2.0),
    Prim::Spaced { x: 138.0, y: 174.0, size: 16.0, ink: Ink::Fg, face: Face::Regular, pitch: 42.0, content: "STORE" },
    // customer block
    line_rect(113.0, 194.0, 217.0, 22.0, Ink::Border, 1.5),
    txt(118.0, 211.0, 14.0, Ink::Fg, "CUSTOMER"),
    txt_end(324.0, 211.0, 14.0, Ink::Fg, "#NC488402"),
    txt(118.0, 243.0, 14.0, Ink::Fg, "LOYALTY DISCOUNT"),
    txt_end(324.0, 243.0, 14.0, Ink::Fg, "10%"),
    txt(118.0, 259.0, 14.0, Ink::Fg, "LAST UPDATE"),
    txt_end(324.0, 259.0, 14.0, Ink::Fg, "10/05/2077"),
    // A: the category nav, one frame with the selection filled solid
    line_rect(112.0, 301.0, 218.0, 439.0, Ink::Border, 2.0),
    Prim::Plate { group: Group::Category, index: 0, x: 112.0, y: 301.0, w: 218.0, h: 57.0, on: NAV_ON_0, off: NAV_OFF_0 },
    Prim::Plate { group: Group::Category, index: 1, x: 112.0, y: 358.0, w: 218.0, h: 63.0, on: NAV_ON_1, off: NAV_OFF_1 },
    Prim::Plate { group: Group::Category, index: 2, x: 112.0, y: 421.0, w: 218.0, h: 61.0, on: NAV_ON_2, off: NAV_OFF_2 },
    Prim::Plate { group: Group::Category, index: 3, x: 112.0, y: 482.0, w: 218.0, h: 62.0, on: NAV_ON_3, off: NAV_OFF_3 },
    Prim::Plate { group: Group::Category, index: 4, x: 112.0, y: 544.0, w: 218.0, h: 62.0, on: NAV_ON_4, off: NAV_OFF_4 },
    fill_rect(112.0, 481.25, 218.0, 1.5, Ink::Border),
    fill_rect(112.0, 543.25, 218.0, 1.5, Ink::Border),
    fill_rect(112.0, 605.25, 218.0, 1.5, Ink::Border),
    // B: 265-wide cards on a 322 pitch; the fourth's primary ink
    // has a persistent open cut at x1564.6, before the screen edge.
    Prim::At { x: 461.0, y: 260.0, prims: SHELF_0 },
    Prim::At { x: 783.0, y: 260.0, prims: SHELF_1 },
    Prim::At { x: 1105.0, y: 260.0, prims: SHELF_2 },
    Prim::At { x: 1429.0, y: 260.0, prims: SHELF_3 },
    // bottom-left caption and the A / B letter boxes. The letters sit
    // BELOW the things they label on this screen. The caption is the
    // photo's faintest text; it took `Ink::Border` for that while the
    // role was the dim #5d7752, and MID since `border` brightened.
    txt(126.0, 787.0, 8.5, Ink::Mid, "SPARE TIME MANAGER WAS DEVELOPED BY"),
    txt(126.0, 796.0, 8.5, Ink::Mid, "SEOCHO. SERVING CUSTOMERS SINCE 2006."),
    line_rect(294.0, 779.0, 26.0, 26.0, Ink::Fg, 1.5),
    txt(300.0, 799.0, 19.0, Ink::Fg, "A"),
    line_rect(464.0, 779.0, 26.0, 26.0, Ink::Fg, 1.5),
    txt(470.0, 799.0, 19.0, Ink::Fg, "B"),
];

// --- end store -----------------------------------------------------------

// --- dashboard -----------------------------------------------------------
//
// `docs/entropism/dashboard-trace.svg`, transcribed: the module hub,
// measured off `images/entropism-store.png` (the two entropism source
// files are named the wrong way round, see `docs/sources.md`). Every
// figure below is the trace's own coordinate in the 1600x900 frame, in
// the trace's paint order; the comment on each group names the trace
// element it came from.
//
// Inks are the trace's sampled hex values. Two are the roles since
// 2026-09-05: the frame stroke is `Ink::Border` (`OUTLINE` #8fba97, the
// 1.25px core of every outlined frame at full resolution, trace header
// "Stroke profile") and the selected tile's fill is `Ink::Select`
// (`SAGE_SOLID` #a6d3a7). None of the other role consts above carries
// any of them (BG #110c07 vs the fitted ground #0f0903; SAGE_TEXT
// #94bb94 vs the label #acddb4; ON_SOLID #1f2a1c vs #22301f), so they
// are spelled here as block-local consts, the way `STORE_BAND` is.
// Reconciling the rest with the palette is ERAS-DELTA work, not this
// block's.

use crate::style::{hline, Anchor};

/// Ink on the solid fill: the selected tile's label and T2.
const HUB_ON_SOLID: iced::Color = rgb(0x22301f);
/// The selected tile's caption box, drawn dark on the fill.
const HUB_ON_CAPTION: iced::Color = rgb(0x2e4a2c);
/// Badge glyphs and the panel heading.
const HUB_LABEL: iced::Color = rgb(0xacddb4);
/// The five idle tile labels, fitted independently of the panel heading.
const HUB_TILE_LABEL: iced::Color = rgb(0xa2d2a8);
/// Header and footer strings.
const HUB_STRIP: iced::Color = rgb(0x97c4a0);
/// The panel's lorem body copy.
const HUB_BODY: iced::Color = rgb(0xadd2aa);
/// The idle tiles' caption box: heavier and brighter than the frames.
const HUB_CAPTION: iced::Color = rgb(0xa8d7a7);
/// The caption lettering is a little dimmer than its rules.
const HUB_CAPTION_TEXT: iced::Color = rgb(0xa2cfa7);
/// The A / B / C letter boxes.
const HUB_LETTER_BOX: iced::Color = rgb(0x9ac3a0);
/// The boxed letters and their MAIL BOX / MESSAGE / SECURITY LEVEL labels.
const HUB_SECTION: iced::Color = rgb(0xa0d2a9);

/// Rajdhani 500 (`font-weight="500"`), start-anchored.
const fn medium(x: f32, y: f32, size: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Text { x, y, size, ink, face: Face::Medium, anchor: Anchor::Start, content }
}

/// Rajdhani 600 (`font-weight="600"`), start-anchored.
const fn semibold(x: f32, y: f32, size: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Text { x, y, size, ink, face: Face::SemiBold, anchor: Anchor::Start, content }
}

/// Rajdhani 600, `text-anchor="middle"`: the tile labels.
const fn label(x: f32, y: f32, ink: Ink, content: &'static str) -> Prim {
    Prim::Text { x, y, size: 22.0, ink, face: Face::SemiBold, anchor: Anchor::Middle, content }
}

/// A stretched glyph run, trace `transform="translate(x,y) scale(sx,1)"`,
/// start-anchored: the caption cells and the panel's body copy.
const fn wide(x: f32, y: f32, size: f32, stretch: f32, ink: Ink, face: Face, content: &'static str) -> Prim {
    Prim::Wide { x, y, size, stretch, ink, face, anchor: Anchor::Start, content }
}

/// [`wide`] with `text-anchor="middle"`: `x` is the trace's
/// `translate()` x, the centre of the stretched run. The section
/// letters and the T1-T4 badges, which until 2026-09-07 were `wide` at
/// the centre less half a run measured by hand.
const fn wide_mid(x: f32, y: f32, size: f32, stretch: f32, ink: Ink, face: Face, content: &'static str) -> Prim {
    Prim::Wide { x, y, size, stretch, ink, face, anchor: Anchor::Middle, content }
}

// One caption box, trace `<g id="caption">` (defs): drawn at the tile's
// foot-left corner with the tile's own frame at y 0 and the box's bottom
// edge lying on it. Top rule 28px up, divider 96px in, both 1.7px (the
// def's `<path d="M 0,-28 H 194 M 96,-28 V 0">`), two cells of 600-weight
// text stretched to the measured runs.
macro_rules! caption {
    ($rule_ink:expr, $text_ink:expr) => {
        &[
            hline(0.0, -28.0, 194.0, $rule_ink, 1.7),
            vline(96.0, -28.0, 0.0, $rule_ink, 1.7),
            wide(6.0, -15.0, 12.0, 0.78, $text_ink, Face::SemiBold, "85SD4F3Q5S41"),
            wide(103.0, -15.0, 12.0, 0.685, $text_ink, Face::SemiBold, "COMBAT COLONIZATION"),
            wide(103.0, -6.0, 10.5, 0.82, $text_ink, Face::SemiBold, "DEFENCE PROGRAM"),
        ]
    };
}
/// The box as the five idle tiles wear it, `<use href="#caption"
/// fill="#a2cfa7" stroke="#a8d7a7">`.
const CAPTION: &[Prim] = caption!(Ink::Fixed(HUB_CAPTION), Ink::Fixed(HUB_CAPTION_TEXT));
/// The box as the selected tile wears it, `<use href="#caption"
/// fill="#2e4a2c" stroke="#2e4a2c">` -- dark on the sage fill.
const CAPTION_ON: &[Prim] = caption!(Ink::Fixed(HUB_ON_CAPTION), Ink::Fixed(HUB_ON_CAPTION));

// One menu tile, foot-anchored like the caption def so the `At` that
// places it carries the trace's own `<use href="#caption" x y>`
// coordinates. `$h` is the idle frame's height (row 1 tiles are 212
// tall, row 2 tiles 211); the labels are `(x, y)` relative to the foot,
// one per line. Two dresses:
//
//   on   the selection as the trace draws BRAINDANCE (row 1 tile 3,
//        "the selection: solid sage fill, dark caption box and text"):
//        a 194x211 solid with no outline, sitting 1px lower than its
//        idle neighbours -- `components.svg` K3 "HUB TILE, SELECTED"
//        keeps that 1px as the measurement, and so does this;
//   off  the idle dress of the other five: a 1.25px outlined frame,
//        bright label, bright caption box (`components.svg` K3 "HUB
//        TILE, PLAIN").
//
// The trace only draws each tile in one state; the other is derived
// from its siblings as above (trace header "BRAINDANCE (row 1, tile 3)
// is filled solid sage -- the selection ... On the selected tile the box
// and text are dark on the sage fill").
macro_rules! tile {
    ($h:expr, $( ($lx:expr, $ly:expr, $label:expr) ),+) => {
        (
            &[
                // the tile's fill lights (`#select-lit`); its label does not
                Prim::Motion { motion: SELECT_LIT, prims: &[fill_rect(0.0, -211.0, 194.0, 211.0, Ink::Select)] },
                $( label($lx, $ly, Ink::Fixed(HUB_ON_SOLID), $label), )+
                Prim::At { x: 0.0, y: 0.0, prims: CAPTION_ON },
            ],
            &[
                line_rect(0.0, -$h, 194.0, $h, Ink::Border, 1.25),
                $( label($lx, $ly, Ink::Fixed(HUB_TILE_LABEL), $label), )+
                Prim::At { x: 0.0, y: 0.0, prims: CAPTION },
            ],
        )
    };
}
// Row 1, foot y 438: labels at baseline 327 (-111). The trace's label x
// values are not all at the tile's centre (221 / 514 / 813 for tiles at
// 128 / 418 / 716), so each is its own offset.
const TILE_ON_0: &[Prim] = tile!(212.0, (93.0, -111.0, "EMAILS")).0;
const TILE_OFF_0: &[Prim] = tile!(212.0, (93.0, -111.0, "EMAILS")).1;
const TILE_ON_1: &[Prim] = tile!(212.0, (96.0, -111.0, "MATRIX")).0;
const TILE_OFF_1: &[Prim] = tile!(212.0, (96.0, -111.0, "MATRIX")).1;
const TILE_ON_2: &[Prim] = tile!(212.0, (97.0, -111.0, "BRAINDANCE")).0;
const TILE_OFF_2: &[Prim] = tile!(212.0, (97.0, -111.0, "BRAINDANCE")).1;
// Row 2, foot y 710: SECURITY / SYSTEMS on two lines at 591 / 615
// (-119 / -95), PRIVATE and DEVICES at 599 (-111).
const TILE_ON_3: &[Prim] = tile!(211.0, (94.0, -119.0, "SECURITY"), (92.0, -95.0, "SYSTEMS")).0;
const TILE_OFF_3: &[Prim] = tile!(211.0, (94.0, -119.0, "SECURITY"), (92.0, -95.0, "SYSTEMS")).1;
const TILE_ON_4: &[Prim] = tile!(211.0, (96.0, -111.0, "PRIVATE")).0;
const TILE_OFF_4: &[Prim] = tile!(211.0, (96.0, -111.0, "PRIVATE")).1;
const TILE_ON_5: &[Prim] = tile!(211.0, (98.0, -111.0, "DEVICES")).0;
const TILE_OFF_5: &[Prim] = tile!(211.0, (98.0, -111.0, "DEVICES")).1;

/// The six tiles as plates, hit box = the idle frame, foot-anchored.
macro_rules! module {
    ($i:expr, $h:expr, $on:expr, $off:expr) => {
        &[Prim::Plate {
            group: Group::Module,
            index: $i,
            x: 0.0,
            y: -$h,
            w: 194.0,
            h: $h,
            on: $on,
            off: $off,
        }]
    };
}
const MODULE_0: &[Prim] = module!(0, 212.0, TILE_ON_0, TILE_OFF_0);
const MODULE_1: &[Prim] = module!(1, 212.0, TILE_ON_1, TILE_OFF_1);
const MODULE_2: &[Prim] = module!(2, 212.0, TILE_ON_2, TILE_OFF_2);
const MODULE_3: &[Prim] = module!(3, 211.0, TILE_ON_3, TILE_OFF_3);
const MODULE_4: &[Prim] = module!(4, 211.0, TILE_ON_4, TILE_OFF_4);
const MODULE_5: &[Prim] = module!(5, 211.0, TILE_ON_5, TILE_OFF_5);

pub const DASHBOARD: &[Prim] = &[
    Prim::Soft { prims: SHARED_GROUND },
    // header strip, y 43..69, dividers at x 465 and 1353
    line_rect(49.0, 43.0, 1498.0, 26.0, Ink::Border, 1.25),
    vline(465.0, 43.0, 69.0, Ink::Border, 1.25),
    vline(1353.0, 43.0, 69.0, Ink::Border, 1.25),
    medium(61.0, 60.0, 17.0, Ink::Fixed(HUB_STRIP), "RIPPERDOC SURGICAL SOFTWAREV2"),
    medium(518.0, 60.0, 17.0, Ink::Fixed(HUB_STRIP), "STORE ACCESS SCREEN"),
    medium(1382.0, 60.0, 17.0, Ink::Fixed(HUB_STRIP), "FLAIR TRS 5MMP"),
    // the body: everything between the strips, scanned down at boot
    // (`#body-scan`, :156, rect x 100 y 130 w 1400 h 605)
    Prim::Motion { motion: body_scan(100.0, 130.0, 1400.0, 605.0), prims: HUB_SCAN },
    // footer strip, y 847..872, no dividers: the frame is chrome, the
    // strings type on (`#footer-type`, :169, x 49 y 835 w 1498 h 50)
    line_rect(49.0, 847.0, 1498.0, 25.0, Ink::Border, 1.25),
    Prim::Motion { motion: footer_type(49.0, 835.0, 1498.0, 50.0), prims: HUB_FOOTER },
];

/// The footer's three strings; only BUILD is end-anchored.
const HUB_FOOTER: &[Prim] = &[
    medium(61.0, 865.0, 17.0, Ink::Fixed(HUB_STRIP), "INTERFACE LOADED"),
    medium(518.0, 865.0, 17.0, Ink::Fixed(HUB_STRIP), "PROVIDED BY NEXUS NETWORK V10.8"),
    Prim::Text { x: 1525.0, y: 865.0, size: 17.0, ink: Ink::Fixed(HUB_STRIP), face: Face::Medium, anchor: Anchor::End, content: "BUILD 6.47.48441.R15" },
];

/// The dashboard's body (:198-316), in the trace's paint order.
const HUB_SCAN: &[Prim] = &[
    // section headings: 26x26 boxes holding a bold letter stretched
    // 1.5-1.6, centred at the trace's translate() x (:154-158; a run
    // measured by hand placed them until 2026-09-07)
    line_rect(133.0, 145.0, 26.0, 26.0, Ink::Fixed(HUB_LETTER_BOX), 1.5),
    wide_mid(146.0, 165.0, 22.0, 1.6, Ink::Fixed(HUB_SECTION), Face::Bold, "A"),
    line_rect(1014.0, 145.0, 26.0, 26.0, Ink::Fixed(HUB_LETTER_BOX), 1.5),
    wide_mid(1027.0, 165.0, 22.0, 1.5, Ink::Fixed(HUB_SECTION), Face::Bold, "B"),
    line_rect(1329.0, 142.0, 26.0, 26.0, Ink::Fixed(HUB_LETTER_BOX), 1.5),
    wide_mid(1342.0, 162.0, 22.0, 1.5, Ink::Fixed(HUB_SECTION), Face::Bold, "C"),
    semibold(169.0, 164.0, 23.0, Ink::Fixed(HUB_SECTION), "MAIL BOX"),
    semibold(1047.0, 164.0, 23.0, Ink::Fixed(HUB_SECTION), "MESSAGE"),
    semibold(1380.0, 162.0, 23.0, Ink::Fixed(HUB_SECTION), "SECURITY"),
    semibold(1379.0, 186.0, 23.0, Ink::Fixed(HUB_SECTION), "LEVEL"),
    // the 3x2 tile grid, each placed at its caption `<use x y>`: row 1
    // feet at y 438 (frames y 226..438), row 2 at y 710 (y 499..710),
    // columns x 128 / 418 / 716
    Prim::At { x: 128.0, y: 438.0, prims: MODULE_0 },
    Prim::At { x: 418.0, y: 438.0, prims: MODULE_1 },
    Prim::At { x: 716.0, y: 438.0, prims: MODULE_2 },
    Prim::At { x: 128.0, y: 710.0, prims: MODULE_3 },
    Prim::At { x: 418.0, y: 710.0, prims: MODULE_4 },
    Prim::At { x: 716.0, y: 710.0, prims: MODULE_5 },
    // MESSAGE detail panel, x 1014..1275, y 215..723: heading over a
    // full-width rule at y 281 (`<path d="M 1014,281 H 1275">`)
    line_rect(1014.0, 215.0, 261.0, 508.0, Ink::Border, 1.25),
    semibold(1033.0, 264.0, 22.0, Ink::Fixed(HUB_LABEL), "BRAINDANCE"),
    hline(1014.0, 281.0, 1275.0, Ink::Border, 1.25),
    // body copy: 7 lines, blank, 3 lines on a 21px pitch. The SVG group
    // uses `translate(1032.2,1.25) scale(1.135,1)`; native Iced glyphs
    // land about 2px lower, so its baselines are 0.85 design px higher.
    wide(1032.2, 321.4, 17.0, 1.135, Ink::Fixed(HUB_BODY), Face::Medium, "Lorem ipsum dolor sit amet,"),
    wide(1032.2, 342.4, 17.0, 1.135, Ink::Fixed(HUB_BODY), Face::Medium, "consectetur adipiscing elit,"),
    wide(1032.2, 363.4, 17.0, 1.135, Ink::Fixed(HUB_BODY), Face::Medium, "sed do eiusmod tempor inci-"),
    wide(1032.2, 384.4, 17.0, 1.135, Ink::Fixed(HUB_BODY), Face::Medium, "didunt ut labore et dolore"),
    wide(1032.2, 405.4, 17.0, 1.135, Ink::Fixed(HUB_BODY), Face::Medium, "magna aliqua. Quis ipsum"),
    wide(1032.2, 426.4, 17.0, 1.135, Ink::Fixed(HUB_BODY), Face::Medium, "suspendisse ultrices gravi-"),
    wide(1032.2, 447.4, 17.0, 1.135, Ink::Fixed(HUB_BODY), Face::Medium, "da."),
    wide(1032.2, 489.4, 17.0, 1.135, Ink::Fixed(HUB_BODY), Face::Medium, "Risus commodo viverra ma-"),
    wide(1032.2, 510.4, 17.0, 1.135, Ink::Fixed(HUB_BODY), Face::Medium, "ecenas accumsan lacus vel"),
    wide(1032.2, 531.4, 17.0, 1.135, Ink::Fixed(HUB_BODY), Face::Medium, "facilisis."),
    // SECURITY LEVEL badges: four 68x68 at x 1380, T2 filled. Glyphs
    // bold 27 stretched to the measured runs, centred at x 1414
    // (:227-233)
    line_rect(1380.0, 214.0, 68.0, 68.0, Ink::Border, 1.25),
    wide_mid(1414.0, 257.0, 27.0, 1.37, Ink::Fixed(HUB_LABEL), Face::Bold, "T1"),
    // T2's fill lights with the tile's (`#select-lit`, :237)
    Prim::Motion { motion: SELECT_LIT, prims: &[fill_rect(1380.0, 304.0, 68.0, 68.0, Ink::Select)] },
    wide_mid(1414.0, 346.0, 27.0, 1.42, Ink::Fixed(HUB_ON_SOLID), Face::Bold, "T2"),
    line_rect(1380.0, 393.0, 68.0, 68.0, Ink::Border, 1.25),
    wide_mid(1414.0, 435.0, 27.0, 1.5, Ink::Fixed(HUB_LABEL), Face::Bold, "T3"),
    line_rect(1380.0, 482.0, 68.0, 68.0, Ink::Border, 1.25),
    wide_mid(1414.0, 524.0, 27.0, 1.38, Ink::Fixed(HUB_LABEL), Face::Bold, "T4"),
];

// --- end dashboard -------------------------------------------------------
