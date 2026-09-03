//! The data table: a header band, rows of cells, one row selected.
//!
//! This is the widget `docs/neomil/target-components.svg` names "TABLE
//! / LIST" and `docs/neomil/target-app.svg` puts where
//! [`crate::screens::dashboard`] puts its menu -- a *services table*,
//! four columns wide, one row picked out. Until it existed that slot
//! wore the cut-diamond hub, which was a stand-in inherited from the
//! pre-generalisation crate and cited no sheet at all; see
//! [`crate::style::Menu::Table`] for the whole of that argument.
//!
//! Both neomil sheets were deleted 2026-09-03: neither sampled the neomil
//! run (`docs/sources.md`), the services table was an original composition,
//! and no neomil trace has one -- the hub photo has a six-module menu. The
//! figures below are kept as the record of what this widget was built to;
//! whether it survives is the `Layout` decision in `TODO.md`.
//!
//! Era-agnostic like the rest of `widgets/`, and cheaply so, because a
//! table turns out to be things this crate already has: [`Surface`] for
//! the selected row's fill -- which is how neokitsch's veneer and
//! kitsch's yellow arrive without this file naming either -- the
//! [`super::row::rule`] hairline for the separators, and [`super::text`]
//! for the three inks. Nothing here asks which era it is in.
//!
//! ## The figures
//!
//! Both neomil sheets draw the same object at two sizes, and the
//! disagreements are hand-drawing rather than intent:
//!
//! * **Header band.** `rect x=116 y=132 w=900 h=30` filled `#FF3B45` at
//!   `fill-opacity 0.12` in `target-app.svg`; `h=26` and `#DE2E2E` at
//!   `0.15` in `target-components.svg`. So: a wash of the era's hot
//!   colour at about an eighth, behind small structural labels. `select`
//!   rather than `alert` is the role, because the band marks the table
//!   rather than warning about it -- and in kitsch, where the two are
//!   the same colour, it makes no difference either way.
//! * **Row pitch.** Baselines 38 apart on the 1920 app sheet, 30 on the
//!   components sheet. We draw at 1600, where the app figure scales to
//!   about 32, which is also the middle of the two.
//! * **Cell inset.** `text x=132` against a table at `x=116`: 16, which
//!   is exactly neomil's `metrics.pad` and within four of every other
//!   era's, so it is that rather than a fresh constant -- and the
//!   table then insets by whatever the era it is worn by says.
//! * **Separators.** `line x1=116 x2=1016 stroke=#5E1112` -- `border` --
//!   between adjacent rows, and *not* on either side of the selected
//!   one, whose own fill already separates it. The app sheet leaves the
//!   last row open and the components sheet closes it with a rule; we
//!   close it, because in a column with slack underneath an open table
//!   reads as truncated.
//! * **Inks.** Three of them, and the sheets are consistent about
//!   which: the first column is the row's name and takes `fg`, the rest
//!   are figures and take the ink between `dim` and `fg`
//!   ([`super::text::mid`], the same call the product card's stat heads
//!   make). A row that is not live -- the `acme-renew` timer, drawn
//!   entirely in `#5E1112` -- drops to `dim` throughout. On the selected
//!   row every cell takes `on_select`, because the fill under them is
//!   the era's selection and not a wash.
//!
//! ## What is deliberately not here
//!
//! The sheets draw a scroll rail beside the table (`rect x=1022 y=132
//! w=6 h=210` with an `#FF3B45` thumb 80 long). It is not drawn here
//! and that is a decision, not an omission: a rail is a statement that
//! rows exist off-screen, this widget shows every row it is handed, and
//! the one caller today sits in a column with room to spare below it. A
//! rail would be decoration that lies. It lands with the first caller
//! that actually windows its rows, which is also the first caller that
//! can say how long the thumb should be.

use super::row::rule;
use super::surface::{surface, Surface};
use super::text;
use crate::palette::Palette;
use crate::style::Style;
use iced::widget::{column as col, container, row as iced_row, Space};
use iced::{Element, Length, Padding};

/// Height of the header band. `h=26` on the components sheet; the app
/// sheet's `30` is the same band on a 1920 sheet.
const HEAD_HEIGHT: f32 = 26.0;
/// Row pitch. 38 on the app sheet at 1920, 30 on the components sheet.
const ROW_HEIGHT: f32 = 32.0;
/// How strongly the header band washes the ground: `0.12` on the app
/// sheet, `0.15` on the components sheet.
const HEAD_WASH: f32 = 0.13;

/// One column: its heading and its share of the table's width.
///
/// A share rather than a width because the sheets set their columns by
/// eye against one fixed table (`UNIT` 356 wide, `MEM` 120, `UPTIME`
/// 208, `STATE` 216, of 900) and a screen here is handed whatever the
/// layout leaves it. The ratios are the sampled part; the pixels are
/// not.
#[derive(Debug, Clone, Copy)]
pub struct Column<'a> {
    pub head: &'a str,
    /// Weight, as [`Length::FillPortion`].
    pub portion: u16,
}

impl<'a> Column<'a> {
    pub const fn new(head: &'a str, portion: u16) -> Self {
        Column { head, portion }
    }
}

/// One row. `cells` runs left to right and is truncated or padded to
/// the column count, so a caller cannot desynchronise the two.
#[derive(Debug, Clone)]
pub struct Row<'a> {
    pub cells: Vec<&'a str>,
    /// The row is present but not live -- the sheet's `acme-renew.timer`
    /// with `--` for its figures, drawn entirely in the deep red. Every
    /// cell drops to `dim`.
    pub muted: bool,
}

impl<'a> Row<'a> {
    pub fn new(cells: impl IntoIterator<Item = &'a str>) -> Self {
        Row {
            cells: cells.into_iter().collect(),
            muted: false,
        }
    }

    pub fn muted(mut self) -> Self {
        self.muted = true;
        self
    }
}

/// A table of `rows` under `columns`, with at most one row selected.
///
/// Sized by its content: the header and the rows keep their sampled
/// heights and leave whatever slack the caller's box has underneath,
/// which is what the tile grid and the card cascade do in
/// [`super::menu`] and what the sheet does under its own table.
pub fn table<'a, Message: 'static>(
    style: &Style,
    columns: &[Column<'a>],
    rows: &[Row<'a>],
    selected: Option<usize>,
) -> Element<'a, Message> {
    let s = style;
    let pad = Padding {
        top: 0.0,
        right: s.metrics.pad,
        bottom: 0.0,
        left: s.metrics.pad,
    };

    // The header band is a plain rectangle even in the eras that treat
    // their corners: it is a band, not a container. `Surface::square`
    // records why, and the product card's stat band is the same call.
    let head_wash = Palette::faded(s.palette.select, HEAD_WASH);
    let mut heads = iced_row![].spacing(0);
    for c in columns {
        heads = heads.push(
            container(text::mid(s, c.head).size(f32::from(s.metrics.text_caption + 2)))
                .width(Length::FillPortion(c.portion.max(1))),
        );
    }
    let header = container(surface(
        Surface::filled(s, head_wash).no_stroke().square(),
        Padding {
            top: centre(HEAD_HEIGHT, s.metrics.text_caption + 2),
            ..pad
        },
        heads,
    ))
    .width(Length::Fill)
    .height(Length::Fixed(HEAD_HEIGHT));

    let mut body = col![].spacing(0);
    for (i, r) in rows.iter().enumerate() {
        let is_selected = selected == Some(i);

        let mut cells = iced_row![].spacing(0);
        for (j, c) in columns.iter().enumerate() {
            let content = r.cells.get(j).copied().unwrap_or("");
            // First column names the row and takes `fg`; the rest are
            // figures and take the ink between `dim` and `fg`. A muted
            // row is `dim` throughout, and a selected one `on_select`.
            let cell = match (is_selected, r.muted, j) {
                (true, _, _) => text::on_select(s, content),
                (false, true, _) => text::label(s, content),
                (false, false, 0) => text::body(s, content),
                (false, false, _) => text::mid(s, content),
            };
            cells = cells.push(container(cell).width(Length::FillPortion(c.portion.max(1))));
        }

        let line = container(surface(
            if is_selected {
                Surface::selected(s)
            } else {
                // Unselected rows are ruled, not boxed -- an outline per
                // row turns the table into a grid, which is the same
                // call `row::mail_row` makes.
                Surface::outlined(s).no_stroke()
            },
            Padding {
                top: centre(ROW_HEIGHT, s.metrics.text_body),
                ..pad
            },
            cells,
        ))
        .width(Length::Fill)
        .height(Length::Fixed(ROW_HEIGHT));

        body = body.push(line);

        // A hairline between two adjacent rows, unless one of them is
        // the selected row -- its fill is already the separation, and
        // the sheet draws no rule against it. The last row is closed
        // off regardless.
        let last = i + 1 == rows.len();
        let next_selected = selected == Some(i + 1);
        if last || !(is_selected || next_selected) {
            body = body.push(rule(s));
        } else {
            // Keep the pitch even where the rule is suppressed.
            body = body.push(Space::new().width(Length::Fill).height(1.0));
        }
    }

    col![header, Space::new().height(s.metrics.gap * 0.5), body]
        .width(Length::Fill)
        .into()
}

/// The top padding that centres one line of `size` text in a box of
/// `height`. Iced lays a line out at roughly 1.3 times its size.
fn centre(height: f32, size: u16) -> f32 {
    ((height - size as f32 * 1.3) / 2.0).max(0.0)
}
