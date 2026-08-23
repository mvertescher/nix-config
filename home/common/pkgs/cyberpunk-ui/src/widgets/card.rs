//! The product card, in both states.
//!
//! This is the widget the whole abstraction was designed against. In the
//! references all four eras show the same 4ST store card -- name, class,
//! silhouette, four stats, three empty sockets -- and the selected one
//! grows a detail block. What differs is the corner treatment, the fill
//! that means "selected", whether the name sits at the head or the foot,
//! whether the stats get a highlight band, and whether the card wears an
//! accent band at all. All five are parameters.

use super::banner::{band_height, banner, banner_colors, blank};
use super::glyph::{glyph, Glyph};
use super::silhouette::silhouette;
use super::surface::{layered, Surface};
use super::text;
use crate::style::{Nameplate, Style};
use iced::widget::{canvas, column, container, row, Space};
use iced::{Color, Element, Length, Padding};

/// How tall the product art is drawn. The three store targets set it at
/// 56, 67 and 56 in their own 1920 space against cards 350-470 tall; we
/// draw at 1600 into cards of much the same height, so this is the
/// middle of that band rather than a scaled figure.
const ART: f32 = 62.0;

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

/// The card's ink: line-work and text take the same colour, whichever
/// side of the selection fill they are on.
fn ink(style: &Style, selected: bool) -> Color {
    if selected {
        style.palette.on_select
    } else {
        style.palette.fg
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

/// The three compliance marks that head a banded card's accent band.
fn glyph_strip<'a, Message: 'static>(
    style: &Style,
    selected: bool,
    size: f32,
) -> Element<'a, Message> {
    let (_, mark) = banner_colors(style, selected);
    row![
        glyph(Glyph::Matrix, mark, 1.0, size),
        glyph(Glyph::Square, mark, 1.0, size),
        glyph(Glyph::Triangle, mark, 1.0, size),
    ]
    .spacing(5)
    .align_y(iced::Alignment::Center)
    .into()
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
        (Some((band, _)), false) => container(super::surface::surface(
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
    let mut r = row![].spacing(6).align_y(iced::Alignment::Center);

    // Kitsch leads the row with a dotted matrix block about as tall as
    // the row; the eras with no glyph vocabulary start straight in on
    // the sockets.
    if style.glyphs {
        r = r.push(glyph(Glyph::Matrix, ink(style, selected), 1.0, 24.0));
    }

    // A point below the card's other captions, as all three targets set
    // it: entropism 8 against 9, kitsch 9 against 10, neokitsch 8. The
    // label is the widest thing in the narrowest cell on the card and
    // the references already made this trade.
    let size = style.metrics.text_caption.saturating_sub(1);

    for _ in 0..product.sockets {
        // Outlined in both states; on a selected card the outline and
        // its label simply switch to the on-select ink.
        let cell = if selected {
            Surface::outlined(style).stroke(style.palette.on_select)
        } else {
            Surface::outlined(style)
        };
        let label = if selected {
            text::caption(style, "EMPTY SOCKET")
                .size(size)
                .color(style.palette.on_select)
        } else {
            text::caption(style, "EMPTY SOCKET").size(size)
        };
        r = r.push(
            container(super::surface::surface(
                cell,
                Padding::from([4, 4]),
                label,
            ))
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

    let pad = style.metrics.pad;
    // The accent band runs wider than the shape behind it, so the card
    // is built the other way up from the usual `backdrop`: content at
    // full width, background inset by the overhang. Every row but the
    // band then pays that inset back on its leading edge.
    let overhang = if style.banded() {
        style.banner.overhang
    } else {
        0.0
    };
    let inset = Padding {
        top: 0.0,
        right: pad,
        bottom: 0.0,
        left: pad + overhang,
    };
    let inner = |el: Element<'a, Message>| -> Element<'a, Message> {
        container(el).padding(inset).width(Length::Fill).into()
    };

    let brand: Element<'a, Message> = if selected {
        text::on_select(style, product.brand)
            .size(style.metrics.text_caption)
            .into()
    } else {
        text::caption(style, product.brand).into()
    };

    let mut body = column![].spacing(10);

    if style.nameplate == Nameplate::Header {
        body = body.push(inner(nameplate(style, product, selected)));
        if style.banded() {
            // Kitsch's shelf band: marks at the head, brand tag at the
            // foot, in the band's own ink.
            let h = band_height(style.metrics.text_caption);
            body = body.push(banner(
                style,
                selected,
                h,
                glyph_strip(style, selected, h * 0.5),
                super::banner::tag(style, selected, product.brand, style.metrics.text_caption),
            ));
        } else {
            body = body.push(inner(brand));
        }
    } else {
        body = body.push(inner(brand));
    }

    body = body.push(inner(silhouette(
        ink(style, selected),
        style.metrics.stroke + 1.0,
        ART,
    )));
    body = body.push(inner(stats_row(style, product, selected)));

    if selected {
        // Detail sits between the stats and the sockets in all three
        // store targets, not after them.
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
        body = body.push(inner(detail.into()));
    }

    body = body.push(inner(sockets_row(style, product, selected)));

    if style.nameplate == Nameplate::Footer {
        let plate: Element<'a, Message> = if style.banded() {
            // Neokitsch's nameplate *is* its banner: name and class run
            // together along the band rather than stacking.
            let h = band_height(style.metrics.text_body);
            let (_, mark) = banner_colors(style, selected);
            banner(
                style,
                selected,
                h,
                row![
                    text::body(style, product.name).color(mark),
                    text::body(style, product.class)
                        .size(style.metrics.text_caption + 2)
                        .color(mark),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center)
                .into(),
                blank(),
            )
        } else {
            inner(nameplate(style, product, selected))
        };
        body = body.push(plate);
    }

    // Sized by the body rather than by the shelf. A surface's canvas
    // fills whatever space it is handed, so a card built with `surface`
    // took the height of the row it sat in -- a fixed `metrics.card` was
    // the only thing keeping it off the bottom of the window, and it
    // bought that with a dead gap under the content. `layered` lays the
    // body out first and fits the shape to it, so the selected card is
    // taller for the reason the references say it is: it carries the
    // detail block.
    let shape: Element<'a, Message> = {
        let face = canvas(bg).width(Length::Fill).height(Length::Fill);
        if overhang > 0.0 {
            container(face)
                .padding(Padding {
                    left: overhang,
                    ..Padding::ZERO
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else {
            face.into()
        }
    };

    container(layered(
        shape,
        container(body)
            .padding(Padding {
                top: pad,
                right: 0.0,
                // A footer band is the card's last edge in the
                // reference -- `rect y=694 h=26` against a card ending
                // at 724 -- so it keeps a hairline rather than a full
                // pad under it.
                bottom: if style.nameplate == Nameplate::Footer && style.banded() {
                    4.0
                } else {
                    pad
                },
                left: 0.0,
            })
            .width(Length::Fill),
    ))
    .width(Length::Fill)
    .into()
}
