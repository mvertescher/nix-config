//! Developer preview of the catalog's actual styles, including states
//! that cannot all be held by one pointer at once. The last row uses
//! live controls. Run `cargo run --example control-states -- --era neomil`.
//! Kitsch and neokitsch intentionally show their unchanged rest coats:
//! their ghost/ring/veneer drawing is still pending.

use cp_eras_ui::{catalog, shell, Element, Era, Style};
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Border, Length};

fn main() -> iced::Result {
    let style = shell::style();
    shell::application(
        move || Preview { style, value: String::new(), clicks: 0 },
        Preview::update,
        Preview::view,
    )
    .title("Control states")
    .run()
}

struct Preview {
    style: Style,
    value: String,
    clicks: usize,
}

#[derive(Debug, Clone)]
enum Message {
    Input(String),
    Click,
    Reset,
    Ignore,
}

impl shell::Wears for Preview {
    fn wears(&self) -> Style { self.style }
}

impl Preview {
    fn update(&mut self, message: Message) {
        match message {
            Message::Input(value) => self.value = value,
            Message::Click => self.clicks += 1,
            Message::Reset => self.clicks = 0,
            Message::Ignore => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut examples = column![text("BUTTONS / FIELDS — REST, HOVER, PRESS / FOCUS, DISABLED").size(20)]
            .spacing(8);
        for era in Era::ALL {
            let style = era.style();
            let caption = match era {
                Era::Neomil | Era::Entropism => era.name().to_string(),
                _ => format!("{} — custom drawing pending", era.name()),
            };
            let mut states = row![].spacing(24);
            for (label, bs, fs) in [
                ("REST", button::Status::Active, text_input::Status::Active),
                ("HOVER", button::Status::Hovered, text_input::Status::Hovered),
                ("PRESS / FOCUS", button::Status::Pressed, text_input::Status::Focused { is_hovered: false }),
                ("DISABLED", button::Status::Disabled, text_input::Status::Disabled),
            ] {
                states = states.push(column![
                    text(label).size(14).color(style.palette.fg),
                    button(text("PRIMARY").size(14)).padding(4).width(Length::Fill)
                        .on_press(Message::Ignore).style(move |_, _| catalog::button::primary(&style, bs)),
                    button(text("GHOST").size(14)).padding(4).width(Length::Fill)
                        .on_press(Message::Ignore).style(move |_, _| catalog::button::ghost(&style, bs)),
                    text_input("Placeholder", "").size(14).padding(4)
                        .on_input(|_| Message::Ignore).style(move |_, _| catalog::field(&style, fs)),
                ].spacing(4).width(Length::Fill));
            }
            examples = examples.push(container(column![
                text(caption).size(16).color(style.palette.fg), states,
            ].spacing(4)).padding(8).width(Length::Fill).style(move |_| container::Style {
                background: Some(style.palette.bg.into()),
                border: Border { color: style.palette.border, width: 1.0, ..Border::default() },
                ..container::Style::default()
            }));
        }
        examples = examples.push(text(format!("LIVE — {} — {} clicks", self.style.era.name(), self.clicks)).size(20))
            .push(row![
                button("PRIMARY").padding(10).on_press(Message::Click).style(catalog::button::primary),
                button("RESET").padding(10).on_press(Message::Reset),
                text_input("Type here; Tab changes focus", &self.value).padding(10).on_input(Message::Input),
                button("DISABLED").padding(10),
            ].spacing(24));
        container(iced::widget::scrollable(examples)).padding(16).into()
    }
}
