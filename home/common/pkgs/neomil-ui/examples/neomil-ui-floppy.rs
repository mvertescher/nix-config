use iced::{Element, Length, Color, Task};
use iced::widget::{column, row, container, text, Space, canvas};
use neomil_ui::fonts;
use neomil_ui::colors;
use neomil_ui::widgets::{floppy_icon, FloppyIcon};

pub fn main() -> iced::Result {
    // If in diff mode, we might want a smaller window, but we can also just let the
    // screenshot script control the window size.
    let window_settings = if std::env::var("DIFF_MODE").is_ok() {
        iced::window::Settings {
            size: iced::Size::new(240.0, 220.0), // Force exact design size
            resizable: false,
            decorations: false,
            ..Default::default()
        }
    } else {
        iced::window::Settings::default()
    };

    iced::application("NEOMIL // FLOPPY ICON TEST", App::update, App::view)
        .font(fonts::ORBITRON_REGULAR)
        .font(fonts::ORBITRON_BOLD)
        .default_font(iced::Font {
            family: iced::font::Family::Name("Orbitron"),
            weight: iced::font::Weight::Normal,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::Normal,
        })
        .window(window_settings)
        .style(|_state, _theme| iced::application::Appearance {
            background_color: colors::COLOR_BG, // Use dark background (0x080808)
            text_color: Color::WHITE,
        })
        .run()
}

#[derive(Default)]
struct App {}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl App {
    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let color_accent = Color::from_rgb8(0xFF, 0x4B, 0x4B); // Bright red

        // --- DIFF MODE (Visual Regression) ---
        // Renders ONLY the floppy icon filling the entire window for pixel-perfect diffing.
        if let Ok(mode) = std::env::var("DIFF_MODE") {
            let is_selected = mode == "selected";
            return container(
                canvas(FloppyIcon { color: color_accent, is_selected, scale: 4.63 })
                    .width(Length::Fill)
                    .height(Length::Fill)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
        }

        // --- STANDARD DEMO MODE ---

        // 1x Section (50x50)
        let unselected_1x = column![
            floppy_icon(color_accent, false, 1.0),
            Space::with_height(5),
            text("UNSELECTED (1x)")
                .font(fonts::FONT_ORBITRON_REGULAR)
                .size(11)
                .color(color_accent)
        ]
        .align_x(iced::Alignment::Center);

        let selected_1x = column![
            floppy_icon(color_accent, true, 1.0),
            Space::with_height(5),
            text("SELECTED (1x)")
                .font(fonts::FONT_ORBITRON_BOLD)
                .size(11)
                .color(color_accent)
        ]
        .align_x(iced::Alignment::Center);

        let row_1x = row![
            unselected_1x,
            Space::with_width(40),
            selected_1x,
        ]
        .align_y(iced::Alignment::Center);


        // 3x Section (150x150)
        let unselected_3x = column![
            floppy_icon(color_accent, false, 3.0),
            Space::with_height(10),
            text("UNSELECTED (3x)")
                .font(fonts::FONT_ORBITRON_REGULAR)
                .size(12)
                .color(color_accent)
        ]
        .align_x(iced::Alignment::Center);

        let selected_3x = column![
            floppy_icon(color_accent, true, 3.0),
            Space::with_height(10),
            text("SELECTED (3x)")
                .font(fonts::FONT_ORBITRON_BOLD)
                .size(12)
                .color(color_accent)
        ]
        .align_x(iced::Alignment::Center);

        let row_3x = row![
            unselected_3x,
            Space::with_width(60),
            selected_3x,
        ]
        .align_y(iced::Alignment::Center);


        // Design Size Section (240x220 Canvas, 4.63x Scale)
        let unselected_design = column![
            canvas(FloppyIcon { color: color_accent, is_selected: false, scale: 4.63 })
                .width(Length::Fixed(240.0))
                .height(Length::Fixed(220.0)),
            Space::with_height(10),
            text("UNSELECTED (DESIGN SIZE 4.63x)")
                .font(fonts::FONT_ORBITRON_REGULAR)
                .size(13)
                .color(color_accent)
        ]
        .align_x(iced::Alignment::Center);

        let selected_design = column![
            canvas(FloppyIcon { color: color_accent, is_selected: true, scale: 4.63 })
                .width(Length::Fixed(240.0))
                .height(Length::Fixed(220.0)),
            Space::with_height(10),
            text("SELECTED (DESIGN SIZE 4.63x)")
                .font(fonts::FONT_ORBITRON_BOLD)
                .size(13)
                .color(color_accent)
        ]
        .align_x(iced::Alignment::Center);

        let row_design = row![
            unselected_design,
            Space::with_width(80),
            selected_design,
        ]
        .align_y(iced::Alignment::Center);


        // Main Layout
        let content = column![
            text("NEOMIL // FLOPPY ICON COMPARISON")
                .font(fonts::FONT_ORBITRON_BOLD)
                .size(20)
                .color(Color::WHITE),
            Space::with_height(30),
            row_1x,
            Space::with_height(30),
            row_3x,
            Space::with_height(30),
            row_design,
        ]
        .align_x(iced::Alignment::Center);

        container(content)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
