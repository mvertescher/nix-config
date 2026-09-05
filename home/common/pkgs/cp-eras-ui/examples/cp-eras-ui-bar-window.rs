//! The bar's view, in an ordinary window.
//!
//!     cp-eras-ui-bar-window                # follow the desktop theme
//!     cp-eras-ui-bar-window --era kitsch   # force one
//!
//! This exists so the bar can be golden-tested. `tests/visual.nix`
//! drives weston's headless backend, which has no `wlr-layer-shell`, so
//! it cannot run `cp-eras-ui-bar` at all -- and even on a compositor
//! that could, a layer surface is not something `weston-screenshooter`
//! is any good at capturing.
//!
//! It is a separate binary rather than a `--windowed` flag on the real
//! bar on purpose. What a golden should hold still is `bar()`, the pure
//! function of `Style` and `Readings`; the live bar is that plus a layer
//! surface plus four sensor threads, none of which a screenshot can say
//! anything about. Keeping them apart means the test exercises exactly
//! what it claims to, and the binary that actually runs on the desktop
//! carries no branch that only a test ever takes.
//!
//! The readings are fixed for the same reason: a clock that ticked would
//! make every capture differ from the last, and the whole point is that
//! two runs are byte-identical.

#[path = "bar/style.rs"]
mod style;

use cp_eras_ui::bar::{
    bar, tray_menu, Audio, MenuEntry, MenuKind, MenuPath, Network, Readings, TrayItem, TrayMenu,
    Workspace,
};
use cp_eras_ui::{Element, Style};
use iced::widget::{column, container, image, row, Space};
use iced::Length;

/// A tray icon, drawn here rather than read from anywhere.
///
/// The golden has to hold the *icon* path still, not just the label
/// fallback, and it cannot do that with a real icon: the sandbox has no
/// icon theme, and even on a desktop the file behind `IconName` belongs
/// to whatever version of whatever package is installed. So the sample
/// synthesises one -- a diamond, because it is four comparisons and it
/// is obvious in a diff which way round it went.
///
/// 28 square is what the tray decodes at for a 26px bar: twice the
/// 14px the cell draws, so the capture shows the same downsampling the
/// live bar does.
fn sample_icon(rgb: [u8; 3]) -> image::Handle {
    const SIDE: i32 = 28;
    let mut data = Vec::with_capacity((SIDE * SIDE * 4) as usize);
    for y in 0..SIDE {
        for x in 0..SIDE {
            let distance = (x * 2 - SIDE + 1).abs() + (y * 2 - SIDE + 1).abs();
            let inside = distance <= SIDE - 2;
            data.extend_from_slice(&[rgb[0], rgb[1], rgb[2], if inside { 255 } else { 0 }]);
        }
    }
    image::Handle::from_rgba(SIDE as u32, SIDE as u32, data)
}

/// Fixed readings. Every optional module is present, because a module
/// that is absent from the sample is a module the golden says nothing
/// about.
fn sample() -> Readings {
    Readings {
        host: "nomad".to_string(),
        workspaces: (1..=6)
            .map(|id| Workspace {
                id,
                active: id == 3,
            })
            .collect(),
        window: "~/src/cp-eras-ui - nvim src/bar.rs".to_string(),
        cpu: 12,
        memory: 47,
        audio: Some(Audio {
            volume: 62,
            muted: false,
        }),
        // All four shapes a tray cell can take: an icon, an icon from
        // an item that is shouting, and the label fallback in both of
        // its inks. A cell that is not in the sample is a cell the
        // golden says nothing about, and the fallback is not a
        // deprecated path -- it is what an item whose icon no installed
        // theme has actually gets.
        tray: vec![
            TrayItem {
                label: "FLAM".to_string(),
                icon: Some(sample_icon([0x88, 0x00, 0xaa])),
                attention: false,
            },
            TrayItem {
                label: "ELEM".to_string(),
                icon: Some(sample_icon([0xff, 0x66, 0x00])),
                attention: true,
            },
            TrayItem {
                label: "BLUE".to_string(),
                icon: None,
                attention: false,
            },
            TrayItem {
                label: "SYNC".to_string(),
                icon: None,
                attention: true,
            },
        ],
        network: Network::Wireless {
            interface: "wlp13s0".to_string(),
            ssid: "AFTERLIFE".to_string(),
        },
        clock: "23:41".to_string(),
        date: "2026-08-23".to_string(),
    }
}

/// A tray item's context menu, drawn beside the bar with one submenu
/// open.
///
/// The live bar draws this on a second surface that only a compositor
/// with `wlr-layer-shell` can create, which is exactly the thing this
/// harness has not got -- but the *panel* is `bar::tray_menu`, a pure
/// function of `Style` like the rest, and every era dresses it
/// differently. Without this the four bar goldens said nothing about
/// menus at all, and the only era anyone had ever looked at one in was
/// neomil.
///
/// One row of each kind the vocabulary has, because a kind that is not
/// in the sample is a kind the golden says nothing about: a submenu
/// (open, so its parent wears the era's selection and its child panel
/// lands beside it), a set toggle, a disabled row, a separator, rows
/// with icons and rows without in the same panel -- which is what
/// pins the reserved icon gutter.
fn sample_menu() -> TrayMenu {
    fn entry(id: i32, label: &str, kind: MenuKind, icon: bool) -> MenuEntry {
        MenuEntry {
            id,
            label: label.to_string(),
            enabled: true,
            kind,
            icon: icon.then(|| sample_icon([0x88, 0x00, 0xaa])),
            children: Vec::new(),
        }
    }

    TrayMenu {
        entries: vec![
            MenuEntry {
                children: vec![
                    entry(21, "Mount", MenuKind::Command, true),
                    entry(22, "Eject", MenuKind::Command, false),
                ],
                ..entry(20, "Devices", MenuKind::Submenu, true)
            },
            MenuEntry {
                enabled: false,
                ..entry(3, "Disconnect", MenuKind::Command, false)
            },
            entry(4, "", MenuKind::Separator, false),
            // Below the rule rather than above it, so the era's
            // selection is not drawn twice running: an open submenu's
            // parent and a set toggle are the same fill, and two of
            // them touching read as one block rather than two rows.
            entry(5, "Enable networking", MenuKind::Toggle(true), false),
            entry(6, "Quit", MenuKind::Command, true),
        ],
    }
}

/// Which submenu the sample has open: the first row's.
const SAMPLE_OPEN: [usize; 1] = [0];

struct BarWindow {
    style: Style,
    readings: Readings,
    menu: TrayMenu,
}

/// The window is a still life; nothing sends these and `update` drops
/// them.
///
/// They exist because `tray_menu` takes the constructors its rows
/// answer with, and unlike `bar()` there is no `None` to hand it: a
/// menu row that could not be clicked would be a different widget,
/// where a tray cell that is not listening is the same one. A
/// `mouse_area` nobody clicks draws identically either way, which is
/// all a capture can see.
///
/// `dead_code` because the payloads really are never read, and that is
/// the point rather than an oversight: the variants exist to be
/// *constructed*, by the message constructors `tray_menu` asks for.
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Message {
    Entry(i32),
    Submenu(MenuPath),
}

impl BarWindow {
    fn title(&self) -> String {
        format!("cp-eras-ui-bar - {}", self.style.era.name())
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        column![
            // `None`: the golden is a still life, and hit-testing it
            // could never exercise would only be a claim the capture
            // cannot check.
            container(bar(&self.style, &self.readings, None))
                .height(Length::Fixed(self.style.bar.height as f32)),
            // Flush under the bar, and right-aligned with a margin,
            // which is where the live bar puts it: the overlay surface
            // begins exactly where the bars end, and the chain hangs
            // leftwards off the pointer because the tray is the last
            // group on the right. `Fill` rather than a computed offset
            // so this holds at whatever width the harness renders.
            row![
                Space::new().width(Length::Fill).height(Length::Shrink),
                tray_menu(
                    &self.style,
                    &self.menu,
                    &SAMPLE_OPEN,
                    Message::Entry,
                    Message::Submenu,
                ),
                // The margin is measured to the root panel's outline;
                // whatever the era draws past that (neokitsch's
                // highlight plate) comes out of it.
                Space::new()
                    .width(Length::Fixed(MENU_MARGIN - cp_eras_ui::bar::menu_overshoot(&self.style)))
                    .height(Length::Shrink),
            ],
            // The bar and its menu do not fill 220px. The empty ground
            // under them is what makes a capture legible as a desktop
            // edge, and it also puts the era's background role in the
            // diff.
            container(Space::new().width(Length::Fill).height(Length::Fill)).height(Length::Fill),
        ]
        .into()
    }
}

/// How far the menu chain's right edge sits from the right of the
/// capture: every era's root panel ends on the design's x=1480.
///
/// Until 2026-09-04 this subtracted `bar::menu_edge_pad`, the 14px of
/// neokitsch onion rings that ran outside the root panel; the rings
/// now nest inside it. Since 2026-09-05 `view` subtracts
/// `bar::menu_overshoot` instead -- the 8px neokitsch's highlight
/// plate runs past the panel, to the design's x=1488.
const MENU_MARGIN: f32 = 120.0;

fn main() -> iced::Result {
    let style = style::resolve();
    iced::application(
        move || BarWindow {
            style,
            readings: sample(),
            menu: sample_menu(),
        },
        BarWindow::update,
        BarWindow::view,
    )
    .title(BarWindow::title)
    .font(cp_eras_ui::fonts::RAJDHANI_REGULAR)
    .font(cp_eras_ui::fonts::RAJDHANI_MEDIUM)
    .font(cp_eras_ui::fonts::RAJDHANI_SEMIBOLD)
    .font(cp_eras_ui::fonts::RAJDHANI_BOLD)
    .font(cp_eras_ui::fonts::NOTO_SANS_CJK_JP_BOLD)
    .default_font(cp_eras_ui::fonts::FONT_RAJDHANI_REGULAR)
    .theme(|app: &BarWindow| app.style)
    .window_size((1600.0, 220.0))
    .antialiasing(true)
    .run()
}
