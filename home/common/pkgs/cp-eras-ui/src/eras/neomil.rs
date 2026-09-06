//! Neo-militarism -- "substance over style".
//!
//! Three reds on near-black with a cold blue ambient glow, chamfered
//! surfaces, stencil labelling. Sampled from Behance Part 1, gallery
//! positions 53-62 per `docs/sources.md` (the run opens black with no
//! title card; the "doc #44-52" this comment used to give came from an
//! earlier, smaller scrape and is shifted by ten), and the shipped game
//! HUD it became (the reference side panel notes
//! the HUD "evolved from and is based on this style").
//!
//! Unlike the other three eras, this one already had an implementation
//! before the sampling pass, in the crate's original `colors.rs`
//! (deleted in `61db2ae`; `theme::Theme::fallback` keeps its values).
//! The figures agree; the values here are the same reds, restated in
//! the era table so all four read alike.

use crate::palette::{rgb, Ornaments, Palette};
use crate::style::{
    Banner, Bar, BarChrome, BarGround, BarMenu, BarOrnament, Chrome, Coat, Compliance, Controls,
    Corner, Dress,
    Era, Face, Footnotes, Ground, Ink, MenuMarker, MenuRule, Metrics, Nameplate,
    PanelEcho, Selection, Style, Ticket, WindowLabel,
};
use crate::widgets::surface::{Corners, Cut};
// --- login ---
use crate::style::{
    Access, Bevel, Blink, Caret, Colophon, Emblem, Entry, Fixture, Legend, Masthead, Plate, Plot, Slot,
};
// --- end login ---

pub const BG: iced::Color = rgb(0x050304);
/// The cold blue behind the reds. As a palette role it is unconsumed as
/// of 2026-09-03: `bloom` is read only by `widgets::ground` under
/// `Ground::Bloom` (this era is `Ground::Flat`), and `panel` only as the
/// `Ink::Inset` fallback, which no neomil arm uses. The const itself is
/// still read by `widgets::floppy_vector` (the floppy example, no
/// golden). Trace value would be the `glowh` gradient every neomil trace
/// carries -- stops `#282824` / `#273743` / `#263953` / `#202b56` /
/// `#1b2253` / `#171f51` / `#121f51` / `#0d1f4e` (identical in
/// `login-`, `dashboard-`, `mailbox-` and `store-trace.svg`), probed
/// on the ground as `#14244e` (dashboard) and `#19274e` (store) -- not
/// the single `#001a33`.
pub const GLOW: iced::Color = rgb(0x001a33);
pub const RED_DEEP: iced::Color = rgb(0x5e1112);
pub const RED_MID: iced::Color = rgb(0xa32226);
pub const RED_FILL: iced::Color = rgb(0xde2e2e);
pub const RED_HOT: iced::Color = rgb(0xff3b45);
pub const OFF_WHITE: iced::Color = rgb(0xdedede);

/// Left stop of the ops dashboard band's cold-blue gradient, and the
/// top stop of the mid-blue zone. Sampled off
/// `images/img-07-dashboard.png` (the 32x16 grid reads the top band as
/// `#2a3a51` at the left running to `#101f3d` at the right); recorded
/// in `docs/neomil/dashboard-trace.svg`.
pub const BAND_TOP: iced::Color = rgb(0x2a3a51);
/// Right stop of the same band gradient. Same citation as [`BAND_TOP`].
pub const BAND_BOTTOM: iced::Color = rgb(0x101f3d);
/// The dark red the ops screen trims its cards and rail with -- the
/// trace's `#350e10`, sampled off the card notches in
/// `images/img-07-dashboard.png` rows 6-8. The era's own `RED_DEEP` is
/// a stop brighter and reads as a filled surface rather than a
/// shadowed one, so it stays a separate constant like neokitsch's
/// `FRAME_LIT`/`STRATA`.
pub const CARD_DARK: iced::Color = rgb(0x350e10);

pub fn palette() -> Palette {
    Palette {
        bg: BG,
        panel: GLOW,
        border: RED_DEEP,
        dim: RED_MID,
        fg: RED_FILL,
        alert: RED_HOT,
        tape: OFF_WHITE,
        // Selected surfaces take the fill red and put off-white on top;
        // the references never invert to white-on-red text.
        select: RED_FILL,
        on_select: rgb(0x1a0405),
        emphasis: None,
        // No band anywhere, so nothing to restate for the selected
        // state either; `banner()` degrades to a tape label and
        // `banner_on_select()` swaps it.
        banner_selected: None,
        // A minimalist era declares no ornament: the vocabulary is
        // additive and nothing here wants it.
        ornaments: Ornaments::default(),
        cta: RED_FILL,
        bloom: GLOW,
    }
}

pub fn style() -> Style {
    Style {
        era: Era::Neomil,
        palette: palette(),
        corner: Corner::Chamfer { cut: 15.0 },
        selection: Selection::Solid,
        ground: Ground::Flat,
        chrome: Chrome::Tape,
        nameplate: Nameplate::Header,
        // --- bar --- (docs/neomil/bar.svg, IMPLEMENTATION DELTA)
        //
        // The SECURITY LEVEL badge row, compressed to 25px and laid
        // across the header glow every neomil screen opens with. The
        // era chamfers the bottom-LEFT of a cell and never the
        // bottom-right the old bar cut: login-trace's badges (15 on
        // 57), the mailbox rows (12 on 68) and the store nav (16 on
        // 62) all agree, and 15/57 * 25 is 6.
        bar: Bar {
            height: 31,
            host_tape: true,

            pad_left: 6.0,
            pad_right: 6.0,
            pad_y: 3.0,
            // The badges sit 1px apart on a 60 pitch; at this scale
            // that is 2, and the chamfers leave a small dark notch at
            // the foot of each joint.
            gap: 2.0,
            ws_gap: 2.0,
            ws_lead: 12.0,
            ws_width: 35.0,
            ws_corners: None,
            pad_x: 13.0,
            trail: 13.0,
            em: 0.58,
            // The design sized its cells by counting characters flat.
            space_em: 0.58,
            alert_track: 0.0,
            stroke: 1.5,
            icon_pad: 18.0,
            label_left: false,
            face: Face::Regular,
            // The code tape's five barcode ticks own its first 26px,
            // and the name starts after them.
            tape_extra: 16.0,
            tape_ticks: true,

            // The header glow: BAND_TOP at the left running to
            // BAND_BOTTOM at the right, closed by the full-width
            // hairline rule login/mailbox/dashboard all end their
            // header with.
            ground: BarGround::Band {
                left: BAND_TOP,
                right: BAND_BOTTOM,
                rule: Ink::Dim,
                rule_width: 1.5,
            },
            chrome: BarChrome::Loose,
            ornament: BarOrnament::None,

            idle: Dress {
                corners: Corners::square().with_bottom_left(Cut::Chamfer { x: 6.0, y: 6.0 }),
                fill: Ink::Border,
                stroke: Ink::Fg,
                ink: Ink::Fg,
                tab: false,
                step: None,
            },
            // The store's VIDEO row and mailbox row 1: solid fill red
            // with dark ink, the outline running unbroken through the
            // row like the badge row's.
            selected: Dress {
                corners: Corners::square().with_bottom_left(Cut::Chamfer { x: 6.0, y: 6.0 }),
                fill: Ink::Select,
                stroke: Ink::Select,
                ink: Ink::OnSelect,
                tab: false,
                step: None,
            },
            // The "NEW" pills and the margin chips: the silhouette
            // holds and the loudest ink in the material takes the
            // stroke and the label.
            alert: Dress {
                corners: Corners::square().with_bottom_left(Cut::Chamfer { x: 6.0, y: 6.0 }),
                fill: Ink::Border,
                stroke: Ink::Alert,
                ink: Ink::Alert,
                tab: false,
                step: None,
            },
            // login-trace's code tape: a filled strip whose LEFT end is
            // cut into a blunt point. The bar's old off-white tape
            // appears on no neomil screen -- the four traces sample no
            // white at all.
            tape: Dress {
                corners: Corners::square()
                    .with_top_left(Cut::Chamfer { x: 5.0, y: 5.0 })
                    .with_bottom_left(Cut::Chamfer { x: 5.0, y: 5.0 }),
                fill: Ink::Fg,
                stroke: Ink::None,
                ink: Ink::OnSelect,
                tab: false,
                step: None,
            },
            tab: None,
            // The tab box of mailbox-trace and dashboard-trace, which
            // in both is a plainer shape than the badge row above it:
            // square corners, dark fill, mid stroke.
            window: WindowLabel {
                dress: Some(Dress {
                    corners: Corners::square(),
                    fill: Ink::Fixed(CARD_DARK),
                    stroke: Ink::Dim,
                    ink: Ink::Fg,
                    tab: false,
                    step: None,
                }),
                ink: Ink::Fg,
                leading: false,
                pad_x: 8.0,
                stroke: Some(1.0),
                face: None,
            },

            alert_suffix: None,
            // The badges set their tier in bold; the workspace numbers
            // and the clock are the bar's tiers.
            bold_tiers: true,
            clock_plain: None,

            menu: BarMenu {
                // The GO HOME panel of dashboard-trace: top-right
                // chamfer 8, bottom-left 12, dark red fill, fill-red
                // 1.5 outline.
                panel: Dress {
                    corners: Corners::square()
                        .with_top_right(Cut::Chamfer { x: 8.0, y: 8.0 })
                        .with_bottom_left(Cut::Chamfer { x: 12.0, y: 12.0 }),
                    fill: Ink::Fixed(CARD_DARK),
                    stroke: Ink::Fg,
                    ink: Ink::Fg,
                    tab: false,
                    step: None,
                },
                air: 6.0,
                side: 0.0,
                row_air: 2.8,
                row_side: 17.0,
                icon_col: 16.0,
                icon_gap: 8.0,
                level_gap: 0.0,
                level_pad: 24.0,
                row_divider: false,
                rule: MenuRule::Inset,
                // The store nav's VIDEO row: solid fill red, dark ink,
                // chamfer 6, and the 3px spine running from the row's
                // top to its chamfer knee.
                row: Dress {
                    corners: Corners::square().with_bottom_left(Cut::Chamfer { x: 6.0, y: 6.0 }),
                    fill: Ink::Select,
                    stroke: Ink::None,
                    ink: Ink::OnSelect,
                    tab: false,
                    step: None,
                },
                open: Dress {
                    corners: Corners::square().with_bottom_left(Cut::Chamfer { x: 6.0, y: 6.0 }),
                    fill: Ink::Select,
                    stroke: Ink::None,
                    ink: Ink::OnSelect,
                    tab: false,
                    step: None,
                },
                open_inset: (9.0, 14.0),
                row_split: None,
                // The login trace's dimmest label ink, card 3's
                // caption.
                disabled: Ink::Dim,
                rule_ink: Ink::Dim,
                // Ending 6px short of the stepped right edge.
                row_inset: (9.0, 14.0),
                row_overshoot: 0.0,
                spine: 3.0,
                foot: 0.0,
                // The arrow the era puts after a code string, not a
                // "<": a 17x2 shaft and an 8-wide, 10-tall head.
                marker: MenuMarker::Arrow { w: 23.0, h: 10.0 },
                // The bright bar riding the panel's right edge on
                // slanted ends, the edge stepping 8 inward below it,
                // and the two glitch echoes trailing to the foot.
                echo: PanelEcho::EdgeBar {
                    step: 8.0,
                    top: 31.0,
                    len: 38.0,
                },
            },
        },
        banner: Banner::default(),
        // A and B under the nav, and a dead lower third the reference
        // is content with.
        footnotes: Footnotes::UnderNav,
        // No store target for this era, and the notice is a
        // maximalist-adjacent flourish; it stays unclaimed.
        compliance: Compliance::None,
        // Neomil chamfers its containers; it does not cut a wedge into
        // a nav pill.
        ticket: Ticket::default(),
        glyphs: false,
        // --- controls --- (components.svg BUTTON, USER CARD)
        controls: Controls {
            // Switch Weapon / Login: the fill red, dark ink.
            primary: Coat::filled(Ink::Cta, Ink::OnSelect),
            // Confirm / Jump: the near-ground fill `mailbox.buttons`
            // reads for the idle three, edged and lettered in the mid
            // red.
            ghost: Coat::filled(Ink::Fixed(rgb(0x1a0607)), Ink::Dim).edged(Ink::Dim, 1.0),
            // The protected card is the idle one "everything one stop
            // darker": edge and ink each drop a role.
            disabled: Coat::filled(Ink::Fixed(rgb(0x1a0607)), Ink::Border).edged(Ink::Border, 1.0),
            // The password well: solid, edged 1 in the mid red.
            field: Coat::filled(Ink::Fixed(WELL), Ink::Fg).edged(Ink::Dim, 1.0),
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
        store_selection: (0, 1),
        // --- end store ---
        // --- dashboard ---
        dashboard: DASHBOARD,
        // The photo distinguishes no unit (trace header :107-121 and
        // components.svg :722-723: "the source shows NO selected
        // state"), so every plate wears one dress and the opening
        // selection is the first unit by convention.
        dashboard_selection: 0,
        // --- end dashboard ---
        metrics: Metrics {
            stroke: 1.5,
            gap: 16.0,
            pad: 16.0,
            ..Metrics::default()
        },
    }
}

// --- login ---
//
// The access screen, transcribed from `docs/neomil/login-trace.svg` at
// 1600x900: the dossier header, a hairline rule, a numbered margin down
// each edge, and three user cards of which the first is live.
//
// The card fills below are the trace's own point samples rather than
// the era's published reds, and they have to be: cards two and three
// are *translucent* red over the screen's cold-blue glow, so they grade
// from a bluer top to a darker foot and neither stop is any role in the
// palette. The trace measures the grades down each card's centre.
// Everything that is a published role -- the bright red, the mid red,
// the deep red the badges are filled with -- is taken from the palette
// so the theme still reaches the screen.

/// Dark ink for text printed on a red card. The palette's `on_select`
/// (#1a0405) is the ink for a *selected* surface and is most of a stop
/// darker than the trace's #420f10, which is what the photo shows under
/// USER 01, `Login` and the code tape.
pub const ON_CARD: iced::Color = rgb(0x420f10);
/// Card two: translucent red over the blue glow, top and foot.
pub const CARD_OPEN: iced::Color = rgb(0x74272c);
pub const CARD_OPEN_FOOT: iced::Color = rgb(0x5e1516);
/// Card three, the protected state: the same card, darker.
pub const CARD_LOCKED: iced::Color = rgb(0x52242c);
pub const CARD_LOCKED_FOOT: iced::Color = rgb(0x481012);
/// The darker lower sections those two cards carry their notices on.
pub const SECTION_OPEN: iced::Color = rgb(0x461012);
pub const SECTION_LOCKED: iced::Color = rgb(0x2f0b0c);
/// Card three's outline: a stop under the mid red the others take.
pub const EDGE_LOCKED: iced::Color = rgb(0x8a2027);
/// The avatar plate on the live card, and the near-black its hexagon
/// glyph is drawn in.
pub const AVATAR: iced::Color = rgb(0xc72a2b);
pub const GLYPH_INK: iced::Color = rgb(0x200506);
/// The portrait silhouette inside the other two cards' avatar photos.
pub const PORTRAIT: iced::Color = rgb(0x4f1a1e);
/// The live card's password well and its Login button.
pub const WELL: iced::Color = rgb(0x430e0f);
pub const COMMIT: iced::Color = rgb(0xa52223);
/// The compliance micro-text under a card, a shade off the bright red.
pub const NOTICE: iced::Color = rgb(0xe63132);

/// The two-line compliance notice every card in the photo carries.
const NOTICE_1: &str = "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED TO";
const NOTICE_2: &str = "MANIPULATE, ACCESS OR DISABLE THIS DEVICE.";

/// The ground as the trace paints it (:120-122): the page `#080405`
/// (a step under the hub's `GROUND`), then the same `#glowh` under
/// `#glowmask` and the same `#vignette` every other neomil screen
/// opens with -- the trace records the login and hub backdrops as
/// pixel-identical at every sampled row. Until 2026-09-05 `login.rs`
/// sampled this through closures of its own and squared the
/// vignette's alpha on the way, where the def is two linear stops.
const LOGIN_GROUND: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(rgb(0x080405))),
    HUB_GLOW,
    HUB_VIGNETTE,
];
const LOGIN_BACKDROP: &[Prim] = &[Prim::Soft { prims: LOGIN_GROUND }];

pub const ACCESS: Access = Access {
    backdrop: LOGIN_BACKDROP,
    masthead: Masthead::Dossier {
        // 59x57, bottom-left chamfer 15. The customer badge at the
        // left, then four security badges of which the second is
        // filled in the mid red.
        badges: &[
            Plate::filled(Plot::new(117.0, 104.0, 59.0, 57.0), Ink::Border)
                .bevelled(Bevel::bl(15.0))
                .edged(Ink::Fg, 1.5),
            Plate::filled(Plot::new(1133.0, 104.0, 59.0, 57.0), Ink::Border)
                .bevelled(Bevel::bl(15.0))
                .edged(Ink::Fg, 1.5),
            Plate::filled(Plot::new(1193.0, 104.0, 59.0, 57.0), Ink::Dim)
                .bevelled(Bevel::bl(15.0))
                .edged(Ink::Fg, 1.5),
            Plate::filled(Plot::new(1253.0, 104.0, 59.0, 57.0), Ink::Border)
                .bevelled(Bevel::bl(15.0))
                .edged(Ink::Fg, 1.5),
            Plate::filled(Plot::new(1313.0, 104.0, 59.0, 57.0), Ink::Border)
                .bevelled(Bevel::bl(15.0))
                .edged(Ink::Fg, 1.5),
        ],
        rule: Plate::filled(Plot::new(42.0, 188.0, 1516.0, 1.5), Ink::Dim),
        labels: &[
            Legend::new("CUSTOMER", 124.0, 90.0, 14.0, Ink::Fg),
            Legend::new("LEVEL", 125.0, 121.0, 12.0, Ink::Fg),
            Legend::new("T1", 132.0, 140.0, 20.0, Ink::Fg).bold(),
            Legend::new("#NC488402", 256.0, 90.0, 14.0, Ink::Fg),
            Legend::new("PROTOCOL", 257.0, 131.0, 9.0, Ink::Fg).bold(),
            Legend::new("6520-A44", 257.0, 141.0, 9.0, Ink::Fg).bold(),
            Legend::new("ONLY CC35 CERTIFIED", 305.0, 108.0, 6.5, Ink::Dim),
            Legend::new("AND DHSF 5TH CLASS OFFICERS", 305.0, 116.5, 6.5, Ink::Dim),
            Legend::new("ARE ALLOWED TO MANIPULATE,", 305.0, 125.0, 6.5, Ink::Dim),
            Legend::new("ACCESS OR DISABLE THIS DEVICE.", 305.0, 133.5, 6.5, Ink::Dim),
            Legend::new("JHN 102 CKC 151 CC10 AS5", 283.0, 158.0, 7.0, Ink::Fixed(ON_CARD)),
            Legend::new("SECURITY LEVEL", 1141.0, 90.0, 14.0, Ink::Fg),
            Legend::new("LEVEL", 1141.0, 121.0, 12.0, Ink::Fg),
            Legend::new("LEVEL", 1201.0, 121.0, 12.0, Ink::Fg),
            Legend::new("LEVEL", 1261.0, 121.0, 12.0, Ink::Fg),
            Legend::new("LEVEL", 1321.0, 121.0, 12.0, Ink::Fg),
            Legend::new("T1", 1148.0, 140.0, 20.0, Ink::Fg).bold(),
            Legend::new("T2", 1208.0, 140.0, 20.0, Ink::Fg).bold(),
            Legend::new("T3", 1268.0, 140.0, 20.0, Ink::Fg).bold(),
            Legend::new("T4", 1328.0, 140.0, 20.0, Ink::Fg).bold(),
        ],
    },
    slots: &[
        // Card 1, x 372..625: the live one. A solid red block with the
        // form under it.
        Slot {
            body: Some(
                Plate::filled(Plot::new(372.0, 315.0, 253.0, 255.0), Ink::Fg)
                    .bevelled(Bevel::tr(51.0)),
            ),
            notch: Some(Plot::new(372.0, 392.0, 15.0, 105.0)),
            mark: Some(Plate::filled(
                Plot::new(452.0, 393.0, 94.0, 94.0),
                Ink::Fixed(AVATAR),
            )),
            emblem: Emblem::Hexagon,
            name: Some(
                Legend::new("USER 01", 499.0, 530.0, 18.0, Ink::Fixed(ON_CARD)).centred(),
            ),
            prompt: Some(Legend::new("password:", 378.0, 595.0, 14.0, Ink::Fg)),
            field: Some(
                Plate::filled(Plot::new(372.5, 602.5, 252.0, 32.0), Ink::Fixed(WELL))
                    .edged(Ink::Dim, 1.0),
            ),
            // The trace's mock is ten stars and two open slots; typed,
            // the field keeps the two slots after however many stars
            // were typed. The caret is on the Login button, not in the
            // field, so it stays put.
            entry: Some(Entry {
                rest: Legend::new("**********  __", 381.0, 626.0, 12.0, Ink::Fg),
                mask: '*',
                tail: "  __",
                caret: Caret::Fixed,
                blink: Blink::Tail,
                busy: "verifying:",
                failed: "access denied:",
            }),
            action: Some(
                Plate::filled(Plot::new(372.0, 635.0, 253.0, 48.0), Ink::Fixed(COMMIT))
                    .bevelled(Bevel::br(9.0)),
            ),
            action_label: Some(Legend::new("Login", 383.0, 657.0, 17.0, Ink::Fixed(ON_CARD))),
            caret: Some(Plate::filled(
                Plot::new(496.0, 640.0, 2.0, 15.0),
                Ink::Fixed(ON_CARD),
            )),
            notes: &[
                Legend::new(NOTICE_1, 378.0, 700.0, 7.5, Ink::Fixed(NOTICE)),
                Legend::new(NOTICE_2, 378.0, 709.0, 7.5, Ink::Fixed(NOTICE)),
            ],
            ..Slot::EMPTY
        },
        // Card 2, x 692..947: dim translucent red over the glow.
        Slot {
            body: Some(
                Plate::filled(Plot::new(692.0, 315.0, 255.0, 340.0), Ink::Fixed(CARD_OPEN))
                    .grading(Ink::Fixed(CARD_OPEN_FOOT))
                    .bevelled(Bevel {
                        tr: 47.0,
                        bl: 22.0,
                        ..Bevel::NONE
                    })
                    .edged(Ink::Dim, 1.5),
            ),
            foot: Some(
                Plate::filled(
                    Plot::new(692.0, 569.0, 255.0, 86.0),
                    Ink::Fixed(SECTION_OPEN),
                )
                .bevelled(Bevel::bl(22.0))
                .edged(Ink::Dim, 1.0),
            ),
            notch: Some(Plot::new(692.0, 392.0, 15.0, 105.0)),
            mark: Some(Plate::filled(Plot::new(772.0, 393.0, 96.0, 94.0), Ink::Fg)),
            emblem: Emblem::Portrait,
            name: Some(Legend::new("USER 01", 821.0, 530.0, 18.0, Ink::Fg).centred()),
            notes: &[
                Legend::new(NOTICE_1, 699.0, 587.0, 7.5, Ink::Fixed(NOTICE)),
                Legend::new(NOTICE_2, 699.0, 596.0, 7.5, Ink::Fixed(NOTICE)),
            ],
            ..Slot::EMPTY
        },
        // Card 3, x 982..1236: the same card, darker still.
        Slot {
            body: Some(
                Plate::filled(
                    Plot::new(982.0, 315.0, 254.0, 340.0),
                    Ink::Fixed(CARD_LOCKED),
                )
                .grading(Ink::Fixed(CARD_LOCKED_FOOT))
                .bevelled(Bevel {
                    tr: 47.0,
                    bl: 22.0,
                    ..Bevel::NONE
                })
                .edged(Ink::Fixed(EDGE_LOCKED), 1.5),
            ),
            foot: Some(
                Plate::filled(
                    Plot::new(982.0, 569.0, 254.0, 86.0),
                    Ink::Fixed(SECTION_LOCKED),
                )
                .bevelled(Bevel::bl(22.0))
                .edged(Ink::Fixed(EDGE_LOCKED), 1.0),
            ),
            notch: Some(Plot::new(982.0, 392.0, 15.0, 105.0)),
            mark: Some(Plate::filled(Plot::new(1061.0, 393.0, 96.0, 94.0), Ink::Fg)),
            emblem: Emblem::Portrait,
            name: Some(Legend::new("USER 01", 1110.0, 530.0, 18.0, Ink::Fg).centred()),
            notes: &[
                Legend::new(NOTICE_1, 989.0, 587.0, 7.5, Ink::Dim),
                Legend::new(NOTICE_2, 989.0, 596.0, 7.5, Ink::Dim),
            ],
            ..Slot::EMPTY
        },
    ],
    fixture: Fixture::Margins {
        chips: &[Plot::new(60.0, 347.0, 12.5, 12.5), Plot::new(1541.0, 347.0, 12.5, 12.5)],
        labels: &[
            Legend::new("1", 63.0, 357.0, 10.0, Ink::Fixed(ON_CARD)).bold(),
            Legend::new("2", 1544.0, 357.0, 10.0, Ink::Fixed(ON_CARD)).bold(),
            Legend::new("00032 05 54 08 CP", 57.0, 472.0, 11.0, Ink::Dim).turned(),
            Legend::new("JHN 102 CKC 151 CC10 AS5", 1556.0, 554.0, 11.0, Ink::Dim).turned(),
            Legend::new("KIROSHI", 1556.0, 651.0, 11.0, Ink::Dim).turned(),
        ],
    },
    // Nothing below y 720 but ground: the cards carry their own
    // notices and this era's login has no footer at all.
    colophon: Colophon::None,
};
// --- end login ---
// --- mailbox ---
//
// `docs/neomil/mailbox-trace.svg`, read at its 1600x900 frame. The
// ground is the trace's (:145-148): the cold-blue hub glow, a warm
// near-black wash under the list and panel drawn through its own
// vertical mask, and the left-margin vignette, as `MAIL_BACKDROP`.
// Until 2026-09-04 this header called those "the photograph's residue
// ... not geometry" and kept `Ground::Flat`; but the trace draws them,
// G2i scores the trace, and `triptych.sh --diff` lit the whole frame
// (`5 3 4` everywhere the design reads `31 31 34` at (50,150)). The
// per-row fill gradient (#280c0d at the top of the list fading to
// #1d0708 at its foot) is still left out: every row takes one fill.
//
// Two things the trace draws that are not here, both noted in the
// conversion report: the rotated BETTERLIFE TEC / PETROCHEM maker's
// marks inside the panel's top-right corner (canvas text cannot be
// rotated through `fill_text`), and the two rotated `00032 05 54 08 CP`
// margin strings, for the same reason. The 0.8 box around PETROCHEM is
// drawn, in `OVERLAY` (it sits on the panel, so CHROME is too early).
use crate::style::{
    Frame, Icons, Mail, MailBadges, MailButtons, MailList, MailPanel, Mailbox, Note,
    Piece, RowDecor, Run, Trim, FromAt, BL, BR, TR,
};

/// `#wash` (:32-39): "the warm near-black wash under the list and
/// panel, sampled at y 500..850: #120205 left margin, #0d0406..#040202
/// mid (fading downward), #020202 right of x~1460" -- a horizontal
/// gradient across the frame ...
const MAIL_WASH: &[(f32, iced::Color)] = &[
    (0.00, rgb(0x110305)),
    (0.40, rgb(0x0b0304)),
    (0.90, rgb(0x020202)),
];
/// ... drawn through `#washmask` (:40-44), clear at the top and solid
/// from y 405 down.
const MAIL_WASH_V: &[(f32, iced::Color)] = &[(0.00, rgb(0x000000)), (0.45, rgb(0xffffff))];
const MAIL_WASH_PRIMS: &[Prim] = &[Prim::Ramp {
    x: 0.0,
    y: 0.0,
    w: 1600.0,
    h: 900.0,
    from: (0.0, 0.0),
    to: (1.0, 0.0),
    stops: MAIL_WASH,
}];
const MAIL_WASH_MASK: &[Prim] = &[Prim::Ramp {
    x: 0.0,
    y: 0.0,
    w: 1600.0,
    h: 900.0,
    from: (0.0, 0.0),
    to: (0.0, 1.0),
    stops: MAIL_WASH_V,
}];
/// The ground as the trace stacks it (:145-148): page `#070304` (the
/// dashboard's `HUB_GROUND`), the masked glow, the masked wash, the
/// vignette.
const MAIL_GROUND: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(HUB_GROUND)),
    HUB_GLOW,
    Prim::Masked { prims: MAIL_WASH_PRIMS, mask: MAIL_WASH_MASK },
    HUB_VIGNETTE,
];
const MAIL_BACKDROP: &[Prim] = &[Prim::Soft { prims: MAIL_GROUND }];

const fn tape(x: f32, y: f32, w: f32) -> Piece {
    Piece::Box {
        at: Frame::new(x, y, w, 2.5),
        fill: Some(Ink::Dim),
        stroke: None,
        width: 0.0,
        trim: Trim::NONE,
    }
}

const fn text(x: f32, y: f32, size: f32, ink: Ink, s: &'static str) -> Piece {
    Piece::Label(Note {
        at: Run::new(x, y, size, ink),
        text: s,
    })
}

const fn strong(x: f32, y: f32, size: f32, ink: Ink, s: &'static str) -> Piece {
    Piece::Label(Note {
        at: Run::new(x, y, size, ink).bold(),
        text: s,
    })
}

/// The host tape at the head of the sidebar, and the arrow flag at the
/// foot: both filled shapes with one cut corner.
static HOST_TAPE: [(f32, f32); 6] = [
    (259.0, 151.0),
    (376.0, 151.0),
    (376.0, 160.0),
    (259.0, 160.0),
    (257.0, 158.0),
    (257.0, 153.0),
];
static FOOT_ARROW: [(f32, f32); 3] = [(886.0, 871.0), (894.0, 866.0), (894.0, 876.0)];
/// The bright bar riding the panel's right edge, y 415..552.
static EDGE_BAR: [(f32, f32); 6] = [
    (1447.0, 415.0),
    (1455.0, 415.0),
    (1455.0, 545.0),
    (1448.0, 552.0),
    (1440.0, 552.0),
    (1440.0, 422.0),
];
// The scroll widget's two thin arrows (mailbox-trace :304-312): heads
// 4.7 wide, both spanning y 692.7..714.6.
static SCROLL_UP: [(f32, f32); 3] = [(559.7, 704.6), (562.0, 692.7), (564.4, 704.6)];
static SCROLL_DOWN: [(f32, f32); 3] = [(569.8, 701.0), (574.6, 701.0), (572.2, 714.6)];
/// The widget's r 10.5 ring around the "R", as four cubics (k = 0.5523 r).
const SCROLL_RING: &[Seg] = &[
    Seg::Cubic { c1x: 577.5, c1y: 725.2, c2x: 572.8, c2y: 720.5, x: 567.0, y: 720.5 },
    Seg::Cubic { c1x: 561.2, c1y: 720.5, c2x: 556.5, c2y: 725.2, x: 556.5, y: 731.0 },
    Seg::Cubic { c1x: 556.5, c1y: 736.8, c2x: 561.2, c2y: 741.5, x: 567.0, y: 741.5 },
    Seg::Cubic { c1x: 572.8, c1y: 741.5, c2x: 577.5, c2y: 736.8, x: 577.5, y: 731.0 },
];

static CHROME: [Piece; 41] = [
    // customer block
    text(124.0, 90.0, 14.0, Ink::Fg, "CUSTOMER"),
    Piece::Box {
        at: Frame::new(117.0, 104.0, 59.0, 57.0),
        fill: Some(Ink::Border),
        stroke: Some(Ink::Fg),
        width: 1.5,
        trim: Trim::chamfer(BL, 15.0),
    },
    text(125.0, 121.0, 12.0, Ink::Fg, "LEVEL"),
    strong(132.0, 140.0, 20.0, Ink::Fg, "T1"),
    text(256.0, 90.0, 14.0, Ink::Fg, "#NC488402"),
    // the barcode tape under #NC488402: eight bars on a 4px pitch
    tape(257.0, 107.0, 18.0),
    tape(279.0, 107.0, 21.0),
    tape(257.0, 111.0, 26.0),
    tape(287.0, 111.0, 13.0),
    tape(257.0, 115.0, 12.0),
    tape(273.0, 115.0, 27.0),
    tape(257.0, 119.0, 22.0),
    tape(283.0, 119.0, 17.0),
    strong(257.0, 131.0, 9.0, Ink::Fg, "PROTOCOL"),
    strong(257.0, 141.0, 9.0, Ink::Fg, "6520-A44"),
    text(305.0, 108.0, 6.5, Ink::Dim, "ONLY CC35 CERTIFIED"),
    text(305.0, 116.5, 6.5, Ink::Dim, "AND DHSF 5TH CLASS OFFICERS"),
    text(305.0, 125.0, 6.5, Ink::Dim, "ARE ALLOWED TO MANIPULATE,"),
    text(305.0, 133.5, 6.5, Ink::Dim, "ACCESS OR DISABLE THIS DEVICE."),
    Piece::Poly {
        points: &HOST_TAPE,
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        close: true,
    },
    text(283.0, 158.0, 7.0, Ink::OnSelect, "JHN 102 CKC 151 CC10 AS5"),
    text(1141.0, 90.0, 14.0, Ink::Fg, "SECURITY LEVEL"),
    // the hairline the header sits on, and the two tab boxes under it
    Piece::Box {
        at: Frame::new(42.0, 187.0, 1516.0, 1.5),
        fill: Some(Ink::Dim),
        stroke: None,
        width: 0.0,
        trim: Trim::NONE,
    },
    Piece::Box {
        at: Frame::new(241.5, 194.5, 210.0, 22.0),
        fill: Some(Ink::Border),
        stroke: Some(Ink::Dim),
        width: 1.0,
        trim: Trim::NONE,
    },
    text(249.0, 210.0, 12.0, Ink::Fg, "COMPUTER SYSTEMS"),
    Piece::Box {
        at: Frame::new(727.5, 194.5, 210.0, 22.0),
        fill: Some(Ink::Border),
        stroke: Some(Ink::Dim),
        width: 1.0,
        trim: Trim::NONE,
    },
    text(742.0, 210.0, 12.0, Ink::Fg, "CONTENT"),
    // the scroll rail beside the list, its thumb, and the R widget
    Piece::Box {
        at: Frame::new(542.0, 313.0, 6.0, 565.0),
        fill: Some(Ink::Border),
        stroke: None,
        width: 0.0,
        trim: Trim::NONE,
    },
    Piece::Box {
        at: Frame::new(542.0, 313.0, 6.0, 71.0),
        fill: Some(Ink::Dim),
        stroke: None,
        width: 0.0,
        trim: Trim::NONE,
    },
    Piece::Poly {
        points: &SCROLL_UP,
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        close: true,
    },
    Piece::Poly {
        points: &SCROLL_DOWN,
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        close: true,
    },
    Piece::Box {
        at: Frame::new(561.3, 704.0, 1.5, 10.6),
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        trim: Trim::NONE,
    },
    Piece::Box {
        at: Frame::new(571.4, 692.7, 1.5, 9.0),
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        trim: Trim::NONE,
    },
    Piece::Curve {
        start: (577.5, 731.0),
        steps: SCROLL_RING,
        fill: None,
        stroke: Some(Ink::Fg),
        width: 2.0,
        close: true,
    },
    // the trace centres the R on x 567; Run is start-anchored
    strong(563.3, 736.0, 13.0, Ink::Fg, "R"),
    Piece::Poly {
        points: &FOOT_ARROW,
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        close: true,
    },
    text(558.0, 750.0, 6.0, Ink::Dim, "SCRLL"),
    text(558.0, 758.0, 6.0, Ink::Dim, "85402"),
    text(771.0, 875.0, 9.0, Ink::Dim, "JHN 102 CKC 151 CC10 AS5"),
    Piece::Box {
        at: Frame::new(1311.0, 861.0, 14.0, 14.0),
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        trim: Trim::NONE,
    },
    strong(1314.0, 872.0, 10.0, Ink::OnSelect, "B"),
];

/// Drawn after the panel, whose `#1c0608` fill (2026-09-04) had buried
/// the inner half of the bar riding its right edge and would bury the
/// maker's-mark box in its top-right corner.
static OVERLAY: [Piece; 2] = [
    Piece::Poly {
        points: &EDGE_BAR,
        fill: Some(Ink::Fg),
        stroke: None,
        width: 0.0,
        close: true,
    },
    // The 0.8 box the trace draws *around* the rotated PETROCHEM in the
    // panel's corner (mailbox-trace :330-335); the text itself cannot be
    // drawn -- `fill_text` does not rotate. Measured on img-08-main.png
    // at photo x 3431..3447, y 812..922 (outer), i.e. design 1429.6..
    // 1436.25 x 338.3..384.2; the frame here is the stroke centreline.
    Piece::Box {
        at: Frame::new(1430.0, 338.7, 6.25, 45.1),
        fill: None,
        stroke: Some(Ink::Dim),
        width: 0.8,
        trim: Trim::NONE,
    },
];

static BUTTONS: [&str; 4] = [
    "Switch Weapon",
    "Confirm / Jump",
    "Confirm / Jump",
    "Confirm / Jump",
];
static LEVELS: [&str; 4] = ["T1", "T2", "T3", "T4"];

/// The eight rows, trace lines 256-257 and 282-288. Not the inbox the
/// other three eras list: the first row is "List of messages", every
/// other row alternates two subjects, and every sender is Jackie. No
/// envelopes; `unread` is the NEW pill, which the trace outlines on
/// rows 1-3 (lines 258-259 and 292-297).
static ROWS: [Mail; 8] = [
    Mail { subject: "List of messages", from: "Jackie", unread: true },
    Mail { subject: "I'm worried man", from: "Jackie", unread: true },
    Mail { subject: "Heist data sent to you", from: "Jackie", unread: true },
    Mail { subject: "I'm worried man", from: "Jackie", unread: false },
    Mail { subject: "Heist data sent to you", from: "Jackie", unread: false },
    Mail { subject: "I'm worried man", from: "Jackie", unread: false },
    Mail { subject: "I'm worried man", from: "Jackie", unread: false },
    Mail { subject: "Heist data sent to you", from: "Jackie", unread: false },
];

/// The body, trace lines 336-345: 4 + 4 + 2 lines, the same three
/// paragraphs entropism sets but broken one word later on the first
/// two lines ("incididunt" / "exercitation" whole, "reprehen-" split).
static PARAGRAPHS: [&[&str]; 3] = [
    &[
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt",
        "ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation",
        "ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehen-",
        "derit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.",
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
        backdrop: MAIL_BACKDROP,
        chrome: &CHROME,
        overlay: &OVERLAY,
        list: MailList {
            frame: None,
            frame_ink: Ink::Dim,
            frame_width: 0.0,
            // eight rows x 241..511, tops at 313 / 385 / 455 / 525 /
            // 595 / 665 / 735 / 805, each with a 12px bottom-left
            // chamfer and a spine at x 237..240
            row: Frame::new(241.0, 315.0, 270.0, 68.0),
            pitch: 70.0,
            rows: &ROWS,
            selected: 0,
            decor: RowDecor::Boxed,
            // `#280c0d` (:262), the flat fill of rows 2-4; rows 5-8 fade
            // to `#1d0708` and take this one too. Was `Ink::Border`
            // (`#60181a`) until 2026-09-04 -- `94 17 18` where the
            // trace has `33 8 9`, and the whole list lit in `--diff`.
            row_fill: Some(Ink::Fixed(rgb(0x280c0d))),
            row_stroke: Some(Ink::Dim),
            row_width: 1.5,
            row_trim: Trim::chamfer(BL, 12.0),
            spine: Some(Frame::new(-4.0, 0.0, 3.0, 56.0)),
            rule: None,
            rule_ink: Ink::Dim,
            tab: None,
            tab_ink: Ink::Fg,
            sel: Frame::new(237.0, 313.0, 274.0, 70.0),
            sel_trim: Trim::chamfer(BL, 12.0),
            sel_icon: None,
            sel_icon_trim: Trim::NONE,
            sel_fill: Ink::Select,
            sel_notch: None,
            veneer: None,
            // no envelope: this era spends the glyph budget on the
            // cartridge column instead
            glyph_x: 0.0,
            glyph_dy: 0.0,
            glyph_w: 0.0,
            text_x: 249.0,
            title_dy: 23.0,
            title_size: 15.0,
            title_bold: false,
            from_dy: 23.0,
            from_size: 15.0,
            from_at: FromAt::Trailing,
            from_prefix: "",
            title_upper: false,
            from_upper: false,
            // the unread rows carry an outlined NEW pill in the lower
            // right
            new_pill: Some(Frame::new(162.0, 48.0, 76.0, 14.0)),
            icons: Some(Icons {
                x: 185.0,
                y: 323.0,
                pitch: 70.0,
            }),
        },
        panel: MailPanel {
            frame: Some(Frame::new(729.0, 312.0, 721.0, 387.0)),
            // `#1c0608` (:324); unfilled until 2026-09-04
            frame_fill: Some(Ink::Fixed(rgb(0x1c0608))),
            frame_stroke: Some(Ink::Dim),
            frame_width: 1.2,
            frame_trim: Trim::chamfer(TR | BR, 8.0),
            head: None,
            head_ink: Ink::Select,
            head_trim: Trim::NONE,
            // the panel is headed "Urgent Information (!)" (trace line
            // 320), which is no row of this era's list; the heading is
            // pinned and `message` only says which row a click returns
            // to. Row 2 is a guess the trace cannot confirm or refute.
            message: 1,
            title: Run::new(742.0, 287.0, 20.0, Ink::Fg).bold(),
            title_upper: false,
            from: None,
            heading: Some("Urgent Information (!)"),
            sender: None,
            body: Run::new(739.0, 347.0, 15.0, Ink::Fg),
            line: 21.0,
            para: 42.0,
            paragraphs: &PARAGRAPHS,
        },
        buttons: MailButtons {
            // four 175x56 buttons on a 180 pitch, bottom-right chamfer
            // 9; the first is filled and takes dark text
            first: Frame::new(728.0, 702.0, 175.0, 56.0),
            dx: 180.0,
            dy: 0.0,
            count: 4,
            filled: Some(0),
            fill: Ink::Select,
            // the idle three are filled `#1a0607` (:353); they drew open
            // until 2026-09-04, 20 levels dark against the trace
            idle_fill: Some(Ink::Fixed(rgb(0x1a0607))),
            joined: false,
            chevron: false,
            trim: Trim::chamfer(BR, 9.0),
            width: 1.0,
            stroke: Ink::Dim,
            label: Run::new(13.0, 44.0, 15.0, Ink::Dim),
            tab: None,
            labels: &BUTTONS,
        },
        badges: MailBadges {
            first: Frame::new(1133.0, 104.0, 59.0, 57.0),
            dx: 60.0,
            dy: 0.0,
            cols: 4,
            count: 4,
            selected: Some(1),
            trim: Trim::chamfer(BL, 15.0),
            width: 1.5,
            fill: Some(Ink::Border),
            stroke: Ink::Fg,
            label: Run::new(15.0, 36.0, 20.0, Ink::Fg).bold(),
            caption: Some(Run::new(8.0, 17.0, 12.0, Ink::Fg)),
            caption_text: "LEVEL",
            labels: &LEVELS,
        },
    }
}
// --- end mailbox ---
// --- store ---------------------------------------------------------------
//
// `docs/neomil/store-trace.svg`, transcribed. Coordinates are the
// trace's own in the 1600x900 frame, measured off
// `images/img-09-store.png`. Each card is placed with `Prim::At` at its
// left edge and no vertical offset, exactly as the trace's
// `transform="translate(437,0)"` groups do, so a figure here reads
// against the SVG line it came from.
//
// Two things the trace draws that are not transcribed, and why: the
// thin horizontal *glitch streaks* trailing the logotype and its band,
// and the ghosted title 15px right of card 4's. Both are the
// photograph's residue rather than the design (docs/PIPELINE.md), and
// the trace's own comment calls the second one "not drawn". The rotated
// micro-text in the left margin and down each card's right edge is
// omitted with it: iced's canvas text has no transform and neither run
// carries a shape in either inventory.

use crate::style::{
    fill_path, fill_rect, line_rect, shut_path, txt, txt_bold, txt_bold_mid, txt_end, Anchor,
    Group, Prim, Seg,
};

/// The three card fills, sampled down each card's own column: card 1
/// carries the warm wash, card 3 the last of the blue glow, card 4 the
/// pure-black field right of the shelf, and the selected card's *body*
/// -- below its wash -- is the warmest of them.
pub const CARD1_FILL: iced::Color = rgb(0x1a0a0d);
pub const CARD3_FILL: iced::Color = rgb(0x0e0d11);
pub const CARD4_FILL: iced::Color = rgb(0x0c0505);
pub const CARD2_FILL: iced::Color = rgb(0x170507);
/// The nav row's own fill, under its `#96282d` outline.
pub const NAV_FILL: iced::Color = rgb(0x2e1012);
/// The selected card's wash, `linearGradient id="c2upper"` (:107-112),
/// sampled down x=900, where the hub's blue glow lifts the card's top
/// into violet before it falls back to the card fill. The four stops
/// as the trace has them: drawn as a two-stop head-to-foot wash the
/// canvas was 14 levels off a third of the way down (2026-09-04).
pub const C2UPPER: &[(f32, iced::Color)] = &[
    (0.00, rgb(0x4e2545)),
    (0.35, rgb(0x5a2a3a)),
    (0.75, rgb(0x35171b)),
    (1.00, rgb(0x2c0c10)),
];
/// The hub backdrop: near-black under a cold blue that is gone by
/// y~540, and a warm near-black wash under the left half. Ground rather
/// than ink -- but ground the extractor's palette split depends on, so
/// the scene draws it rather than leaving the page flat.
pub const GROUND: iced::Color = rgb(0x0b0405);

// The cold-blue glow every neomil screen opens with, as the four
// traces (dashboard :75-101, mailbox and store :2-27, login :2-25)
// define it, to the stop: `#glowh`, a ten-stop horizontal gradient
// across the frame, drawn through `#glowmask`, a nine-stop luminance
// ramp down it -- opaque to y 225, gone by 540. Until 2026-09-04 the
// dashboard rasterised the pair at compile time into 640 strips and
// the mailbox and store drew a flat ground and a two-stop wash;
// `Prim::Masked` over two `Prim::Ramp`s is the construct itself,
// composited by `soft.rs`, so those three shared these from then and
// the login (its own sampler until 2026-09-05) since.

/// `#glowh`: the sampled hex, offsets as fractions of the frame width.
pub const GLOW_H: &[(f32, iced::Color)] = &[
    (0.000, rgb(0x282824)),
    (0.063, rgb(0x273743)),
    (0.188, rgb(0x263953)),
    (0.313, rgb(0x202b56)),
    (0.438, rgb(0x1b2253)),
    (0.563, rgb(0x171f51)),
    (0.688, rgb(0x121f51)),
    (0.813, rgb(0x0d1f4e)),
    (0.938, rgb(0x082447)),
    (1.000, rgb(0x080b0e)),
];
/// `#glowv`, the mask's luminance down the frame ("stops read off the
/// source's blue channel at x=1000, normalised" -- the trace).
pub const GLOW_V: &[(f32, iced::Color)] = &[
    (0.00, rgb(0xffffff)),
    (0.25, rgb(0xffffff)),
    (0.30, rgb(0xe3e3e3)),
    (0.35, rgb(0xbababa)),
    (0.40, rgb(0x8c8c8c)),
    (0.45, rgb(0x545454)),
    (0.50, rgb(0x2b2b2b)),
    (0.55, rgb(0x121212)),
    (0.60, rgb(0x000000)),
];
const GLOW_RECT: &[Prim] = &[Prim::Ramp {
    x: 0.0,
    y: 0.0,
    w: 1600.0,
    h: 900.0,
    from: (0.0, 0.0),
    to: (1.0, 0.0),
    stops: GLOW_H,
}];
const GLOW_MASK: &[Prim] = &[Prim::Ramp {
    x: 0.0,
    y: 0.0,
    w: 1600.0,
    h: 900.0,
    from: (0.0, 0.0),
    to: (0.0, 1.0),
    stops: GLOW_V,
}];
/// `<rect width=1600 height=900 fill="url(#glowh)" mask="url(#glowmask)">`.
pub const HUB_GLOW: Prim = Prim::Masked { prims: GLOW_RECT, mask: GLOW_MASK };

/// The warm near-black vignette down the left margin, `radialGradient
/// id="vignette"`, `cx 0.02 cy 0.60 r 0.34` of the page: one colour
/// from opaque to clear. The same on all three screens.
const VIGNETTE: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x241012)),
    (1.0, iced::Color { a: 0.0, ..rgb(0x241012) }),
];
pub const HUB_VIGNETTE: Prim = Prim::Lobe { x: 32.0, y: 540.0, rx: 544.0, ry: 306.0, stops: VIGNETTE };

/// `#wash` on the store (:34-38): the warm wash under the left half,
/// `cx 0.18 cy 0.5 r 0.45` of the page -- centre (288,450), radii
/// (720,405) -- `#1a0c0e` fading to a clear `#120608`.
const STORE_WASH: &[(f32, iced::Color)] = &[
    (0.0, rgb(0x1a0c0e)),
    (0.7, rgb(0x120608)),
    (1.0, iced::Color { a: 0.0, ..rgb(0x120608) }),
];
/// `#blackv` (:40-43): the pure-black field right of the cards below
/// the glow, clear at the top of its 540x520 rect at (1060,380) and
/// solid a quarter of the way down.
const STORE_BLACK: &[(f32, iced::Color)] = &[
    (0.00, iced::Color { a: 0.0, ..rgb(0x020203) }),
    (0.25, rgb(0x020203)),
];
/// The store's ground as its trace stacks it (:225-229).
const STORE_GROUND: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(GROUND)),
    HUB_GLOW,
    Prim::Lobe { x: 288.0, y: 450.0, rx: 720.0, ry: 405.0, stops: STORE_WASH },
    Prim::Ramp {
        x: 1060.0,
        y: 380.0,
        w: 540.0,
        h: 520.0,
        from: (0.0, 0.0),
        to: (0.0, 1.0),
        stops: STORE_BLACK,
    },
    HUB_VIGNETTE,
];
/// The two tones the gun drawing takes on the selected card, and the
/// two dark faces it takes on the others.
pub const GUN_LIT: iced::Color = rgb(0xb02c30);
pub const GUN_CRADLE: iced::Color = rgb(0x902d34);
pub const GUN_SHADE: iced::Color = rgb(0x5a1e22);

/// The scatter-code glyph: 24 loose 3px squares on a 9x9 lattice at a
/// 3.75px pitch, with no finder patterns -- not a QR.
const QR: &[&str] = &[
    "#..#.#..#",
    ".#....#..",
    "..#.....#",
    "#..#.#.#.",
    ".........",
    ".#.#..#.#",
    "#....#...",
    ".#.....#.",
    "#..#.#..#",
];

// The gun, card-local in x and canvas-absolute in y, as the trace's
// four restylable layers. The body inherits fill and stroke, the dark
// and lit layers inherit fill only, which is what lets card 2 draw the
// same drawing solid where the others draw it outlined.

const MUZZLE: &[Seg] = &[
    Seg::Line(56.0, 294.0),
    Seg::Line(79.0, 297.0),
    Seg::Line(79.0, 300.0),
    Seg::Line(63.0, 300.0),
    Seg::Line(63.0, 311.0),
    Seg::Line(36.0, 311.0),
];
const TRIGGER: &[Seg] = &[
    Seg::Line(175.0, 312.0),
    Seg::Line(181.0, 338.0),
    Seg::Line(167.0, 338.0),
];
const RAIL_ARM: &[Seg] = &[
    Seg::Line(180.0, 300.0),
    Seg::Line(200.0, 317.0),
    Seg::Line(200.0, 323.0),
    Seg::Line(175.0, 306.0),
];
const SLING: &[Seg] = &[
    Seg::Line(197.0, 328.0),
    Seg::Line(197.0, 331.0),
    Seg::Line(182.0, 338.0),
];
const BUFFER: &[Seg] = &[
    Seg::Line(224.0, 322.0),
    Seg::Line(224.0, 330.0),
    Seg::Line(198.0, 330.0),
];
const STOCK_UP: &[Seg] = &[
    Seg::Line(230.0, 315.0),
    Seg::Line(253.0, 322.0),
    Seg::Line(253.0, 327.0),
    Seg::Line(243.0, 327.0),
    Seg::Line(225.0, 320.0),
];
const STOCK_LOW: &[Seg] = &[
    Seg::Line(231.0, 330.0),
    Seg::Line(247.0, 352.0),
    Seg::Line(247.0, 355.0),
    Seg::Line(237.0, 355.0),
    Seg::Line(237.0, 350.0),
    Seg::Line(226.0, 334.0),
];
const FOREND_A: &[Seg] = &[Seg::Line(89.0, 314.0), Seg::Line(89.0, 340.0)];
const FOREND_B: &[Seg] = &[
    Seg::Line(119.0, 324.0),
    Seg::Line(123.0, 324.0),
    Seg::Line(123.0, 346.0),
    Seg::Line(89.0, 346.0),
];
const WEDGE: &[Seg] = &[
    Seg::Line(144.0, 310.0),
    Seg::Line(154.0, 322.0),
    Seg::Line(143.0, 325.0),
];

/// The three small icons at a card's head and foot: two outlined
/// squares (the second holding a circle) and a C-bracket whose arc the
/// scene walks as two quadratics. Card-local, with the row's own top.
macro_rules! icons {
    ($y:expr) => {
        &[
            line_rect(9.0, $y, 16.0, 17.0, Ink::Fg, 1.5),
            line_rect(31.0, $y, 18.0, 17.0, Ink::Fg, 1.5),
            Prim::Circle { x: 40.0, y: $y + 8.5, r: 4.5, fill: None, stroke: Some(Ink::Fg), width: 1.5 },
            Prim::Path {
                x: 73.0,
                y: $y + 1.0,
                segs: &[
                    Seg::Line(62.0, $y + 1.0),
                    Seg::Quad { cx: 54.0, cy: $y + 1.0, x: 54.0, y: $y + 9.0 },
                    Seg::Quad { cx: 54.0, cy: $y + 17.0, x: 62.0, y: $y + 17.0 },
                    Seg::Line(73.0, $y + 17.0),
                ],
                close: false,
                fill: None,
                stroke: Some(Ink::Fg),
                width: 1.5,
            },
        ]
    };
}

const ICONS_HEAD: &[Prim] = icons!(163.0);
const ICONS_FOOT: &[Prim] = icons!(584.0);
const ICONS_FOOT_SEL: &[Prim] = icons!(776.0);

macro_rules! gun {
    ($body:expr, $edge:expr, $w:expr, $dark:expr, $hi:expr) => {
        &[
            Prim::Path { x: 36.0, y: 294.0, segs: MUZZLE, close: true, fill: Some($body), stroke: $edge, width: $w },
            Prim::Rect { x: 61.0, y: 300.0, w: 114.0, h: 10.0, fill: Some($body), stroke: $edge, width: $w },
            Prim::Rect { x: 42.0, y: 315.0, w: 11.0, h: 31.0, fill: Some($body), stroke: $edge, width: $w },
            Prim::Rect { x: 53.0, y: 313.0, w: 70.0, h: 33.0, fill: Some($body), stroke: $edge, width: $w },
            Prim::Rect { x: 123.0, y: 313.0, w: 9.0, h: 33.0, fill: Some($body), stroke: $edge, width: $w },
            Prim::Rect { x: 136.0, y: 322.0, w: 19.0, h: 24.0, fill: Some($body), stroke: $edge, width: $w },
            Prim::Rect { x: 149.0, y: 300.0, w: 26.0, h: 13.0, fill: Some($body), stroke: $edge, width: $w },
            Prim::Path { x: 163.0, y: 312.0, segs: TRIGGER, close: true, fill: Some($body), stroke: $edge, width: $w },
            Prim::Path { x: 175.0, y: 300.0, segs: RAIL_ARM, close: true, fill: Some($body), stroke: $edge, width: $w },
            Prim::Path { x: 181.0, y: 335.0, segs: SLING, close: true, fill: Some($body), stroke: $edge, width: $w },
            Prim::Path { x: 198.0, y: 318.0, segs: BUFFER, close: true, fill: Some($body), stroke: $edge, width: $w },
            Prim::Path { x: 225.0, y: 315.0, segs: STOCK_UP, close: true, fill: Some($body), stroke: $edge, width: $w },
            Prim::Path { x: 226.0, y: 330.0, segs: STOCK_LOW, close: true, fill: Some($body), stroke: $edge, width: $w },
            Prim::Rect { x: 252.0, y: 322.0, w: 10.0, h: 33.0, fill: Some($body), stroke: $edge, width: $w },
            // the forend's shaded triangle and its hatched grid
            fill_path(63.0, 330.0, FOREND_A, $dark),
            fill_path(89.0, 340.0, FOREND_B, $dark),
            // the magazine studs, the toothed under-rail, the module
            // studs and the bright angled block
            fill_rect(33.0, 315.0, 9.0, 7.0, $hi),
            fill_rect(33.0, 323.0, 9.0, 7.0, $hi),
            fill_rect(33.0, 331.0, 9.0, 7.0, $hi),
            fill_rect(33.0, 339.0, 9.0, 7.0, $hi),
            fill_rect(55.0, 340.0, 34.0, 9.0, $hi),
            fill_rect(125.5, 317.0, 4.0, 4.0, $hi),
            fill_rect(125.5, 325.0, 4.0, 4.0, $hi),
            fill_rect(125.5, 333.0, 4.0, 4.0, $hi),
            fill_rect(125.5, 341.0, 4.0, 4.0, $hi),
            fill_path(133.0, 312.0, WEDGE, $hi),
            fill_rect(206.0, 311.0, 6.0, 4.0, $hi),
        ]
    };
}

const GUN_OUTLINED: &[Prim] = gun!(Ink::Dim, Some(Ink::Fg), 1.0, Ink::Fixed(GUN_SHADE), Ink::Fg);
const GUN_SOLID: &[Prim] = gun!(Ink::Fg, None, 0.0, Ink::Fg, Ink::Fixed(GUN_LIT));

/// The stats block and everything under it, on an unselected card.
const STATS: &[Prim] = &[
    txt(32.0, 434.0, 15.0, Ink::Fg, "DPS"),
    txt(96.0, 434.0, 15.0, Ink::Fg, "PNT"),
    txt(161.0, 434.0, 15.0, Ink::Fg, "ACC"),
    txt(225.0, 434.0, 15.0, Ink::Fg, "ROF"),
    txt_bold_mid(46.0, 467.0, 20.0, Ink::Fg, "86"),
    txt_bold_mid(109.0, 467.0, 20.0, Ink::Fg, "30"),
    txt_bold_mid(174.0, 467.0, 20.0, Ink::Fg, "5"),
    txt_bold_mid(238.0, 467.0, 20.0, Ink::Fg, "5"),
    txt(11.0, 492.0, 8.0, Ink::Dim, "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE ALLOWED"),
    txt(11.0, 500.0, 8.0, Ink::Dim, "TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE."),
    Prim::Dots { x: 14.0, y: 520.0, cell: 3.0, pitch: 3.75, ink: Ink::Fg, rows: QR },
    txt_bold_mid(90.0, 534.0, 12.0, Ink::Fg, "EMPTY"),
    txt_bold_mid(90.0, 547.5, 12.0, Ink::Fg, "SOCKET"),
    txt_bold_mid(165.0, 534.0, 12.0, Ink::Fg, "EMPTY"),
    txt_bold_mid(165.0, 547.5, 12.0, Ink::Fg, "SOCKET"),
    txt_bold_mid(240.0, 534.0, 12.0, Ink::Fg, "EMPTY"),
    txt_bold_mid(240.0, 547.5, 12.0, Ink::Fg, "SOCKET"),
];

/// A standard card's frame: a 13px top-right chamfer, the right edge
/// stepping 12px inward below the bright bar's foot at y 416.
const FRAME_STD: &[Seg] = &[
    Seg::Line(271.0, 151.0),
    Seg::Line(284.0, 164.0),
    Seg::Line(284.0, 270.0),
    Seg::Line(272.0, 282.0),
    Seg::Line(272.0, 613.0),
    Seg::Line(0.0, 613.0),
];
/// The bright bar on the right edge, y 266..416, chamfered at both ends.
const EDGE_BAR_PATH: &[Seg] = &[
    Seg::Line(284.0, 404.0),
    Seg::Line(272.0, 416.0),
    Seg::Line(272.0, 278.0),
];

macro_rules! card {
    ($fill:expr) => {
        &[
            Prim::Path { x: 0.0, y: 151.0, segs: FRAME_STD, close: true, fill: Some(Ink::Fixed($fill)), stroke: Some(Ink::Fg), width: 1.2 },
            fill_path(284.0, 266.0, EDGE_BAR_PATH, Ink::Fg),
            Prim::At { x: 0.0, y: 0.0, prims: ICONS_HEAD },
            Prim::At { x: 0.0, y: 0.0, prims: ICONS_FOOT },
            txt_bold(11.0, 202.0, 20.0, Ink::Fg, "MAGNUM 650"),
            txt(11.0, 223.0, 17.0, Ink::Fg, "HAND GUN"),
            line_rect(261.4, 182.9, 7.2, 48.4, Ink::Dim, 0.8),
            Prim::At { x: 0.0, y: 0.0, prims: GUN_OUTLINED },
            Prim::At { x: 0.0, y: 0.0, prims: STATS },
        ]
    };
}

const CARD1: &[Prim] = card!(CARD1_FILL);
const CARD3: &[Prim] = card!(CARD3_FILL);

/// The selected card: the same drawing grown to y 800, its upper two
/// thirds washed, its gun solid and 14px further left, and the detail
/// block in the room the growth buys.
const FRAME_SEL: &[Seg] = &[
    Seg::Line(269.0, 151.0),
    Seg::Line(282.0, 164.0),
    Seg::Line(282.0, 270.0),
    Seg::Line(270.0, 282.0),
    Seg::Line(270.0, 800.0),
    Seg::Line(0.0, 800.0),
];
const EDGE_BAR_SEL: &[Seg] = &[
    Seg::Line(282.0, 404.0),
    Seg::Line(270.0, 416.0),
    Seg::Line(270.0, 278.0),
];

const GROWN: &[Prim] = &[
    Prim::Path { x: 0.0, y: 151.0, segs: FRAME_SEL, close: true, fill: Some(Ink::Fixed(CARD2_FILL)), stroke: Some(Ink::Fg), width: 1.2 },
    Prim::Ramp { x: 0.0, y: 151.0, w: 282.0, h: 365.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: C2UPPER },
    fill_path(282.0, 266.0, EDGE_BAR_SEL, Ink::Fg),
    Prim::At { x: 0.0, y: 0.0, prims: ICONS_HEAD },
    Prim::At { x: 0.0, y: 0.0, prims: ICONS_FOOT_SEL },
    txt_bold(11.0, 202.0, 20.0, Ink::Fg, "MAGNUM 650"),
    txt(11.0, 223.0, 17.0, Ink::Fg, "HAND GUN"),
    line_rect(261.4, 182.9, 7.2, 48.4, Ink::Fixed(GUN_LIT), 0.8),
    // the solid variant, 14px left of the frame-relative position
    Prim::At { x: -14.0, y: 0.0, prims: GUN_SOLID },
    // on the solid gun only the trigger cradle reads dark
    fill_rect(122.0, 322.0, 19.0, 24.0, Ink::Fixed(GUN_CRADLE)),
    txt(32.0, 434.0, 15.0, Ink::Fg, "DPS"),
    txt(96.0, 434.0, 15.0, Ink::Fg, "PNT"),
    txt(161.0, 434.0, 15.0, Ink::Fg, "ACC"),
    txt(225.0, 434.0, 15.0, Ink::Fg, "ROF"),
    txt_bold_mid(46.0, 467.0, 20.0, Ink::Fg, "86"),
    txt_bold_mid(109.0, 467.0, 20.0, Ink::Fg, "30"),
    txt_bold_mid(174.0, 467.0, 20.0, Ink::Fg, "5"),
    txt_bold_mid(238.0, 467.0, 20.0, Ink::Fg, "5"),
    txt(11.0, 492.0, 8.0, Ink::Fg, "ONLY CC35 CERTIFIED AND DHSF 5TH CLASS OFFICERS ARE"),
    txt(11.0, 500.0, 8.0, Ink::Fg, "ALLOWED TO MANIPULATE, ACCESS OR DISABLE THIS DEVICE."),
    txt(27.0, 548.0, 17.0, Ink::Fg, "20"),
    txt(56.0, 548.0, 17.0, Ink::Fg, "RECOIL"),
    txt(27.0, 571.0, 17.0, Ink::Fg, "22"),
    txt(56.0, 571.0, 17.0, Ink::Fg, "SPERAD"),
    txt(27.0, 593.0, 17.0, Ink::Fg, "12"),
    txt(56.0, 593.0, 17.0, Ink::Fg, "RANGE"),
    txt(27.0, 631.0, 17.0, Ink::Fg, "BONUS"),
    txt(27.0, 656.0, 17.0, Ink::Fg, "+9"),
    txt(53.0, 656.0, 17.0, Ink::Fg, "REFLEXES"),
    txt(27.0, 680.0, 17.0, Ink::Fg, "+2"),
    txt(53.0, 680.0, 17.0, Ink::Fg, "MODULES SLOTS"),
    Prim::Dots { x: 16.0, y: 713.0, cell: 3.0, pitch: 3.75, ink: Ink::Fg, rows: QR },
    txt_bold_mid(88.0, 723.0, 12.0, Ink::Fg, "EMPTY"),
    txt_bold_mid(88.0, 737.5, 12.0, Ink::Fg, "SOCKET"),
    txt_bold_mid(163.0, 723.0, 12.0, Ink::Fg, "EMPTY"),
    txt_bold_mid(163.0, 737.5, 12.0, Ink::Fg, "SOCKET"),
    txt_bold_mid(238.0, 723.0, 12.0, Ink::Fg, "EMPTY"),
    txt_bold_mid(238.0, 737.5, 12.0, Ink::Fg, "SOCKET"),
];

/// Card 4 is the same drawing cut off by the frame edge at x=1557: no
/// chamfer, no right edge line, no bright bar, and its content clipped
/// at the cut.
const CARD_CUT: &[Prim] = &[
    Prim::At { x: 0.0, y: 0.0, prims: ICONS_HEAD },
    Prim::At { x: 0.0, y: 0.0, prims: ICONS_FOOT },
    txt_bold(11.0, 202.0, 20.0, Ink::Fg, "MAGNUM 650"),
    txt(11.0, 223.0, 17.0, Ink::Fg, "HAND GUN"),
    Prim::At { x: 0.0, y: 0.0, prims: GUN_OUTLINED },
    txt(32.0, 434.0, 15.0, Ink::Fg, "DPS"),
    txt(96.0, 434.0, 15.0, Ink::Fg, "PNT"),
    txt_bold_mid(46.0, 467.0, 20.0, Ink::Fg, "86"),
    txt_bold_mid(109.0, 467.0, 20.0, Ink::Fg, "30"),
    // The caption is cut at the frame edge, and it is cut *here* rather
    // than by the covering strip below: a canvas frame layers all of
    // its text above all of its geometry, whatever order it was drawn
    // in, so a shape can never hide a glyph. These are the two lines as
    // far as the design shows them.
    txt(11.0, 492.0, 8.0, Ink::Dim, "ONLY CC35 CERTIFIED AND DHSF 5TH"),
    txt(11.0, 500.0, 8.0, Ink::Dim, "TO MANIPULATE, ACCESS OR DISABLE"),
    Prim::Dots { x: 14.0, y: 520.0, cell: 3.0, pitch: 3.75, ink: Ink::Fg, rows: QR },
    txt_bold_mid(90.0, 534.0, 12.0, Ink::Fg, "EMPTY"),
    txt_bold_mid(90.0, 547.5, 12.0, Ink::Fg, "SOCKET"),
];
/// Card 4's frame is an open path: its top and bottom edges run to the
/// cut and there is no right edge line at all.
const CARD4_EDGE: &[Seg] = &[
    Seg::Line(0.0, 151.0),
    Seg::Line(0.0, 613.0),
    Seg::Line(132.0, 613.0),
];
/// The page as it stands right of card 4's cut: `STORE_GROUND` as the
/// design renders it, sampled down two columns (x 1567 and 1589) of the
/// 43px strip, offsets as fractions of the card's 462 height. The glow
/// sits opaque to y 225 and is gone by 540; the `#blackv` ramp takes
/// over from y 380. Two columns because the glow's last stop runs
/// #082447 -> #080b0e across x 1500..1600, 22 levels over the strip;
/// one column would be 11 off at its edges. A flat `GROUND` rect under
/// a two-stop wash sat here until 2026-09-04, up to 25 levels off.
const CUT_GROUND_L: &[(f32, iced::Color)] = &[
    (0.000, rgb(0x081320)),
    (0.214, rgb(0x08121e)),
    (0.322, rgb(0x08101a)),
    (0.431, rgb(0x0a0d15)),
    (0.496, rgb(0x0a0b11)),
    (0.539, rgb(0x08080c)),
    (0.604, rgb(0x070608)),
    (0.669, rgb(0x050305)),
    (0.734, rgb(0x040304)),
    (0.777, rgb(0x020203)),
    (1.000, rgb(0x020203)),
];
const CUT_GROUND_R: &[(f32, iced::Color)] = &[
    (0.000, rgb(0x080d14)),
    (0.214, rgb(0x080c13)),
    (0.322, rgb(0x080b11)),
    (0.431, rgb(0x0a0a0e)),
    (0.496, rgb(0x0a080c)),
    (0.539, rgb(0x080708)),
    (0.604, rgb(0x070507)),
    (0.669, rgb(0x050305)),
    (0.734, rgb(0x040304)),
    (0.777, rgb(0x020203)),
    (1.000, rgb(0x020203)),
];

/// Card 4's unselected drawing: its own dark fill, the full card
/// content, the page restored right of the cut, and the open frame.
///
/// The cut is done by *covering* rather than clipping, and that is not
/// a shortcut. `Frame::with_clip` is unusable here: `iced_wgpu` drafts
/// a frame, pastes its meshes back with `Transformation::IDENTITY` and
/// keeps the region only as a scissor -- and measured on this screen,
/// the drafted frame's *mesh* geometry never arrives at all (its text
/// does). A clipped card came out as an empty outline with its title
/// and stats but no icons, no gun and no socket glyph. The design has
/// zero ink right of x=1557, so restoring the page there says the same
/// thing and actually renders.
const CARD4: &[Prim] = &[
    fill_rect(0.0, 151.0, 132.0, 462.0, Ink::Fixed(CARD4_FILL)),
    Prim::At { x: 0.0, y: 0.0, prims: CARD_CUT },
    Prim::Ramp { x: 132.0, y: 151.0, w: 21.0, h: 462.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: CUT_GROUND_L },
    Prim::Ramp { x: 153.0, y: 151.0, w: 22.0, h: 462.0, from: (0.0, 0.0), to: (0.0, 1.0), stops: CUT_GROUND_R },
    shut_path(132.0, 151.0, CARD4_EDGE, Ink::Fg, 1.2),
];

// The nav's five rows and the shelf's four positions, as plates. The
// selected nav row is 5px taller and filled where the others are dark
// boxes, which is what the material shows; the selected card is the
// grown one.
macro_rules! nav {
    ($top:expr, $base:expr, $label:expr) => {
        (
            &[
                Prim::At { x: 153.0, y: $top, prims: NAV_SELECTED },
                txt(163.0, $base, 15.0, Ink::OnSelect, $label),
            ],
            &[
                Prim::At { x: 153.0, y: $top, prims: NAV_ROW },
                txt(163.0, $base, 15.0, Ink::Fg, $label),
            ],
        )
    };
}
const NAV_ON_0: &[Prim] = nav!(248.0, 297.0, "VIDEO").0;
const NAV_OFF_0: &[Prim] = nav!(248.0, 297.0, "VIDEO").1;
const NAV_ON_1: &[Prim] = nav!(318.0, 365.0, "AUDIO").0;
const NAV_OFF_1: &[Prim] = nav!(318.0, 365.0, "AUDIO").1;
const NAV_ON_2: &[Prim] = nav!(385.0, 432.0, "GAMEPLAY").0;
const NAV_OFF_2: &[Prim] = nav!(385.0, 432.0, "GAMEPLAY").1;
const NAV_ON_3: &[Prim] = nav!(452.0, 499.0, "CYBERWARE").0;
const NAV_OFF_3: &[Prim] = nav!(452.0, 499.0, "CYBERWARE").1;
const NAV_ON_4: &[Prim] = nav!(519.0, 566.0, "CONTROLLER").0;
const NAV_OFF_4: &[Prim] = nav!(519.0, 566.0, "CONTROLLER").1;

macro_rules! shelf {
    ($i:expr, $off:expr) => {
        &[Prim::Plate {
            group: Group::Card,
            index: $i,
            x: 0.0,
            y: 151.0,
            w: 284.0,
            h: 462.0,
            on: GROWN,
            off: $off,
        }]
    };
}
const SHELF_0: &[Prim] = shelf!(0, CARD1);
const SHELF_1: &[Prim] = shelf!(1, CARD1);
const SHELF_2: &[Prim] = shelf!(2, CARD3);
const SHELF_3: &[Prim] = shelf!(3, CARD4);

/// A nav row, at its own origin: 208 wide with a 16px bottom-left
/// chamfer. The selected row is 5 taller.
const NAV62: &[Seg] = &[
    Seg::Line(208.0, 0.0),
    Seg::Line(208.0, 62.0),
    Seg::Line(16.0, 62.0),
    Seg::Line(0.0, 46.0),
];
const NAV67: &[Seg] = &[
    Seg::Line(208.0, 0.0),
    Seg::Line(208.0, 67.0),
    Seg::Line(16.0, 67.0),
    Seg::Line(0.0, 51.0),
];
const NAV_ROW: &[Prim] = &[
    Prim::Path { x: 0.0, y: 0.0, segs: NAV62, close: true, fill: Some(Ink::Fixed(NAV_FILL)), stroke: Some(Ink::Dim), width: 1.2 },
    fill_rect(-5.0, 0.0, 3.0, 46.0, Ink::Dim),
];
const NAV_SELECTED: &[Prim] = &[
    fill_path(0.0, 0.0, NAV67, Ink::Select),
    fill_rect(-5.0, 0.0, 3.0, 51.0, Ink::Fg),
];

/// The left-margin chip: two ticks, a block and the numbered 12.5
/// square, one below the origin (`#chip`; photo 12.5x12.1 at (62.5,187.9)).
const CHIP: &[Prim] = &[
    fill_rect(-21.0, 3.0, 8.0, 1.5, Ink::Fg),
    fill_rect(-21.0, 7.0, 6.0, 1.5, Ink::Fg),
    fill_rect(-11.0, 3.0, 6.0, 6.0, Ink::Fg),
    fill_rect(0.0, 1.0, 12.5, 12.5, Ink::Fg),
];
/// The MASURAO band, solid to x~275, and the slanted bar left of the
/// kanji. The band's decay into diagonal hatching past x 275 is the
/// photograph's, not the design's, and is left out.
const BAND: &[Seg] = &[
    Seg::Line(275.0, 108.0),
    Seg::Line(265.0, 130.0),
    Seg::Line(153.0, 130.0),
];
const SLANT: &[Seg] = &[
    Seg::Line(190.0, 72.0),
    Seg::Line(173.0, 105.0),
    Seg::Line(170.0, 105.0),
];
const ARROW: &[Seg] = &[Seg::Line(985.0, 35.0), Seg::Line(985.0, 45.0)];

pub const STORE: &[Prim] = &[
    // the hub backdrop: near-black, the masked cold blue over the top,
    // the warm wash under the left half, the black field at the
    // bottom right, the vignette -- composited, from the trace's defs
    Prim::Soft { prims: STORE_GROUND },
    // top strip
    fill_rect(752.0, 35.0, 5.0, 5.0, Ink::Fg),
    fill_rect(762.0, 31.0, 14.0, 15.0, Ink::Fg),
    txt_bold(765.0, 43.0, 10.0, Ink::OnSelect, "2"),
    txt(780.0, 44.0, 11.0, Ink::Dim, "KIROSHI"),
    txt(862.0, 44.0, 9.0, Ink::Dim, "JHN 102 CKC 151 CC10 AS5"),
    fill_rect(983.0, 39.0, 17.0, 2.0, Ink::Fg),
    fill_path(977.0, 40.0, ARROW, Ink::Fg),
    // MASURAO logotype
    // (`:252-254`: translate(188,102) skewX(-13), letter-spacing 5 -- the
    // tracking is drawn, the 13-degree skew is not: `Prim` has no shear)
    Prim::Tracked { x: 192.0, y: 104.0, size: 36.0, ink: Ink::Fg, face: Face::Bold, anchor: Anchor::Start, tracking: 5.0, content: "益荒男" },
    fill_path(187.0, 72.0, SLANT, Ink::Fg),
    fill_path(160.0, 108.0, BAND, Ink::Fg),
    txt_bold(165.0, 125.0, 15.0, Ink::OnSelect, "MASURAO"),
    // customer block
    fill_rect(153.0, 158.0, 210.0, 21.0, Ink::Dim),
    txt(160.0, 173.0, 12.0, Ink::Fg, "CUSTOMER"),
    txt_end(353.0, 173.0, 12.0, Ink::Fg, "#NC488402"),
    txt(160.0, 204.0, 12.0, Ink::Fg, "LOYALTY DISCOUNT"),
    txt_end(353.0, 204.0, 12.0, Ink::Fg, "10%"),
    txt(160.0, 219.0, 12.0, Ink::Fg, "LAST UPDATE"),
    txt_end(353.0, 219.0, 12.0, Ink::Fg, "10/05/2077"),
    // nav column: the selection filled, the rest dark boxes, each on a
    // 3px spine at x 148
    Prim::Plate { group: Group::Category, index: 0, x: 148.0, y: 248.0, w: 213.0, h: 67.0, on: NAV_ON_0, off: NAV_OFF_0 },
    Prim::Plate { group: Group::Category, index: 1, x: 148.0, y: 318.0, w: 213.0, h: 67.0, on: NAV_ON_1, off: NAV_OFF_1 },
    Prim::Plate { group: Group::Category, index: 2, x: 148.0, y: 385.0, w: 213.0, h: 67.0, on: NAV_ON_2, off: NAV_OFF_2 },
    Prim::Plate { group: Group::Category, index: 3, x: 148.0, y: 452.0, w: 213.0, h: 67.0, on: NAV_ON_3, off: NAV_OFF_3 },
    Prim::Plate { group: Group::Category, index: 4, x: 148.0, y: 519.0, w: 213.0, h: 67.0, on: NAV_ON_4, off: NAV_OFF_4 },
    // left margin
    Prim::At { x: 62.0, y: 186.0, prims: CHIP },
    txt_bold(65.0, 197.0, 10.0, Ink::OnSelect, "1"),
    // the shelf
    Prim::At { x: 437.0, y: 0.0, prims: SHELF_0 },
    Prim::At { x: 769.0, y: 0.0, prims: SHELF_1 },
    Prim::At { x: 1096.0, y: 0.0, prims: SHELF_2 },
    Prim::At { x: 1425.0, y: 0.0, prims: SHELF_3 },
    // footer
    Prim::Rect { x: 153.5, y: 851.5, w: 144.0, h: 22.0, fill: Some(Ink::Border), stroke: Some(Ink::Fg), width: 1.0 },
    fill_rect(215.0, 851.5, 1.0, 22.0, Ink::Fg),
    txt_bold(159.0, 866.0, 10.0, Ink::Fg, "68SD1D1100D1S"),
    txt(221.0, 861.0, 8.0, Ink::Fg, "COMBAT COLONIZATION"),
    txt(221.0, 870.0, 8.0, Ink::Fg, "DEFENCE PROGRAM"),
    fill_rect(153.0, 878.0, 147.0, 1.0, Ink::Border),
    txt(313.0, 872.0, 11.0, Ink::Dim, "00032 05 54 08 CP"),
];
// --- end store -----------------------------------------------------------

// --- dashboard -----------------------------------------------------------
//
// `docs/neomil/dashboard-trace.svg`, transcribed. Coordinates are the
// trace's own in the 1600x900 frame, measured off
// `images/img-07-dashboard.png`; each group below names the trace
// lines it came from, in the trace's paint order. The two `<use>`
// defs -- `#badge` and the two menu cells -- are written once as
// consts and placed with `Prim::At`, and each menu unit is a
// `Prim::Plate` whose hit box is the cell's bounding box.
//
// The three reds are the trace's `#ef3333` / `#ae272b` / `#671b21`,
// which are the same three roles the store block maps to `Ink::Fg` /
// `Ink::Dim` / `Ink::Border` (its `#df3131` / `#96282d` / `#60181a`),
// so the palette still reaches the screen; the ground and the glow
// stops are the trace's own hex, as the store's `GROUND` is.
//
// What the trace draws that is not transcribed as drawn, and why. (The
// blue glow, a horizontal gradient under a vertical mask (:75-101),
// used to head this list: until 2026-09-04 `glow()` rasterised it at
// compile time into 640 strips, three linear pieces standing in for
// the mask's nine stops. It is now `HUB_GLOW`, the construct itself.)
// The `next` logotype (:151-152) is
// *outlined* Orbitron, and the
// scene has neither a stroked text nor an Orbitron face, so it is set
// filled in the bold Rajdhani face. Letter-spacing on the header and
// tab labels is dropped, as the store block drops it (`Prim::Tracked`
// carries it since 2026-09-04; only the module labels below use it
// so far). The body copy of
// the GO HOME panel is drawn as the trace's measured run boxes
// (:232-239) because the trace carries no copy for it.

/// The ground: `#070304`, the k-means' largest cluster (38.8%), two
/// levels off the era's `BG`.
pub const HUB_GROUND: iced::Color = rgb(0x070304);
/// The panel's translucent fill: `#671b21` at `fill-opacity 0.55`
/// (:218) -- the deep red *over* the ground, not a fourth red.
const PANEL_FILL: iced::Color = iced::Color { a: 0.55, ..rgb(0x671b21) };
/// The ground, the masked glow and the vignette (:138-140), composited
/// as one: see `HUB_GLOW`.
const HUB_BACK: &[Prim] = &[
    fill_rect(0.0, 0.0, 1600.0, 900.0, Ink::Fixed(HUB_GROUND)),
    HUB_GLOW,
    HUB_VIGNETTE,
];

/// `#badge` (:135): 59x57 with a 15px bottom-left chamfer, at its own
/// origin. Filled deep red on four of the five uses, mid red on the
/// selected T2.
const BADGE_SEGS: &[Seg] = &[
    Seg::Line(59.0, 0.0),
    Seg::Line(59.0, 57.0),
    Seg::Line(15.0, 57.0),
    Seg::Line(0.0, 42.0),
];
const BADGE: &[Prim] = &[Prim::Path {
    x: 0.0,
    y: 0.0,
    segs: BADGE_SEGS,
    close: true,
    fill: Some(Ink::Border),
    stroke: Some(Ink::Fg),
    width: 1.5,
}];
const BADGE_ON: &[Prim] = &[Prim::Path {
    x: 0.0,
    y: 0.0,
    segs: BADGE_SEGS,
    close: true,
    fill: Some(Ink::Dim),
    stroke: Some(Ink::Fg),
    width: 1.5,
}];

/// `#cell-up` (:122-127): row 1's menu cell at its own centre. A solid
/// diamond of half-diagonal 104 whose top tip is cut flat at 89 (a
/// 30px plateau), an inset outline at 68 cut the same way at 59, and
/// the 43x36 glyph plate.
const CELL_UP_OUTER: &[Seg] = &[
    Seg::Line(15.0, -89.0),
    Seg::Line(104.0, 0.0),
    Seg::Line(0.0, 104.0),
    Seg::Line(-104.0, 0.0),
];
const CELL_UP_INNER: &[Seg] = &[
    Seg::Line(9.0, -59.0),
    Seg::Line(68.0, 0.0),
    Seg::Line(0.0, 68.0),
    Seg::Line(-68.0, 0.0),
];
const CELL_UP: &[Prim] = &[
    fill_path(-15.0, -89.0, CELL_UP_OUTER, Ink::Fg),
    shut_path(-9.0, -59.0, CELL_UP_INNER, Ink::Border, 2.0),
    fill_rect(-22.0, -21.0, 43.0, 36.0, Ink::Border),
];
/// `#cell-down` (:128-133): row 2's cell, `#cell-up` mirrored in y --
/// the bottom tip is the cut one.
const CELL_DOWN_OUTER: &[Seg] = &[
    Seg::Line(104.0, 0.0),
    Seg::Line(15.0, 89.0),
    Seg::Line(-15.0, 89.0),
    Seg::Line(-104.0, 0.0),
];
const CELL_DOWN_INNER: &[Seg] = &[
    Seg::Line(68.0, 0.0),
    Seg::Line(9.0, 59.0),
    Seg::Line(-9.0, 59.0),
    Seg::Line(-68.0, 0.0),
];
const CELL_DOWN: &[Prim] = &[
    fill_path(0.0, -104.0, CELL_DOWN_OUTER, Ink::Fg),
    shut_path(0.0, -68.0, CELL_DOWN_INNER, Ink::Border, 2.0),
    fill_rect(-22.0, -21.0, 43.0, 36.0, Ink::Border),
];

// One menu unit, at its own centre: the plate's box is the cell's
// bounding box (208 wide; 89 above and 104 below the centre for row 1,
// the reverse for row 2), and `on` and `off` are the same drawing
// because the photo shows no selected state.
macro_rules! unit {
    ($i:expr, $top:expr, $cell:expr) => {
        &[Prim::Plate {
            group: Group::Module,
            index: $i,
            x: -104.0,
            y: $top,
            w: 208.0,
            h: 193.0,
            on: $cell,
            off: $cell,
        }]
    };
}
const UNIT_0: &[Prim] = unit!(0, -89.0, CELL_UP);
const UNIT_1: &[Prim] = unit!(1, -89.0, CELL_UP);
const UNIT_2: &[Prim] = unit!(2, -89.0, CELL_UP);
const UNIT_3: &[Prim] = unit!(3, -104.0, CELL_DOWN);
const UNIT_4: &[Prim] = unit!(4, -104.0, CELL_DOWN);
const UNIT_5: &[Prim] = unit!(5, -104.0, CELL_DOWN);

/// The code tape under the logotype (:154), chamfered 2 at both bottom
/// corners.
const CODE_TAPE: &[Seg] = &[
    Seg::Line(379.0, 151.0),
    Seg::Line(379.0, 158.0),
    Seg::Line(377.0, 160.0),
    Seg::Line(259.0, 160.0),
    Seg::Line(257.0, 158.0),
];
/// The DESCRIPTION tab (:181): a box with its left end chamfered 7 at
/// both corners.
const DESCRIPTION_TAB: &[Seg] = &[
    Seg::Line(1354.0, 237.0),
    Seg::Line(1354.0, 258.0),
    Seg::Line(1139.0, 258.0),
    Seg::Line(1132.0, 251.0),
    Seg::Line(1132.0, 244.0),
];
/// The GO HOME panel's outline (:217): square top-left and
/// bottom-right, an 8px top-right chamfer, the right edge stepping 8
/// inward at y 516 below the bright bar, a 42px bottom-left chamfer.
const PANEL: &[Seg] = &[
    Seg::Line(1358.0, 314.0),
    Seg::Line(1366.0, 322.0),
    Seg::Line(1366.0, 508.0),
    Seg::Line(1358.0, 516.0),
    Seg::Line(1358.0, 756.0),
    Seg::Line(1170.0, 756.0),
    Seg::Line(1128.0, 714.0),
];
/// The bright bar on the panel's right edge, y 405..516, chamfered 8
/// at both ends (:220).
const PANEL_BAR: &[Seg] = &[
    Seg::Line(1366.0, 508.0),
    Seg::Line(1358.0, 516.0),
    Seg::Line(1358.0, 414.0),
];
/// The maker's mark (:241-252): a stencil "M" with both stems leaning
/// in from the top, a shoulder on the left stem and a diagonal notch
/// on the right one around a detached square dot. Absolute points of
/// the trace's path, which starts at (1238.8,682.1).
const MAKER_MARK: &[Seg] = &[
    Seg::Line(1255.4, 682.1),
    Seg::Line(1255.8, 698.3),
    Seg::Line(1267.1, 682.1),
    Seg::Line(1283.3, 682.1),
    Seg::Line(1283.3, 707.1),
    Seg::Line(1274.6, 714.6),
    Seg::Line(1271.7, 719.6),
    Seg::Line(1267.9, 725.0),
    Seg::Line(1248.3, 725.0),
    Seg::Line(1248.3, 714.6),
    Seg::Line(1247.1, 714.6),
    Seg::Line(1240.0, 725.0),
    Seg::Line(1220.4, 725.0),
    Seg::Line(1220.4, 699.6),
    Seg::Line(1227.5, 698.8),
    Seg::Line(1229.2, 697.1),
];

pub const DASHBOARD: &[Prim] = &[
    // ground, glow, vignette (:138-140)
    Prim::Soft { prims: HUB_BACK },
    // header, left (:144-154)
    txt(109.0, 90.0, 14.0, Ink::Fg, "CUSTOMER"),
    Prim::At { x: 117.0, y: 104.0, prims: BADGE },
    txt(125.0, 121.0, 12.0, Ink::Fg, "LEVEL"),
    txt_bold(132.0, 140.0, 20.0, Ink::Fg, "T1"),
    txt(240.0, 90.0, 14.0, Ink::Fg, "#NC488402"),
    txt_bold(242.0, 132.0, 42.0, Ink::Fg, "next"),
    fill_path(257.0, 151.0, CODE_TAPE, Ink::Fg),
    // header, right (:156-169): four badges, T2 filled
    txt(1125.0, 90.0, 14.0, Ink::Fg, "SECURITY LEVEL"),
    Prim::At { x: 1133.0, y: 104.0, prims: BADGE },
    Prim::At { x: 1193.0, y: 104.0, prims: BADGE_ON },
    Prim::At { x: 1253.0, y: 104.0, prims: BADGE },
    Prim::At { x: 1313.0, y: 104.0, prims: BADGE },
    txt(1141.0, 121.0, 12.0, Ink::Fg, "LEVEL"),
    txt(1201.0, 121.0, 12.0, Ink::Fg, "LEVEL"),
    txt(1261.0, 121.0, 12.0, Ink::Fg, "LEVEL"),
    txt(1321.0, 121.0, 12.0, Ink::Fg, "LEVEL"),
    txt_bold(1148.0, 140.0, 20.0, Ink::Fg, "T1"),
    txt_bold(1208.0, 140.0, 20.0, Ink::Fg, "T2"),
    txt_bold(1268.0, 140.0, 20.0, Ink::Fg, "T3"),
    txt_bold(1328.0, 140.0, 20.0, Ink::Fg, "T4"),
    // the hairline rule (:174)
    fill_rect(42.0, 187.0, 1516.0, 2.0, Ink::Dim),
    // tab row (:178-184)
    Prim::Rect { x: 27.0, y: 240.0, w: 55.0, h: 17.0, fill: Some(Ink::Border), stroke: Some(Ink::Fg), width: 1.0 },
    Prim::Rect { x: 475.0, y: 237.0, w: 211.0, h: 21.0, fill: Some(Ink::Border), stroke: Some(Ink::Dim), width: 1.0 },
    txt(515.0, 252.0, 12.0, Ink::Fg, "COMPUTER SYSTEMS"),
    Prim::Path { x: 1139.0, y: 237.0, segs: DESCRIPTION_TAB, close: true, fill: Some(Ink::Border), stroke: Some(Ink::Dim), width: 1.0 },
    txt(1145.0, 252.0, 12.0, Ink::Fg, "DESCRIPTION"),
    Prim::Rect { x: 1516.0, y: 239.0, w: 54.0, h: 17.0, fill: Some(Ink::Border), stroke: Some(Ink::Fg), width: 1.0 },
    // the six-diamond menu (:190-195), each cell at its centre
    Prim::At { x: 334.0, y: 460.0, prims: UNIT_0 },
    Prim::At { x: 530.0, y: 460.0, prims: UNIT_1 },
    Prim::At { x: 725.0, y: 460.0, prims: UNIT_2 },
    Prim::At { x: 431.0, y: 593.0, prims: UNIT_3 },
    Prim::At { x: 628.0, y: 592.0, prims: UNIT_4 },
    Prim::At { x: 822.0, y: 592.0, prims: UNIT_5 },
    // the separators (:199-202): 16x30 ground-coloured bars on the
    // midpoint of each same-row pair, cutting the facing side tips
    fill_rect(424.0, 445.0, 16.0, 30.0, Ink::Fixed(HUB_GROUND)),
    fill_rect(619.5, 445.0, 16.0, 30.0, Ink::Fixed(HUB_GROUND)),
    fill_rect(521.5, 577.0, 16.0, 30.0, Ink::Fixed(HUB_GROUND)),
    fill_rect(717.0, 577.0, 16.0, 30.0, Ink::Fixed(HUB_GROUND)),
    // labels (:206-212): 600/19, centred on each cell's x, tracked 1.2
    Prim::Tracked { x: 334.0, y: 347.0, size: 19.0, ink: Ink::Fg, face: Face::SemiBold, anchor: Anchor::Middle, tracking: 1.2, content: "VEHICLES" },
    Prim::Tracked { x: 530.0, y: 347.0, size: 19.0, ink: Ink::Fg, face: Face::SemiBold, anchor: Anchor::Middle, tracking: 1.2, content: "LOCATIONS" },
    Prim::Tracked { x: 725.0, y: 347.0, size: 19.0, ink: Ink::Fg, face: Face::SemiBold, anchor: Anchor::Middle, tracking: 1.2, content: "FACTIONS" },
    Prim::Tracked { x: 431.0, y: 721.0, size: 19.0, ink: Ink::Fg, face: Face::SemiBold, anchor: Anchor::Middle, tracking: 1.2, content: "WEAPONS" },
    Prim::Tracked { x: 628.0, y: 721.0, size: 19.0, ink: Ink::Fg, face: Face::SemiBold, anchor: Anchor::Middle, tracking: 1.2, content: "PRODUCTS" },
    Prim::Tracked { x: 822.0, y: 721.0, size: 19.0, ink: Ink::Fg, face: Face::SemiBold, anchor: Anchor::Middle, tracking: 1.2, content: "CORPORATIONS" },
    // GO HOME panel (:217-244): the outline, the bright edge bar, the
    // glitch echoes 3 and 5 out, the heading, the body run boxes and
    // the maker's mark
    Prim::Path { x: 1128.0, y: 314.0, segs: PANEL, close: true, fill: Some(Ink::Fixed(PANEL_FILL)), stroke: Some(Ink::Fg), width: 1.5 },
    fill_path(1366.0, 405.0, PANEL_BAR, Ink::Fg),
    fill_rect(1369.0, 322.0, 1.5, 193.0, Ink::Border),
    fill_rect(1371.0, 322.0, 1.5, 193.0, Ink::Border),
    fill_rect(1361.0, 516.0, 1.5, 240.0, Ink::Border),
    fill_rect(1363.0, 516.0, 1.5, 240.0, Ink::Border),
    txt_bold(1140.0, 333.0, 20.0, Ink::Fg, "GO HOME"),
    fill_rect(1140.0, 368.0, 202.0, 12.0, Ink::Dim),
    fill_rect(1140.0, 389.0, 215.0, 12.0, Ink::Dim),
    fill_rect(1140.0, 410.0, 197.0, 12.0, Ink::Dim),
    fill_rect(1140.0, 431.0, 121.0, 12.0, Ink::Dim),
    fill_rect(1140.0, 474.0, 205.0, 12.0, Ink::Dim),
    fill_rect(1140.0, 495.0, 216.0, 12.0, Ink::Dim),
    fill_rect(1140.0, 516.0, 186.0, 12.0, Ink::Dim),
    fill_rect(1140.0, 537.0, 193.0, 12.0, Ink::Dim),
    fill_path(1238.8, 682.1, MAKER_MARK, Ink::Fg),
    fill_rect(1273.0, 715.0, 10.75, 11.7, Ink::Fg),
    Prim::Text { x: 1252.0, y: 737.5, size: 8.0, ink: Ink::Fg, face: Face::Bold, anchor: Anchor::Middle, content: "PRECISION LIQUID" },
    Prim::Text { x: 1252.0, y: 746.0, size: 8.0, ink: Ink::Fg, face: Face::Bold, anchor: Anchor::Middle, content: "POLYMER MUSCLE" },
    // margins (:249-251): the rotated micro-text runs as the trace's
    // own bars
    fill_rect(33.0, 463.0, 8.0, 113.0, Ink::Border),
    fill_rect(1533.0, 527.0, 8.0, 238.0, Ink::Border),
    fill_rect(1348.0, 341.0, 4.0, 49.0, Ink::Border),
    // footer tape (:262-271): the dim echo 3px right and down, the
    // bright frame, the divider and the two cells' text
    line_rect(1212.5, 867.5, 144.0, 24.0, Ink::Border, 1.0),
    line_rect(1209.5, 864.5, 144.0, 24.0, Ink::Fg, 1.0),
    fill_rect(1270.0, 864.5, 1.2, 24.0, Ink::Fg),
    txt_bold(1215.0, 875.0, 8.0, Ink::Fg, "68SD1D1100D1S"),
    Prim::Text { x: 1277.0, y: 874.0, size: 7.5, ink: Ink::Fg, face: Face::SemiBold, anchor: Anchor::Start, content: "COMBAT COLONIZATION" },
    Prim::Text { x: 1277.0, y: 882.0, size: 7.5, ink: Ink::Fg, face: Face::SemiBold, anchor: Anchor::Start, content: "DEFENCE PROGRAM" },
];
// --- end dashboard -------------------------------------------------------
