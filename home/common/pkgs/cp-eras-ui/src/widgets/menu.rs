//! The menu, in whichever shape the era means by one.
//!
//! Every other widget here is a rectangle four eras dress differently.
//! This is the exception the crate's own README records: offered a
//! choice of modules, the four references reach for four different
//! *objects*, and no corner radius turns a table into a fan. So the
//! choice is a value on [`Style`] -- [`crate::style::Menu`], beside
//! `Chrome` and `Footnotes` -- and this module is the four-armed
//! `match` that value exists to permit. One `match`, in one file, on a
//! parameter the era table declares: that is the shape the abstraction
//! allows, and it is why `screens/` still contains no `if era ==`.
//!
//! Three arms are laid out and one is painted:
//!
//! * [`Menu::Tiles`] is `Surface` in a grid, so entropism's selection
//!   fill and square corners arrive without this file naming them.
//! * [`Menu::Cascade`] is `Surface` in a staggered row, which is what
//!   gets neokitsch's clipped corner *and* its veneer for free -- the
//!   cascade's active card is filled with a material, and nothing here
//!   had to know that.
//! * [`Menu::Table`] hands off to [`super::table`], which is the same
//!   deal one level up: a header band, ruled rows and a selected row,
//!   all of it the shared vocabulary. It replaced the cut-diamond hub,
//!   the one arm here that had ever cited no sheet -- see
//!   [`crate::style::Menu::Table`] for that whole argument.
//! * [`Menu::Fan`] is the one genuinely new drawing: rotated slabs
//!   cannot be laid out, only painted.

use super::surface::{surface, Surface};
use super::text;
use crate::fonts::FONT_RAJDHANI_BOLD;
use crate::style::{Menu, Style};
use iced::widget::{canvas, column, container, row, Space};
use iced::{mouse, Color, Element, Length, Padding, Point, Rectangle, Renderer, Theme};

/// One entry. The three strings are what every era's sheet prints
/// against a module: its name, the catalogue code beneath it, and the
/// one-line blurb the entropism set puts inside the block. An era that
/// has no room for a field simply does not draw it -- the fan has no
/// room for a blurb and only the table has room for all three.
#[derive(Debug, Clone, Copy)]
pub struct MenuItem<'a> {
    pub label: &'a str,
    pub code: &'a str,
    pub blurb: &'a str,
}

/// A menu of `items` with one of them selected, in the era's own idea
/// of what a menu is.
///
/// The `Message: Clone` bound is gone with the diamond hub, which was
/// the one arm that emitted a message of its own. Every arm here is
/// posed rather than wired now; a screen that wants a menu to be
/// clickable wraps its items in `mouse_area`, the way `panels::mail`
/// and `bar` already do, rather than asking a canvas to hit-test.
pub fn menu<'a, Message: 'static>(
    style: &Style,
    items: &'a [MenuItem<'a>],
    selected: usize,
) -> Element<'a, Message> {
    match style.menu {
        Menu::Tiles { columns } => tiles(style, items, selected, columns),
        Menu::Cascade => cascade(style, items, selected),
        Menu::Fan => fan(style, items, selected),
        Menu::Table => table_menu(style, items, selected),
    }
}

// ---------------------------------------------------------------- table

/// The columns neomil's services table is drawn with, restated against
/// what a [`MenuItem`] actually carries.
///
/// `docs/neomil/target-app.svg` runs `UNIT | MEM | UPTIME | STATE` --
/// a name and then facts about it -- at widths of roughly `356 | 120 |
/// 208 | 216` out of 900. A module has a name, a catalogue code and a
/// blurb, so it is three columns in the same shape: the name wide, the
/// code narrow, the sentence taking the rest.
///
/// This is also the one arm that draws all three fields. The screen
/// hands over label, code and blurb without knowing which era will use
/// which, precisely so that each object can take what it has room for;
/// the fan has room for a name, the tiles for a name and a code, and a
/// table row for the lot.
const TABLE_COLUMNS: [super::table::Column<'static>; 3] = [
    super::table::Column::new("MODULE", 5),
    super::table::Column::new("CODE", 2),
    super::table::Column::new("DESCRIPTION", 12),
];

/// Neomil: the services table its own sheet puts in this slot.
fn table_menu<'a, Message: 'static>(
    style: &Style,
    items: &'a [MenuItem<'a>],
    selected: usize,
) -> Element<'a, Message> {
    let rows: Vec<super::table::Row<'a>> = items
        .iter()
        .map(|item| super::table::Row::new([item.label, item.code, item.blurb]))
        .collect();

    super::table::table(style, &TABLE_COLUMNS, &rows, Some(selected))
}

// ---------------------------------------------------------------- tiles

/// Sampled from `docs/entropism/target-components.svg`: the tiles are
/// `120x120`, the hairline sits 8 below one and the caption strip 14
/// below that, at 7pt in `dim`.
const TILE_HEIGHT: f32 = 148.0;
const CAPTION_GAP: f32 = 8.0;
const STRIP: &str = "REPORT ERROR · V2.11 · CERTIFIED";

/// Entropism: a grid of square tiles, each under a rule and a caption.
///
/// The reference is unusually literal about the caption -- the same
/// string under every tile, on every screen, whatever the tile is --
/// and that is the era rather than a placeholder: "dense small
/// maintenance captions throughout" is one of its recorded rules.
fn tiles<'a, Message: 'static>(
    style: &Style,
    items: &'a [MenuItem<'a>],
    selected: usize,
    columns: usize,
) -> Element<'a, Message> {
    let s = style;
    let columns = columns.max(1);

    let mut grid = column![].spacing(s.metrics.gap);
    let mut line = row![].spacing(s.metrics.gap);
    let mut in_line = 0;

    for (i, item) in items.iter().enumerate() {
        line = line.push(tile(s, item, i == selected));
        in_line += 1;
        if in_line == columns {
            grid = grid.push(line);
            line = row![].spacing(s.metrics.gap);
            in_line = 0;
        }
    }
    // A short last row keeps its column width rather than stretching to
    // fill: the reference grid is ragged and the tiles stay square.
    if in_line > 0 {
        for _ in in_line..columns {
            line = line.push(Space::new().width(Length::Fill));
        }
        grid = grid.push(line);
    }

    grid.into()
}

fn tile<'a, Message: 'static>(
    style: &Style,
    item: &'a MenuItem<'a>,
    selected: bool,
) -> Element<'a, Message> {
    let s = style;
    let bg = if selected {
        Surface::selected(s)
    } else {
        Surface::outlined(s)
    };

    let (name, code) = if selected {
        (
            text::on_select(s, item.label),
            text::on_select(s, item.code).size(f32::from(s.metrics.text_caption)),
        )
    } else {
        (text::body(s, item.label), text::caption(s, item.code))
    };

    column![
        container(surface(
            bg,
            Padding::from([10, 12]),
            column![name, code].spacing(1),
        ))
        .width(Length::Fill)
        .height(Length::Fixed(TILE_HEIGHT)),
        Space::new().height(CAPTION_GAP),
        // The hairline the caption hangs off. `rule` is the mail list's
        // separator and this is the same 1px in `dim`, so it is that
        // widget rather than a second idea of a rule.
        super::row::rule(s),
        text::caption(s, STRIP).size(f32::from(s.metrics.text_caption - 2)),
    ]
    .width(Length::Fill)
    .into()
}

// -------------------------------------------------------------- cascade

/// Sampled from `docs/neokitsch/target-components.svg`: the cards are
/// `68x134` -- about one to two -- at an `88` pitch, and the second,
/// third and fourth sit `30`, `34` and `30` above the first.
const CASCADE_HEIGHT: f32 = 240.0;
const CASCADE_STAGGER: f32 = 30.0;

/// Neokitsch: staggered clipped-corner cards, the active one veneered.
///
/// The stagger in the reference is `0, -30, -34, -30`: one card down
/// and the rest up at roughly one height, not a monotone staircase. The
/// `34` is hand-drawn jitter, so this lifts every card but the first by
/// the one figure.
///
/// Nothing here mentions veneer or clipped corners. `Surface::selected`
/// carries the material and `Corner::ClipTopRight` the shape, which is
/// the whole reason this arm is layout and not a canvas.
fn cascade<'a, Message: 'static>(
    style: &Style,
    items: &'a [MenuItem<'a>],
    selected: usize,
) -> Element<'a, Message> {
    let s = style;
    // `Top`, and it is not cosmetic. With the row aligned `Bottom` the
    // spacer above the first card grows its column and the alignment
    // then pulls every card's foot back onto the same line, so the
    // stagger cancels itself out exactly. Rendered that way it is a
    // plain row of cards -- which is what it looked like the first
    // time, before the render said otherwise.
    let mut deck = row![]
        .spacing(s.metrics.gap)
        .align_y(iced::alignment::Vertical::Top);

    for (i, item) in items.iter().enumerate() {
        let is_selected = i == selected;
        let bg = if is_selected {
            Surface::selected(s)
        } else {
            Surface::outlined(s)
        };

        // The name sits at the card's own foot, as the sheet draws it:
        // a spacer above rather than an alignment, so the label keeps
        // its natural height when the card is short.
        let name = if is_selected {
            text::on_select(s, item.label).size(f32::from(s.metrics.text_caption + 2))
        } else {
            text::body(s, item.label).size(f32::from(s.metrics.text_caption + 2))
        };

        let card = container(surface(
            bg,
            Padding::from([12, 12]),
            column![Space::new().height(Length::Fill), name],
        ))
        .width(Length::Fill)
        .height(Length::Fixed(CASCADE_HEIGHT));

        deck = deck.push(
            column![
                // `Length::Shrink` rather than a fixed 0 for the
                // un-staggered cards: iced 0.14 drops a child whose
                // width or height is exactly `Fixed(0.0)` out of the
                // row or column entirely, and this spacer is meant to
                // be present and empty, not absent.
                Space::new().height(if i == 0 {
                    Length::Fixed(CASCADE_STAGGER)
                } else {
                    Length::Shrink
                }),
                card,
            ]
            .width(Length::Fill),
        );
    }

    deck.into()
}

// ------------------------------------------------------------------ fan

/// Sampled from `docs/kitsch/target-components.svg`, "EXTRUDED FAN
/// MENU". The three slabs run at `-15.14`, `+6.98` and `+26.57`
/// degrees -- so `21` apart about a `+6` centre -- and each is about
/// `99` long by `25` thick. Solving the three inner ends against their
/// own directions puts a common pivot at roughly `140` behind them,
/// which is the figure that regularises a hand-drawn fan into one that
/// works for any number of slabs.
const FAN_SPREAD: f32 = 21.0;
const FAN_CENTRE: f32 = 6.0;
const FAN_INNER: f32 = 140.0;
const FAN_LENGTH: f32 = 99.0;
const FAN_THICK: f32 = 25.0;

/// How far the *whole* fan may open, first slab's axis to last.
///
/// The sampled `21` is a step between neighbours, and a step is the
/// wrong thing to hold fixed: six modules at `21` span `105` degrees,
/// which is not a fan, it is most of a quarter turn -- and that is what
/// the kitsch dashboard was rendering, the top slab pointing up-left
/// and the bottom one down-left with the labels running vertically.
///
/// The sheet is the measure of how far a fan opens. Its three slabs
/// span `41.7` degrees axis to axis, and about `52` from the first
/// slab's leading edge to the last one's trailing edge, since a `25`
/// thick slab standing off a `140` pivot is itself some `10` degrees
/// wide. `60` is the round figure just past that silhouette: a sixth of
/// a turn, still unmistakably a fan, and comfortably clear of the
/// sampled case so that three slabs land exactly where the sheet puts
/// them and six still open wider than three.
///
/// Capping the total rather than shrinking [`FAN_SPREAD`] is the point.
/// The step is a sampled figure and the reference fan must keep it; the
/// cap only ever binds on counts the sheet never drew.
const FAN_MAX_SPREAD: f32 = 60.0;

/// The angle between neighbouring slabs in a fan of `n`.
fn slab_step(n: usize) -> f32 {
    if n < 2 {
        return FAN_SPREAD;
    }
    FAN_SPREAD.min(FAN_MAX_SPREAD / (n - 1) as f32)
}

/// The axis of slab `i` of `n`, in degrees, measured clockwise from the
/// x axis the way canvas coordinates run.
fn slab_angle(i: usize, n: usize) -> f32 {
    let mid = (n as f32 - 1.0) / 2.0;
    FAN_CENTRE + (i as f32 - mid) * slab_step(n)
}

/// How far behind the slabs the pivot sits, for a fan of `n`.
///
/// The sampled `140` for as long as it works, and further back when it
/// does not. Two slabs `t` thick whose axes are `step` apart clear each
/// other at the inner end only while the pivot is at least
/// `t / tan(step)` away, so a fan that keeps opening but is capped --
/// and therefore packs its slabs closer as it grows -- would eventually
/// draw them through one another. At the sampled three that bound is
/// `65` and at six it is `118`, both well inside `140`, so this changes
/// nothing any screen draws today; it is what stops the eighth module
/// from being a wedge of overlapping slabs.
fn fan_inner(n: usize) -> f32 {
    let step = slab_step(n).to_radians();
    if step <= 0.0 {
        return FAN_INNER;
    }
    FAN_INNER.max(FAN_THICK / step.tan())
}
/// The extrusion: two stacked outline copies receding up-right at half
/// opacity, `transform="translate(6 -8)"` and `translate(12 -16)`.
const FAN_STEP: (f32, f32) = (6.0, -8.0);
const FAN_LAYERS: usize = 2;

/// Kitsch: extruded slabs fanned about a pivot.
fn fan<'a, Message: 'static>(
    style: &Style,
    items: &'a [MenuItem<'a>],
    selected: usize,
) -> Element<'a, Message> {
    let (bevel, shade) = style.relief();
    canvas(Fan {
        slabs: items
            .iter()
            .enumerate()
            .map(|(i, item)| Slab {
                label: item.label,
                selected: i == selected,
            })
            .collect(),
        face: bevel,
        shade,
        select: style.palette.select,
        on_face: style.palette.bg,
        on_select: style.palette.on_select,
        stroke: style.metrics.stroke,
        text_size: style.metrics.text_body as f32,
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

struct Slab<'a> {
    label: &'a str,
    selected: bool,
}

struct Fan<'a> {
    slabs: Vec<Slab<'a>>,
    /// The lit face of an unselected slab, and the darker tone its
    /// extrusion recedes in: `Style::relief`, which is exactly the pair
    /// `roles.nix` added `bevel`/`shade` for.
    face: Color,
    shade: Color,
    select: Color,
    on_face: Color,
    on_select: Color,
    stroke: f32,
    text_size: f32,
}

/// The four corners of slab `i` of `n`, in fan coordinates: the pivot
/// at the origin, x to the right, y down.
///
/// Pulled out of the drawing so the geometry can be reasoned about --
/// and tested -- without a renderer.
fn slab_quad(i: usize, n: usize, thick: f32) -> [Point; 4] {
    let rad = slab_angle(i, n).to_radians();
    let (sin, cos) = rad.sin_cos();
    // Along the slab, and across it.
    let along = |r: f32| Point::new(r * cos, r * sin);
    let across = (-sin * thick, cos * thick);

    let inner = fan_inner(n);
    let a = along(inner);
    let b = along(inner + FAN_LENGTH);
    [
        a,
        b,
        Point::new(b.x + across.0, b.y + across.1),
        Point::new(a.x + across.0, a.y + across.1),
    ]
}

impl<Message> canvas::Program<Message> for Fan<'_> {
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
        let n = self.slabs.len();
        if w <= 0.0 || h <= 0.0 || n == 0 {
            return vec![frame.into_geometry()];
        }

        let quads: Vec<[Point; 4]> = (0..n).map(|i| slab_quad(i, n, FAN_THICK)).collect();

        // Fit the whole fan, extrusion included, into the box it was
        // handed. Even capped at `FAN_MAX_SPREAD` a six-slab fan is
        // half again the reference's sweep, so a fixed placement would
        // run off the edge the first time a screen offered more
        // modules.
        let step_x = FAN_STEP.0 * FAN_LAYERS as f32;
        let step_y = FAN_STEP.1 * FAN_LAYERS as f32;
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for quad in &quads {
            for p in quad {
                x0 = x0.min(p.x).min(p.x + step_x);
                y0 = y0.min(p.y).min(p.y + step_y);
                x1 = x1.max(p.x).max(p.x + step_x);
                y1 = y1.max(p.y).max(p.y + step_y);
            }
        }
        let (span_x, span_y) = ((x1 - x0).max(1.0), (y1 - y0).max(1.0));
        // Grows as well as shrinks, unlike the diamond hub, and the
        // difference is deliberate. The hub's sampled `220` diagonal is
        // already about a seventh of the 1600 it was drawn for, so
        // holding it fixed keeps the sampled proportion. The fan's
        // slabs are `99` long on a 1920 sheet whose fan occupies a
        // quarter of a column, so holding *those* fixed in a menu-sized
        // box draws a postage stamp in the middle of it -- which is
        // what the first render of this showed.
        let scale = (w / span_x).min(h / span_y);
        let place = |p: Point| {
            Point::new(
                (p.x - x0) * scale + (w - span_x * scale) / 2.0,
                (p.y - y0) * scale + (h - span_y * scale) / 2.0,
            )
        };

        let quad_path = |quad: &[Point; 4], dx: f32, dy: f32| {
            canvas::Path::new(|b| {
                let shift = |p: &Point| place(Point::new(p.x + dx, p.y + dy));
                b.move_to(shift(&quad[0]));
                for p in &quad[1..] {
                    b.line_to(shift(p));
                }
                b.close();
            })
        };

        // Extrusions first, behind: stacked *outlines*, not fills, at
        // half opacity, which is what makes the slab read as a solid
        // with depth rather than as a drop shadow.
        for layer in 1..=FAN_LAYERS {
            let (dx, dy) = (FAN_STEP.0 * layer as f32, FAN_STEP.1 * layer as f32);
            for quad in &quads {
                frame.stroke(
                    &quad_path(quad, dx, dy),
                    canvas::Stroke::default()
                        .with_color(Color {
                            a: 0.5,
                            ..self.shade
                        })
                        .with_width(self.stroke),
                );
            }
        }

        for (i, quad) in quads.iter().enumerate() {
            let slab = &self.slabs[i];
            let (fill, ink) = if slab.selected {
                (self.select, self.on_select)
            } else {
                (self.face, self.on_face)
            };
            frame.fill(&quad_path(quad, 0.0, 0.0), fill);

            // The label runs along the slab, so it is drawn under the
            // slab's own rotation. Rotated text falls out of iced's
            // cached-glyph path and is filled as outlines instead,
            // which is exactly what is wanted here and the reason the
            // fan is a canvas at all.
            let angle = slab_angle(i, n).to_radians();
            // A third of the way along the slab and halfway across it:
            // the reference sets its labels off the inner end rather
            // than centred, so a long name runs outwards.
            let anchor = place(Point::new(
                (quad[0].x + quad[1].x) / 2.0 + (quad[3].x - quad[0].x) / 2.0,
                (quad[0].y + quad[1].y) / 2.0 + (quad[3].y - quad[0].y) / 2.0,
            ));
            frame.with_save(|frame| {
                frame.translate(iced::Vector::new(anchor.x, anchor.y));
                frame.rotate(angle);
                frame.fill_text(canvas::Text {
                    content: slab.label.to_string(),
                    position: Point::ORIGIN,
                    color: ink,
                    size: (self.text_size * scale).into(),
                    font: FONT_RAJDHANI_BOLD,
                    align_x: iced::advanced::text::Alignment::Center,
                    align_y: iced::alignment::Vertical::Center,
                    ..Default::default()
                });
            });
        }

        vec![frame.into_geometry()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The three-slab fan is the reference's: `-15`, `+7`, `+27`
    /// degrees. Checked through the geometry rather than the constants,
    /// so a change to how the spread is centred is caught.
    #[test]
    fn three_slabs_sit_at_the_sampled_angles() {
        let expected = [-15.0f32, 6.0, 27.0];
        for (i, want) in expected.iter().enumerate() {
            let quad = slab_quad(i, 3, 0.0);
            let (dx, dy) = (quad[1].x - quad[0].x, quad[1].y - quad[0].y);
            let got = dy.atan2(dx).to_degrees();
            assert!(
                (got - want).abs() < 0.01,
                "slab {i}: wanted {want} degrees, got {got}"
            );
        }
    }

    /// The sampled slab is 99 long and 25 thick, whatever the fan's
    /// width, because a fan of one is still a slab.
    #[test]
    fn a_slab_keeps_its_sampled_size() {
        let quad = slab_quad(1, 3, FAN_THICK);
        let len = ((quad[1].x - quad[0].x).powi(2) + (quad[1].y - quad[0].y).powi(2)).sqrt();
        let thick = ((quad[3].x - quad[0].x).powi(2) + (quad[3].y - quad[0].y).powi(2)).sqrt();
        assert!((len - FAN_LENGTH).abs() < 0.01, "length {len}");
        assert!((thick - FAN_THICK).abs() < 0.01, "thickness {thick}");
    }

    /// Six modules fan wider than three and stay in order, rather than
    /// wrapping or overlapping.
    #[test]
    fn more_slabs_widen_the_fan_in_order() {
        let angle = |i: usize, n: usize| {
            let quad = slab_quad(i, n, 0.0);
            (quad[1].y - quad[0].y).atan2(quad[1].x - quad[0].x).to_degrees()
        };
        for n in [1usize, 3, 6] {
            for i in 1..n {
                assert!(angle(i, n) > angle(i - 1, n), "n={n} i={i} out of order");
            }
        }
        assert!((angle(0, 6) - angle(5, 6)).abs() > (angle(0, 3) - angle(2, 3)).abs());
    }

    /// However many modules a screen offers, the fan stays a fan: the
    /// whole sweep is capped, so it never opens past `FAN_MAX_SPREAD`.
    /// Six slabs at the sampled step would have spanned 105 degrees.
    #[test]
    fn the_fan_never_opens_past_the_cap() {
        for n in 1..=12usize {
            let span = (slab_angle(n - 1, n) - slab_angle(0, n)).abs();
            assert!(
                span <= FAN_MAX_SPREAD + 0.01,
                "n={n} spans {span} degrees"
            );
        }
    }

    /// Packing more slabs into a capped sweep pushes them together, so
    /// the pivot goes back far enough that they never draw through one
    /// another at the inner end.
    #[test]
    fn slabs_never_overlap_at_the_pivot() {
        for n in 2..=12usize {
            // The inner corner of slab `i`'s far edge must not reach
            // past the inner corner of slab `i+1`'s near edge.
            for i in 0..n - 1 {
                let a = slab_quad(i, n, FAN_THICK);
                let b = slab_quad(i + 1, n, FAN_THICK);
                let angle = |p: Point| p.y.atan2(p.x).to_degrees();
                assert!(
                    angle(a[3]) <= angle(b[0]) + 0.01,
                    "n={n} slab {i} overlaps its neighbour"
                );
            }
        }
    }

    /// The sampled fan is untouched by the cap: three slabs still stand
    /// off the sheet's own 140 pivot.
    #[test]
    fn the_sampled_fan_keeps_its_pivot() {
        assert!((fan_inner(3) - FAN_INNER).abs() < 0.01);
        assert!((fan_inner(6) - FAN_INNER).abs() < 0.01);
    }
}
