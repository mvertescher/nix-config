//! A chart card: the unit the ops-charts dashboard is made of.
//!
//! `docs/neomil/dashboard-trace.svg` -- the schematic of
//! `images/img-07-dashboard.png` -- is an ops screen made of three
//! large bright-red chart cards side by side with dark slits between
//! them. That is a widget, not a screen: the dashboard's
//! [`crate::style::Layout::OpsCharts`] arm positions the row, and this
//! module draws one member of it -- the chamfered card, the dark trim
//! the trace cuts into its top and foot, and a simple chart inked on
//! the red body.
//!
//! It is era-agnostic the way every widget here is. The card takes the
//! era's corner ([`crate::style::Corner`], bottom-right per the trace)
//! and the era's palette -- `select` is neomil's `RED_FILL`, the
//! `on_select` ink the charts are drawn in -- and only the dark trim
//! colour is read from the neomil table, exactly as `surface` reads
//! `neokitsch`'s veneer consts when it needs them. Only `OpsCharts`
//! wears it today, but a second era adopting that layout is a table
//! entry, not a rewrite.

use super::surface::{era_cut, outline, Corners};
use crate::style::{Style, Ticket};
use iced::widget::canvas;
use iced::{mouse, Element, Length, Point, Rectangle, Renderer, Size, Theme, Vector};

/// Where in the trio a cell sits. Each cell is the whole frame -- the
/// arm stacks three of these canvases -- and the slot says which of the
/// trace's three card columns this one draws, plus whether it owns the
/// dark slit on its right.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    Left,
    Middle,
    Right,
}

/// What the card plots inside its red body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Chart {
    /// A single polyline across the plot.
    Line,
    /// A row of vertical bars.
    Bars,
}

// Geometry, as fractions of the 1600x900 frame, read off
// `docs/neomil/dashboard-trace.svg` so the widget and the schematic
// cannot disagree: cards at x=250/450/650, width 150, y 337..790, and
// 50px dark slits between them.
const CARD_X: [f32; 3] = [0.15625, 0.28125, 0.40625]; // 250, 450, 650
const CARD_W: f32 = 0.09375; // 150
const CARD_Y: f32 = 0.37444; // 337 / 900
const CARD_H: f32 = 0.50333; // 453 / 900
const SLIT_W: f32 = 0.03125; // 50
// The dark notch the trace walks across the top of each card
// (`M250 337 h150 v12 l-6 10 h-144 Z`): 12px down, then a 6/10 step.
const TOP_STEP: f32 = 0.013333; // 12 / 900
const TOP_STEP_W: f32 = 0.00375; // 6 / 1600
const TOP_DIAG: f32 = 0.011111; // 10 / 900
// The dark foot strip, 18px tall, stopped on the chamfer line.
const FOOT_H: f32 = 0.02; // 18 / 900

/// The card in `slot`, plotting `chart`, filling its frame.
pub fn chart_card<'a, Message: 'static>(
    style: &'a Style,
    slot: Slot,
    chart: Chart,
) -> Element<'a, Message> {
    canvas(ChartCard { style, slot, chart })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

struct ChartCard<'a> {
    style: &'a Style,
    slot: Slot,
    chart: Chart,
}

impl<Message> canvas::Program<Message> for ChartCard<'_> {
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
        let slot = self.slot as usize;
        let (x0, y0, cw, ch) = (
            CARD_X[slot] * w,
            CARD_Y * h,
            CARD_W * w,
            CARD_H * h,
        );

        // The dark slit to this card's right -- the trace's 50px gap
        // between cards. The background alone would show through, but
        // the material reads as a deliberate dark channel, and drawing
        // it here keeps the gap part of the widget rather than an
        // accident of the arm's composition. Omitted after the last
        // card, which has nothing on its right.
        if slot < 2 {
            frame.fill(
                &canvas::Path::rectangle(
                    Point::new(x0 + cw, y0),
                    Size::new(SLIT_W * w, ch),
                ),
                s.palette.on_select,
            );
        }

        // The card: the era's corner treatment cut into the bottom-right
        // only, exactly where the trace's cards trail off. `outline`
        // builds the walk in local coordinates, so translate to the
        // card's origin.
        let cut = s.corner.inset().min(cw / 2.0).min(ch / 2.0);
        let card = outline(
            Corners::square().with_bottom_right(era_cut(s.corner)),
            Ticket::default(),
            cw,
            ch,
        );
        frame.with_save(|frame| {
            frame.translate(Vector::new(x0, y0));
            frame.fill(&card, s.palette.select);
        });

        // Dark trim: the top notch and the foot strip from the trace.
        // `CARD_DARK` is the neomil table's sampled dark red -- see
        // `eras::neomil`, same deal as `surface` reading neokitsch's
        // veneer consts.
        let dark = crate::eras::neomil::CARD_DARK;
        frame.fill(
            &canvas::Path::new(|b| {
                b.move_to(Point::new(x0, y0));
                b.line_to(Point::new(x0 + cw, y0));
                b.line_to(Point::new(x0 + cw, y0 + TOP_STEP * h));
                b.line_to(Point::new(x0 + cw - TOP_STEP_W * w, y0 + (TOP_STEP + TOP_DIAG) * h));
                b.line_to(Point::new(x0, y0 + (TOP_STEP + TOP_DIAG) * h));
                b.close();
            }),
            dark,
        );
        frame.fill(
            &canvas::Path::new(|b| {
                b.move_to(Point::new(x0, y0 + ch - FOOT_H * h));
                b.line_to(Point::new(x0 + cw, y0 + ch - FOOT_H * h));
                // Walk the chamfer so the strip does not bleed past the
                // card's cut corner.
                b.line_to(Point::new(x0 + cw, y0 + ch - cut));
                b.line_to(Point::new(x0 + cw - cut, y0 + ch));
                b.line_to(Point::new(x0, y0 + ch));
                b.close();
            }),
            dark,
        );

        // The chart, inked in the dark selection ink between the trims.
        let ink = s.palette.on_select;
        let (px0, px1) = (x0 + 0.10 * cw, x0 + 0.90 * cw);
        let (py_top, py_bot) = (
            y0 + (TOP_STEP + TOP_DIAG) * h + 0.12 * ch,
            y0 + ch - FOOT_H * h - 0.12 * ch,
        );
        let plot = (py_bot - py_top).max(1.0);
        match self.chart {
            Chart::Line => {
                // A polyline with a couple of ascents: the trace offers
                // no plot geometry, so this is the simple chart the
                // material's red bodies read as, not a sampled curve.
                let heights = [0.52f32, 0.74, 0.38, 0.82, 0.46, 0.70, 0.44];
                let n = heights.len();
                let path = canvas::Path::new(|b| {
                    for (i, hgt) in heights.iter().enumerate() {
                        let x = px0 + (px1 - px0) * i as f32 / (n - 1) as f32;
                        let y = py_bot - hgt * plot;
                        if i == 0 {
                            b.move_to(Point::new(x, y));
                        } else {
                            b.line_to(Point::new(x, y));
                        }
                    }
                });
                frame.stroke(
                    &path,
                    canvas::Stroke::default().with_color(ink).with_width(2.5),
                );
            }
            Chart::Bars => {
                let heights = [0.55f32, 0.85, 0.40, 0.70, 0.48];
                let n = heights.len();
                let step = (px1 - px0) / n as f32;
                let bar_w = step * 0.5;
                for (i, hgt) in heights.iter().enumerate() {
                    let x = px0 + step * i as f32 + (step - bar_w) / 2.0;
                    let bh = hgt * plot;
                    frame.fill(
                        &canvas::Path::rectangle(
                            Point::new(x, py_bot - bh),
                            Size::new(bar_w, bh),
                        ),
                        ink,
                    );
                }
            }
        }

        vec![frame.into_geometry()]
    }
}