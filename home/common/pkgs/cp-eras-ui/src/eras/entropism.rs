//! Entropism -- "necessity over style".
//!
//! One hue. Sage green on a warm dark olive-brown ground, square
//! everything, 1px strokes, no glow and no gradients. Selection is a
//! solid sage fill. Sampled from Behance Part 1, doc #24-32.
//!
//! The predecessor crate (`entropism-ui`) carried twelve colours --
//! including cybr's red, cyan, mint, violet, orange and gold -- and a
//! radial glow module. None of that is in the reference; see
//! `docs/entropism/README.md`. This is the one-hue system.

use crate::palette::{rgb, Ornaments, Palette};
use crate::style::{
    Banner, Bar, BarChrome, BarGround, BarMenu, BarOrnament, Chrome, Compliance, Corner, Dress,
    Era, Face, Footnotes, Ground, Ink, Layout, MenuMarker, MenuRule, Metrics, Nameplate, Menu,
    PanelEcho, Selection, Style, Ticket, WindowLabel,
};
use crate::widgets::surface::Corners;
// --- login ---
use crate::style::{Access, Colophon, Fixture, Legend, Masthead, Plate, Plot, Slot, Wash};
// --- end login ---

pub const BG: iced::Color = rgb(0x110c07);
pub const SAGE_SOLID: iced::Color = rgb(0x9cb795);
pub const SAGE_TEXT: iced::Color = rgb(0x94bb94);
pub const MID: iced::Color = rgb(0x728f76);
pub const OUTLINE: iced::Color = rgb(0x5d7752);
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
                // tone and is not legible at 14px.
                disabled: Ink::Border,
                rule_ink: Ink::Border,
                row_inset: (0.0, 0.0),
                spine: 0.0,
                foot: 0.0,
                marker: MenuMarker::Text,
                echo: PanelEcho::None,
                echo_pad: 0.0,
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
        // "MENU TILES" on the components sheet: 120x120 squares, three
        // to a row, each under a hairline and a caption strip.
        //
        // Retained-dormant since 2026-08-31, the same way Menu::Table
        // is for neomil: the dashboard below (Layout::TileRow) draws
        // its own row of four tiles and does not go through the menu,
        // but any era or host wanting the hub grid still can -- the
        // widget stays live, the screen just does not wear it today.
        menu: Menu::Tiles { columns: 3 },
        // The material's dashboard is not the hub: per
        // docs/entropism/dashboard-trace.svg (the schematic of the
        // Behance screen #42, see docs/sources.md) it is a dim-olive
        // top field, a single row of four menu tiles -- the second
        // solid sage, selected -- caption strips under each tile, and a
        // thin build-rule at the foot. No sidebar, no detail panel, no
        // footer band: the material's frame contains none of them, so
        // this era's dashboard arm does not draw them.
        layout: Layout::TileRow,
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
        metrics: Metrics {
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
// record: band and button `select`/`cta` #9cb795 against the trace's
// #8aac8c, outlines `border` #5d7752 against #739479, text `fg`
// #94bb94 against #8aac8c, dark ink on the band `on_select` #1f2a1c
// against #20281c.

/// The line ink of *this* screen.
///
/// Probed at full 3840 resolution on the 2026-09-03 trace pass: the
/// header strip and the field frame are a 3px line with a peak core of
/// #6c8a77..#819b82, sitting straight on the ground with no dark ring.
/// The era's published `border` (#5d7752) was sampled off the hub,
/// mail and store sheets and this screen is dimmer than those; the
/// k-means #739479 the trace used to carry is the 1600 rescale's
/// dilution of the same line over two pixels.
pub const LINE: iced::Color = rgb(0x75967b);
/// The sage of the footer band and the NEXT button on this screen, and
/// the two text inks over the ground, all bright-tail means over the
/// runs on the same pass. This screen photographs a stop under the era
/// sheets the palette was sampled from, and the difference is not
/// cosmetic: on the published `select` (#9cb795) the k-means spends a
/// whole cluster on the extra antialiasing ramp and has none left for
/// the ground's warm lift, which is 72% of the design's shape area.
pub const BAND: iced::Color = rgb(0x8aac8c);
pub const ON_BAND: iced::Color = rgb(0x20281c);
pub const HEADING: iced::Color = rgb(0x799d81);

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
    wash: Wash::WarmLift,
    // Header strip, x 49..1547, y 43..69, dividers at 465 and 1353.
    masthead: Masthead::Strip {
        plate: Plate::outlined(Plot::new(49.0, 43.0, 1498.0, 26.0), Ink::Fixed(LINE), 1.25),
        dividers: &[465.0, 1353.0],
        labels: &[
            Legend::new("RIPPERDOC SURGICAL SOFTWAREV2", 61.0, 60.0, 17.0, Ink::Fixed(HEADING))
                .medium(),
            Legend::new("STORE ACCESS SCREEN", 518.0, 60.0, 17.0, Ink::Fixed(HEADING)).medium(),
            Legend::new("FLAIR TRS 5MMP", 1382.0, 60.0, 17.0, Ink::Fixed(HEADING)).medium(),
        ],
    },
    // One slot, alone in the upper two thirds of an empty frame.
    slots: &[Slot {
        prompt: Some(
            Legend::new("USERNAME:", 576.0, 402.0, 23.0, Ink::Fixed(BAND))
                .medium()
                .tracked(0.75),
        ),
        field: Some(Plate::outlined(
            Plot::new(563.0, 414.0, 359.0, 33.0),
            Ink::Fixed(LINE),
            1.25,
        )),
        // Eleven asterisks, and the photo's are 7px glyphs at a ~9.7
        // pitch, not the 4-5px at pitch 10 an earlier reading drew: a
        // bold 22 at natural tracking, run 94 wide.
        value: Some(Legend::new("***********", 578.0, 439.0, 22.0, Ink::Fixed(BAND)).bold()),
        // The short caret underline under the first pair of characters.
        caret: Some(Plate::filled(Plot::new(577.0, 439.5, 17.0, 1.25), Ink::Fixed(LINE))),
        action: Some(Plate::filled(Plot::new(932.0, 413.0, 105.0, 33.0), Ink::Fixed(BAND))),
        action_label: Some(
            Legend::new("NEXT", 940.0, 433.0, 22.0, Ink::Fixed(ON_BAND))
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
        plate: Plate::filled(Plot::new(36.0, 765.0, 1529.0, 115.0), Ink::Fixed(BAND)),
        labels: &[
            Legend::new("INTERFACE LOADED", 61.0, 863.0, 15.0, Ink::Fixed(ON_BAND)).tracked(1.0),
            Legend::new("PROVIDED BY NEXUS NETWORK V10.8", 519.0, 863.0, 15.0, Ink::Fixed(ON_BAND))
                .tracked(1.0),
            Legend::new("BUILD 6.47.48441.R15", 1383.0, 863.0, 15.0, Ink::Fixed(ON_BAND)).tracked(1.0),
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
// Outline ink is `Ink::Mid`, not `Ink::Border`: the trace samples every
// frame at #709174 and the era's `MID` is #728f76, while `OUTLINE` is a
// stop darker at #5d7752. (The crate TODO already carries this as an
// entropism-wide question.)
use crate::style::{
    Frame, MailBadges, MailButtons, MailList, MailPanel, Mailbox, Note, Piece, RowDecor,
    Run, Trim, FromAt,
};

/// Header strip, footer strip, the three boxed section letters, and
/// every fixed string the trace prints around them.
static CHROME: [Piece; 22] = [
    // header strip x 49..1547, y 43..69, dividers at x 465 and 1353
    Piece::Box {
        at: Frame::new(49.0, 43.0, 1498.0, 26.0),
        fill: None,
        stroke: Some(Ink::Mid),
        width: 2.0,
        trim: Trim::NONE,
    },
    Piece::Box {
        at: Frame::new(465.0, 43.0, 2.0, 26.0),
        fill: Some(Ink::Mid),
        stroke: None,
        width: 0.0,
        trim: Trim::NONE,
    },
    Piece::Box {
        at: Frame::new(1353.0, 43.0, 2.0, 26.0),
        fill: Some(Ink::Mid),
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
        stroke: Some(Ink::Mid),
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

pub fn mailbox() -> Mailbox {
    Mailbox {
        // this era's mailbox is content with its `Ground`
        haze: &[],
        chrome: &CHROME,
        list: MailList {
            // frame x 84..451, y 205..686
            frame: Some(Frame::new(84.0, 205.0, 367.0, 481.0)),
            frame_ink: Ink::Mid,
            frame_width: 2.0,
            // seven rows on a 62px pitch starting 21px below the top
            // edge; the dividers at 349 / 411 / 473 / 535 / 596 / 658
            // are each row's own foot.
            row: Frame::new(85.0, 226.0, 366.0, 62.0),
            pitch: 61.95,
            count: 7,
            selected: 0,
            decor: RowDecor::Framed,
            row_fill: None,
            row_stroke: None,
            row_width: 0.0,
            row_trim: Trim::NONE,
            spine: None,
            rule: Some(Frame::new(0.0, 61.95, 366.0, 2.0)),
            rule_ink: Ink::Mid,
            tab: None,
            tab_ink: Ink::Fg,
            sel: Frame::new(85.0, 226.0, 366.0, 62.0),
            sel_trim: Trim::NONE,
            sel_icon: None,
            sel_icon_trim: Trim::NONE,
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
            new_rows: 0,
            icons: None,
        },
        panel: MailPanel {
            frame: Some(Frame::new(529.0, 205.0, 750.0, 481.0)),
            frame_fill: None,
            frame_stroke: Some(Ink::Mid),
            frame_width: 2.0,
            frame_trim: Trim::NONE,
            head: Some(Frame::new(531.0, 227.0, 746.0, 61.0)),
            head_ink: Ink::Select,
            head_trim: Trim::NONE,
            message: 1,
            title: Run::new(557.0, 253.0, 22.0, Ink::OnSelect),
            title_upper: true,
            from: Some(Run::new(557.0, 279.0, 17.0, Ink::OnSelect)),
            body: Run::new(557.0, 325.0, 17.0, Ink::Fg),
            line: 21.7,
            para: 39.0,
            wrap: 690.0,
        },
        buttons: MailButtons {
            // one outlined strip y 694..746 split at x 716 / 903 /
            // 1091, the last cell filled solid with dark text
            first: Frame::new(529.0, 694.0, 187.33, 52.0),
            dx: 187.33,
            dy: 0.0,
            count: 4,
            filled: Some(3),
            joined: true,
            chevron: false,
            trim: Trim::NONE,
            width: 2.0,
            stroke: Ink::Mid,
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
            stroke: Ink::Mid,
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

pub const STORE: &[Prim] = &[
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
    // BELOW the things they label on this screen.
    txt(126.0, 787.0, 8.5, Ink::Border, "SPARE TIME MANAGER WAS DEVELOPED BY"),
    txt(126.0, 796.0, 8.5, Ink::Border, "SEOCHO. SERVING CUSTOMERS SINCE 2006."),
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
