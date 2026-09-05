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
//! Ground: the traces are not flat. All four sit on a radial lift
//! (`login-trace.svg` `<radialGradient id="lift" cx="0.45" cy="0.4"
//! r="0.8">` #1a1810 / #141107 / #0f0a04; mailbox #1c1b11 / #151207 /
//! #100a03; store #1e1d12 / #151207 / #100b03; dashboard #1c1a10 /
//! #141107 / #0f0a03). `ground: Ground::Flat` stands as a deliberate
//! choice (the mailbox block below reads the lift as the photograph's
//! falloff, on the trace's own instruction).
//!
//! The predecessor crate (`entropism-ui`) carried twelve colours --
//! including cybr's red, cyan, mint, violet, orange and gold -- and a
//! radial glow module. None of that is in the reference; see
//! `docs/entropism/README.md`. This is the one-hue system.

use crate::palette::{rgb, Ornaments, Palette};
use crate::style::{
    Banner, Bar, BarChrome, BarGround, BarMenu, BarOrnament, Chrome, Compliance, Corner, Dress,
    Era, Face, Footnotes, Ground, Ink, MenuMarker, MenuRule, Metrics, Nameplate,
    PanelEcho, Selection, Style, Ticket, WindowLabel,
};
use crate::widgets::surface::Corners;
// --- login ---
use crate::style::{Access, Colophon, Fixture, Legend, Masthead, Plate, Plot, Slot};
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
        // --- end store ---
        // --- dashboard ---
        dashboard: DASHBOARD,
        // BRAINDANCE, row 1 tile 3: the one tile dashboard-trace.svg
        // fills solid (`<rect x="716" y="227" ... fill="#a6d3a7"/>`
        // under "the selection: solid sage fill, dark caption box and
        // text").
        dashboard_selection: 2,
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

/// `#lift` (login-trace.svg :2-6), `cx 0.45 cy 0.4 r 0.8` of the
/// page: centre (720,360), radii (1280,720), its own hex -- the store
/// measured its own `STORE_LIFT`, the hub its `LIFT`. The rim colour
/// is also the page under it, because the frame's far corner sits at
/// t 1.016, just outside the ellipse.
///
/// Why the screen has a lift at all when `Ground::Flat` is the era:
/// the shape gate proved it. The trace's lift is bright enough at its
/// centre to be a palette cluster of its own that never reaches the
/// frame edge, so the extractor bins it as *ink* and reads a 1044x586
/// shape in the middle of the screen -- 72% of the design's shape
/// area, against a flat ground that offers nothing to match it.
const LOGIN_LIFT: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x1a1810)),
    (0.7, rgb(0x141107)),
    (1.0, rgb(0x0f0a04)),
];
const LOGIN_GROUND: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(rgb(0x0f0a04))),
    Prim::Lobe { x: 720.0, y: 360.0, rx: 1280.0, ry: 720.0, stops: LOGIN_LIFT },
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
/// run bold 22 at natural tracking; NEXT medium ls 2 at x 940 baseline
/// 433; the footer strings to baseline 863 and x 61/519/1383.
pub const ACCESS: Access = Access {
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
        // Eleven asterisks, and the photo's are 7px glyphs at a ~9.7
        // pitch, not the 4-5px at pitch 10 an earlier reading drew: a
        // bold 22 at natural tracking, run 94 wide.
        value: Some(Legend::new("***********", 578.0, 439.0, 22.0, Ink::Fg).bold()),
        // The short caret underline under the first pair of characters.
        caret: Some(Plate::filled(Plot::new(577.0, 439.5, 17.0, 1.25), Ink::Border)),
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
        plate: Plate::filled(Plot::new(36.0, 765.0, 1529.0, 115.0), Ink::Select),
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
//   * the trace's `lift` radial ground is the photograph's falloff;
//     `Ground::Flat` stands.
//
// Outline ink is `Ink::Border`. It was `Ink::Mid` until 2026-09-05
// because the trace samples every frame at #709174, next to the era's
// `MID` #728f76 and a stop above the `OUTLINE` of the time (#5d7752);
// the #709174 is the 1600 rescale's dilution of a bright 2px line,
// which is what `border` now is (#8fba97), so the frames read the role
// like the hub's and the store's. The two 8.5px captions under A MAIL
// BOX stay `Ink::Mid`: they are the screen's faintest text, not frames.

use crate::style::{
    Frame, Mail, MailBadges, MailButtons, MailList, MailPanel, Mailbox, Note, Piece,
    RowDecor, Run, Trim, FromAt,
};

/// Header strip, footer strip, the three boxed section letters, and
/// every fixed string the trace prints around them.
static CHROME: [Piece; 22] = [
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
    // A MAIL BOX, with the two lines of micro-print under it
    Piece::Box {
        at: Frame::new(100.0, 98.0, 26.0, 26.0),
        fill: None,
        stroke: Some(Ink::Fg),
        width: 1.5,
        trim: Trim::NONE,
    },
    Piece::Label(Note {
        at: Run::new(106.0, 118.0, 19.0, Ink::Fg),
        text: "A",
    }),
    Piece::Label(Note {
        at: Run::new(139.0, 119.0, 22.0, Ink::Fg),
        text: "MAIL BOX",
    }),
    Piece::Label(Note {
        at: Run::new(102.0, 145.0, 8.5, Ink::Mid),
        text: "SPARE TIME MANAGER WAS DEVELOPED BY",
    }),
    Piece::Label(Note {
        at: Run::new(102.0, 155.0, 8.5, Ink::Mid),
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
        at: Run::new(562.0, 118.0, 19.0, Ink::Fg),
        text: "B",
    }),
    Piece::Label(Note {
        at: Run::new(592.0, 119.0, 22.0, Ink::Fg),
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
        at: Run::new(1353.0, 118.0, 19.0, Ink::Fg),
        text: "C",
    }),
    Piece::Label(Note {
        at: Run::new(1383.0, 119.0, 22.0, Ink::Fg),
        text: "ENCRIPTION",
    }),
    Piece::Label(Note {
        at: Run::new(1383.0, 144.0, 22.0, Ink::Fg),
        text: "LEVEL",
    }),
    // footer strip x 49..1547, y 847..873, no dividers
    Piece::Box {
        at: Frame::new(49.0, 847.0, 1498.0, 26.0),
        fill: None,
        stroke: Some(Ink::Border),
        width: 2.0,
        trim: Trim::NONE,
    },
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

static BUTTONS: [&str; 4] = ["REPLY", "FORWARD", "DELETE", "REPORT SPAM"];
/// The trace reads them T1 T3 over T2 T4, and T2 -- bottom left -- is
/// the filled one.
static LEVELS: [&str; 4] = ["T1", "T3", "T2", "T4"];

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

/// The body, trace lines 233-242: 4 + 4 + 2 lines, each set with its
/// own `textLength`, broken where the trace breaks them.
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
        // this era's mailbox is content with its `Ground`
        backdrop: &[],
        chrome: &CHROME,
        overlay: &[],
        list: MailList {
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
            selected: 0,
            decor: RowDecor::Framed,
            row_fill: None,
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
            glyph_w: 17.0,
            text_x: 138.0,
            title_dy: 30.0,
            title_size: 20.0,
            title_bold: false,
            from_dy: 48.0,
            from_size: 14.0,
            from_at: FromAt::Beneath,
            from_prefix: "FROM: ",
            title_upper: true,
            from_upper: true,
            new_pill: None,
            icons: None,
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
            title: Run::new(557.0, 253.0, 22.0, Ink::OnSelect),
            title_upper: true,
            from: Some(Run::new(557.0, 279.0, 17.0, Ink::OnSelect)),
            heading: None,
            // trace line 229: the list shouts "FROM: MOM", the panel
            // does not
            sender: Some("from: Mom"),
            body: Run::new(557.0, 325.0, 17.0, Ink::Fg),
            line: 21.7,
            para: 39.0,
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
            label: Run::new(20.0, 25.0, 22.0, Ink::Fg),
            tab: None,
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
            label: Run::new(35.0, 43.5, 27.0, Ink::Fg).bold().centered(),
            caption: None,
            caption_text: "",
            labels: &LEVELS,
        },
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

/// The rifle illustration's outline envelope, card-local. The photo has
/// a detailed line drawing; the trace keeps its four blocks.
const RIFLE_STOCK: &[Seg] = &[
    Seg::Line(242.0, 108.0),
    Seg::Line(242.0, 157.0),
    Seg::Line(226.0, 157.0),
    Seg::Line(226.0, 136.0),
    Seg::Line(154.0, 136.0),
];

/// An unselected product card, at its own origin. 265 wide, outlined,
/// with the socket row hung off its foot so the two frames share an
/// edge -- which is why the extractor reads them as one component.
const CARD: &[Prim] = &[
    line_rect(0.0, 0.0, 265.0, 237.0, Ink::Border, 2.0),
    txt(12.0, 26.0, 24.0, Ink::Select, "MAGNUM 650"),
    txt(12.0, 45.0, 20.0, Ink::Select, "HAND GUN"),
    fill_rect(3.0, 55.0, 259.0, 20.0, Ink::Fixed(STORE_BAND)),
    fill_rect(3.0, 62.0, 58.0, 11.0, Ink::Fixed(STORE_ON_BAND)),
    txt(6.0, 71.0, 9.5, Ink::Fixed(STORE_BAND), "PETROCHEM"),
    txt_end(259.0, 71.0, 9.5, Ink::Fixed(STORE_ON_BAND), "BETTERLIFE TEC"),
    // the rifle: fill in the bright ink, hairlined in the solid
    Prim::Rect { x: 60.0, y: 102.0, w: 94.0, h: 48.0, fill: Some(Ink::Fg), stroke: Some(Ink::Select), width: 1.0 },
    Prim::Rect { x: 36.0, y: 116.0, w: 40.0, h: 41.0, fill: Some(Ink::Fg), stroke: Some(Ink::Select), width: 1.0 },
    Prim::Rect { x: 150.0, y: 125.0, w: 21.0, h: 32.0, fill: Some(Ink::Fg), stroke: Some(Ink::Select), width: 1.0 },
    Prim::Path { x: 154.0, y: 108.0, segs: RIFLE_STOCK, close: true, fill: Some(Ink::Fg), stroke: Some(Ink::Select), width: 1.0 },
    txt_mid(41.0, 200.0, 20.0, Ink::Select, "DPS"),
    txt_mid(102.0, 200.0, 20.0, Ink::Select, "PNT"),
    txt_mid(164.0, 200.0, 20.0, Ink::Select, "ACC"),
    txt_mid(225.0, 200.0, 20.0, Ink::Select, "ROF"),
    fill_rect(5.0, 209.0, 255.0, 25.0, Ink::Select),
    txt_mid(41.0, 230.0, 22.0, Ink::OnSelect, "86"),
    txt_mid(102.0, 230.0, 22.0, Ink::OnSelect, "30"),
    txt_mid(164.0, 230.0, 22.0, Ink::OnSelect, "5"),
    txt_mid(225.0, 230.0, 22.0, Ink::OnSelect, "5"),
    // socket row y 237..286, dividers at 52 / 119 / 191
    line_rect(0.0, 237.0, 265.0, 49.0, Ink::Border, 2.0),
    vline(52.0, 237.0, 286.0, Ink::Border, 1.5),
    vline(119.0, 237.0, 286.0, Ink::Border, 1.5),
    vline(191.0, 237.0, 286.0, Ink::Border, 1.5),
    Prim::Dots { x: 10.5, y: 246.5, cell: 3.8, pitch: 3.5, ink: Ink::Select, rows: QR },
    txt_mid(85.0, 258.0, 12.0, Ink::Select, "EMPTY"),
    txt_mid(85.0, 272.0, 12.0, Ink::Select, "SOCKET"),
    txt_mid(155.0, 258.0, 12.0, Ink::Select, "EMPTY"),
    txt_mid(155.0, 272.0, 12.0, Ink::Select, "SOCKET"),
    txt_mid(228.0, 258.0, 12.0, Ink::Select, "EMPTY"),
    txt_mid(228.0, 272.0, 12.0, Ink::Select, "SOCKET"),
    txt(5.0, 304.0, 7.5, Ink::Fg, "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO"),
    txt(5.0, 314.0, 7.5, Ink::Fg, "MANIPULATE, ACCESS OR DISABLE THIS DEVICE."),
];

/// The grown card: the header block down through the values row is one
/// solid, the outline runs 412 tall, and the detail block takes the
/// room the unselected card spends on its compliance notice.
const GROWN: &[Prim] = &[
    fill_rect(0.0, 0.0, 265.0, 234.0, Ink::Select),
    line_rect(0.0, 0.0, 265.0, 412.0, Ink::Border, 2.0),
    txt(13.0, 26.0, 24.0, Ink::OnSelect, "MAGNUM 650"),
    txt(13.0, 45.0, 20.0, Ink::OnSelect, "HAND GUN"),
    fill_rect(3.0, 55.0, 259.0, 20.0, Ink::Fixed(STORE_BAND)),
    fill_rect(3.0, 62.0, 58.0, 11.0, Ink::Fixed(STORE_ON_BAND)),
    txt(6.0, 71.0, 9.5, Ink::Fixed(STORE_BAND), "PETROCHEM"),
    txt_end(259.0, 71.0, 9.5, Ink::Fixed(STORE_ON_BAND), "BETTERLIFE TEC"),
    Prim::Rect { x: 60.0, y: 102.0, w: 94.0, h: 48.0, fill: Some(Ink::OnSelect), stroke: Some(Ink::Dim), width: 1.0 },
    Prim::Rect { x: 36.0, y: 116.0, w: 40.0, h: 41.0, fill: Some(Ink::OnSelect), stroke: Some(Ink::Dim), width: 1.0 },
    Prim::Rect { x: 150.0, y: 125.0, w: 21.0, h: 32.0, fill: Some(Ink::OnSelect), stroke: Some(Ink::Dim), width: 1.0 },
    Prim::Path { x: 154.0, y: 108.0, segs: RIFLE_STOCK, close: true, fill: Some(Ink::OnSelect), stroke: Some(Ink::Dim), width: 1.0 },
    txt_mid(41.0, 200.0, 20.0, Ink::OnSelect, "DPS"),
    txt_mid(102.0, 200.0, 20.0, Ink::OnSelect, "PNT"),
    txt_mid(164.0, 200.0, 20.0, Ink::OnSelect, "ACC"),
    txt_mid(225.0, 200.0, 20.0, Ink::OnSelect, "ROF"),
    fill_rect(0.0, 207.25, 265.0, 1.5, Ink::OnSelect),
    txt_mid(41.0, 230.0, 22.0, Ink::OnSelect, "86"),
    txt_mid(102.0, 230.0, 22.0, Ink::OnSelect, "30"),
    txt_mid(164.0, 230.0, 22.0, Ink::OnSelect, "5"),
    txt_mid(225.0, 230.0, 22.0, Ink::OnSelect, "5"),
    // detail block, y 494..672 on the page
    txt(17.0, 270.0, 19.0, Ink::Select, "20"),
    txt(51.0, 270.0, 19.0, Ink::Select, "Recoil"),
    txt(17.0, 291.0, 19.0, Ink::Select, "22"),
    txt(51.0, 291.0, 19.0, Ink::Select, "Sperad"),
    txt(17.0, 312.0, 19.0, Ink::Select, "12"),
    txt(51.0, 312.0, 19.0, Ink::Select, "Range"),
    txt(17.0, 348.0, 19.0, Ink::Select, "Bonus"),
    txt(17.0, 368.0, 19.0, Ink::Select, "+9 Reflexes"),
    txt(17.0, 390.0, 19.0, Ink::Select, "+2 Modules Slots"),
    line_rect(0.0, 412.0, 265.0, 48.0, Ink::Border, 2.0),
    vline(52.0, 412.0, 460.0, Ink::Border, 1.5),
    vline(120.0, 412.0, 460.0, Ink::Border, 1.5),
    vline(192.0, 412.0, 460.0, Ink::Border, 1.5),
    Prim::Dots { x: 10.5, y: 420.5, cell: 3.8, pitch: 3.5, ink: Ink::Select, rows: QR },
    txt_mid(86.0, 433.0, 12.0, Ink::Select, "EMPTY"),
    txt_mid(86.0, 447.0, 12.0, Ink::Select, "SOCKET"),
    txt_mid(156.0, 433.0, 12.0, Ink::Select, "EMPTY"),
    txt_mid(156.0, 447.0, 12.0, Ink::Select, "SOCKET"),
    txt_mid(228.0, 433.0, 12.0, Ink::Select, "EMPTY"),
    txt_mid(228.0, 447.0, 12.0, Ink::Select, "SOCKET"),
    txt(5.0, 478.0, 7.5, Ink::Fg, "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO"),
    txt(5.0, 488.0, 7.5, Ink::Fg, "MANIPULATE, ACCESS OR DISABLE THIS DEVICE."),
];

/// The "4" of the logotype, drawn at the extractor's own bbox for it.
/// Two subpaths, filled even-odd: the counter is the hole.
const FOUR: &[Seg] = &[
    Seg::Line(197.0, 105.0),
    Seg::Line(197.0, 151.0),
    Seg::Line(170.0, 151.0),
    Seg::Line(170.0, 139.0),
    Seg::Line(138.0, 139.0),
    Seg::Line(138.0, 126.0),
    Seg::Line(165.0, 105.0),
    Seg::Move(155.0, 126.0),
    Seg::Line(170.0, 126.0),
    Seg::Line(170.0, 114.0),
];

const ESS: &[Seg] = &[
    Seg::Line(254.0, 102.0),
    Seg::Line(254.0, 116.0),
    Seg::Line(216.0, 116.0),
    Seg::Line(216.0, 120.0),
    Seg::Line(254.0, 120.0),
    Seg::Line(254.0, 151.0),
    Seg::Line(202.0, 151.0),
    Seg::Line(202.0, 137.0),
    Seg::Line(240.0, 137.0),
    Seg::Line(240.0, 133.0),
    Seg::Line(202.0, 133.0),
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
                fill_rect(112.0, $top, 218.0, $h, Ink::Select),
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
const SHELF_3: &[Prim] = shelf!(3);

/// The store's `#lift` (:2-6), `cx 0.45 cy 0.4 r 0.8` of the page:
/// centre (720,360), radii (1280,720), padded past the rim with its
/// last stop. Not the dashboard's `LIFT` (cy 0.45, r 0.75, its own
/// hex): each trace measured its own photo. Until 2026-09-04 the store
/// drew no lift at all -- flat `17 12 7` where the design runs
/// `22 20 9` .. `28 27 16`, 5-11 levels under it everywhere.
const STORE_LIFT: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x1e1d12)),
    (0.7, rgb(0x151207)),
    (1.0, rgb(0x100b03)),
];
const STORE_GROUND: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(rgb(0x100b03))),
    Prim::Lobe { x: 720.0, y: 360.0, rx: 1280.0, ry: 720.0, stops: STORE_LIFT },
];

pub const STORE: &[Prim] = &[
    // ground (:168), composited
    Prim::Soft { prims: STORE_GROUND },
    // header strip, y 43..69, dividers at x 467 and 1357
    line_rect(49.0, 43.0, 1502.0, 26.0, Ink::Border, 1.5),
    vline(467.0, 43.0, 69.0, Ink::Border, 1.5),
    vline(1357.0, 43.0, 69.0, Ink::Border, 1.5),
    txt(61.0, 61.0, 15.0, Ink::Fg, "DIGITAL DISTRIBUTION SOFTWAREV2"),
    txt(506.0, 61.0, 15.0, Ink::Fg, "STORE ACCESS SCREEN"),
    txt(1372.0, 61.0, 15.0, Ink::Fg, "FLAIR TRS 5MMP"),
    // 4ST logotype
    fill_path(170.0, 105.0, FOUR, Ink::Select),
    fill_path(202.0, 102.0, ESS, Ink::Select),
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
    // B: the shelf. 265 wide on a 322 pitch; the fourth runs off the
    // right edge of the frame, as the source has it.
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
    // footer strip, y 847..873, no dividers
    line_rect(52.0, 847.0, 1497.0, 26.0, Ink::Border, 1.5),
    txt(61.0, 865.0, 15.0, Ink::Fg, "INTERFACE LOADED"),
    txt(506.0, 865.0, 15.0, Ink::Fg, "PROVIDED BY NEXUS NETWORK V10.8"),
    txt(1400.0, 865.0, 15.0, Ink::Fg, "BUILD 6.47.48441.R15"),
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
// any of them (BG #110c07 vs the trace's ground #0f0a03; SAGE_TEXT
// #94bb94 vs the label #acddb4; ON_SOLID #1f2a1c vs #22301f), so they
// are spelled here as block-local consts, the way `STORE_BAND` is.
// Reconciling the rest with the palette is ERAS-DELTA work, not this
// block's.

use crate::style::{hline, Anchor};

/// Ink on the solid fill: the selected tile's label and T2.
const HUB_ON_SOLID: iced::Color = rgb(0x22301f);
/// The selected tile's caption box, drawn dark on the fill.
const HUB_ON_CAPTION: iced::Color = rgb(0x2e4a2c);
/// Tile labels, badge glyphs and the panel heading.
const HUB_LABEL: iced::Color = rgb(0xacddb4);
/// Header and footer strings.
const HUB_STRIP: iced::Color = rgb(0x97c4a0);
/// The panel's lorem body copy.
const HUB_BODY: iced::Color = rgb(0x9fc09c);
/// The idle tiles' caption box: heavier and brighter than the frames.
const HUB_CAPTION: iced::Color = rgb(0xa8d7a7);
/// The A / B / C letter boxes.
const HUB_LETTER_BOX: iced::Color = rgb(0x9ac3a0);
/// The boxed letters and their MAIL BOX / MESSAGE / SECURITY LEVEL labels.
const HUB_SECTION: iced::Color = rgb(0xa0d2a9);

/// The ground's radial lift, trace `<radialGradient id="lift">`: the
/// three stops, with the outermost also painted flat under the lobe
/// because r 0.75 leaves the page corners outside the ellipse.
const LIFT: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x1c1a10)),
    (0.7, rgb(0x141107)),
    (1.0, rgb(0x0f0a03)),
];

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

/// A stretched glyph run, trace `transform="translate(..) scale(sx,1)"`.
/// The trace centres these (`text-anchor="middle"`) and `Prim::Wide` is
/// start-anchored, so `x` here is the trace's centre less half the run
/// the header measured for it; the caller says which.
const fn wide(x: f32, y: f32, size: f32, stretch: f32, ink: Ink, face: Face, content: &'static str) -> Prim {
    Prim::Wide { x, y, size, stretch, ink, face, content }
}

// One caption box, trace `<g id="caption">` (defs): drawn at the tile's
// foot-left corner with the tile's own frame at y 0 and the box's bottom
// edge lying on it. Top rule 28px up, divider 96px in, both 1.7px (the
// def's `<path d="M 0,-28 H 194 M 96,-28 V 0">`), two cells of 600-weight
// text stretched to the measured runs.
macro_rules! caption {
    ($ink:expr) => {
        &[
            hline(0.0, -28.0, 194.0, $ink, 1.7),
            vline(96.0, -28.0, 0.0, $ink, 1.7),
            wide(6.0, -15.0, 12.0, 0.78, $ink, Face::SemiBold, "85SD4F3Q5S41"),
            wide(103.0, -15.0, 12.0, 0.685, $ink, Face::SemiBold, "COMBAT COLONIZATION"),
            wide(103.0, -6.0, 10.5, 0.82, $ink, Face::SemiBold, "DEFENCE PROGRAM"),
        ]
    };
}
/// The box as the five idle tiles wear it, `<use href="#caption"
/// fill="#a8d7a7" stroke="#a8d7a7">`.
const CAPTION: &[Prim] = caption!(Ink::Fixed(HUB_CAPTION));
/// The box as the selected tile wears it, `<use href="#caption"
/// fill="#2e4a2c" stroke="#2e4a2c">` -- dark on the sage fill.
const CAPTION_ON: &[Prim] = caption!(Ink::Fixed(HUB_ON_CAPTION));

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
                fill_rect(0.0, -211.0, 194.0, 211.0, Ink::Select),
                $( label($lx, $ly, Ink::Fixed(HUB_ON_SOLID), $label), )+
                Prim::At { x: 0.0, y: 0.0, prims: CAPTION_ON },
            ],
            &[
                line_rect(0.0, -$h, 194.0, $h, Ink::Border, 1.25),
                $( label($lx, $ly, Ink::Fixed(HUB_LABEL), $label), )+
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

/// The ground, `<rect width=1600 height=900 fill="url(#lift)">`: the
/// radial at cx 0.45 cy 0.45 r 0.75 of the page's box -> centre
/// (720, 405), radii (1200, 675). Composited like every other ground
/// since 2026-09-05; until then it was the last `Lobe` the canvas
/// drew as annuli, and the only entropism screen with a banded lift.
const HUB_GROUND: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(rgb(0x0f0a03))),
    Prim::Lobe { x: 720.0, y: 405.0, rx: 1200.0, ry: 675.0, stops: LIFT },
];

pub const DASHBOARD: &[Prim] = &[
    Prim::Soft { prims: HUB_GROUND },
    // header strip, y 43..69, dividers at x 465 and 1353
    line_rect(49.0, 43.0, 1498.0, 26.0, Ink::Border, 1.25),
    vline(465.0, 43.0, 69.0, Ink::Border, 1.25),
    vline(1353.0, 43.0, 69.0, Ink::Border, 1.25),
    medium(61.0, 60.0, 17.0, Ink::Fixed(HUB_STRIP), "RIPPERDOC SURGICAL SOFTWAREV2"),
    medium(518.0, 60.0, 17.0, Ink::Fixed(HUB_STRIP), "STORE ACCESS SCREEN"),
    medium(1382.0, 60.0, 17.0, Ink::Fixed(HUB_STRIP), "FLAIR TRS 5MMP"),
    // section headings: 26x26 boxes holding a bold letter stretched
    // 1.5-1.6, centred at the trace's translate() x and placed here by
    // its measured run (A 18 wide at x 137..154, B and C 15 wide)
    line_rect(133.0, 145.0, 26.0, 26.0, Ink::Fixed(HUB_LETTER_BOX), 1.5),
    wide(137.0, 165.0, 22.0, 1.6, Ink::Fixed(HUB_SECTION), Face::Bold, "A"),
    line_rect(1014.0, 145.0, 26.0, 26.0, Ink::Fixed(HUB_LETTER_BOX), 1.5),
    wide(1019.5, 165.0, 22.0, 1.5, Ink::Fixed(HUB_SECTION), Face::Bold, "B"),
    line_rect(1329.0, 142.0, 26.0, 26.0, Ink::Fixed(HUB_LETTER_BOX), 1.5),
    wide(1334.5, 162.0, 22.0, 1.5, Ink::Fixed(HUB_SECTION), Face::Bold, "C"),
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
    // body copy: 7 lines, blank, 3 lines on a 21px pitch, the group's
    // `transform="translate(1033,0) scale(1.16,1)"` as the stretch
    wide(1033.0, 321.0, 17.0, 1.16, Ink::Fixed(HUB_BODY), Face::Medium, "Lorem ipsum dolor sit amet,"),
    wide(1033.0, 342.0, 17.0, 1.16, Ink::Fixed(HUB_BODY), Face::Medium, "consectetur adipiscing elit,"),
    wide(1033.0, 363.0, 17.0, 1.16, Ink::Fixed(HUB_BODY), Face::Medium, "sed do eiusmod tempor inci-"),
    wide(1033.0, 384.0, 17.0, 1.16, Ink::Fixed(HUB_BODY), Face::Medium, "didunt ut labore et dolore"),
    wide(1033.0, 405.0, 17.0, 1.16, Ink::Fixed(HUB_BODY), Face::Medium, "magna aliqua. Quis ipsum"),
    wide(1033.0, 426.0, 17.0, 1.16, Ink::Fixed(HUB_BODY), Face::Medium, "suspendisse ultrices gravi-"),
    wide(1033.0, 447.0, 17.0, 1.16, Ink::Fixed(HUB_BODY), Face::Medium, "da."),
    wide(1033.0, 489.0, 17.0, 1.16, Ink::Fixed(HUB_BODY), Face::Medium, "Risus commodo viverra ma-"),
    wide(1033.0, 510.0, 17.0, 1.16, Ink::Fixed(HUB_BODY), Face::Medium, "ecenas accumsan lacus vel"),
    wide(1033.0, 531.0, 17.0, 1.16, Ink::Fixed(HUB_BODY), Face::Medium, "facilisis."),
    // SECURITY LEVEL badges: four 68x68 at x 1380, T2 filled. Glyphs
    // bold 27 stretched to the measured runs, centred at x 1414 (T1 26
    // wide, T2 34, T3 36, T4 36), hence the start x here
    line_rect(1380.0, 214.0, 68.0, 68.0, Ink::Border, 1.25),
    wide(1401.0, 257.0, 27.0, 1.37, Ink::Fixed(HUB_LABEL), Face::Bold, "T1"),
    fill_rect(1380.0, 304.0, 68.0, 68.0, Ink::Select),
    wide(1397.0, 346.0, 27.0, 1.42, Ink::Fixed(HUB_ON_SOLID), Face::Bold, "T2"),
    line_rect(1380.0, 393.0, 68.0, 68.0, Ink::Border, 1.25),
    wide(1396.0, 435.0, 27.0, 1.5, Ink::Fixed(HUB_LABEL), Face::Bold, "T3"),
    line_rect(1380.0, 482.0, 68.0, 68.0, Ink::Border, 1.25),
    wide(1396.0, 524.0, 27.0, 1.38, Ink::Fixed(HUB_LABEL), Face::Bold, "T4"),
    // footer strip, y 847..872, no dividers; only BUILD is end-anchored
    line_rect(49.0, 847.0, 1498.0, 25.0, Ink::Border, 1.25),
    medium(61.0, 865.0, 17.0, Ink::Fixed(HUB_STRIP), "INTERFACE LOADED"),
    medium(518.0, 865.0, 17.0, Ink::Fixed(HUB_STRIP), "PROVIDED BY NEXUS NETWORK V10.8"),
    Prim::Text { x: 1525.0, y: 865.0, size: 17.0, ink: Ink::Fixed(HUB_STRIP), face: Face::Medium, anchor: Anchor::End, content: "BUILD 6.47.48441.R15" },
];

// --- end dashboard -------------------------------------------------------
