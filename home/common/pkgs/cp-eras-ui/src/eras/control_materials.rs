//! Component-sheet material adaptations for native form controls.
use crate::style::{Era, Frame, Ink, MailRowEcho, Style};
use crate::palette::rgb;
use crate::widgets::surface::{Corners, Cut, Fill, Surface, SurfaceFace};
use crate::widgets::controls::{Kind, State};

#[derive(Clone, Copy)]
pub(crate) enum Shape { Surface, Step { shoulder: f32, run: f32, rise: f32 } }
#[derive(Clone, Copy)]
pub(crate) struct Material {
    pub face: SurfaceFace,
    pub ink: iced::Color,
    pub shape: Shape,
    pub tab: Option<(iced::Color, bool)>,
}
impl Material {
    pub fn outset(&self) -> [f32; 4] {
        let mut out = self.face.outset();
        if self.tab.is_some_and(|(_, held)| held) { out[3] = out[3].max(1.0); }
        out
    }
}
pub(crate) fn material(style: &Style, kind: Kind, state: State) -> Option<Material> {
    if !matches!(style.era, Era::Kitsch | Era::Neokitsch) { return None; }
    let coat = match (kind, state) {
        (_, State::Disabled) => style.controls.disabled,
        (Kind::Primary, _) => style.controls.primary,
        (Kind::Ghost, _) => style.controls.ghost,
        (Kind::Field, _) => style.controls.field,
    };
    let corners = match (style.era, kind) {
        (Era::Kitsch, _) => Corners::all(Cut::Round { radius: 2.0 }),
        (Era::Neokitsch, Kind::Field) => Corners::square(),
        _ => Corners::all(Cut::Round { radius: 5.0 })
            .with_bottom_left(Cut::Chamfer { x: 13.0, y: 10.0 }),
    };
    let mut surface = Surface::outlined(style);
    surface.corners = corners;
    surface.fill = style.ink(coat.fill).map(Fill::Solid).unwrap_or(Fill::None);
    surface.stroke = (coat.weight > 0.0).then_some(coat.edge.of(&style.palette));
    surface.stroke_width = coat.weight;
    let mut ink = coat.ink.of(&style.palette);
    if state == State::Pressed && kind != Kind::Field {
        if style.era == Era::Kitsch {
            surface.fill = Fill::Solid(rgb(0xffbe18));
            surface.stroke = None;
            ink = rgb(0x5a3a08);
        } else {
            surface.fill = Surface::selected(style).fill;
            surface.stroke = None;
            ink = rgb(0x3a2010);
        }
    }
    let echo = if state == State::Hovered {
        if style.era == Era::Kitsch {
            if kind == Kind::Ghost {
                surface.fill = Fill::Solid(rgb(0x2c9798));
                surface.stroke = Some(rgb(0xa9e6df));
                surface.stroke_width = 1.8;
                ink = rgb(0x123c38);
            }
            Some(MailRowEcho { rings: 1, step: Frame::new(20.0, -20.0, 0.0, 0.0),
                fill: Some(Ink::Fixed(rgb(0x0f9f80))), fill_alpha: 0.58,
                ink: Ink::Fixed(rgb(0x6cc4bd)), width: 1.2, alpha: 0.80, fade: 0.0 })
        } else {
            Some(MailRowEcho { rings: 7, step: Frame::new(-0.6, -2.1, 2.2, 2.4),
                fill: None, fill_alpha: 0.0, ink: Ink::Fixed(rgb(0xa97c48)),
                width: 0.7, alpha: 0.85, fade: 0.05 })
        }
    } else { None };
    Some(Material { face: SurfaceFace { surface, echo }, ink,
        shape: if style.era == Era::Kitsch && kind != Kind::Field {
            Shape::Step { shoulder: 161.0 / 334.5, run: 12.0, rise: 7.0 }
        } else { Shape::Surface },
        tab: (style.era == Era::Neokitsch && kind == Kind::Ghost)
            .then_some((if state == State::Disabled { ink } else if state == State::Pressed {
                rgb(0xf7cc8c)
            } else { rgb(0xfdcf8c) }, state == State::Pressed)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn feedback_keeps_geometry_and_focused_fields_flat() {
        for era in [Era::Kitsch, Era::Neokitsch] {
            let style = era.style();
            for kind in [Kind::Primary, Kind::Ghost, Kind::Field] {
                let rest = material(&style, kind, State::Active).unwrap().face;
                let hover = material(&style, kind, State::Hovered).unwrap().face;
                let held = material(&style, kind, State::Pressed).unwrap().face;
                assert_eq!(rest.surface.corners, hover.surface.corners);
                assert_eq!(rest.surface.corners, held.surface.corners);
                assert_eq!(hover.echo.unwrap().rings, if era == Era::Kitsch { 1 } else { 7 });
                assert!(held.echo.is_none());
                assert!(material(&style, kind, State::Disabled).unwrap().face.echo.is_none());
                if kind == Kind::Field { assert_eq!(rest.surface.fill, held.surface.fill); }
                else if era == Era::Neokitsch { assert!(matches!(held.surface.fill, Fill::Veneer { .. })); }
            }
        }
    }
}
