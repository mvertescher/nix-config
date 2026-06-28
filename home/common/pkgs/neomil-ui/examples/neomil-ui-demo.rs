use iced::widget::{canvas, column, container, row, stack, text, Space};
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Theme, Vector};
use neomil_ui::colors;
use neomil_ui::fonts;
use neomil_ui::widgets::{diamond_menu, info_panel, level_badge, DiamondMenuItem};

// Helper constants for Orbitron
const FONT_ORBITRON_REGULAR: iced::Font = iced::Font {
    family: iced::font::Family::Name("Orbitron"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

const FONT_ORBITRON_MEDIUM: iced::Font = iced::Font {
    family: iced::font::Family::Name("Orbitron"),
    weight: iced::font::Weight::Medium,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

const FONT_ORBITRON_BOLD: iced::Font = iced::Font {
    family: iced::font::Family::Name("Orbitron"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

// Helper constants for Rajdhani
const FONT_RAJDHANI_REGULAR: iced::Font = iced::Font {
    family: iced::font::Family::Name("Rajdhani"),
    weight: iced::font::Weight::Normal,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

const FONT_RAJDHANI_MEDIUM: iced::Font = iced::Font {
    family: iced::font::Family::Name("Rajdhani"),
    weight: iced::font::Weight::Medium,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

const FONT_RAJDHANI_BOLD: iced::Font = iced::Font {
    family: iced::font::Family::Name("Rajdhani"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

// Aliases to minimize changes
const FONT_REGULAR: iced::Font = FONT_RAJDHANI_REGULAR; // Main font
const FONT_MEDIUM: iced::Font = FONT_ORBITRON_MEDIUM;   // Title font
const FONT_BOLD: iced::Font = FONT_ORBITRON_BOLD;       // Title font

pub fn main() -> iced::Result {
    iced::application("NEOMIL // UI TOOLKIT DEMO", App::update, App::view)
        .font(fonts::ORBITRON_REGULAR)
        .font(fonts::ORBITRON_MEDIUM)
        .font(fonts::ORBITRON_SEMIBOLD)
        .font(fonts::ORBITRON_BOLD)
        .font(fonts::RAJDHANI_LIGHT)
        .font(fonts::RAJDHANI_REGULAR)
        .font(fonts::RAJDHANI_MEDIUM)
        .font(fonts::RAJDHANI_SEMIBOLD)
        .font(fonts::RAJDHANI_BOLD)
        .default_font(iced::Font {
            family: iced::font::Family::Name("Rajdhani"),
            weight: iced::font::Weight::Medium,
            stretch: iced::font::Stretch::Normal,
            style: iced::font::Style::Normal,
        })
        .run()
}

#[derive(Default)]
struct App;

#[derive(Debug, Clone)]
enum Message {
    MenuSelected(usize),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::MenuSelected(idx) => {
                println!("Selected menu item: {}", idx);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // --- 1. BACKGROUND LAYER ---
        let bg_canvas = canvas(BackgroundProgram {
            glow_color: colors::COLOR_GLOW,
            bg_color: colors::COLOR_BG,
        })
        .width(Length::Fill)
        .height(Length::Fill);

        // --- 2. TOP BAR ---
        let customer_level = column![
            text("CUSTOMER")
                .size(10)
                .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
            container(
                text("LEVEL T1")
                    .size(14)
                    .font(FONT_BOLD)
                    .style(|_| text::Style { color: Some(Color::WHITE) })
            )
            .padding([5, 15])
            .style(|_| container::Style {
                border: iced::Border {
                    color: colors::COLOR_PRIMARY_RED,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
        ]
        .spacing(5);

        let logo = column![
            text("#NC488402")
                .size(10)
                .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
            text("next")
                .size(36)
                .font(FONT_BOLD)
                .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
            text("TECHNOLOGY")
                .size(12)
                .font(FONT_MEDIUM)
                .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
            container(
                text("JHN 102 CKC 151 CC10 S111")
                    .size(8)
                    .font(FONT_ORBITRON_REGULAR)
                    .style(|_| text::Style { color: Some(colors::COLOR_BG) })
            )
            .padding([2, 8])
            .style(|_| container::Style {
                background: Some(iced::Background::Color(colors::COLOR_PRIMARY_RED)),
                ..Default::default()
            })
        ]
        .align_x(iced::Alignment::Center);

        let security_levels = column![
            text("SECURITY LEVEL")
                .size(10)
                .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
            row![
                make_sec_level("T1", true),
                make_sec_level("T2", false),
                make_sec_level("T3", false),
                make_sec_level("T4", false),
            ]
            .spacing(5)
        ]
        .spacing(5);

        let top_bar = row![
            customer_level,
            Space::with_width(Length::Fill),
            logo,
            Space::with_width(Length::Fill),
            security_levels,
        ]
        .align_y(iced::Alignment::Center)
        .padding([10, 20]);

        // --- 3. RED DIVIDER LINE ---
        let red_line = container(Space::new(Length::Fill, 1.5))
            .style(|_| container::Style {
                background: Some(iced::Background::Color(colors::COLOR_PRIMARY_RED)),
                ..Default::default()
            });

        // --- 4. MAIN CONTENT AREA ---
        // Left Column: Diamond Menu
        let left_col = column![
            container(
                text("COMPUTER SYSTEMS")
                    .size(12)
                    .font(FONT_MEDIUM)
                    .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) })
            )
            .padding([5, 15])
            .style(|_| container::Style {
                border: iced::Border {
                    color: colors::COLOR_PRIMARY_RED,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }),
            Space::with_height(20),
            container(
                diamond_menu(
                    vec![
                        DiamondMenuItem {
                            label: "VEHICLES".to_string(),
                            subtext: "161-9A".to_string(),
                            on_press: Message::MenuSelected(0),
                        },
                        DiamondMenuItem {
                            label: "LOCATIONS".to_string(),
                            subtext: "161-9A".to_string(),
                            on_press: Message::MenuSelected(1),
                        },
                        DiamondMenuItem {
                            label: "FACTIONS".to_string(),
                            subtext: "161-9A".to_string(),
                            on_press: Message::MenuSelected(2),
                        },
                        DiamondMenuItem {
                            label: "WEAPONS".to_string(),
                            subtext: "161-9A".to_string(),
                            on_press: Message::MenuSelected(3),
                        },
                        DiamondMenuItem {
                            label: "PRODUCTS".to_string(),
                            subtext: "161-9A".to_string(),
                            on_press: Message::MenuSelected(4),
                        },
                        DiamondMenuItem {
                            label: "CORPORATIONS".to_string(),
                            subtext: "161-9A".to_string(),
                            on_press: Message::MenuSelected(5),
                        },
                    ],
                    colors::COLOR_PRIMARY_RED,
                    colors::COLOR_BG,
                )
            )
            .width(Length::Fill)
            .height(Length::Fill)
        ]
        .width(Length::FillPortion(3))
        .align_x(iced::Alignment::Center);

        // Right Column: Info Panel Content
        let info_text_left = column![
            text("GO HOME")
                .size(24)
                .font(FONT_BOLD)
                .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
            Space::with_height(15),
            text("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.")
                .size(14)
                .font(FONT_REGULAR)
                .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
            Space::with_height(15),
            text("Quis ipsum suspendisse ultrices gravida. Risus commodo viverra maecenas accumsan lacus vel facilisis.")
                .size(14)
                .font(FONT_REGULAR)
                .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
            Space::with_height(Length::Fill),
            // Bottom logo in info panel
            row![
                text("M")
                    .size(54)
                    .font(FONT_BOLD)
                    .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
                Space::with_width(10),
                column![
                    text("PRECISION LIQUID")
                        .size(10)
                        .font(FONT_BOLD)
                        .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
                    text("POLYMER MUSCLE")
                        .size(10)
                        .font(FONT_BOLD)
                        .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) }),
                ]
            ]
            .align_y(iced::Alignment::Center)
        ]
        .spacing(10)
        .width(Length::Fill);

        // Decorative vertical text on the right of the InfoPanel
        let info_deco_right = row![
            canvas(VerticalText {
                text: "PETROCHEM".to_string(),
                color: colors::COLOR_PRIMARY_RED,
                size: 8.0,
                font: FONT_BOLD,
            })
            .width(Length::Fixed(12.0))
            .height(Length::Fill),
            canvas(VerticalText {
                text: "BETTERLIFE TEC".to_string(),
                color: colors::COLOR_PRIMARY_RED,
                size: 8.0,
                font: FONT_BOLD,
            })
            .width(Length::Fixed(12.0))
            .height(Length::Fill),
        ]
        .spacing(4)
        .height(Length::Fill);

        let info_content = row![
            info_text_left,
            Space::with_width(15),
            info_deco_right,
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        let right_col = column![
            container(
                text("DESCRIPTION")
                    .size(12)
                    .font(FONT_MEDIUM)
                    .style(|_| text::Style { color: Some(colors::COLOR_PRIMARY_RED) })
            )
            .padding([5, 15])
            .style(|_| container::Style {
                border: iced::Border {
                    color: colors::COLOR_PRIMARY_RED,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }),
            Space::with_height(20),
            info_panel(
                info_content,
                colors::COLOR_PRIMARY_RED,
                colors::COLOR_BG,
            )
        ]
        .width(Length::FillPortion(2))
        .align_x(iced::Alignment::Center);

        let main_area = row![
            left_col,
            Space::with_width(40),
            right_col,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(40);

        // --- 5. BOTTOM BAR ---
        let bottom_bar = row![
            Space::with_width(Length::Fill),
            container(
                row![
                    text("68SD1D1100D15")
                        .size(10)
                        .font(FONT_BOLD)
                        .style(|_| text::Style { color: Some(colors::COLOR_BG) }),
                    Space::with_width(15),
                    text("COMBAT COLONIZATION\nDEFENCE PROGRAM")
                        .size(8)
                        .font(FONT_MEDIUM)
                        .style(|_| text::Style { color: Some(colors::COLOR_BG) }),
                ]
                .align_y(iced::Alignment::Center)
            )
            .padding([8, 15])
            .style(|_| container::Style {
                background: Some(iced::Background::Color(colors::COLOR_PRIMARY_RED)),
                ..Default::default()
            })
        ]
        .padding([10, 20]);

        // --- 6. ASSEMBLE LAYOUT WITH EDGE DECORATIONS ---
        let main_dashboard = column![
            top_bar,
            red_line,
            main_area,
            bottom_bar,
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        // Far left vertical text
        let left_edge = canvas(VerticalText {
            text: "JHN 102 CKC 151 CC10 S111".to_string(),
            color: colors::COLOR_PRIMARY_RED,
            size: 8.0,
            font: FONT_ORBITRON_REGULAR,
        })
        .width(Length::Fixed(20.0))
        .height(Length::Fill);

        // Far right vertical texts (stacked)
        let right_edge = column![
            Space::with_height(Length::FillPortion(1)),
            container(
                canvas(VerticalText {
                    text: "JHN 102 CKC 151 CC10 S111".to_string(),
                    color: colors::COLOR_PRIMARY_RED,
                    size: 8.0,
                    font: FONT_ORBITRON_REGULAR,
                })
                .width(Length::Fill)
                .height(Length::Fill)
            )
            .width(Length::Fill)
            .height(Length::Fixed(200.0)),
            Space::with_height(20),
            container(
                canvas(VerticalText {
                    text: "KIROSHI".to_string(),
                    color: colors::COLOR_PRIMARY_RED,
                    size: 10.0,
                    font: FONT_BOLD,
                })
                .width(Length::Fill)
                .height(Length::Fill)
            )
            .width(Length::Fill)
            .height(Length::Fixed(100.0)),
            Space::with_height(Length::FillPortion(1)),
        ]
        .width(Length::Fixed(20.0))
        .height(Length::Fill);

        let screen_layout = row![
            left_edge,
            main_dashboard,
            right_edge,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([0, 10]);

        stack![
            bg_canvas,
            container(screen_layout)
                .width(Length::Fill)
                .height(Length::Fill)
        ]
        .into()
    }
}

// --- HELPERS ---

fn make_sec_level<'a>(level: &'a str, active: bool) -> Element<'a, Message> {
    let title = text("LEVEL")
        .size(8)
        .font(FONT_ORBITRON_REGULAR)
        .style(move |_| text::Style {
            color: Some(if active { colors::COLOR_BG } else { colors::COLOR_PRIMARY_RED }),
        });

    let val = text(level)
        .size(14)
        .font(FONT_ORBITRON_BOLD)
        .style(move |_| text::Style {
            color: Some(if active { colors::COLOR_BG } else { colors::COLOR_PRIMARY_RED }),
        });

    let content = column![title, val]
        .spacing(2)
        .align_x(iced::Alignment::Center);

    level_badge(
        content,
        colors::COLOR_PRIMARY_RED,
        colors::COLOR_BG,
        active,
    )
}

// --- CANVAS PROGRAMS ---

struct BackgroundProgram {
    glow_color: Color,
    bg_color: Color,
}

impl<Message> canvas::Program<Message> for BackgroundProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // Fill background with solid BG color
        frame.fill(&canvas::Path::rectangle(Point::ORIGIN, bounds.size()), self.bg_color);

        // Draw radial glow using concentric circles
        // Center of glow: 75% width, 40% height (behind InfoPanel)
        let center = Point::new(bounds.width * 0.75, bounds.height * 0.4);

        let max_radius = bounds.width.max(bounds.height) * 0.8;
        let steps = 60;
        let base_alpha = 0.012; // Very subtle glow

        for i in 0..steps {
            let t = i as f32 / steps as f32;
            let radius = max_radius * (1.0 - t);
            let alpha = base_alpha * (1.0 - t).powf(2.0); // Quadratic falloff

            let circle_color = Color {
                a: alpha,
                ..self.glow_color
            };

            let path = canvas::Path::circle(center, radius);
            frame.fill(&path, circle_color);
        }

        vec![frame.into_geometry()]
    }
}

struct VerticalText {
    text: String,
    color: Color,
    size: f32,
    font: iced::Font,
}

impl<Message> canvas::Program<Message> for VerticalText {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        frame.with_save(|frame| {
            // Translate to center of canvas
            frame.translate(Vector::new(bounds.width / 2.0, bounds.height / 2.0));
            // Rotate by -90 degrees (counter-clockwise)
            frame.rotate(-std::f32::consts::FRAC_PI_2);

            let txt = canvas::Text {
                content: self.text.clone(),
                position: Point::ORIGIN,
                color: self.color,
                size: self.size.into(),
                font: self.font,
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                ..Default::default()
            };
            frame.fill_text(txt);
        });

        vec![frame.into_geometry()]
    }
}
