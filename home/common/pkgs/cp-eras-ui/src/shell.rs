//! Booting an era app.
//!
//! Every binary that opens a window in an era performs the same
//! ritual: decide which era (a `--era` flag, else the desktop's
//! published theme), load the faces, hand iced the [`Style`] as its
//! theme, and open the 1600x900 frame the traces are measured in. Five
//! examples and `examples/bar/style.rs` each carried their own copy,
//! and the font lists had drifted -- a screen that names a weight its
//! binary never loaded is shaped in the wrong face (`fonts.rs`). This
//! is the one copy.
//!
//! ```text
//! let style = shell::style();
//! shell::application(move || Login::new(style), Login::update, Login::view)
//!     .title(Login::title)
//!     .run()
//! ```
//!
//! A layershell daemon builds its own `Settings`; it takes [`faces`]
//! for the `fonts` field and `style()` for the era, and sets the theme
//! itself.
//!
//! Not here: a transparent window with a background layer. The old
//! neomil mock wanted one; every screen the traces show paints its own
//! ground edge to edge (`screens::scene`, `widgets::ground`), and the
//! theme base fills whatever is left in the palette's `bg`.

use crate::fonts;
use crate::style::{Era, Style};
use iced::application::{BootFn, UpdateFn, ViewFn};
use iced::Program;
use iced::{Font, Settings, Size};
use std::borrow::Cow;

/// The frame the traces are drawn in and `scripts/fidelity_check.sh`
/// captures at.
pub const FRAME: Size = Size::new(1600.0, 900.0);

/// A state that knows which era it wears.
///
/// [`application`] reads the theme from it. The screens implement it
/// over their `style` field; an example's own state does the same.
pub trait Wears {
    fn wears(&self) -> Style;
}

/// The era named on the command line, or the desktop's.
///
/// `--era <name>` / `--era=<name>` takes the era's compiled defaults
/// and layers the published palette over them only when the desktop
/// is *already* in that era: forcing kitsch on a neomil desktop should
/// give kitsch's own colours, not neomil's wearing kitsch's geometry.
/// Without the flag this is [`Style::from_desktop`].
pub fn style() -> Style {
    style_for(era_from(std::env::args().skip(1)))
}

/// [`style`] with the era already decided (`None` = follow the desktop).
pub fn style_for(era: Option<Era>) -> Style {
    match era {
        Some(era) => {
            let mut style = era.style();
            let theme = crate::theme::Theme::load();
            if Era::parse(&theme.era) == Some(era) {
                style.palette = style.palette.with_theme(&theme);
            }
            style
        }
        None => Style::from_desktop(),
    }
}

/// The `--era` flag out of an argument list, in either spelling. An
/// unknown name is `None`, the same as no flag: the desktop decides.
pub fn era_from<I>(args: I) -> Option<Era>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    flag(args, "--era").and_then(|name| Era::parse(&name))
}

/// The value of `--name <value>` or `--name=<value>` in an argument
/// list, `None` when absent or when the flag ends the list.
pub fn flag<I>(args: I, name: &str) -> Option<String>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        let arg = arg.as_ref();
        if let Some(value) = arg.strip_prefix(name).and_then(|rest| rest.strip_prefix('=')) {
            return Some(value.to_string());
        }
        if arg == name {
            return args.next().map(|v| v.as_ref().to_string());
        }
    }
    None
}

/// Whether a bare `--name` is in an argument list.
pub fn switch<I>(args: I, name: &str) -> bool
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    args.into_iter().any(|arg| arg.as_ref() == name)
}

/// Every face the crate ships, for the fonts an app loads at boot.
///
/// All of them, always: the eras between them set Rajdhani at five
/// weights and Orbitron at four, the shaper substitutes silently for a
/// weight it was not given, and the bytes are in the binary either way.
/// The CJK subset rides along for the Han fallback (`fonts.rs`).
pub fn faces() -> Vec<Cow<'static, [u8]>> {
    [
        fonts::RAJDHANI_LIGHT,
        fonts::RAJDHANI_REGULAR,
        fonts::RAJDHANI_MEDIUM,
        fonts::RAJDHANI_SEMIBOLD,
        fonts::RAJDHANI_BOLD,
        fonts::ORBITRON_REGULAR,
        fonts::ORBITRON_MEDIUM,
        fonts::ORBITRON_SEMIBOLD,
        fonts::ORBITRON_BOLD,
        fonts::NOTO_SANS_CJK_JP_BOLD,
    ]
    .into_iter()
    .map(Cow::Borrowed)
    .collect()
}

/// The face a run falls to when nothing names one.
pub const DEFAULT_FONT: Font = fonts::FONT_RAJDHANI_REGULAR;

/// iced settings for an era app: the faces, Rajdhani regular by
/// default, antialiasing on (canvas edges and chamfers are most of
/// what the eras draw).
pub fn settings() -> Settings {
    Settings {
        fonts: faces(),
        default_font: DEFAULT_FONT,
        antialiasing: true,
        ..Settings::default()
    }
}

/// The window's app id: the binary's own name, `cp-eras-ui-store` and
/// so on. It is what a compositor rule matches on (`class` in
/// Hyprland), and without one every screen is a window with an empty
/// class that no rule can tell from any other. The executable name
/// rather than a constant so the six binaries need not each say it.
pub fn app_id() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.file_name().map(|name| name.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "cp-eras-ui".to_string())
}

/// The window an era app opens in: the trace [`FRAME`], named by
/// [`app_id`].
pub fn window() -> iced::window::Settings {
    iced::window::Settings {
        size: FRAME,
        platform_specific: iced::window::settings::PlatformSpecific {
            application_id: app_id(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// [`iced::application()`] with the ritual done: [`settings`], the
/// state's era as the theme, the trace [`FRAME`] as the window under
/// the binary's app id. Chain `.title(..)`, `.subscription(..)`, or
/// another `.window_size(..)` on the result as usual.
pub fn application<State, Message>(
    boot: impl BootFn<State, Message>,
    update: impl UpdateFn<State, Message>,
    view: impl for<'a> ViewFn<'a, State, Message, Style, iced::Renderer>,
) -> iced::Application<impl Program<State = State, Message = Message, Theme = Style>>
where
    State: Wears + 'static,
    Message: Send + 'static,
{
    iced::application(boot, update, view)
        .theme(|state: &State| state.wears())
        .settings(settings())
        .window(window())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn era_flag_in_both_spellings() {
        assert_eq!(era_from(["--era", "kitsch"]), Some(Era::Kitsch));
        assert_eq!(era_from(["--era=neomil"]), Some(Era::Neomil));
        assert_eq!(era_from(["--verbose", "--era", "entropism"]), Some(Era::Entropism));
    }

    #[test]
    fn missing_or_unknown_era_is_the_desktops() {
        assert_eq!(era_from::<[&str; 0]>([]), None);
        assert_eq!(era_from(["--era"]), None);
        assert_eq!(era_from(["--era", "brutalism"]), None);
        assert_eq!(era_from(["--era=", "kitsch"]), None);
    }

    #[test]
    fn flags_and_switches() {
        let args = ["--greet", "--user", "mverte", "--cmd=uwsm start x", "--era", "kitsch"];
        assert!(switch(args, "--greet"));
        assert!(!switch(args, "--nope"));
        assert_eq!(flag(args, "--user").as_deref(), Some("mverte"));
        assert_eq!(flag(args, "--cmd").as_deref(), Some("uwsm start x"));
        assert_eq!(flag(args, "--era").as_deref(), Some("kitsch"));
        assert_eq!(flag(args, "--use"), None);
        assert_eq!(flag(["--user"], "--user"), None);
    }

    /// Every byte constant in `fonts.rs` is loaded, so no screen can
    /// name a face the app does not have.
    #[test]
    fn every_face_is_loaded() {
        let faces = faces();
        for bytes in [
            fonts::RAJDHANI_LIGHT,
            fonts::RAJDHANI_REGULAR,
            fonts::RAJDHANI_MEDIUM,
            fonts::RAJDHANI_SEMIBOLD,
            fonts::RAJDHANI_BOLD,
            fonts::ORBITRON_REGULAR,
            fonts::ORBITRON_MEDIUM,
            fonts::ORBITRON_SEMIBOLD,
            fonts::ORBITRON_BOLD,
            fonts::NOTO_SANS_CJK_JP_BOLD,
        ] {
            assert!(faces.iter().any(|f| f.as_ref() == bytes));
        }
        assert_eq!(faces.len(), 10);
    }
}
