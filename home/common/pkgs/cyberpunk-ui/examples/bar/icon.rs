//! Turning what a StatusNotifierItem says about its icon into pixels.
//!
//! An item describes its icon in up to five ways and a host has to
//! reconcile them:
//!
//!   * `IconName` -- a name to look up in the icon theme, the way every
//!     other desktop icon is found.
//!   * `IconThemePath` -- a directory the item ships its own icons in,
//!     for applications that are not installed into a theme at all.
//!     Searched ahead of the system themes, both as a flat directory
//!     (`<path>/<name>.png`, which is what libappindicator does) and as
//!     a theme root.
//!   * `IconPixmap` -- the pixels inline, ARGB32, at one or more sizes.
//!     What Qt sends, and the only thing some applications ever send.
//!   * `AttentionIconName` / `AttentionIconPixmap` -- the same pair
//!     again, for an item whose `Status` is `NeedsAttention`.
//!   * `OverlayIconName` / `OverlayIconPixmap` -- a badge composited
//!     over the corner of whichever of the above won.
//!
//! All of it resolves to the same thing, so it all resolves *here*, and
//! what leaves this module is one `iced` image handle. `bar()` never
//! learns that any of the above exists.
//!
//! Nothing in here decodes on the drawing thread. [`Icons::resolve`] is
//! called from the tray thread, which is already the thread allowed to
//! block, and its results are memoised -- an icon-theme miss costs a
//! directory walk and it would be a poor trade to repeat it every time
//! an item changes its tooltip.
//!
//! ## Colour conventions, which are easy to get wrong
//!
//! * The protocol's ARGB32 is *straight* (non-premultiplied) and
//!   big-endian per byte: `A R G B`.
//! * `png` decodes to straight RGBA.
//! * `tiny_skia`, and so everything resvg renders, is *premultiplied*.
//! * `iced` blends with `SrcAlpha, OneMinusSrcAlpha`, so what it wants
//!   is straight RGBA.
//!
//! So the one internal currency is straight RGBA ([`Rgba`]), resvg's
//! output is demultiplied on the way out, and [`Rgba::over`] premultiplies
//! for the duration of the composite and no longer.

use iced::widget::image;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Icon file extensions, in the order they are preferred at equal size
/// distance. SVG first because it is the one that will not be resampled
/// twice, and because on a desktop carrying Papirus it is the only one
/// there is.
///
/// All three are decoded; a search path that finds a file it cannot
/// read is worse than one that never looks, because the caller cannot
/// tell "this item has no icon" from "this item has an icon in a format
/// I declined". XPM earns its place in [`load`], not here.
const EXTENSIONS: [&str; 3] = ["svg", "png", "xpm"];

/// The theme every icon theme is required to fall back to, and the only
/// one that is always installed.
const FALLBACK_THEME: &str = "hicolor";

/// The freedesktop default when nothing names a theme. Frequently not
/// installed -- on a desktop that has never set `gtk-icon-theme-name`
/// this simply misses and the search moves on to `hicolor`.
const DEFAULT_THEME: &str = "Adwaita";

/// Refuse to rasterise or resample anything larger than this. A tray
/// icon is about 16 pixels; a 4096-square PNG in `IconPixmap` is either
/// a mistake or an attempt to make the bar allocate a gigabyte.
const MAX_SIDE: u32 = 512;

/// How much larger than the drawn size an icon is decoded, so that the
/// GPU is downsampling rather than upsampling and the bar stays sharp
/// on a scaled output.
const OVERSAMPLE: u32 = 2;

/// A decoded icon: straight (non-premultiplied) RGBA8, row-major.
#[derive(Clone)]
pub struct Rgba {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Rgba {
    fn new(width: u32, height: u32) -> Option<Rgba> {
        let len = (width as usize)
            .checked_mul(height as usize)?
            .checked_mul(4)?;
        (width > 0 && height > 0).then(|| Rgba {
            width,
            height,
            data: vec![0; len],
        })
    }

    /// Bilinear resample. Written out rather than taken from a crate
    /// because it is twenty lines and the alternative is a direct
    /// dependency on `image`'s version, which `iced` already pins.
    fn resample(&self, width: u32, height: u32) -> Option<Rgba> {
        if width == self.width && height == self.height {
            return Some(self.clone());
        }
        let mut out = Rgba::new(width.min(MAX_SIDE), height.min(MAX_SIDE))?;

        // Map destination pixel centres back into the source, which is
        // the half-pixel offset that keeps the image from drifting up
        // and left as it shrinks.
        let sx = self.width as f32 / out.width as f32;
        let sy = self.height as f32 / out.height as f32;

        for y in 0..out.height {
            let fy = ((y as f32 + 0.5) * sy - 0.5).max(0.0);
            let y0 = fy.floor() as u32;
            let y1 = (y0 + 1).min(self.height - 1);
            let ty = fy - y0 as f32;

            for x in 0..out.width {
                let fx = ((x as f32 + 0.5) * sx - 0.5).max(0.0);
                let x0 = fx.floor() as u32;
                let x1 = (x0 + 1).min(self.width - 1);
                let tx = fx - x0 as f32;

                let at = |px: u32, py: u32| {
                    let i = ((py * self.width + px) * 4) as usize;
                    &self.data[i..i + 4]
                };
                let (a, b, c, d) = (at(x0, y0), at(x1, y0), at(x0, y1), at(x1, y1));

                let o = ((y * out.width + x) * 4) as usize;
                for channel in 0..4 {
                    let top = a[channel] as f32 + (b[channel] as f32 - a[channel] as f32) * tx;
                    let bottom = c[channel] as f32 + (d[channel] as f32 - c[channel] as f32) * tx;
                    out.data[o + channel] =
                        (top + (bottom - top) * ty).round().clamp(0.0, 255.0) as u8;
                }
            }
        }
        Some(out)
    }

    /// Composite `top` over the bottom-right of `self`, in place.
    ///
    /// The corner and the half-size come from the spec's own reading of
    /// `OverlayIconName`: a badge on an icon, not a second icon. The
    /// arithmetic is premultiplied because straight alpha does not
    /// compose; the operands and the result are both straight.
    fn over(&mut self, top: &Rgba) {
        let side = (self.width.min(self.height) / 2).max(1);
        let Some(top) = top.resample(side, side) else {
            return;
        };
        let ox = self.width.saturating_sub(top.width);
        let oy = self.height.saturating_sub(top.height);

        for y in 0..top.height {
            for x in 0..top.width {
                let s = ((y * top.width + x) * 4) as usize;
                let d = (((y + oy) * self.width + x + ox) * 4) as usize;
                let sa = top.data[s + 3] as f32 / 255.0;
                if sa == 0.0 {
                    continue;
                }
                let da = self.data[d + 3] as f32 / 255.0;
                let out_a = sa + da * (1.0 - sa);
                for channel in 0..3 {
                    let src = top.data[s + channel] as f32 * sa;
                    let dst = self.data[d + channel] as f32 * da * (1.0 - sa);
                    // Back to straight alpha, which is what everything
                    // downstream of here expects.
                    self.data[d + channel] = ((src + dst) / out_a).round().clamp(0.0, 255.0) as u8;
                }
                self.data[d + 3] = (out_a * 255.0).round() as u8;
            }
        }
    }

    fn into_handle(self) -> image::Handle {
        image::Handle::from_rgba(self.width, self.height, self.data)
    }
}

/// One entry of the protocol's `a(iiay)` pixmap array: width, height,
/// and `width * height` ARGB32 pixels.
pub struct Pixmap {
    pub width: i32,
    pub height: i32,
    pub argb: Vec<u8>,
}

/// Everything an item says about how it wants to be drawn.
///
/// Assembled by the caller from one `GetAll`, so that resolving an icon
/// is a pure function of a property bag rather than a series of round
/// trips to an application that may be wedged.
#[derive(Default)]
pub struct Request {
    pub name: String,
    pub pixmaps: Vec<Pixmap>,
    pub overlay_name: String,
    pub overlay_pixmaps: Vec<Pixmap>,
    pub theme_path: String,
}

impl Request {
    /// Whether this is worth asking about at all. An item with no name
    /// and no pixels has told us nothing, and the caller falls back to
    /// its four-character label.
    fn is_empty(&self) -> bool {
        self.name.is_empty() && self.pixmaps.is_empty()
    }

    /// The memo key. The pixmaps are keyed by their dimensions rather
    /// than their contents: hashing a few kilobytes on every poll to
    /// avoid a memcpy of the same few kilobytes is not a trade, and an
    /// item that changes its pixels announces it with `NewIcon`, which
    /// clears the cache entry anyway.
    fn key(&self, size: u32) -> String {
        let shape = |p: &Vec<Pixmap>| {
            p.iter()
                .map(|p| format!("{}x{}", p.width, p.height))
                .collect::<Vec<_>>()
                .join(",")
        };
        format!(
            "{size}\u{1}{}\u{1}{}\u{1}{}\u{1}{}\u{1}{}",
            self.name,
            shape(&self.pixmaps),
            self.overlay_name,
            shape(&self.overlay_pixmaps),
            self.theme_path,
        )
    }
}

/// A directory listed in a theme's `index.theme`, with the size rules
/// the spec attaches to it.
struct ThemeDir {
    path: String,
    size: u32,
    scale: u32,
    kind: DirKind,
    min: u32,
    max: u32,
    threshold: u32,
}

enum DirKind {
    Fixed,
    Scalable,
    Threshold,
}

impl ThemeDir {
    /// The spec's `DirectoryMatchesSize`, restricted to scale 1: the bar
    /// asks for the pixel size it is going to draw, so a `@2` directory
    /// of the same nominal size is a different, larger icon.
    fn matches(&self, size: u32) -> bool {
        if self.scale != 1 {
            return false;
        }
        match self.kind {
            DirKind::Fixed => self.size == size,
            DirKind::Scalable => self.min <= size && size <= self.max,
            DirKind::Threshold => {
                self.size.saturating_sub(self.threshold) <= size
                    && size <= self.size.saturating_add(self.threshold)
            }
        }
    }

    /// The spec's `DirectorySizeDistance`.
    fn distance(&self, size: u32) -> u32 {
        if self.scale != 1 {
            return u32::MAX / 2;
        }
        match self.kind {
            DirKind::Fixed => self.size.abs_diff(size),
            DirKind::Scalable | DirKind::Threshold => {
                let (min, max) = match self.kind {
                    DirKind::Scalable => (self.min, self.max),
                    _ => (
                        self.size.saturating_sub(self.threshold),
                        self.size.saturating_add(self.threshold),
                    ),
                };
                // Zero inside the range; the distance to whichever end
                // it fell outside otherwise.
                min.saturating_sub(size) + size.saturating_sub(max)
            }
        }
    }
}

/// A parsed `index.theme`.
struct Index {
    dirs: Vec<ThemeDir>,
    inherits: Vec<String>,
}

/// The icon-theme search, and its caches.
///
/// One of these lives on the tray thread for the life of the process.
/// It is deliberately not refreshed: an icon theme is installed by a
/// `switch`, and a `switch` restarts the bar.
pub struct Icons {
    /// Icon *base directories*, in the spec's order.
    roots: Vec<PathBuf>,
    /// Theme names, the configured one first and `hicolor` last, with
    /// every `Inherits` in between already expanded.
    themes: Vec<String>,
    /// `index.theme` per theme directory, parsed once. `None` records
    /// that the directory has no index, so it is not re-stat'd.
    indexes: HashMap<PathBuf, Option<Index>>,
    /// Resolved icons, keyed by [`Request::key`].
    cache: HashMap<String, Option<image::Handle>>,
}

impl Icons {
    /// Build the search path.
    ///
    /// `theme` names the icon theme to prefer; when it is `None` the
    /// desktop's GTK setting is read, and failing that the freedesktop
    /// default. Note that nothing on this desktop currently *sets* an
    /// icon theme, so the honest default resolves to `hicolor` plus
    /// whatever an item ships in its own `IconThemePath`.
    pub fn new(theme: Option<String>) -> Icons {
        let mut themes = Vec::new();
        let named = theme
            .filter(|t| !t.trim().is_empty())
            .or_else(gtk_icon_theme)
            .unwrap_or_else(|| DEFAULT_THEME.to_string());
        themes.push(named);

        let mut icons = Icons {
            roots: roots(),
            themes,
            indexes: HashMap::new(),
            cache: HashMap::new(),
        };

        // Expand `Inherits` breadth-first. Bounded by the number of
        // installed themes; a cycle cannot loop because a name is only
        // pushed if it is not already present.
        let mut at = 0;
        while at < icons.themes.len() {
            let theme = icons.themes[at].clone();
            at += 1;
            for parent in icons.inherits(&theme) {
                if !icons.themes.contains(&parent) {
                    icons.themes.push(parent);
                }
            }
        }
        if !icons.themes.iter().any(|t| t == FALLBACK_THEME) {
            icons.themes.push(FALLBACK_THEME.to_string());
        }
        icons
    }

    /// The icon for one item, at `size` pixels, or `None` when the item
    /// named nothing that could be found.
    pub fn resolve(&mut self, request: &Request, size: u32) -> Option<image::Handle> {
        if request.is_empty() {
            return None;
        }
        let key = request.key(size);
        if let Some(hit) = self.cache.get(&key) {
            return hit.clone();
        }
        let handle = self.render(request, size).map(Rgba::into_handle);
        self.cache.insert(key, handle.clone());
        handle
    }

    /// Forget everything resolved for `theme_path`-and-name combinations.
    ///
    /// Called on `NewIcon`: the item is telling us the pixels behind an
    /// unchanged name have changed, which is exactly the case the memo
    /// cannot see.
    pub fn forget(&mut self) {
        self.cache.clear();
    }

    /// The icon for one row of a `com.canonical.dbusmenu`.
    ///
    /// A menu row says the same thing as a tray item in two of the same
    /// five ways -- a themed name, or its own pixels -- so this is the
    /// search and the decoder that already exist, entered at a
    /// different door. The one real difference is the format: dbusmenu's
    /// `icon-data` is a **PNG file**, where the item interface's
    /// `IconPixmap` is raw ARGB32. Both land in [`decode_png`] and
    /// [`from_pixmaps`] respectively and come out as the same [`Rgba`].
    ///
    /// Name before data, for the reason [`Icons::render`] gives: a row
    /// that offers both is offering a themed icon that will match the
    /// rest of the desktop and a bitmap of whatever its toolkit
    /// happened to render.
    ///
    /// Only the *named* half is memoised. That is the half that costs a
    /// directory walk; keying a cache on a blob means either hashing it
    /// or trusting its length, and a menu is read once when it opens
    /// rather than on every poll.
    pub fn menu(&mut self, name: &str, data: &[u8], size: u32) -> Option<image::Handle> {
        let decoded = size.saturating_mul(OVERSAMPLE).min(MAX_SIDE);

        if !name.is_empty() {
            // `menu` cannot collide with a `Request::key`, which always
            // starts with a size.
            let key = format!("menu\u{1}{size}\u{1}{name}");
            if let Some(hit) = self.cache.get(&key) {
                if hit.is_some() {
                    return hit.clone();
                }
            } else {
                // The row names no `IconThemePath` of its own: a
                // dbusmenu carries no equivalent, and the item's is a
                // property of the item and not of its menu.
                let found = self.named(name, "", decoded).map(Rgba::into_handle);
                self.cache.insert(key, found.clone());
                if found.is_some() {
                    return found;
                }
            }
        }

        decode_png(data)
            .and_then(|rgba| fit(rgba, decoded))
            .map(Rgba::into_handle)
    }

    fn render(&mut self, request: &Request, size: u32) -> Option<Rgba> {
        let decoded = size.saturating_mul(OVERSAMPLE).min(MAX_SIDE);

        // Name before pixmap. An item that offers both is offering a
        // themed icon that will match the rest of the desktop and a
        // bitmap of whatever size its toolkit happened to render.
        let mut base = self
            .named(&request.name, &request.theme_path, decoded)
            .or_else(|| from_pixmaps(&request.pixmaps, decoded))?;

        if let Some(overlay) = self
            .named(&request.overlay_name, &request.theme_path, decoded)
            .or_else(|| from_pixmaps(&request.overlay_pixmaps, decoded))
        {
            base.over(&overlay);
        }
        Some(base)
    }

    /// Look up one icon by name: a path if the item gave one, an
    /// icon-theme search otherwise.
    fn named(&mut self, name: &str, theme_path: &str, size: u32) -> Option<Rgba> {
        if name.is_empty() {
            return None;
        }
        // Some applications put a path where the spec asks for a name.
        // Cheaper to honour it than to explain it.
        if name.starts_with('/') {
            return load(Path::new(name), size);
        }
        let path = self.find(name, theme_path, size)?;
        load(&path, size)
    }

    /// The spec's `FindIcon`, plus the item's own directory in front.
    fn find(&mut self, name: &str, theme_path: &str, size: u32) -> Option<PathBuf> {
        // `IconThemePath` is a colon-separated list in practice even
        // though the spec shows one path.
        let shipped: Vec<PathBuf> = theme_path
            .split(':')
            .filter(|p| !p.is_empty())
            .map(PathBuf::from)
            .collect();

        // Flat first: libappindicator drops `<name>.png` straight into
        // the directory it advertises, with no theme structure at all.
        for dir in &shipped {
            if let Some(hit) = flat(dir, name) {
                return Some(hit);
            }
        }

        // Cloned rather than borrowed because the search below takes
        // `&mut self` to fill the `index.theme` cache as it goes. Two
        // short `Vec<PathBuf>` per lookup, and lookups are memoised.
        let roots: Vec<PathBuf> = shipped.iter().chain(self.roots.iter()).cloned().collect();

        for theme in self.themes.clone() {
            for root in &roots {
                if let Some(hit) = self.find_in(&root.join(&theme), name, size) {
                    return Some(hit);
                }
            }
        }

        // Unthemed, the spec's last resort: `/usr/share/pixmaps` and
        // friends, and the theme roots themselves.
        roots.iter().find_map(|root| flat(root, name))
    }

    fn find_in(&mut self, base: &Path, name: &str, size: u32) -> Option<PathBuf> {
        let index = self.index(base)?;

        let mut best: Option<(u32, PathBuf)> = None;
        for dir in &index.dirs {
            let Some(hit) = flat(&base.join(&dir.path), name) else {
                continue;
            };
            if dir.matches(size) {
                return Some(hit);
            }
            let distance = dir.distance(size);
            if best.as_ref().is_none_or(|(d, _)| distance < *d) {
                best = Some((distance, hit));
            }
        }
        best.map(|(_, path)| path)
    }

    fn inherits(&mut self, theme: &str) -> Vec<String> {
        let roots = self.roots.clone();
        let mut out = Vec::new();
        for root in roots {
            if let Some(index) = self.index(&root.join(theme)) {
                out.extend(index.inherits.iter().cloned());
            }
        }
        out
    }

    fn index(&mut self, base: &Path) -> Option<&Index> {
        if !self.indexes.contains_key(base) {
            let parsed = std::fs::read_to_string(base.join("index.theme"))
                .ok()
                .map(|text| parse_index(&text));
            self.indexes.insert(base.to_path_buf(), parsed);
        }
        self.indexes.get(base)?.as_ref()
    }
}

/// `<dir>/<name>.<ext>` for the first extension that exists.
fn flat(dir: &Path, name: &str) -> Option<PathBuf> {
    for ext in EXTENSIONS {
        let candidate = dir.join(format!("{name}.{ext}"));
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

/// The icon base directories, in the order the spec searches them.
fn roots() -> Vec<PathBuf> {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let mut roots = Vec::new();

    if let Some(home) = &home {
        roots.push(home.join(".icons"));
    }
    let data_home = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| home.as_ref().map(|h| h.join(".local/share")));
    if let Some(data_home) = data_home {
        roots.push(data_home.join("icons"));
    }
    let data_dirs = std::env::var("XDG_DATA_DIRS").unwrap_or_default();
    let data_dirs = if data_dirs.trim().is_empty() {
        "/usr/local/share:/usr/share".to_string()
    } else {
        data_dirs
    };
    for dir in data_dirs.split(':').filter(|d| !d.is_empty()) {
        roots.push(Path::new(dir).join("icons"));
        // Not an icon base directory in the spec's sense, but it is
        // where a great many applications still install, and it costs a
        // failed `stat` to look.
        roots.push(Path::new(dir).join("pixmaps"));
    }
    roots.push(PathBuf::from("/usr/share/pixmaps"));
    roots.dedup();
    roots
}

/// `gtk-icon-theme-name`, from whichever GTK settings file has it.
fn gtk_icon_theme() -> Option<String> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;

    for version in ["gtk-4.0", "gtk-3.0"] {
        let Ok(text) = std::fs::read_to_string(base.join(version).join("settings.ini")) else {
            continue;
        };
        for line in text.lines() {
            if let Some(value) = line.trim().strip_prefix("gtk-icon-theme-name") {
                let value = value.trim_start().strip_prefix('=')?.trim();
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}

/// Parse `index.theme`: the `[Icon Theme]` header for `Directories` and
/// `Inherits`, then one section per directory for its size rules.
///
/// Hand-written rather than an INI crate because the file is a flat
/// `key=value` list under `[section]` headers with no quoting, no
/// escapes and no nesting -- and because `hicolor`'s has four hundred
/// sections, so the one thing that matters is not doing anything
/// quadratic with them.
fn parse_index(text: &str) -> Index {
    let mut listed: Vec<String> = Vec::new();
    let mut inherits = Vec::new();
    let mut sections: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
    let mut section = "";

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            section = name;
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let (key, value) = (key.trim(), value.trim());
        match (section, key) {
            ("Icon Theme", "Directories") | ("Icon Theme", "ScaledDirectories") => {
                listed.extend(value.split(',').map(|d| d.trim().to_string()));
            }
            ("Icon Theme", "Inherits") => {
                inherits.extend(
                    value
                        .split(',')
                        .map(|t| t.trim().to_string())
                        .filter(|t| !t.is_empty()),
                );
            }
            _ => {
                sections.entry(section).or_default().insert(key, value);
            }
        }
    }

    let dirs = listed
        .into_iter()
        .filter(|path| !path.is_empty())
        .map(|path| {
            let fields = sections.get(path.as_str());
            let field = |key: &str| fields.and_then(|f| f.get(key)).and_then(|v| v.parse().ok());
            // The spec's defaults, which a surprising number of themes
            // rely on rather than spelling out.
            let size: u32 = field("Size").unwrap_or(0);
            let kind = match fields.and_then(|f| f.get("Type")).copied() {
                Some("Fixed") => DirKind::Fixed,
                Some("Scalable") => DirKind::Scalable,
                _ => DirKind::Threshold,
            };
            ThemeDir {
                path,
                size,
                scale: field("Scale").unwrap_or(1),
                kind,
                min: field("MinSize").unwrap_or(size),
                max: field("MaxSize").unwrap_or(size),
                threshold: field("Threshold").unwrap_or(2),
            }
        })
        .collect();

    Index { dirs, inherits }
}

/// The best of an item's inline pixmaps, as RGBA at `size`.
fn from_pixmaps(pixmaps: &[Pixmap], size: u32) -> Option<Rgba> {
    // Prefer the smallest that is still at least the size wanted --
    // downsampling keeps detail, upsampling invents it -- and settle
    // for the largest available if every candidate is too small.
    let best = pixmaps
        .iter()
        .filter(|p| p.width > 0 && p.height > 0)
        .filter(|p| p.argb.len() >= (p.width as usize) * (p.height as usize) * 4)
        .min_by_key(|p| {
            let side = p.width.max(p.height) as u32;
            if side >= size {
                (0, side)
            } else {
                (1, u32::MAX - side)
            }
        })?;

    let (width, height) = (best.width as u32, best.height as u32);
    if width > MAX_SIDE || height > MAX_SIDE {
        return None;
    }
    let mut out = Rgba::new(width, height)?;
    for (pixel, chunk) in out.data.chunks_exact_mut(4).zip(best.argb.chunks_exact(4)) {
        // ARGB32, byte order A R G B, straight alpha.
        pixel[0] = chunk[1];
        pixel[1] = chunk[2];
        pixel[2] = chunk[3];
        pixel[3] = chunk[0];
    }
    fit(out, size)
}

/// Read one icon file. The extension decides how, because that is what
/// the theme spec says an icon file's extension means.
fn load(path: &Path, size: u32) -> Option<Rgba> {
    let bytes = std::fs::read(path).ok()?;
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    match extension.as_str() {
        "svg" | "svgz" => rasterise(&bytes, size),
        "png" => decode_png(&bytes).and_then(|rgba| fit(rgba, size)),
        "xpm" => decode_xpm(&bytes).and_then(|rgba| fit(rgba, size)),
        _ => None,
    }
}

/// Rasterise an SVG at `size`, letterboxed into a square.
fn rasterise(bytes: &[u8], size: u32) -> Option<Rgba> {
    let tree = resvg::usvg::Tree::from_data(bytes, &resvg::usvg::Options::default()).ok()?;
    let source = tree.size();
    if source.width() <= 0.0 || source.height() <= 0.0 {
        return None;
    }

    let side = size.clamp(1, MAX_SIDE);
    // One scale for both axes, so a non-square icon keeps its shape and
    // sits centred in the square the bar reserves for it.
    let scale = (side as f32 / source.width()).min(side as f32 / source.height());
    let dx = (side as f32 - source.width() * scale) / 2.0;
    let dy = (side as f32 - source.height() * scale) / 2.0;

    let mut pixmap = resvg::tiny_skia::Pixmap::new(side, side)?;
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::from_translate(dx, dy).pre_scale(scale, scale),
        &mut pixmap.as_mut(),
    );

    // tiny_skia is premultiplied; iced is not.
    let mut out = Rgba::new(side, side)?;
    for (pixel, source) in out.data.chunks_exact_mut(4).zip(pixmap.pixels()) {
        let straight = source.demultiply();
        pixel[0] = straight.red();
        pixel[1] = straight.green();
        pixel[2] = straight.blue();
        pixel[3] = straight.alpha();
    }
    Some(out)
}

/// Decode a PNG to straight RGBA8, whatever colour type it was written in.
fn decode_png(bytes: &[u8]) -> Option<Rgba> {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    // Expands palette and `tRNS` to real channels and drops 16-bit
    // samples to 8, so the match below has four cases rather than
    // twenty.
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().ok()?;

    let (width, height) = (reader.info().width, reader.info().height);
    if width > MAX_SIDE || height > MAX_SIDE {
        return None;
    }
    let mut buffer = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buffer).ok()?;
    let source = &buffer[..info.buffer_size()];

    let mut out = Rgba::new(width, height)?;
    let channels = match info.color_type {
        png::ColorType::Rgba => 4,
        png::ColorType::Rgb => 3,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::Grayscale => 1,
        // `normalize_to_color8` has already expanded `Indexed`; anything
        // else is a PNG this bar has no business guessing at.
        png::ColorType::Indexed => return None,
    };
    for (pixel, chunk) in out
        .data
        .chunks_exact_mut(4)
        .zip(source.chunks_exact(channels))
    {
        let (rgb, alpha) = match channels {
            4 => ([chunk[0], chunk[1], chunk[2]], chunk[3]),
            3 => ([chunk[0], chunk[1], chunk[2]], 255),
            2 => ([chunk[0], chunk[0], chunk[0]], chunk[1]),
            _ => ([chunk[0], chunk[0], chunk[0]], 255),
        };
        pixel[..3].copy_from_slice(&rgb);
        pixel[3] = alpha;
    }
    Some(out)
}

/// Decode an XPM to straight RGBA8.
///
/// XPM is a C source file: a `static char *name[]` whose entries are a
/// values line, one line per colour, and then one line per row of
/// pixels. Nothing but the string literals matters, so the parse is a
/// scan for double-quoted runs and then three fixed sections of them --
/// which is also why a truncated or reordered file falls out as `None`
/// rather than as a wrong image.
///
/// Hand-rolled for the same reason [`Rgba::resample`] is: the format is
/// this function long, and the crates that read it are either
/// unmaintained or arrive with a rendering stack attached. A tray icon
/// is 22 pixels square.
///
/// The freedesktop icon theme spec lists XPM beside PNG and SVG, and
/// libappindicator applications from the GNOME 2 era still ship them,
/// so the alternative to decoding it is taking it out of
/// [`EXTENSIONS`] -- a search that finds `<name>.xpm`, stops, and then
/// draws the four-character label anyway.
fn decode_xpm(bytes: &[u8]) -> Option<Rgba> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut lines = quoted_strings(text);

    // The values line: width, height, colours, characters per pixel.
    // Hotspot and `XPMEXT` may follow and are none of our business.
    let values = lines.next()?;
    let mut fields = values.split_whitespace();
    let width: u32 = fields.next()?.parse().ok()?;
    let height: u32 = fields.next()?.parse().ok()?;
    let colours: usize = fields.next()?.parse().ok()?;
    let per_pixel: usize = fields.next()?.parse().ok()?;

    if width == 0 || height == 0 || width > MAX_SIDE || height > MAX_SIDE {
        return None;
    }
    // One byte per pixel is the overwhelming case and two is the rest;
    // the cap is a bound on the palette walk below, not a judgement.
    if per_pixel == 0 || per_pixel > 4 || colours == 0 || colours > 1 << 16 {
        return None;
    }

    // Keyed by the pixel's own characters rather than by an index: the
    // file chooses them and they need not be contiguous or ordered.
    let mut palette: HashMap<&str, [u8; 4]> = HashMap::with_capacity(colours);
    for _ in 0..colours {
        let entry = lines.next()?;
        // Byte slicing rather than `chars`: the key is measured in
        // characters by the format, but a non-ASCII XPM is not a thing
        // that exists, and a multi-byte character here would make the
        // pixel rows unindexable anyway.
        if !entry.is_char_boundary(per_pixel) {
            return None;
        }
        let (key, rest) = entry.split_at(per_pixel);
        palette.insert(key, xpm_colour(rest)?);
    }

    let mut out = Rgba::new(width, height)?;
    for y in 0..height {
        let row = lines.next()?;
        // A row that is short would otherwise be padded with whatever
        // the last colour was, which reads as a corrupt icon rather
        // than as a refusal.
        if row.len() < width as usize * per_pixel {
            return None;
        }
        for x in 0..width {
            let at = x as usize * per_pixel;
            let key = row.get(at..at + per_pixel)?;
            let colour = palette.get(key)?;
            let offset = ((y * width + x) * 4) as usize;
            out.data[offset..offset + 4].copy_from_slice(colour);
        }
    }
    Some(out)
}

/// Every double-quoted run in the file, in order.
///
/// The C around them -- the comment, the declaration, the commas -- is
/// not parsed at all, which is deliberate: the only thing an XPM says
/// that a decoder needs is the sequence of its string literals.
fn quoted_strings(text: &str) -> impl Iterator<Item = &str> {
    // `\"` cannot appear: the format reserves `"` and `\` out of the
    // pixel alphabet precisely so this scan is unambiguous, and no
    // colour name contains either.
    text.split('"').skip(1).step_by(2)
}

/// One colour-table entry's worth of `{key colour}+`.
///
/// The keys are the visual classes: `c` colour, `g` greyscale, `g4`
/// four-level greyscale, `m` monochrome, `s` symbolic. Preferred in
/// that order, because a file that defines both is describing the same
/// icon for screens of different depth and ours is the deep one.
/// `s` is skipped rather than read: it names a colour the toolkit is
/// supposed to supply, and this is not a toolkit.
fn xpm_colour(entry: &str) -> Option<[u8; 4]> {
    const KEYS: [&str; 5] = ["c", "g4", "g", "m", "s"];

    // Values may be several words long ("light blue"), so the split is
    // on the keys rather than on whitespace.
    let mut found: HashMap<&str, String> = HashMap::new();
    let mut key: Option<&str> = None;
    let mut value = String::new();
    for token in entry.split_whitespace() {
        if KEYS.contains(&token) {
            if let Some(key) = key.take() {
                found.insert(key, std::mem::take(&mut value));
            }
            key = Some(token);
            continue;
        }
        if key.is_some() {
            if !value.is_empty() {
                value.push(' ');
            }
            value.push_str(token);
        }
    }
    if let Some(key) = key {
        found.insert(key, value);
    }

    for key in KEYS {
        if key == "s" {
            continue;
        }
        if let Some(value) = found.get(key) {
            return parse_colour(value);
        }
    }
    None
}

/// An XPM colour value: `None`, `#rrggbb` at any of four depths, or one
/// of the handful of X11 names that turn up in icon files.
///
/// Anything else -- `%hhssvv`, the other nine hundred names in
/// `rgb.txt` -- fails the whole decode rather than guessing, so an
/// unreadable icon draws the item's label instead of a plausible wrong
/// colour. Shipping `rgb.txt` to read a 22-pixel icon is the trade this
/// declines.
fn parse_colour(value: &str) -> Option<[u8; 4]> {
    let value = value.trim();
    if value.eq_ignore_ascii_case("none") {
        return Some([0, 0, 0, 0]);
    }
    if let Some(hex) = value.strip_prefix('#') {
        return parse_hex(hex);
    }
    named_colour(&value.to_ascii_lowercase())
}

/// `#rgb`, `#rrggbb`, `#rrrgggbbb` and `#rrrrggggbbbb`, all of which
/// XPM allows. The wider forms keep their most significant byte, which
/// is what every other reader does with them.
fn parse_hex(hex: &str) -> Option<[u8; 4]> {
    let per_channel = match hex.len() {
        3 | 6 | 9 | 12 => hex.len() / 3,
        _ => return None,
    };
    let mut out = [0u8, 0, 0, 255];
    for (channel, slot) in out.iter_mut().take(3).enumerate() {
        let at = channel * per_channel;
        let digits = hex.get(at..at + per_channel)?;
        let scaled = u32::from_str_radix(digits, 16).ok()?;
        *slot = match per_channel {
            1 => (scaled * 17) as u8,
            2 => scaled as u8,
            3 => (scaled >> 4) as u8,
            _ => (scaled >> 8) as u8,
        };
    }
    Some(out)
}

/// The X11 names an icon file plausibly uses, and `gray<n>`.
fn named_colour(name: &str) -> Option<[u8; 4]> {
    let level = |v: u8| Some([v, v, v, 255]);
    if let Some(percent) = name
        .strip_prefix("gray")
        .or_else(|| name.strip_prefix("grey"))
    {
        if percent.is_empty() {
            return level(0xbe);
        }
        let percent: u32 = percent.parse().ok()?;
        if percent > 100 {
            return None;
        }
        return level(((percent * 255 + 50) / 100) as u8);
    }
    let rgb = match name {
        "black" => [0x00, 0x00, 0x00],
        "white" => [0xff, 0xff, 0xff],
        "red" => [0xff, 0x00, 0x00],
        "green" => [0x00, 0xff, 0x00],
        "blue" => [0x00, 0x00, 0xff],
        "cyan" => [0x00, 0xff, 0xff],
        "magenta" => [0xff, 0x00, 0xff],
        "yellow" => [0xff, 0xff, 0x00],
        "orange" => [0xff, 0xa5, 0x00],
        "brown" => [0xa5, 0x2a, 0x2a],
        "pink" => [0xff, 0xc0, 0xcb],
        "purple" => [0xa0, 0x20, 0xf0],
        _ => return None,
    };
    Some([rgb[0], rgb[1], rgb[2], 255])
}

/// Scale a decoded raster down to `size` if it is larger.
///
/// Only down. An icon smaller than asked for is left alone and iced
/// scales it on the GPU, which is both sharper and free.
fn fit(rgba: Rgba, size: u32) -> Option<Rgba> {
    let side = rgba.width.max(rgba.height);
    if side <= size {
        return Some(rgba);
    }
    let scale = size as f32 / side as f32;
    let width = ((rgba.width as f32 * scale).round() as u32).max(1);
    let height = ((rgba.height as f32 * scale).round() as u32).max(1);
    rgba.resample(width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solid(width: u32, height: u32, colour: [u8; 4]) -> Rgba {
        let mut rgba = Rgba::new(width, height).expect("non-empty");
        for pixel in rgba.data.chunks_exact_mut(4) {
            pixel.copy_from_slice(&colour);
        }
        rgba
    }

    /// The shape gdk-pixbuf writes and libappindicator applications
    /// still ship: a transparent ground, one hex colour, one name.
    const SAMPLE_XPM: &str = r#"/* XPM */
static char * sample_xpm[] = {
"4 2 3 1",
" 	c None",
".	c #FF8800",
"+	c black",
" ..+",
"+.. "};
"#;

    #[test]
    fn an_xpm_decodes_to_the_pixels_it_names() {
        let out = decode_xpm(SAMPLE_XPM.as_bytes()).expect("a well-formed xpm");
        assert_eq!((out.width, out.height), (4, 2));

        let at = |x: usize, y: usize| &out.data[(y * 4 + x) * 4..(y * 4 + x) * 4 + 4];
        // `None` is transparent, not black-with-alpha.
        assert_eq!(at(0, 0), [0, 0, 0, 0]);
        assert_eq!(at(1, 0), [0xff, 0x88, 0x00, 0xff]);
        assert_eq!(at(3, 0), [0x00, 0x00, 0x00, 0xff]);
        assert_eq!(at(0, 1), [0x00, 0x00, 0x00, 0xff]);
        assert_eq!(at(3, 1), [0, 0, 0, 0]);
    }

    #[test]
    fn the_wide_hex_forms_keep_their_leading_byte() {
        assert_eq!(parse_hex("f80"), Some([0xff, 0x88, 0x00, 0xff]));
        assert_eq!(parse_hex("ff8800"), Some([0xff, 0x88, 0x00, 0xff]));
        assert_eq!(parse_hex("fff888000"), Some([0xff, 0x88, 0x00, 0xff]));
        assert_eq!(parse_hex("ffff88880000"), Some([0xff, 0x88, 0x00, 0xff]));
        // Not a channel count XPM defines.
        assert_eq!(parse_hex("ff88"), None);
    }

    #[test]
    fn a_colour_line_prefers_the_deepest_visual_it_offers() {
        // The same entry for a colour screen, a grey one and a mono
        // one. A tray is drawn on the first.
        assert_eq!(
            xpm_colour(" c #ff0000 g #808080 m white"),
            Some([0xff, 0x00, 0x00, 0xff])
        );
        // Values can be several words, and `s` names a colour we are
        // not the one supplying.
        assert_eq!(
            xpm_colour(" s icon_background c white"),
            Some([0xff, 0xff, 0xff, 0xff])
        );
        // A greyscale-only file still draws.
        assert_eq!(xpm_colour(" g gray50"), Some([0x80, 0x80, 0x80, 0xff]));
    }

    #[test]
    fn an_xpm_naming_a_colour_we_cannot_resolve_is_a_miss_not_a_guess() {
        // `%` is HSV, and `rgb.txt` is not shipped for a 22px icon --
        // both have to fail the file rather than pick something.
        assert!(xpm_colour(" c %ff8000").is_none());
        assert!(xpm_colour(" c lightgoldenrodyellow").is_none());

        let broken = SAMPLE_XPM.replace("#FF8800", "%FF8800");
        assert!(decode_xpm(broken.as_bytes()).is_none());
    }

    #[test]
    fn a_truncated_xpm_is_refused_rather_than_padded() {
        // One pixel row short of what the values line promised.
        let short = SAMPLE_XPM.replace("\" ..+\",\n", "");
        assert!(decode_xpm(short.as_bytes()).is_none());
        // A row shorter than the declared width.
        let clipped = SAMPLE_XPM.replace("\" ..+\"", "\" ..\"");
        assert!(decode_xpm(clipped.as_bytes()).is_none());
    }

    #[test]
    fn an_xpm_larger_than_the_bar_will_ever_draw_is_refused() {
        let huge = SAMPLE_XPM.replace("\"4 2 3 1\"", "\"4096 4096 3 1\"");
        assert!(decode_xpm(huge.as_bytes()).is_none());
    }

    #[test]
    fn argb_becomes_rgba_with_the_channels_in_the_right_order() {
        // One pixel: alpha 0x11, red 0x22, green 0x33, blue 0x44.
        let pixmaps = vec![Pixmap {
            width: 1,
            height: 1,
            argb: vec![0x11, 0x22, 0x33, 0x44],
        }];
        let out = from_pixmaps(&pixmaps, 16).expect("one pixel decodes");
        assert_eq!(out.data, vec![0x22, 0x33, 0x44, 0x11]);
    }

    #[test]
    fn a_pixmap_shorter_than_its_own_dimensions_is_refused() {
        // A truncated `a(iiay)` entry would otherwise index off the end.
        let pixmaps = vec![Pixmap {
            width: 8,
            height: 8,
            argb: vec![0; 4],
        }];
        assert!(from_pixmaps(&pixmaps, 16).is_none());
    }

    #[test]
    fn the_smallest_pixmap_at_least_as_large_as_asked_for_wins() {
        let make = |side: i32| Pixmap {
            width: side,
            height: side,
            argb: vec![0xff; (side * side * 4) as usize],
        };
        let pixmaps = vec![make(8), make(22), make(64)];
        let out = from_pixmaps(&pixmaps, 16).expect("decodes");
        assert_eq!((out.width, out.height), (16, 16));

        // Nothing big enough: take the largest rather than nothing.
        let out = from_pixmaps(&[make(8), make(12)], 32).expect("decodes");
        assert_eq!((out.width, out.height), (12, 12));
    }

    #[test]
    fn resampling_a_flat_colour_leaves_it_flat() {
        let out = solid(22, 22, [10, 20, 30, 255])
            .resample(14, 14)
            .expect("resamples");
        assert_eq!((out.width, out.height), (14, 14));
        for pixel in out.data.chunks_exact(4) {
            assert_eq!(pixel, [10, 20, 30, 255]);
        }
    }

    #[test]
    fn an_opaque_overlay_covers_the_corner_and_only_the_corner() {
        let mut base = solid(16, 16, [0, 0, 0, 255]);
        base.over(&solid(4, 4, [255, 255, 255, 255]));

        let at = |x: u32, y: u32| {
            let i = ((y * 16 + x) * 4) as usize;
            base.data[i..i + 4].to_vec()
        };
        assert_eq!(at(15, 15), vec![255, 255, 255, 255]);
        assert_eq!(at(8, 8), vec![255, 255, 255, 255]);
        // One pixel outside the bottom-right eighth is untouched.
        assert_eq!(at(7, 7), vec![0, 0, 0, 255]);
        assert_eq!(at(0, 0), vec![0, 0, 0, 255]);
    }

    #[test]
    fn a_fully_transparent_overlay_changes_nothing() {
        let mut base = solid(16, 16, [9, 9, 9, 255]);
        let before = base.data.clone();
        base.over(&solid(8, 8, [255, 0, 0, 0]));
        assert_eq!(base.data, before);
    }

    #[test]
    fn threshold_is_the_default_type_and_two_the_default_threshold() {
        let index = parse_index("[Icon Theme]\nDirectories=24x24/apps\n\n[24x24/apps]\nSize=24\n");
        let dir = &index.dirs[0];
        assert!(dir.matches(24));
        assert!(dir.matches(22));
        assert!(!dir.matches(21));
    }

    #[test]
    fn a_scalable_directory_matches_its_whole_range() {
        let index = parse_index(
            "[Icon Theme]\nDirectories=scalable/apps\n\n\
             [scalable/apps]\nSize=128\nType=Scalable\nMinSize=8\nMaxSize=512\n",
        );
        let dir = &index.dirs[0];
        assert!(dir.matches(16));
        assert!(dir.matches(512));
        assert!(!dir.matches(513));
        assert_eq!(dir.distance(600), 88);
    }

    #[test]
    fn a_doubled_scale_directory_never_matches() {
        // `16x16@2` holds 32px art. Taking it for a 16px request would
        // hand the bar an icon twice the size it asked for.
        let index = parse_index(
            "[Icon Theme]\nDirectories=16x16@2/apps\n\n\
             [16x16@2/apps]\nSize=16\nScale=2\nType=Fixed\n",
        );
        assert!(!index.dirs[0].matches(16));
    }

    #[test]
    fn inherits_is_read_and_split() {
        let index = parse_index("[Icon Theme]\nName=Papirus\nInherits=hicolor,breeze\n");
        assert_eq!(index.inherits, vec!["hicolor", "breeze"]);
    }

    #[test]
    fn hicolor_is_always_last_in_the_search_even_if_nothing_names_it() {
        let icons = Icons::new(Some("NoSuchTheme".to_string()));
        assert_eq!(
            icons.themes.first().map(String::as_str),
            Some("NoSuchTheme")
        );
        assert_eq!(
            icons.themes.last().map(String::as_str),
            Some(FALLBACK_THEME)
        );
    }

    #[test]
    fn an_item_that_named_no_icon_at_all_resolves_to_nothing() {
        let mut icons = Icons::new(Some(FALLBACK_THEME.to_string()));
        assert!(icons.resolve(&Request::default(), 16).is_none());
    }
}
