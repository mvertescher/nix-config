//! The parts of a screen header, as canvas painters.
//!
//! The Neomil desktop design (`home/themes/neomil/design`) builds its bar
//! from the header the era's recreations open with: a caption-over-value
//! badge, a tab plate holding a line of text, a full-width rule printed
//! with a faint echo, and a barcode strip. Each is one function here, and
//! every figure it draws comes from its table in [`crate::style`]
//! ([`Badge`], [`TabPlate`], [`EchoRule`], [`Barcode`], [`HeaderGlow`]),
//! so a screen other than the bar sets the same parts by naming the era's
//! table rather than restating it. Nothing in this file knows which era
//! it is drawing.
//!
//! Painters rather than widgets because the era's screens are canvas
//! scenes: a part is placed at design coordinates on a [`Frame`], the way
//! the traces place it. Coordinates and sizes are design px (dp); the
//! window's scale factor (`Metrics::scale`) takes them to native pixels.
//!
//! Text is set as the design's SVG sets it: positioned on its baseline,
//! anchored, and tracked by adding `track` ems after every glyph (the
//! last included, which is what a browser does and what moves a centred
//! run half a track to the left).
//!
//! Translucency is printed, not blended. The design's opacities (a
//! caption at .75, the plate's face at .7, the rule's echo at .16) are
//! mixed by a browser in sRGB, and this renderer blends in linear light,
//! where the same alpha over a dark ground lands two to three times as
//! bright (the echo measured R 112 against the design's 47). So each
//! painter mixes a translucent ink with what is under it itself -- the
//! badge's own face, or the ground the caller describes with a
//! [`Backdrop`] -- and draws the result opaque.

use crate::style::{Badge, BadgeCoat, Barcode, EchoRule, Face, HeaderGlow, Style, TabPlate, Type};
use iced::widget::canvas::{self, Frame, Path, Stroke};
use iced::widget::image;
use iced::{Color, Point, Rectangle, Size};

/// Where a run of text sits against its x: SVG's `text-anchor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    Start,
    Middle,
    End,
}

/// The loaded font a [`Face`] names.
pub fn font(face: Face) -> iced::Font {
    use crate::fonts::*;
    match face {
        Face::Regular => FONT_RAJDHANI_REGULAR,
        Face::Medium => FONT_RAJDHANI_MEDIUM,
        Face::SemiBold => FONT_RAJDHANI_SEMIBOLD,
        Face::Bold => FONT_RAJDHANI_BOLD,
        Face::OrbitronBold => FONT_ORBITRON_BOLD,
        Face::OrbitronSemiBold => FONT_ORBITRON_SEMIBOLD,
        Face::FreeSansBold => FONT_FREE_SANS_BOLD,
        Face::CpErasKitschSansBold => FONT_CP_ERAS_KITSCH_SANS_BOLD,
        Face::Michroma => FONT_MICHROMA,
        Face::KronaOne => FONT_KRONA_ONE,
    }
}

/// How far below the top of a canvas text box its baseline sits, as a
/// fraction of the size: the canvas centres the face's hhea box in a
/// 1.2 line, so the baseline is `0.6 + (ascent - descent) / 2`
/// (Rajdhani 930 / -346, Orbitron 1011 / -243, both per 1000).
fn baseline(face: Face) -> f32 {
    match face {
        Face::OrbitronBold | Face::OrbitronSemiBold => 0.6 + (1.011 - 0.243) / 2.0,
        Face::Michroma => crate::fonts::michroma_baseline_in(1.2),
        Face::KronaOne => crate::fonts::krona_one_baseline_in(1.2),
        Face::FreeSansBold | Face::CpErasKitschSansBold => 0.6 + (0.9 - 0.2) / 2.0,
        _ => 0.6 + (0.930 - 0.346) / 2.0,
    }
}

/// What lies under a part at a point: the colour a translucent ink is
/// mixed with. The bar's is its glow ([`glow_at`]).
pub type Backdrop<'a> = &'a dyn Fn(f32, f32) -> Color;

/// `ink` at `alpha` over `under`, mixed as encoded sRGB the way the
/// design's browser composites it, and opaque.
pub fn over(ink: Color, alpha: f32, under: Color) -> Color {
    let a = (ink.a * alpha).clamp(0.0, 1.0);
    let mix = |f: f32, b: f32| f * a + b * (1.0 - a);
    Color { r: mix(ink.r, under.r), g: mix(ink.g, under.g), b: mix(ink.b, under.b), a: 1.0 }
}

/// The text a [`Type`] sets: in capitals when it says so.
pub fn cased(t: &Type, text: &str) -> String {
    if t.upper {
        text.to_uppercase()
    } else {
        text.to_string()
    }
}

/// Each glyph's advance in `t`, untracked: prefix widths of the shaped
/// run, as the scenes measure their tracked runs.
fn advances(t: &Type, text: &str) -> Vec<f32> {
    crate::screens::scene::advances(text, t.size, font(t.face))
}

/// The width a run takes in `t`, tracking included.
pub fn measure(t: &Type, text: &str) -> f32 {
    if text.is_empty() {
        return 0.0;
    }
    let n = text.chars().count() as f32;
    crate::screens::scene::run_width(text, t.size, font(t.face)) + n * t.track * t.size
}

/// `text` cut to fit `width` in `t`, with an ellipsis where it was cut;
/// empty when not even the ellipsis fits.
pub fn fit(t: &Type, text: &str, width: f32) -> String {
    if measure(t, text) <= width {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    // Longest prefix that fits with the ellipsis, by bisection: a window
    // title can be long and each probe is a shaping pass.
    let (mut lo, mut hi) = (0usize, chars.len());
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        let probe: String = chars[..mid].iter().collect::<String>().trim_end().to_string() + "…";
        if measure(t, &probe) <= width {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    if lo == 0 {
        return if measure(t, "…") <= width { "…".into() } else { String::new() };
    }
    chars[..lo].iter().collect::<String>().trim_end().to_string() + "…"
}

/// Set `text` in `t` with its baseline at `y`, anchored at `x`.
pub fn set(frame: &mut Frame, t: &Type, text: &str, x: f32, y: f32, anchor: Anchor, color: Color) {
    if text.is_empty() || color.a <= 0.0 {
        return;
    }
    let width = measure(t, text);
    let left = match anchor {
        Anchor::Start => x,
        Anchor::Middle => x - width / 2.0,
        Anchor::End => x - width,
    };
    let top = y - t.size * baseline(t.face);
    let glyph = |frame: &mut Frame, content: String, at: f32| {
        frame.fill_text(canvas::Text {
            content,
            position: Point::new(at, top),
            color,
            size: t.size.into(),
            font: font(t.face),
            align_x: iced::advanced::text::Alignment::Left,
            align_y: iced::alignment::Vertical::Top,
            shaping: iced::advanced::text::Shaping::Advanced,
            ..canvas::Text::default()
        });
    };
    if t.track == 0.0 {
        glyph(frame, text.to_string(), left);
        return;
    }
    let mut pen = left;
    for (c, advance) in text.chars().zip(advances(t, text)) {
        if c != ' ' {
            glyph(frame, c.to_string(), pen);
        }
        pen += advance + t.track * t.size;
    }
}

/// What a badge shows under its caption.
#[derive(Debug, Clone)]
pub enum Value {
    /// A figure, in the badge's `value` type: a reading, a workspace.
    Text(String),
    /// A name, in its smaller `word` type.
    Word(String),
    /// A decoded pixmap (a tray icon), drawn at the badge's icon size.
    Icon(image::Handle),
}

/// Which of a badge's coats it wears.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wear {
    Rest,
    On,
    Alert,
    /// A workspace with no windows.
    Empty,
    /// Under the pointer, on a badge that answers it.
    Lit,
}

impl Wear {
    /// The coat this one wears while the pointer is over its badge: the
    /// era's hover rung for a resting or empty badge. The selected badge
    /// and one in alarm keep theirs -- the first is where a click would
    /// take you already, and the second outranks a hover.
    pub fn hovered(self) -> Wear {
        match self {
            Wear::Rest | Wear::Empty | Wear::Lit => Wear::Lit,
            Wear::On | Wear::Alert => self,
        }
    }
}

fn coat(b: &Badge, wear: Wear) -> &BadgeCoat {
    match wear {
        Wear::Rest => &b.rest,
        Wear::On => &b.on,
        Wear::Alert => &b.alert,
        Wear::Empty => &b.empty,
        Wear::Lit => &b.lit,
    }
}

/// A badge's width for this caption and value: the table's, or wider
/// where the text needs it.
pub fn badge_width(b: &Badge, caption: &str, value: &Value) -> f32 {
    let caption = measure(&b.caption, &cased(&b.caption, caption));
    let value = match value {
        Value::Text(text) => measure(&b.value, &cased(&b.value, text)),
        Value::Word(text) => measure(&b.word, &cased(&b.word, text)),
        Value::Icon(_) => b.icon,
    };
    b.width.max(caption + 2.0 * b.pad).max(value + 2.0 * b.pad).ceil()
}

/// A rectangle with its bottom-left corner cut, as the design's
/// `plate(x, y, w, h, { bl })`.
pub fn plate_path(x: f32, y: f32, w: f32, h: f32, bl: f32) -> Path {
    let bl = bl.min(w).min(h).max(0.0);
    Path::new(|p| {
        p.move_to(Point::new(x, y));
        p.line_to(Point::new(x + w, y));
        p.line_to(Point::new(x + w, y + h));
        p.line_to(Point::new(x + bl, y + h));
        p.line_to(Point::new(x, y + h - bl));
        p.close();
    })
}

/// One badge with its top-left at `(x, y)`, `w` wide (see
/// [`badge_width`]), outlined at `line` -- centred on the plate's edge,
/// as an SVG stroke is.
#[allow(clippy::too_many_arguments)]
pub fn badge(
    frame: &mut Frame,
    style: &Style,
    b: &Badge,
    line: f32,
    x: f32,
    y: f32,
    w: f32,
    caption: &str,
    value: &Value,
    wear: Wear,
) {
    let c = coat(b, wear);
    let ink = c.ink.of(&style.palette);
    let path = plate_path(x, y, w, b.height, b.chamfer);
    let face = style.ink(c.face);
    if let Some(face) = face {
        frame.fill(&path, face);
    }
    // Printed over the badge's own face; over nothing only on a badge
    // with no face, where the renderer's blend is all there is.
    let caption_ink = match face {
        Some(face) => over(ink, c.caption_alpha, face),
        None => Color { a: ink.a * c.caption_alpha, ..ink },
    };
    if let Some(edge) = style.ink(c.edge) {
        frame.stroke(&path, Stroke::default().with_color(edge).with_width(line));
    }
    let mid = x + w / 2.0;
    set(
        frame,
        &b.caption,
        &cased(&b.caption, caption),
        mid,
        y + b.caption_baseline,
        Anchor::Middle,
        caption_ink,
    );
    match value {
        Value::Text(text) => set(
            frame,
            &b.value,
            &cased(&b.value, text),
            mid,
            y + b.value_baseline,
            Anchor::Middle,
            ink,
        ),
        Value::Word(text) => set(
            frame,
            &b.word,
            &cased(&b.word, text),
            mid,
            y + b.word_baseline,
            Anchor::Middle,
            ink,
        ),
        Value::Icon(handle) => frame.draw_image(
            Rectangle {
                x: mid - b.icon / 2.0,
                y: y + b.icon_centre - b.icon / 2.0,
                width: b.icon,
                height: b.icon,
            },
            canvas::Image::new(handle.clone()).filter_method(image::FilterMethod::Linear),
        ),
    }
}

/// The width of the strips a translucent face or line over a varying
/// backdrop is printed in: the ground changes far less than one level
/// across one.
const STRIP: f32 = 8.0;

/// Fill `x..x + w` by `y..y + h` with `ink` at `alpha` over `under`, in
/// strips so a backdrop that varies across it is followed.
#[allow(clippy::too_many_arguments)]
fn printed(frame: &mut Frame, ink: Color, alpha: f32, under: Backdrop, x: f32, y: f32, w: f32, h: f32) {
    let n = (w / STRIP).ceil().max(1.0) as usize;
    let step = w / n as f32;
    for i in 0..n {
        let x0 = x + i as f32 * step;
        let color = over(ink, alpha, under(x0 + step / 2.0, y + h / 2.0));
        frame.fill(&Path::rectangle(Point::new(x0, y), Size::new(step, h)), color);
    }
}

/// A tab plate at `(x, y)`, `w` wide, holding `text` centred and cut to
/// fit, its translucent face printed over `under`.
#[allow(clippy::too_many_arguments)]
pub fn tab_plate(
    frame: &mut Frame,
    style: &Style,
    p: &TabPlate,
    line: f32,
    x: f32,
    y: f32,
    w: f32,
    text: &str,
    under: Backdrop,
) {
    if w <= 0.0 {
        return;
    }
    let path = Path::rectangle(Point::new(x, y), Size::new(w, p.height));
    if let Some(face) = style.ink(p.face) {
        printed(frame, face, p.face_alpha, under, x, y, w, p.height);
    }
    if let Some(edge) = style.ink(p.edge) {
        frame.stroke(&path, Stroke::default().with_color(edge).with_width(line));
    }
    let text = fit(&p.text, &cased(&p.text, text), w - 2.0 * p.pad);
    set(frame, &p.text, &text, x + w / 2.0, y + p.baseline, Anchor::Middle, p.ink.of(&style.palette));
}

/// A rule from `x0` to `x1` with its top edge at `y`, `weight` thick,
/// and its echo `echo_gap` below, printed over `under`.
#[allow(clippy::too_many_arguments)]
pub fn echo_rule(frame: &mut Frame, style: &Style, r: &EchoRule, weight: f32, x0: f32, x1: f32, y: f32, under: Backdrop) {
    let Some(ink) = style.ink(r.ink) else { return };
    let w = (x1 - x0).max(0.0);
    printed(frame, ink, r.echo_alpha, under, x0, y + r.echo_gap, w, weight);
    frame.fill(&Path::rectangle(Point::new(x0, y), Size::new(w, weight)), ink);
}

/// A barcode strip filling `w` by `h` at `(x, y)`.
pub fn barcode(frame: &mut Frame, style: &Style, c: &Barcode, x: f32, y: f32, w: f32, h: f32) {
    let Some(ink) = style.ink(c.ink) else { return };
    if c.pattern.is_empty() || h <= 0.0 {
        return;
    }
    let unit = h / 14.0;
    let (mut cx, mut i) = (x, 0usize);
    while cx < x + w {
        let bw = f32::from(c.pattern[i % c.pattern.len()]) * unit;
        if i % 2 == 0 {
            frame.fill(&Path::rectangle(Point::new(cx, y), Size::new(bw, h)), ink);
        }
        cx += bw;
        i += 1;
    }
}

/// The glow as straight (unpremultiplied) RGBA, `px` by `py` pixels
/// covering `width` by `height` dp of the top of a screen `width` wide.
///
/// Computed rather than drawn with a renderer gradient because the
/// design's browser blends its stops in sRGB, and so does this: the
/// stops are mixed as encoded bytes, which a linear-light gradient
/// shader would not do.
pub fn glow_pixels(g: &HeaderGlow, bg: Color, width: f32, height: f32, px: u32, py: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity((px * py * 4) as usize);
    for j in 0..py {
        let y = (j as f32 + 0.5) / py as f32 * height;
        for i in 0..px {
            let x = (i as f32 + 0.5) / px as f32 * width;
            let c = glow_at(g, bg, width, x, y);
            out.extend_from_slice(&[
                (c.r * 255.0).round() as u8,
                (c.g * 255.0).round() as u8,
                (c.b * 255.0).round() as u8,
                255,
            ]);
        }
    }
    out
}

/// The glow's colour at `(x, y)` dp on a screen `width` dp wide.
pub fn glow_at(g: &HeaderGlow, bg: Color, width: f32, x: f32, y: f32) -> Color {
    let box_h = (width * g.aspect * g.reach).max(1.0);
    let (u, v) = (x / width.max(1.0), y / box_h);
    let mix = |a: Color, b: Color, t: f32| over(b, t, a);
    let s = &g.stops;
    let t = (g.skew * u + v) / (g.skew * g.skew + 1.0);
    let base = if v > 1.0 {
        bg
    } else if t <= s[0].0 {
        s[0].1
    } else {
        let mut c = s[s.len() - 1].1;
        for k in 1..s.len() {
            if t <= s[k].0 {
                let span = (s[k].0 - s[k - 1].0).max(f32::EPSILON);
                c = mix(s[k - 1].1, s[k].1, (t - s[k - 1].0) / span);
                break;
            }
        }
        c
    };
    let ((la, lu), (ra, ru)) = (g.left, g.right);
    let edge = if u < lu {
        la * (1.0 - u / lu.max(f32::EPSILON))
    } else if u > ru {
        ra * ((u - ru) / (1.0 - ru).max(f32::EPSILON)).min(1.0)
    } else {
        0.0
    };
    mix(base, bg, edge)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rajdhani() -> Type {
        Type { face: Face::Medium, size: 7.5, track: 0.1, upper: true }
    }

    #[test]
    fn tracking_is_added_after_every_glyph() {
        crate::fonts::ensure_test_fonts();
        let t = rajdhani();
        let solid = Type { track: 0.0, ..t };
        let gap = measure(&t, "WORKSPACE") - measure(&solid, "WORKSPACE");
        assert!((gap - 9.0 * 0.75).abs() < 1e-3, "{gap}");
    }

    #[test]
    fn a_long_title_is_cut_with_an_ellipsis_and_fits() {
        crate::fonts::ensure_test_fonts();
        let t = rajdhani();
        let long = "KITTY  //  A VERY LONG WINDOW TITLE THAT WILL NOT FIT IN A TAB PLATE";
        let cut = fit(&t, long, 120.0);
        assert!(cut.ends_with('…'), "{cut}");
        assert!(measure(&t, &cut) <= 120.0);
        assert_eq!(fit(&t, "KITTY", 120.0), "KITTY");
        assert_eq!(fit(&t, "KITTY", 0.5), "");
    }

    #[test]
    fn a_badge_grows_only_for_text_that_outgrows_it() {
        crate::fonts::ensure_test_fonts();
        let b = crate::eras::neomil::style().bar.header.expect("neomil header").badge;
        assert_eq!(badge_width(&b, "LEVEL", &Value::Text("T3".into())), b.width);
        assert!(badge_width(&b, "WIFI", &Value::Text("AFTERLIFE".into())) > b.width);
    }

    #[test]
    fn the_glow_runs_from_its_first_stop_to_bg_past_its_reach() {
        let blue = crate::palette::rgb(0x131f52);
        let bg = crate::palette::rgb(0x050304);
        let g = HeaderGlow {
            stops: [(0.0, blue), (0.42, crate::palette::rgb(0x0a0a1c)), (0.7, bg)],
            skew: 0.25,
            reach: 0.62,
            aspect: 0.5625,
            left: (0.0, 0.12),
            right: (0.0, 0.9),
        };
        assert_eq!(glow_at(&g, bg, 1600.0, 0.0, 0.0), blue);
        // Past 0.7 along the gradient, and below its box, it is bg.
        assert_eq!(glow_at(&g, bg, 1600.0, 800.0, 500.0), bg);
        assert_eq!(glow_at(&g, bg, 1600.0, 800.0, 899.0), bg);
        // The pixels are the same function, sampled at pixel centres.
        let px = glow_pixels(&g, bg, 1600.0, 900.0, 1600, 1);
        let mid = glow_at(&g, bg, 1600.0, 800.5, 450.0);
        assert_eq!(px[800 * 4], (mid.r * 255.0).round() as u8);
    }
}
