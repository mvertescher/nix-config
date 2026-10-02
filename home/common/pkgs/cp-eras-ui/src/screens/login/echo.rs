//! Source-reference lower echoes for login card notices.
//!
//! A separate canvas sits between Backdrop and Art, leaving the primary
//! note geometry unchanged. Each configured slot contributes a local tile;
//! the canvas geometry is rebuilt only when bounds or static style change.

use super::*;
use crate::palette::Palette;
use crate::style::NoteEcho;
use iced::widget::image;
use resvg::tiny_skia as ts;
use std::cell::RefCell;

pub(super) struct NoticeEcho {
    pub style: Style,
}

#[derive(Default)]
pub(super) struct EchoState {
    cache: canvas::Cache<Renderer>,
    key: RefCell<Option<EchoKey>>,
}

#[derive(Clone, Copy)]
struct EchoKey {
    palette: Palette,
    slots_ptr: usize,
    slots_len: usize,
}

impl<Message> canvas::Program<Message, Style> for NoticeEcho {
    type State = EchoState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Style,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // Check eligibility outside the cache. A custom palette must not
        // reveal a prior reference tile held for the same bounds.
        if !self.style.access_reference_palette()
            || !self.style.access.slots.iter().any(|slot| slot.reference_note_echo.is_some())
        {
            return Vec::new();
        }
        let slots = self.style.access.slots;
        let next = EchoKey {
            palette: self.style.palette,
            slots_ptr: slots.as_ptr() as usize,
            slots_len: slots.len(),
        };
        let mut key = state.key.borrow_mut();
        let stale = key.as_ref().is_none_or(|old|
            old.palette != next.palette || old.slots_ptr != next.slots_ptr || old.slots_len != next.slots_len);
        if stale {
            state.cache.clear();
            *key = Some(next);
        }
        drop(key);
        vec![state.cache.draw(renderer, bounds.size(), |frame| {
            for slot in slots {
                let Some(echo) = slot.reference_note_echo else { continue };
                if let Some(tile) = raster_tile(&self.style, bounds.size(), slot, echo) {
                    let handle = image::Handle::from_rgba(tile.width, tile.height, tile.rgba);
                    frame.draw_image(
                        Rectangle::new(
                            Point::new(tile.x, tile.y),
                            Size::new(tile.width as f32, tile.height as f32),
                        ),
                        canvas::Image::new(handle).filter_method(image::FilterMethod::Linear),
                    );
                }
            }
        })]
    }
}

struct Tile {
    x: f32,
    y: f32,
    width: u32,
    height: u32,
    rgba: Vec<u8>,
}

fn raster_tile(style: &Style, bounds: Size, slot: &Slot, echo: NoteEcho) -> Option<Tile> {
    if echo.reference_size <= 0.0 || echo.opacity <= 0.0 { return None; }
    let grid = Grid::new(bounds);
    let notes: Vec<_> = bounded_notes(grid, style, slot).collect();
    if notes.is_empty() { return None; }

    // The measured treatment is one plain, horizontal, same-size block.
    // A different-size line needs its own blur radius and source fit.
    if notes.iter().any(|note| note.turned || note.centred
        || note.tracking != 0.0 || note.stretch != 1.0
        || (note.size - notes[0].size).abs() > 1e-4) {
        return None;
    }

    let mut left = f32::INFINITY;
    let mut top = f32::INFINITY;
    let mut right = f32::NEG_INFINITY;
    let mut bottom = f32::NEG_INFINITY;
    for note in &notes {
        let size = grid.span(note.size);
        let ratio = size / echo.reference_size;
        let anchor = grid.at(note.x, note.baseline);
        let dx = echo.offset.0 * ratio;
        let dy = echo.offset.1 * ratio;
        let sigma = echo.blur * ratio;
        let pad = (3.0 * sigma).ceil() + echo.stroke * ratio + 3.0;
        left = left.min(anchor.x + dx - pad);
        top = top.min(anchor.y - size * ASCENT + dy - pad);
        right = right.max(anchor.x + dx + note_extent(grid, note) * grid.sx + pad);
        bottom = bottom.max(anchor.y + dy + size * (LINE - ASCENT) + pad);
    }
    let x = left.floor().max(0.0);
    let y = top.floor().max(0.0);
    let width = (right.ceil() - x).max(0.0) as u32;
    let height = (bottom.ceil() - y).max(0.0) as u32;
    // A notice is a local strip. Reject dimensions that would allocate
    // a full-screen image if a future table accidentally supplies them.
    if width == 0 || height == 0 || width > 4096 || height > 512 { return None; }
    let mut pixmap = ts::Pixmap::new(width, height)?;
    let mut white = ts::Paint::default();
    white.set_color_rgba8(255, 255, 255, 255);

    for note in &notes {
        let size = grid.span(note.size);
        let ratio = size / echo.reference_size;
        let anchor = grid.at(note.x, note.baseline);
        // Keep the fractional Grid anchor. Snapping to whole pixels here
        // changes both the primary/echo phase and the fitted letter shape.
        let text = canvas::Text {
            content: note.text.to_string(),
            position: Point::new(
                anchor.x + echo.offset.0 * ratio - x,
                anchor.y + echo.offset.1 * ratio - size * ASCENT - y,
            ),
            color: Color::WHITE,
            size: size.into(),
            line_height: iced::widget::text::LineHeight::Absolute((size * LINE).into()),
            font: font_of(note),
            align_x: iced::advanced::text::Alignment::Left,
            align_y: iced::alignment::Vertical::Top,
            ..Default::default()
        };
        let stroke = ts::Stroke { width: echo.stroke * ratio, ..ts::Stroke::default() };
        text.draw_with(|path, _| {
            if let Some(path) = skia_path(&path) {
                // Opaque white stroke+fill form one mask. The group opacity
                // is applied once after the common Gaussian blur below.
                if stroke.width > 0.0 {
                    pixmap.stroke_path(&path, &white, &stroke, ts::Transform::identity(), None);
                }
                pixmap.fill_path(&path, &white, ts::FillRule::Winding, ts::Transform::identity(), None);
            }
        });
    }

    // Alpha only: the ink stays at the era's configured reference color.
    // A 3-sigma radius is fully covered by the padded local tile.
    let sigma = echo.blur * grid.span(notes[0].size) / echo.reference_size;
    let mask: Vec<f32> = pixmap.data().chunks_exact(4).map(|p| p[3] as f32 / 255.0).collect();
    let blurred = gaussian_alpha(&mask, width as usize, height as usize, sigma);
    let ink = echo.ink;
    let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
    for coverage in blurred {
        let alpha = (coverage * echo.opacity).clamp(0.0, 1.0);
        // The image layer is blended by the GPU in linear light. Match
        // the source's sRGB opacity approximately against palette.bg;
        // the photographed backdrop varies spatially, so this remains
        // an approximation even where the source palette is active.
        let adjusted = crate::screens::scene::blend_over(Color { a: alpha, ..ink }, style.palette.bg);
        rgba.extend_from_slice(&[
            (adjusted.r * 255.0).round() as u8,
            (adjusted.g * 255.0).round() as u8,
            (adjusted.b * 255.0).round() as u8,
            (adjusted.a * 255.0).round() as u8,
        ]);
    }
    Some(Tile { x, y, width, height, rgba })
}

fn gaussian_alpha(input: &[f32], width: usize, height: usize, sigma: f32) -> Vec<f32> {
    if sigma <= 0.01 { return input.to_vec(); }
    let radius = (3.0 * sigma).ceil() as isize;
    let mut kernel = Vec::with_capacity((2 * radius + 1) as usize);
    for offset in -radius..=radius {
        kernel.push((-(offset as f32).powi(2) / (2.0 * sigma * sigma)).exp());
    }
    let sum: f32 = kernel.iter().sum();
    for weight in &mut kernel { *weight /= sum; }
    let mut horizontal = vec![0.0; input.len()];
    let mut output = vec![0.0; input.len()];
    for y in 0..height {
        for x in 0..width {
            let mut value = 0.0;
            for offset in -radius..=radius {
                let sx = x as isize + offset;
                if sx >= 0 && sx < width as isize {
                    value += input[y * width + sx as usize] * kernel[(offset + radius) as usize];
                }
            }
            horizontal[y * width + x] = value;
        }
    }
    for y in 0..height {
        for x in 0..width {
            let mut value = 0.0;
            for offset in -radius..=radius {
                let sy = y as isize + offset;
                if sy >= 0 && sy < height as isize {
                    value += horizontal[sy as usize * width + x] * kernel[(offset + radius) as usize];
                }
            }
            output[y * width + x] = value.clamp(0.0, 1.0);
        }
    }
    output
}

fn skia_path(path: &canvas::Path) -> Option<ts::Path> {
    use canvas::path::lyon_path;
    let mut builder = ts::PathBuilder::new();
    let mut last = lyon_path::math::Point::default();
    for event in path.raw() {
        match event {
            lyon_path::Event::Begin { at } => { builder.move_to(at.x, at.y); last = at; }
            lyon_path::Event::Line { from, to } => {
                if last != from { builder.move_to(from.x, from.y); }
                builder.line_to(to.x, to.y); last = to;
            }
            lyon_path::Event::Quadratic { from, ctrl, to } => {
                if last != from { builder.move_to(from.x, from.y); }
                builder.quad_to(ctrl.x, ctrl.y, to.x, to.y); last = to;
            }
            lyon_path::Event::Cubic { from, ctrl1, ctrl2, to } => {
                if last != from { builder.move_to(from.x, from.y); }
                builder.cubic_to(ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, to.x, to.y); last = to;
            }
            lyon_path::Event::End { close, .. } => { if close { builder.close(); } }
        }
    }
    builder.finish()
}
