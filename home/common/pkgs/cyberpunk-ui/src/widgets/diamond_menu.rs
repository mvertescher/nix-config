//! Neo-militarism's cut-diamond hub.
//!
//! This is the oldest widget in the crate and the one that started the
//! argument the rest of it settles: a menu that is not a dressed
//! rectangle. It is now the [`crate::style::Menu::Diamonds`] arm of
//! [`super::menu`], which is what finally makes it reachable from a
//! screen -- until that variant existed the crate compiled this and
//! nothing could ask for it.
//!
//! Worth recording, because every other era feature in this crate cites
//! a file: **the diamond hub is not in `docs/`.** Neomil's
//! `target-app.svg` is an ops screen with a services table and its
//! `target-components.svg` is a widget sheet; neither draws a diamond.
//! The shape comes from the pre-generalisation `neomil-ui`, so it is
//! era-specific by inheritance rather than by sampling. Anyone
//! re-deriving the era from the references should expect to find no
//! support for it there.

use crate::fonts::FONT_ORBITRON_BOLD;
use crate::style::Style;
use iced::widget::canvas;
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Size, Theme, Vector};

const DIAMOND_DIAGONAL: f32 = 220.0;
const GAP: f32 = 20.0;
const LABEL_GAP: f32 = 20.0;

#[derive(Debug, Clone)]
pub struct DiamondMenuItem<Message> {
    pub label: String,
    pub subtext: String,
    /// `None` for a display-only hub. The screens in this crate are
    /// design targets and emit nothing; the variant is kept because
    /// this widget is the only interaction model here that was written
    /// to hit-test, and throwing that away to satisfy a menu API would
    /// be the wrong trade.
    pub on_press: Option<Message>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TabEdge {
    None,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TabAnchor {
    None,
    Start, // Anchor at p1
    End,   // Anchor at p2
}

struct ButtonGeometry {
    cut_top: bool,
    cut_bottom: bool,
    cut_left: bool,
    cut_right: bool,
    tab_edge: TabEdge,
}

/// How many diamonds sit in the top row of an `n`-diamond hub. The
/// reference layout is six in two rows of three, and the general rule
/// that reproduces it is "half, rounded up".
fn top_row(n: usize) -> usize {
    n.div_ceil(2)
}

/// Which sides diamond `index` cuts.
///
/// Was a six-armed table with a `panic!` past the end, which is what
/// pinned the hub at exactly six items. The table was regular all
/// along: a diamond cuts the side it shares with the other row, and
/// each side it shares with a neighbour in its own row. Written out,
/// it reproduces the old six entries exactly and stops being a
/// crash for any other count.
fn get_geometry(index: usize, n: usize) -> ButtonGeometry {
    let top = top_row(n);
    let is_top = index < top;
    let (first, last) = if is_top {
        (0, top.saturating_sub(1))
    } else {
        (top, n.saturating_sub(1))
    };

    ButtonGeometry {
        cut_top: is_top,
        cut_bottom: !is_top,
        cut_left: index > first,
        cut_right: index < last,
        tab_edge: if is_top {
            TabEdge::BottomRight
        } else {
            TabEdge::TopRight
        },
    }
}

pub struct DiamondMenuProgram<Message> {
    items: Vec<DiamondMenuItem<Message>>,
    /// Line-work and the diamond's own fill.
    accent: Color,
    /// What sits inside it: glyph, subtext, inner outline.
    inside: Color,
    /// The selected diamond's fill and ink, which in three of the four
    /// eras is a flat `select` and in neokitsch would be a material --
    /// the hub is neomil's, so this stays a pair of colours.
    select: Color,
    on_select: Color,
    selected: Option<usize>,
}

pub struct MenuState {
    hovered_index: Option<usize>,
    cache: canvas::Cache,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            hovered_index: None,
            cache: canvas::Cache::new(),
        }
    }
}

fn hash_str(s: &str) -> u32 {
    let mut hash: u32 = 5381;
    for c in s.bytes() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as u32);
    }
    hash
}

/// The diagonal that fits `n` diamonds into `size`, and their centres.
///
/// The packing is the reference's: two rows, the lower one offset by
/// half a pitch so the diamonds interlock. What is new is that the
/// diagonal shrinks to fit rather than being a constant -- a hub in a
/// dashboard column has nothing like the 1500px the sampled `220`
/// assumes, and at the old fixed size the outer diamonds simply left
/// the canvas.
fn get_centers(size: Size, n: usize) -> (f32, Vec<Point>) {
    let top = top_row(n);
    let bottom = n - top;
    let wide = top.max(bottom).max(1);

    // In units of the diagonal `d`, before scaling.
    let unit_dx = 1.0 + GAP / DIAMOND_DIAGONAL;
    let unit_dy = 0.5 + GAP * (2.0f32.sqrt() - 0.5) / DIAMOND_DIAGONAL;
    // Widest row, plus the half-pitch the other row is offset by.
    let span_x = (wide as f32 - 1.0) * unit_dx + if bottom > 0 { 0.5 } else { 0.0 } + 1.0;
    let span_y = if bottom > 0 { unit_dy + 1.0 } else { 1.0 };
    // Room for the labels, which sit outside the diamonds.
    let pad = LABEL_GAP * 2.0 + 24.0;

    let d = DIAMOND_DIAGONAL
        .min((size.width - pad).max(1.0) / span_x)
        .min((size.height - pad).max(1.0) / span_y);
    let (dx, dy) = (d * unit_dx, d * unit_dy);

    let x_center = size.width / 2.0;
    let y_center = size.height / 2.0;
    // The two rows straddle the centre; a single row sits on it.
    let y0 = y_center - if bottom > 0 { dy / 2.0 } else { 0.0 };

    let row_x = |count: usize, offset: f32| {
        x_center - (count as f32 - 1.0) * dx / 2.0 + offset
    };

    let mut centers = Vec::with_capacity(n);
    let top_x = row_x(top, if bottom > 0 { -dx / 4.0 } else { 0.0 });
    for i in 0..top {
        centers.push(Point::new(top_x + i as f32 * dx, y0));
    }
    let bottom_x = row_x(bottom, dx / 4.0);
    for i in 0..bottom {
        centers.push(Point::new(bottom_x + i as f32 * dx, y0 + dy));
    }
    (d, centers)
}

impl<Message: Clone> canvas::Program<Message> for DiamondMenuProgram<Message> {
    type State = MenuState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let geometry = state.cache.draw(renderer, bounds.size(), |frame| {
            let n = self.items.len();
            if n == 0 {
                return;
            }
            let (d, centers) = get_centers(bounds.size(), n);

            for (i, item) in self.items.iter().enumerate() {
                let center = centers[i];
                let is_hovered = state.hovered_index == Some(i);

                self.draw_button(frame, center, d, item, is_hovered, i, n);
            }
        });

        vec![geometry]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        let n = self.items.len();
        let mut new_hovered_index = None;
        if let Some(local_pos) = cursor.position_in(bounds) {
            let (d, centers) = get_centers(bounds.size(), n);
            let cut_val = d * 0.10;

            for (i, center) in centers.iter().enumerate() {
                let geom = get_geometry(i, n);
                let dx = (local_pos.x - center.x).abs();
                let dy = (local_pos.y - center.y).abs();

                // 1. Must be inside the basic diamond
                if dx + dy <= d / 2.0 {
                    // 2. Check horizontal limits (left/right cuts)
                    let limit_left = if geom.cut_left { d / 2.0 - cut_val } else { d / 2.0 };
                    let limit_right = if geom.cut_right { d / 2.0 - cut_val } else { d / 2.0 };
                    let x_ok = if local_pos.x < center.x {
                        dx <= limit_left
                    } else {
                        dx <= limit_right
                    };

                    // 3. Check vertical limits (top/bottom cuts)
                    let limit_top = if geom.cut_top { d / 2.0 - cut_val } else { d / 2.0 };
                    let limit_bottom = if geom.cut_bottom { d / 2.0 - cut_val } else { d / 2.0 };
                    let y_ok = if local_pos.y < center.y {
                        dy <= limit_top
                    } else {
                        dy <= limit_bottom
                    };

                    if x_ok && y_ok {
                        new_hovered_index = Some(i);
                        break;
                    }
                }
            }
        }

        if new_hovered_index != state.hovered_index {
            state.hovered_index = new_hovered_index;
            state.cache.clear();
        }

        let mut message = None;
        if let canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if let Some(idx) = state.hovered_index {
                message = self.items[idx].on_press.clone();
            }
        }

        let status = if message.is_some() {
            canvas::event::Status::Captured
        } else {
            canvas::event::Status::Ignored
        };

        (status, message)
    }
}

impl<Message> DiamondMenuProgram<Message> {
    #[allow(clippy::too_many_arguments)]
    fn draw_button(
        &self,
        frame: &mut canvas::Frame,
        center: Point,
        d: f32,
        item: &DiamondMenuItem<Message>,
        is_hovered: bool,
        index: usize,
        count: usize,
    ) {
        let geom = get_geometry(index, count);
        let is_top_row = index < top_row(count);
        let is_selected = self.selected == Some(index);

        // Selected fills, unselected outlines.
        //
        // Every diamond used to be filled solid in `accent` and the
        // selected one had nothing to distinguish it -- which in neomil
        // is not "subtle", it is invisible: that era's `fg` and
        // `select` are the same `#de2e2e`, so a hub with one module
        // chosen rendered as six identical red diamonds. The render
        // said so; the code could not.
        //
        // Filled-against-outlined is the era's own convention anyway.
        // `docs/neomil/target-app.svg` draws its action buttons as
        // `fill="#FF3B45"` for the primary and
        // `fill="none" stroke="#FF3B45"` for the ghost, and it is what
        // `Surface::selected` against `Surface::outlined` does
        // everywhere else in this crate.
        // The unselected fill is the page ground rather than nothing,
        // because the diamonds interlock: a transparent one would show
        // its neighbour's corner through its own body.
        let (bg_color, border_color, inside_color) = if is_selected {
            (self.select, self.select, self.on_select)
        } else if is_hovered {
            (self.accent, self.accent, self.inside)
        } else {
            (self.inside, self.accent, self.accent)
        };
        let border_width = if is_hovered { 2.5 } else { 1.5 };

        let cut_val = d * 0.10; // 10% of diagonal (22px for 220px)

        // Calculate outer vertices based on custom cuts
        let p_tl_top = Point::new(center.x - cut_val, center.y - d / 2.0 + cut_val);
        let p_tr_top = Point::new(center.x + cut_val, center.y - d / 2.0 + cut_val);
        let p_tr_right = Point::new(center.x + d / 2.0 - cut_val, center.y - cut_val);
        let p_br_right = Point::new(center.x + d / 2.0 - cut_val, center.y + cut_val);
        let p_br_bottom = Point::new(center.x + cut_val, center.y + d / 2.0 - cut_val);
        let p_bl_bottom = Point::new(center.x - cut_val, center.y + d / 2.0 - cut_val);
        let p_bl_left = Point::new(center.x - d / 2.0 + cut_val, center.y + cut_val);
        let p_tl_left = Point::new(center.x - d / 2.0 + cut_val, center.y - cut_val);

        // Draw outer path with custom cuts and single tab
        let path = canvas::Path::new(|builder| {
            // Start at top-left of top corner if cut, else start at top point
            let start_pt = if geom.cut_top { p_tl_top } else { Point::new(center.x, center.y - d / 2.0) };
            builder.move_to(start_pt);

            // 1. Top edge (horizontal)
            if geom.cut_top {
                builder.line_to(p_tr_top);
            }

            // 2. TR edge (slanted) - Bottom row has TR tab anchored at Top corner (Start)
            let tr_start = if geom.cut_top { p_tr_top } else { Point::new(center.x, center.y - d / 2.0) };
            let tr_end = if geom.cut_right { p_tr_right } else { Point::new(center.x + d / 2.0, center.y) };
            let tr_anchor = if geom.tab_edge == TabEdge::TopRight { TabAnchor::Start } else { TabAnchor::None };
            self.draw_edge(builder, tr_start, tr_end, tr_anchor, d);

            // 3. Right edge (vertical)
            if geom.cut_right {
                builder.line_to(p_br_right);
            }

            // 4. BR edge (slanted) - Top row has BR tab anchored at Bottom corner (End)
            let br_start = if geom.cut_right { p_br_right } else { Point::new(center.x + d / 2.0, center.y) };
            let br_end = if geom.cut_bottom { p_br_bottom } else { Point::new(center.x, center.y + d / 2.0) };
            let br_anchor = if geom.tab_edge == TabEdge::BottomRight { TabAnchor::End } else { TabAnchor::None };
            self.draw_edge(builder, br_start, br_end, br_anchor, d);

            // 5. Bottom edge (horizontal)
            if geom.cut_bottom {
                builder.line_to(p_bl_bottom);
            }

            // 6. BL edge (slanted) - never has tab
            let bl_start = if geom.cut_bottom { p_bl_bottom } else { Point::new(center.x, center.y + d / 2.0) };
            let bl_end = if geom.cut_left { p_bl_left } else { Point::new(center.x - d / 2.0, center.y) };
            self.draw_edge(builder, bl_start, bl_end, TabAnchor::None, d);

            // 7. Left edge (vertical)
            if geom.cut_left {
                builder.line_to(p_tl_left);
            }

            // 8. TL edge (slanted) - never has tab
            let tl_start = if geom.cut_left { p_tl_left } else { Point::new(center.x - d / 2.0, center.y) };
            let tl_end = if geom.cut_top { p_tl_top } else { Point::new(center.x, center.y - d / 2.0) };
            self.draw_edge(builder, tl_start, tl_end, TabAnchor::None, d);

            builder.close();
        });

        frame.fill(&path, bg_color);
        frame.stroke(
            &path,
            canvas::Stroke::default()
                .with_color(border_color)
                .with_width(border_width)
                .with_line_join(canvas::LineJoin::Round),
        );

        // Draw inner path (always clean, matches outer cuts but scaled)
        let d_inner = d * 0.85;
        let cut_val_inner = cut_val * 0.85;

        let pi_tl_top = Point::new(center.x - cut_val_inner, center.y - d_inner / 2.0 + cut_val_inner);
        let pi_tr_top = Point::new(center.x + cut_val_inner, center.y - d_inner / 2.0 + cut_val_inner);
        let pi_tr_right = Point::new(center.x + d_inner / 2.0 - cut_val_inner, center.y - cut_val_inner);
        let pi_br_right = Point::new(center.x + d_inner / 2.0 - cut_val_inner, center.y + cut_val_inner);
        let pi_br_bottom = Point::new(center.x + cut_val_inner, center.y + d_inner / 2.0 - cut_val_inner);
        let pi_bl_bottom = Point::new(center.x - cut_val_inner, center.y + d_inner / 2.0 - cut_val_inner);
        let pi_bl_left = Point::new(center.x - d_inner / 2.0 + cut_val_inner, center.y + cut_val_inner);
        let pi_tl_left = Point::new(center.x - d_inner / 2.0 + cut_val_inner, center.y - cut_val_inner);

        let inner_path = canvas::Path::new(|builder| {
            let start_pt = if geom.cut_top { pi_tl_top } else { Point::new(center.x, center.y - d_inner / 2.0) };
            builder.move_to(start_pt);

            if geom.cut_top {
                builder.line_to(pi_tr_top);
            }

            let tr_end = if geom.cut_right { pi_tr_right } else { Point::new(center.x + d_inner / 2.0, center.y) };
            builder.line_to(tr_end);

            if geom.cut_right {
                builder.line_to(pi_br_right);
            }

            let br_end = if geom.cut_bottom { pi_br_bottom } else { Point::new(center.x, center.y + d_inner / 2.0) };
            builder.line_to(br_end);

            if geom.cut_bottom {
                builder.line_to(pi_bl_bottom);
            }

            let bl_end = if geom.cut_left { pi_bl_left } else { Point::new(center.x - d_inner / 2.0, center.y) };
            builder.line_to(bl_end);

            if geom.cut_left {
                builder.line_to(pi_tl_left);
            }

            let tl_end = if geom.cut_top { pi_tl_top } else { Point::new(center.x, center.y - d_inner / 2.0) };
            builder.line_to(tl_end);

            builder.close();
        });

        frame.stroke(
            &inner_path,
            canvas::Stroke::default()
                .with_color(inside_color)
                .with_width(1.0),
        );

        // Draw procedural glyph (5x5 symmetric grid), scaled with the
        // diamond so a small hub does not wear a full-size stamp.
        let s = d / DIAMOND_DIAGONAL;
        let cell_size = 6.0 * s;
        let cell_gap = 1.0 * s;
        let grid_width = 5.0 * cell_size + 4.0 * cell_gap;
        let grid_left = center.x - grid_width / 2.0;
        let grid_top = center.y - grid_width / 2.0 - 16.0 * s;

        let hash = hash_str(&item.label);
        for r in 0..5 {
            for c in 0..5 {
                let bit_idx = match c {
                    0 | 4 => r,
                    1 | 3 => 5 + r,
                    2 => 10 + r,
                    _ => unreachable!(),
                };
                let is_filled = (hash & (1 << bit_idx)) != 0;
                if is_filled {
                    let px = grid_left + c as f32 * (cell_size + cell_gap);
                    let py = grid_top + r as f32 * (cell_size + cell_gap);
                    frame.fill_rectangle(
                        Point::new(px, py),
                        iced::Size::new(cell_size, cell_size),
                        inside_color,
                    );
                }
            }
        }

        // Draw subtext (inside, below glyph)
        let subtext = canvas::Text {
            content: item.subtext.clone(),
            position: Point::new(center.x, center.y + 24.0 * s),
            color: inside_color,
            size: (13.0 * s).into(),
            font: FONT_ORBITRON_BOLD,
            horizontal_alignment: iced::alignment::Horizontal::Center,
            vertical_alignment: iced::alignment::Vertical::Center,
            ..Default::default()
        };
        frame.fill_text(subtext);

        // Draw label (outside)
        let label_pos = if is_top_row {
            Point::new(center.x, center.y - d / 2.0 - LABEL_GAP)
        } else {
            Point::new(center.x, center.y + d / 2.0 + LABEL_GAP)
        };
        let label_text = canvas::Text {
            content: item.label.clone(),
            position: label_pos,
            color: border_color,
            size: (16.0 * s).max(9.0).into(),
            font: FONT_ORBITRON_BOLD,
            horizontal_alignment: iced::alignment::Horizontal::Center,
            vertical_alignment: if is_top_row {
                iced::alignment::Vertical::Bottom
            } else {
                iced::alignment::Vertical::Top
            },
            ..Default::default()
        };
        frame.fill_text(label_text);
    }

    fn draw_edge(
        &self,
        builder: &mut canvas::path::Builder,
        p1: Point,
        p2: Point,
        anchor: TabAnchor,
        d: f32,
    ) {
        let s = d / DIAMOND_DIAGONAL;
        let depth = 10.0 * s;
        let transition = 10.0 * s;
        let tab_width: f32 = 95.0 * s;

        if anchor == TabAnchor::None {
            builder.line_to(p2);
        } else {
            let v = Vector::new(p2.x - p1.x, p2.y - p1.y);
            let len = (v.x * v.x + v.y * v.y).sqrt();
            let u = Vector::new(v.x / len, v.y / len);
            let n = Vector::new(u.y, -u.x);

            // Ensure tab_width doesn't exceed edge length
            let w = tab_width.min(len);

            let (pt_start, pt_end) = match anchor {
                TabAnchor::Start => {
                    // Start at p1, extend along edge by w
                    (p1, p1 + u * w)
                }
                TabAnchor::End => {
                    // Start at p2 - w, extend to p2
                    (p2 - u * w, p2)
                }
                _ => unreachable!(),
            };

            let pt_step1 = pt_start + u * transition + n * depth;
            let pt_step2 = pt_end - u * transition + n * depth;

            builder.line_to(pt_start);
            builder.line_to(pt_step1);
            builder.line_to(pt_step2);
            builder.line_to(pt_end);
            builder.line_to(p2);
        }
    }
}

/// The hub as a menu: display-only, dressed from the era's palette.
///
/// The [`crate::style::Menu::Diamonds`] arm of [`super::menu`]. It
/// takes the shared [`super::menu::MenuItem`] rather than this module's
/// own item type, which is what stops the vocabulary from having two
/// ideas of what a module is.
pub fn hub<'a, Message: Clone + 'static>(
    style: &Style,
    items: &'a [super::menu::MenuItem<'a>],
    selected: usize,
) -> Element<'a, Message> {
    let program = DiamondMenuProgram {
        items: items
            .iter()
            .map(|item| DiamondMenuItem {
                label: item.label.to_string(),
                subtext: item.code.to_string(),
                on_press: None,
            })
            .collect(),
        accent: style.palette.fg,
        inside: style.palette.bg,
        select: style.palette.select,
        on_select: style.palette.on_select,
        selected: (selected < items.len()).then_some(selected),
    };

    canvas(program)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// The interactive hub, in arbitrary colours.
///
/// Kept beside [`hub`] rather than replaced by it: this is the only
/// widget in the crate that hit-tests, and the screens that would use
/// it are display-only design targets today.
pub fn diamond_menu<'a, Message: 'static + Clone>(
    items: Vec<DiamondMenuItem<Message>>,
    color_accent: Color,
    color_bg: Color,
) -> Element<'a, Message> {
    canvas(DiamondMenuProgram {
        items,
        accent: color_accent,
        inside: color_bg,
        select: color_accent,
        on_select: color_bg,
        selected: None,
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The generalised cut table reproduces the six-item one it
    /// replaced, entry for entry.
    #[test]
    fn six_diamonds_cut_where_the_old_table_said() {
        // (cut_top, cut_bottom, cut_left, cut_right) for indices 0..6.
        let want = [
            (true, false, false, true),
            (true, false, true, true),
            (true, false, true, false),
            (false, true, false, true),
            (false, true, true, true),
            (false, true, true, false),
        ];
        for (i, w) in want.iter().enumerate() {
            let g = get_geometry(i, 6);
            assert_eq!(
                (g.cut_top, g.cut_bottom, g.cut_left, g.cut_right),
                *w,
                "diamond {i}"
            );
        }
    }

    /// Counts other than six used to panic. They lay out instead.
    #[test]
    fn other_counts_lay_out_rather_than_panicking() {
        for n in 1..=9usize {
            let (d, centers) = get_centers(Size::new(900.0, 500.0), n);
            assert_eq!(centers.len(), n, "n={n}");
            assert!(d > 0.0, "n={n} diagonal {d}");
            for i in 0..n {
                let _ = get_geometry(i, n);
            }
        }
    }

    /// The hub shrinks to whatever box it is handed, which is what
    /// makes it usable in a dashboard column rather than only on a
    /// full-screen canvas.
    #[test]
    fn the_diagonal_shrinks_to_fit() {
        let (big, _) = get_centers(Size::new(1600.0, 900.0), 6);
        let (small, centers) = get_centers(Size::new(600.0, 400.0), 6);
        assert!(small < big, "{small} should be under {big}");
        for c in centers {
            assert!(c.x - small / 2.0 > -1.0 && c.x + small / 2.0 < 601.0, "{c:?}");
        }
    }
}
