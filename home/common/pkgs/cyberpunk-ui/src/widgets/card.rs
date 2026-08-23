//! The product card, in both states.
//!
//! This is the widget the whole abstraction was designed against. In the
//! references all four eras show the same 4ST store card -- name, class,
//! silhouette, four stats, three empty sockets -- and the selected one
//! grows a detail block. What differs is the corner treatment, the fill
//! that means "selected", whether the name sits at the head or the foot,
//! and whether the stats get a highlight band. All four are parameters.

use super::surface::{surface, Surface};
use super::text;
use crate::style::{Nameplate, Style};
use iced::widget::{column, container, row, Space};
use iced::{Element, Length, Padding};

/// One weapon, as the store shows it.
pub struct Product<'a> {
    pub name: &'a str,
    pub class: &'a str,
    pub brand: &'a str,
    pub stats: [(&'a str, &'a str); 4],
    pub detail: &'a [(&'a str, &'a str)],
    pub bonus: &'a [&'a str],
    pub sockets: usize,
}

impl<'a> Product<'a> {
    pub fn magnum() -> Self {
        Product {
            name: "MAGNUM 650",
            class: "HAND GUN",
            brand: "PETROCHEM · BETTERLIFE TEC",
            stats: [("DPS", "86"), ("PNT", "30"), ("ACC", "5"), ("ROF", "5")],
            detail: &[("20", "Recoil"), ("22", "Spread"), ("12", "Range")],
            bonus: &["+9 Reflexes", "+2 Modules Slots"],
            sockets: 3,
        }
    }
}

fn nameplate<'a, Message: 'static>(
    style: &Style,
    product: &Product<'a>,
    selected: bool,
) -> Element<'a, Message> {
    let (name, class) = if selected {
        (
            text::on_select(style, product.name),
            text::on_select(style, product.class).size(style.metrics.text_caption + 2),
        )
    } else {
        (
            text::title(style, product.name),
            text::label(style, product.class).size(style.metrics.text_caption + 2),
        )
    };
    column![name, class].spacing(2).into()
}

fn stats_row<'a, Message: 'static>(
    style: &Style,
    product: &Product<'a>,
    selected: bool,
) -> Element<'a, Message> {
    let mut heads = row![].spacing(0);
    let mut values = row![].spacing(0);

    // On an unselected card the figures may sit on the emphasis band,
    // which carries its own ink.
    let figure_ink = match (style.palette.emphasis, selected) {
        (Some((_, ink)), false) => ink,
        (_, true) => style.palette.on_select,
        _ => style.palette.fg,
    };

    for (head, value) in product.stats {
        let h = if selected {
            text::on_select(style, head).size(style.metrics.text_caption + 3)
        } else {
            text::label(style, head).size(style.metrics.text_caption + 3)
        };
        let v = text::body(style, value)
            .size(style.metrics.text_title - 2)
            .color(figure_ink);
        heads = heads.push(container(h).center_x(Length::Fill));
        values = values.push(container(v).center_x(Length::Fill));
    }

    // Kitsch bands its stat row in mint; the other eras leave it bare.
    // Expressed as an optional palette slot so an era opts in rather
    // than the widget testing which era it is in.
    let values: Element<'a, Message> = match (style.palette.emphasis, selected) {
        (Some((band, _)), false) => container(surface(
            Surface::filled(style, band).no_stroke(),
            Padding::from([2, 0]),
            values,
        ))
        .height(Length::Fixed(26.0))
        .into(),
        _ => container(values).padding(Padding::from([2, 0])).into(),
    };

    column![heads, values].spacing(2).into()
}

fn sockets_row<'a, Message: 'static>(
    style: &Style,
    product: &Product<'a>,
    selected: bool,
) -> Element<'a, Message> {
    let mut r = row![].spacing(8);
    for _ in 0..product.sockets {
        // Outlined in both states; on a selected card the outline and
        // its label simply switch to the on-select ink.
        let cell = if selected {
            Surface::outlined(style).stroke(style.palette.on_select)
        } else {
            Surface::outlined(style)
        };
        let label = if selected {
            text::caption(style, "EMPTY SOCKET").color(style.palette.on_select)
        } else {
            text::caption(style, "EMPTY SOCKET")
        };
        r = r.push(
            container(surface(cell, Padding::from([4, 6]), label))
                .width(Length::Fill)
                .height(Length::Fixed(26.0)),
        );
    }
    r.into()
}

/// A product card. `selected` swaps the fill for the era's selection
/// idiom and grows the detail block, exactly as the references do.
pub fn product_card<'a, Message: 'static>(
    style: &Style,
    product: &Product<'a>,
    selected: bool,
) -> Element<'a, Message> {
    let bg = if selected {
        Surface::selected(style)
    } else {
        Surface::outlined(style)
    };

    let brand = if selected {
        text::on_select(style, product.brand).size(style.metrics.text_caption)
    } else {
        text::caption(style, product.brand)
    };

    let mut body = column![].spacing(10);

    if style.nameplate == Nameplate::Header {
        body = body.push(nameplate(style, product, selected));
    }
    body = body.push(brand);
    body = body.push(Space::new(0.0, 12.0));
    body = body.push(stats_row(style, product, selected));
    body = body.push(sockets_row(style, product, selected));

    if selected {
        let mut detail = column![].spacing(3);
        for (value, name) in product.detail {
            detail = detail.push(
                row![
                    container(text::on_select(style, *value)).width(Length::Fixed(34.0)),
                    text::on_select(style, *name),
                ]
                .spacing(4),
            );
        }
        detail = detail.push(Space::new(0.0, 6.0));
        detail = detail.push(text::on_select(style, "Bonus"));
        for line in product.bonus {
            detail = detail.push(text::on_select(style, *line));
        }
        body = body.push(Space::new(0.0, 6.0));
        body = body.push(detail);
    }

    if style.nameplate == Nameplate::Footer {
        body = body.push(Space::new(0.0, 10.0));
        body = body.push(nameplate(style, product, selected));
    }

    let height = if selected {
        style.metrics.card_selected
    } else {
        style.metrics.card
    };

    container(surface(bg, style.metrics.pad, body))
        .width(Length::Fill)
        .height(Length::Fixed(height))
        .into()
}
