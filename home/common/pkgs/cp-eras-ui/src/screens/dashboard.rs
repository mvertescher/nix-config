//! The ops dashboard, in any era.
//!
//! The four references do not agree about what a dashboard is.
//! Kitsch and neokitsch frame a module hub -- six modules, one
//! selected, the selected one's description beside it -- while
//! neomil's `images/img-07-dashboard.png` is an ops screen: a
//! full-width cold-blue band carrying red crest blocks and the OPS
//! DASHBOARD wordmark, three large bright-red chart cards side by side
//! with dark slits between them, a vertical red rail on the right and a
//! red corner block bottom-right, and no hub anywhere in it. And
//! entropism's dashboard (`images/entropism-dashboard.png`, Behance
//! screen #42) is neither: a dim-olive top field over a single row of
//! four menu tiles, the second one a solid sage fill. So the *layout*
//! is a value on the era table, [`crate::style::Layout`] -- beside
//! `Menu`, the second thing four dressed rectangles could not express
//! -- and this file is the dispatch that value exists to permit:
//! [`Layout::ModuleHub`] is the hub shell below,
//! [`Layout::OpsCharts`] is [`Dashboard::ops_charts`],
//! [`Layout::TileRow`] is [`Dashboard::tile_row`]. Nothing in this
//! file asks which era it is, which is the standing test and not a
//! comment.
//!
//! The hub shell is written once: six modules, one selected, the
//! selected one's description, and the era's chrome. The four eras do
//! not merely dress a module chooser differently, they reach for four
//! different *objects* -- tiles, an extruded fan, a services table, a
//! card cascade -- and this file cannot pick between them without an
//! era branch. So it names none of them: it hands `menu` six
//! [`MenuItem`]s and a selected index, and [`crate::style::Menu`] on
//! the era table says what a menu is. Until that variant existed the
//! hub drew a hardcoded two-column grid of `Surface`s, which was the
//! right stopgap and the wrong screen for three eras out of four.
//!
//! The [`Layout::OpsCharts`] arm draws none of that. It mirrors the
//! material's own screen straight off
//! `docs/neomil/dashboard-trace.svg`, edge to edge the way the trace
//! draws it: the band is the chrome, the chart-card row is the working
//! area, and the rail and corner block are the trim. The six-module
//! data stays on this screen -- the OpsCharts arm simply does not draw
//! it, the same way each menu arm takes what its object has room for.
//!
//! The [`Layout::TileRow`] arm is the same story for entropism,
//! straight off `docs/entropism/dashboard-trace.svg`: the dim-olive
//! top field, the row of four tiles (T2 selected, solid sage), the
//! caption strips under them carrying each module's code and blurb --
//! reused from `MODULES`, since the six-module data stays on this
//! screen no matter which arm draws it -- and the thin build-rule at
//! the foot. No sidebar, no detail panel, no footer: the material's
//! frame contains none of them, so the arm draws none of them.
//!
//! Neomil's hub arm was a services table until the layout split, and
//! it is now dormant rather than deleted: `Layout::OpsCharts` never
//! consults `menu`, so `Menu::Table` and [`crate::widgets::table`]
//! remain the retained services-table hub arm for any era or host that
//! wants one -- see [`crate::style::Menu::Table`]. Entropism's
//! `Menu::Tiles` grid is retained-dormant the same way: the TileRow
//! arm draws its own tiles and never consults `menu`, so the hub's
//! tile grid stays live for any era or host that wants the hub.
//!
//! Note the column count. The grid here was two wide; entropism's
//! sheet draws its tiles three to a row and the era table says so, so
//! wiring the menu up was also what stopped this screen overriding the
//! reference from the outside.
//!
//! Run it with `cp-eras-ui-dashboard --era <name>`; with no flag it
//! follows the desktop theme.

use crate::palette::Palette;
use crate::style::{Layout, Style};
use crate::widgets::surface::{layered, surface, Surface};
use crate::widgets::{badge, chart_card, footer, ground, marker, menu, text, top_bar, Chart, MenuItem, Slot};
use iced::widget::{canvas, column, container, row, stack, Space};
use iced::mouse;
use iced::{Element, Length, Padding, Point, Rectangle, Renderer, Size, Theme};

/// The modules the hub offers: label, the catalogue code the references
/// print under it, and the one-line blurb the entropism set puts inside
/// each block.
///
/// `static` rather than `const` so `&MODULES` is one `'static` slice
/// the menu borrows, rather than a fresh temporary per call that only
/// lives long enough because of rvalue promotion.
///
/// All three fields are handed over even though only one era draws all
/// three, and that is deliberate: the fan has no room for a blurb and
/// only the table has room for all three, so each arm takes what its
/// object has room for instead of this screen deciding on its behalf.
static MODULES: [MenuItem<'static>; 6] = [
    MenuItem {
        label: "VEHICLES",
        code: "161-9A",
        blurb: "Registered chassis, plates and transit permits.",
    },
    MenuItem {
        label: "LOCATIONS",
        code: "161-9B",
        blurb: "Districts, checkpoints and mapped access routes.",
    },
    MenuItem {
        label: "FACTIONS",
        code: "161-9C",
        blurb: "Standing, known contacts and open contracts.",
    },
    MenuItem {
        label: "WEAPONS",
        code: "161-9D",
        blurb: "Licensed hardware and surplus combat inventory.",
    },
    MenuItem {
        label: "PRODUCTS",
        code: "161-9E",
        blurb: "Catalogue, stock levels and delivery windows.",
    },
    MenuItem {
        label: "CORPORATIONS",
        code: "161-9F",
        blurb: "Charters, subsidiaries and trade agreements.",
    },
];
const SELECTED_MODULE: usize = 3;

const LEVELS: [&str; 4] = ["T1", "T2", "T3", "T4"];
const LEVEL_SELECTED: usize = 1;

const DETAIL: [&str; 2] = [
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
     eiusmod tempor incididunt ut labore et dolore magna aliqua.",
    "Quis ipsum suspendisse ultrices gravida. Risus commodo viverra \
     maecenas accumsan lacus vel facilisis.",
];

const SPEC: [(&str, &str); 4] = [
    ("RECORDS", "1 842"),
    ("CLEARANCE REQUIRED", "T2"),
    ("LAST SYNC", "10/05/2077 04:12"),
    ("SOURCE", "NEXUS NETWORK V10.8"),
];

const VENDORS: [&str; 2] = ["PETROCHEM", "BETTERLIFE TEC"];

const NOTE: [&str; 2] = [
    "SPARE TIME MANAGER WAS DEVELOPED BY SEOCHO.",
    "SERVING CUSTOMERS SINCE 2006.",
];

pub struct Dashboard {
    pub style: Style,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl Dashboard {
    pub fn new(style: Style) -> Self {
        Dashboard { style }
    }

    pub fn title(&self) -> String {
        format!("OPS DASHBOARD — {}", self.style.era.name())
    }

    pub fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> Element<'_, Message> {
        let s = &self.style;
        match s.layout {
            // The hub shell pads its content 40px all round, like the
            // other screens. The ops-charts and tile-row screens do
            // not: their traces draw edge to edge, so the arms do too.
            Layout::ModuleHub => stack![ground(s), container(self.screen()).padding(40)].into(),
            Layout::OpsCharts => stack![ground(s), self.ops_charts()].into(),
            Layout::TileRow => stack![ground(s), self.tile_row()].into(),
        }
    }

    fn screen(&self) -> Element<'_, Message> {
        let s = &self.style;

        column![
            top_bar(
                s,
                [
                    "COMPUTER SYSTEMS SOFTWAREV2",
                    "OPS DASHBOARD",
                    "FLAIR TRS 5MMP",
                ],
            ),
            row![
                container(self.sidebar()).width(Length::Fixed(260.0)),
                Space::new(s.metrics.gap * 2.0, 0.0),
                container(self.modules())
                    .width(Length::FillPortion(6))
                    .height(Length::Fill),
                Space::new(s.metrics.gap * 2.0, 0.0),
                container(self.detail()).width(Length::FillPortion(5)),
            ]
            .height(Length::Fill),
            footer(
                s,
                "INTERFACE LOADED",
                "PROVIDED BY NEXUS NETWORK V10.8",
                "BUILD 6.47.48441.R15",
            ),
        ]
        .spacing(s.metrics.gap)
        .into()
    }

    /// The ops-charts screen from the material, for
    /// [`Layout::OpsCharts`].
    ///
    /// A stack of full-frame layers, each computing its geometry as
    /// fractions of the frame so the proportions hold at any window
    /// size: the backdrop (ground, band, crests, left margin, blue
    /// zone), the three chart cards with their dark slits
    /// ([`crate::widgets::chart_card`]), and the trim (right rail,
    /// corner block, bottom dots, the band's wordmark). The fraction
    /// table is `docs/neomil/dashboard-trace.svg` read at its 1600x900
    /// frame.
    fn ops_charts(&self) -> Element<'_, Message> {
        let s = &self.style;
        stack![
            canvas(OpsBackdrop { style: s })
                .width(Length::Fill)
                .height(Length::Fill),
            chart_card(s, Slot::Left, Chart::Line),
            chart_card(s, Slot::Middle, Chart::Bars),
            chart_card(s, Slot::Right, Chart::Line),
            canvas(OpsTrim { style: s })
                .width(Length::Fill)
                .height(Length::Fill),
        ]
        .into()
    }

    /// The four-tile row from the material, for [`Layout::TileRow`].
    ///
    /// A single full-frame canvas, fraction geometry exactly like the
    /// ops-charts arm -- `docs/entropism/dashboard-trace.svg` read at
    /// its 1600x900 frame -- drawing the dim-olive top field, the
    /// boxed letter header, the four tiles (T2, `LOCATIONS`, the solid
    /// sage selection), the caption strips under them and the thin
    /// build-rule at the foot. The first four `MODULES` become the
    /// tiles, labels and codes included, the same way the hub's menu
    /// takes what its object has room for.
    fn tile_row(&self) -> Element<'_, Message> {
        stack![
            canvas(TileRowBackdrop {
                style: &self.style,
                tiles: &MODULES[..4],
                selected: TILE_SELECTED,
            })
            .width(Length::Fill)
            .height(Length::Fill),
        ]
        .into()
    }

    /// Logotype, operator meta, clearance grid, footnote.
    fn sidebar(&self) -> Element<'_, Message> {
        let s = &self.style;

        let meta = |k: &'static str, v: &'static str| {
            row![
                text::label(s, k),
                Space::new(Length::Fill, Length::Shrink),
                text::body(s, v),
            ]
        };

        let mut grid = column![].spacing(s.metrics.gap * 0.5);
        let mut pair = row![].spacing(s.metrics.gap * 0.5);
        for (i, level) in LEVELS.iter().enumerate() {
            pair = pair.push(badge(s, level, i == LEVEL_SELECTED, 56.0));
            if i % 2 == 1 {
                grid = grid.push(pair);
                pair = row![].spacing(s.metrics.gap * 0.5);
            }
        }

        column![
            text::title(s, "next").size(52),
            text::label(s, "T E C H N O L O G Y"),
            Space::new(0.0, s.metrics.gap),
            meta("CUSTOMER", "#NC488402"),
            meta("CLEARANCE", "T2"),
            meta("LAST UPDATE", "10/05/2077"),
            Space::new(0.0, s.metrics.gap * 1.5),
            self.heading("A", "SECURITY LEVEL"),
            Space::new(0.0, s.metrics.gap),
            grid,
            Space::new(0.0, s.metrics.gap * 2.0),
            marker(s, "B", &NOTE),
        ]
        .spacing(4)
        .into()
    }

    /// The module hub: six modules, one of them selected, in whatever
    /// the era means by a menu.
    ///
    /// The menu is given the whole remaining column rather than its
    /// natural height, and that is load-bearing for two of the four
    /// arms. `Menu::Fan` is a canvas that fits itself to the box it is
    /// handed; in a `Shrink` column it is handed nothing and draws
    /// nothing. The three layout arms are indifferent to it -- tiles,
    /// the cascade and the table keep their sampled heights and leave
    /// the slack below, which is what the sheets do under their own.
    fn modules(&self) -> Element<'_, Message> {
        let s = &self.style;

        column![
            self.heading("C", "COMPUTER SYSTEMS"),
            Space::new(0.0, s.metrics.gap),
            container(menu(s, &MODULES, SELECTED_MODULE)).height(Length::Fill),
        ]
        .height(Length::Fill)
        .into()
    }

    /// The selected module's description, and the vendors it is
    /// licensed from.
    fn detail(&self) -> Element<'_, Message> {
        let s = &self.style;
        let module = &MODULES[SELECTED_MODULE];

        let mut body = column![
            text::title(s, module.label).size(s.metrics.text_title - 3),
            text::caption(s, module.code),
            Space::new(0.0, s.metrics.gap),
        ]
        .spacing(2);

        for para in DETAIL {
            body = body.push(text::body(s, para));
            body = body.push(Space::new(0.0, 10.0));
        }

        // Key-value rows under the prose, as every era's detail panel
        // has: the panel is the tall element on this screen and prose
        // alone leaves it reading as an empty box.
        body = body.push(Space::new(0.0, s.metrics.gap));
        for (k, v) in SPEC {
            body = body.push(
                row![
                    text::label(s, k),
                    Space::new(Length::Fill, Length::Shrink),
                    text::body(s, v),
                ]
                .width(Length::Fill),
            );
            body = body.push(Space::new(0.0, 6.0));
        }

        // `layered` rather than `backdrop`: the latter runs its content
        // at `Length::Fill`, which is right for a panel and wrong for a
        // chip -- two vendor tags would take half the column each.
        let mut vendors = row![].spacing(s.metrics.gap * 0.5);
        for vendor in VENDORS {
            vendors = vendors.push(layered(
                canvas(Surface::outlined(s))
                    .width(Length::Fill)
                    .height(Length::Fill),
                container(text::caption(s, vendor)).padding(Padding::from([4, 10])),
            ));
        }

        column![
            self.heading("D", "DESCRIPTION"),
            Space::new(0.0, s.metrics.gap),
            container(surface(Surface::outlined(s), s.metrics.pad, body)).height(Length::Fill),
            Space::new(0.0, s.metrics.gap),
            vendors,
        ]
        .spacing(0)
        .into()
    }

    /// A boxed letter and a caption: the section header every era uses.
    fn heading<'a>(&'a self, letter: &'a str, label: &'a str) -> Element<'a, Message> {
        row![marker(&self.style, letter, &[]), text::body(&self.style, label)]
            .spacing(8)
            .align_y(iced::Alignment::Center)
            .into()
    }
}

// ------------------------------------------------------------- ops-charts

/// Linear interpolation for the band's strip gradient. A helper rather
/// than a dependency: `Color` is just four `f32`s.
fn lerp(a: iced::Color, b: iced::Color, t: f32) -> iced::Color {
    iced::Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: 1.0,
    }
}

/// The backdrop of the ops-charts screen: ground, the full-width cold
/// band, its crest blocks, and the two dark zones the trace draws
/// behind the working area.
///
/// All geometry is `docs/neomil/dashboard-trace.svg` at 1600x900,
/// restated as fractions of the frame. The band's gradient is drawn as
/// stacked vertical strips interpolating `BAND_TOP` to `BAND_BOTTOM`
/// -- the same call `ground` makes with its stacked discs: smooth
/// enough at this scale, and it keeps the crate off renderer-specific
/// gradient support.
struct OpsBackdrop<'a> {
    style: &'a Style,
}

impl<Message> canvas::Program<Message> for OpsBackdrop<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let (w, h) = (bounds.width, bounds.height);
        if w <= 0.0 || h <= 0.0 {
            return vec![frame.into_geometry()];
        }
        let s = self.style;

        frame.fill(
            &canvas::Path::rectangle(Point::ORIGIN, bounds.size()),
            s.palette.bg,
        );

        // The band: the trace's rows 0-2, 172px of 900. The sampled
        // consts live on the neomil table; `palette` has no role for a
        // band that only one layout draws.
        let (band_h, band_top, band_bot) = (0.19111 * h, crate::eras::neomil::BAND_TOP, crate::eras::neomil::BAND_BOTTOM);
        let steps = 24;
        for i in 0..steps {
            let t0 = i as f32 / steps as f32;
            let t1 = (i + 1) as f32 / steps as f32;
            let c = lerp(band_top, band_bot, (t0 + t1) / 2.0);
            frame.fill(
                &canvas::Path::rectangle(
                    Point::new(t0 * w, 0.0),
                    Size::new((t1 - t0) * w + 1.0, band_h),
                ),
                c,
            );
        }

        // The crest blocks on the band: top-left x 240..400, top-right
        // x 1150..1302, both y 112..170 in the trace. `dim` is the
        // era's mid red.
        let (crest_y, crest_h) = (0.12444 * h, 0.06444 * h);
        frame.fill(
            &canvas::Path::rectangle(Point::new(0.15 * w, crest_y), Size::new(0.10 * w, crest_h)),
            s.palette.dim,
        );
        frame.fill(
            &canvas::Path::rectangle(
                Point::new(0.71875 * w, crest_y),
                Size::new(0.095 * w, crest_h),
            ),
            s.palette.dim,
        );

        // The dark left margin (a plain background zone in the source,
        // not a sidebar with content) and the mid-left blue zone it
        // borders. Both read as near-black red / a stop of the band's
        // own blue; `on_select` and `BAND_BOTTOM` are the family.
        frame.fill(
            &canvas::Path::rectangle(
                Point::new(0.0, band_h),
                Size::new(0.11875 * w, h - band_h),
            ),
            s.palette.on_select,
        );
        frame.fill(
            &canvas::Path::rectangle(
                Point::new(0.1875 * w, band_h),
                Size::new(0.51875 * w, 0.25111 * h),
            ),
            band_bot,
        );

        vec![frame.into_geometry()]
    }
}

/// The foreground trim of the ops-charts screen: the right rail and its
/// dark header strip, the bottom-right corner block, the three faint
/// dots under the cards, and the band's wordmark.
struct OpsTrim<'a> {
    style: &'a Style,
}

impl<Message> canvas::Program<Message> for OpsTrim<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let (w, h) = (bounds.width, bounds.height);
        if w <= 0.0 || h <= 0.0 {
            return vec![frame.into_geometry()];
        }
        let s = self.style;
        // The trace's dark trim red; same const as the cards' notches.
        let dark = crate::eras::neomil::CARD_DARK;

        // The vertical rail on the right: trace x 1150..1410, y 396..837,
        // the trace's #501414, which `border` (RED_DEEP) is the family
        // of. Its 24px dark header strip reads as the rail's own cap.
        let (rail_x, rail_y, rail_w, rail_h) = (0.71875 * w, 0.44 * h, 0.1625 * w, 0.49 * h);
        frame.fill(
            &canvas::Path::rectangle(Point::new(rail_x, rail_y), Size::new(rail_w, rail_h)),
            s.palette.border,
        );
        frame.fill(
            &canvas::Path::rectangle(
                Point::new(rail_x, rail_y),
                Size::new(rail_w, 0.026667 * h),
            ),
            dark,
        );

        // Bottom-right corner block: trace x 1200..1356, y 792..900,
        // the trace's #310b0d -- `CARD_DARK`'s own family.
        frame.fill(
            &canvas::Path::rectangle(
                Point::new(0.75 * w, 0.88 * h),
                Size::new(0.0975 * w, 0.12 * h),
            ),
            dark,
        );

        // The three faint markers under the cards, trace x 250/450/650,
        // y 837..855.
        for x in [0.15625f32, 0.28125, 0.40625] {
            frame.fill(
                &canvas::Path::rectangle(
                    Point::new(x * w, 0.93 * h),
                    Size::new(0.01875 * w, 0.02 * h),
                ),
                s.palette.on_select,
            );
        }

        // The band's wordmark, centred between the crest blocks the
        // way the source carries it. The tape (off-white) on the cold
        // blue is the stencil reading of the material.
        frame.fill_text(canvas::Text {
            content: "OPS DASHBOARD".to_string(),
            position: Point::new(0.5 * w, 0.155 * h),
            color: s.palette.tape,
            size: (0.036 * h).into(),
            font: crate::fonts::FONT_RAJDHANI_BOLD,
            horizontal_alignment: iced::alignment::Horizontal::Center,
            vertical_alignment: iced::alignment::Vertical::Center,
            ..Default::default()
        });

        vec![frame.into_geometry()]
    }
}

// ------------------------------------------------------------ tile-row

/// Which of the four tiles the material shows selected. The trace's T2
/// -- second from the left, `LOCATIONS` -- is the solid sage fill. The
/// hub's `SELECTED_MODULE` selects the third module instead; the tile
/// row deliberately does not reuse it, because the two screens select
/// different things and the material is explicit about this one.
const TILE_SELECTED: usize = 1;

/// The tile row's geometry, from `docs/entropism/dashboard-trace.svg`
/// at its 1600x900 frame, restated as fractions of the frame: `(x,
/// w)` for each of the four tiles.
///
/// The row is deliberately *not* an even grid, because the trace is
/// not one: T2, the selected tile, is the widest (x 368..864) and the
/// others taper around it -- 178 / 496 / 400 / 208 of 1600 -- leaving
/// a 94px gap at the row's start and 32px gaps between the rest.
const TILE_ROW: [(f32, f32); 4] = [
    (96.0 / 1600.0, 178.0 / 1600.0),
    (368.0 / 1600.0, 496.0 / 1600.0),
    (896.0 / 1600.0, 400.0 / 1600.0),
    (1328.0 / 1600.0, 208.0 / 1600.0),
];

/// The caption strips under the tiles, same frame and convention. Each
/// sits 8px inside its tile's leading edge; only T1 and T4's reach the
/// tile's width, which is how the trace draws them.
const TILE_STRIPS: [(f32, f32); 4] = [
    (104.0 / 1600.0, 178.0 / 1600.0),
    (376.0 / 1600.0, 200.0 / 1600.0),
    (904.0 / 1600.0, 240.0 / 1600.0),
    (1336.0 / 1600.0, 200.0 / 1600.0),
];

/// The backdrop of the tile-row screen: ground, the dim-olive top
/// field, the boxed letter header, the four tiles with their labels
/// and codes, the caption strips, and the thin build-rule at the foot.
///
/// All geometry is `docs/entropism/dashboard-trace.svg` at 1600x900,
/// restated as fractions of the frame. The era table has no dedicated
/// field or strip colour -- the material's are blends -- so the field
/// is `border` (OUTLINE) faded to a quarter alpha over the page ground
/// and the strips to two-fifths, via [`crate::palette::Palette::faded`].
struct TileRowBackdrop<'a> {
    style: &'a Style,
    tiles: &'a [MenuItem<'a>],
    selected: usize,
}

impl<Message> canvas::Program<Message> for TileRowBackdrop<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let (w, h) = (bounds.width, bounds.height);
        if w <= 0.0 || h <= 0.0 {
            return vec![frame.into_geometry()];
        }
        let s = self.style;

        frame.fill(&canvas::Path::rectangle(Point::ORIGIN, bounds.size()), s.palette.bg);

        // The dim-olive top field: the trace's rows 0-5, y 0..230 of
        // 900. The trace's own field colour (#272a1c) sits between the
        // page ground and the era's `dim`, and the read that lands
        // there is `border` at a quarter alpha -- the same trick
        // `ground` uses for its stacked discs, so no gradient support
        // is needed.
        frame.fill(
            &canvas::Path::rectangle(Point::new(0.0, 0.0), Size::new(w, 230.0 / 900.0 * h)),
            Palette::faded(s.palette.border, 0.25),
        );

        // The boxed-letter header top-left: [A] and TILE MENU, trace
        // x 100..146 and 160..490, y 120..166, both olive outlines in
        // `fg` ink. The era's boxed headings are exactly this object,
        // drawn at the trace's proportions rather than through the
        // hub's `heading` helper, which does not box its label.
        let (hy, hh) = (120.0 / 900.0 * h, 46.0 / 900.0 * h);
        let mut boxed = |x: f32, bw: f32, cx: f32, label: &str, size: f32| {
            let rect = canvas::Path::rectangle(Point::new(x * w, hy), Size::new(bw * w, hh));
            frame.stroke(
                &rect,
                canvas::Stroke::default()
                    .with_color(s.palette.border)
                    .with_width(s.metrics.stroke),
            );
            frame.fill_text(canvas::Text {
                content: label.to_string(),
                position: Point::new(cx * w, hy + hh / 2.0),
                color: s.palette.fg,
                size: size.into(),
                font: crate::fonts::FONT_RAJDHANI_REGULAR,
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                ..Default::default()
            });
        };
        boxed(100.0 / 1600.0, 46.0 / 1600.0, 123.0 / 1600.0, "A", 26.0);
        boxed(160.0 / 1600.0, 330.0 / 1600.0, 325.0 / 1600.0, "TILE MENU", 20.0);

        // The four tiles, trace y 270..560 (rows 25-60% of the frame).
        // T2 is the selection: solid sage with the era's selection ink.
        // The other three are dim-olive fills under olive outlines --
        // the trace's T1/T3/T4 family, which the palette reads as
        // `dim` + `border`.
        let (ty, th) = (270.0 / 900.0 * h, 290.0 / 900.0 * h);
        for (i, (tx, tw)) in TILE_ROW.iter().enumerate() {
            let item = &self.tiles[i];
            let selected = i == self.selected;
            let (fill, ink, edge) = if selected {
                (s.palette.select, s.palette.on_select, s.palette.select)
            } else {
                (s.palette.dim, s.palette.fg, s.palette.border)
            };
            let rect = canvas::Path::rectangle(Point::new(tx * w, ty), Size::new(tw * w, th));
            frame.fill(&rect, fill);
            frame.stroke(
                &rect,
                canvas::Stroke::default()
                    .with_color(edge)
                    .with_width(s.metrics.stroke),
            );

            // Label and catalogue code in the tile's top content zone
            // (the trace's inner block, y 286..406): the name at body
            // size, the code beneath at caption size, both in the
            // tile's own ink.
            let x = tx * w + 8.0;
            frame.fill_text(canvas::Text {
                content: item.label.to_string(),
                position: Point::new(x, ty + 24.0),
                color: ink,
                size: (s.metrics.text_body as f32).into(),
                font: crate::fonts::FONT_RAJDHANI_REGULAR,
                ..Default::default()
            });
            frame.fill_text(canvas::Text {
                content: item.code.to_string(),
                position: Point::new(x, ty + 44.0),
                color: ink,
                size: (s.metrics.text_caption as f32).into(),
                font: crate::fonts::FONT_RAJDHANI_REGULAR,
                ..Default::default()
            });
        }

        // The caption strips under each tile, trace y 596..630: faint
        // olive bars carrying the module's catalogue code and blurb at
        // caption size. Only the table arm has room for the blurb, so
        // here the strip is where the material puts it. The ink is the
        // era's secondary tone -- `mid_ink`, dim-to-fg at 0.6 -- which
        // is what the trace's tiny captions read as against the dark
        // bars, and it is derived rather than published because no era
        // publishes a mid colour of its own.
        let (sy, sh) = (596.0 / 900.0 * h, 34.0 / 900.0 * h);
        for (i, (sx, sw)) in TILE_STRIPS.iter().enumerate() {
            let item = &self.tiles[i];
            let rect = canvas::Path::rectangle(Point::new(sx * w, sy), Size::new(sw * w, sh));
            frame.fill(&rect, Palette::faded(s.palette.border, 0.4));
            frame.fill_text(canvas::Text {
                content: format!("{} · {}", item.code, item.blurb),
                position: Point::new(sx * w + 6.0, sy + sh / 2.0),
                color: text::mid_ink(s),
                size: ((s.metrics.text_caption - 2) as f32).into(),
                font: crate::fonts::FONT_RAJDHANI_REGULAR,
                vertical_alignment: iced::alignment::Vertical::Center,
                ..Default::default()
            });
        }

        // The thin build-rule at the foot, trace y 860..862: 2px in
        // `dim`, full width -- the whole of the chrome this frame
        // keeps.
        frame.fill(
            &canvas::Path::rectangle(Point::new(0.0, 860.0 / 900.0 * h), Size::new(w, 2.0)),
            s.palette.dim,
        );

        vec![frame.into_geometry()]
    }
}
