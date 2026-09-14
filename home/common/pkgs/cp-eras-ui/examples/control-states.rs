//! Developer preview of the catalog's actual styles, including states
//! that cannot all be held by one pointer at once. The last row uses
//! live controls. Run `cargo run --example control-states -- --era neomil`.
//! Native controls retain input state beneath custom material backdrops.

use cp_eras_ui::{shell, Element, Era, Style};
use cp_eras_ui::widgets::controls::{self, Kind, State};
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
            let caption = era.name();
            let mut states = row![].spacing(24);
            for (label, state) in [
                ("REST", State::Active), ("HOVER", State::Hovered),
                ("PRESS / FOCUS COAT", State::Pressed), ("DISABLED", State::Disabled),
            ] {
                states = states.push(column![
                    text(label).size(14).color(style.palette.fg),
                    controls::button(style, Kind::Primary,
                        button(text("PRIMARY").size(14)).padding(10).width(Length::Fill),
                        (state != State::Disabled).then_some(Message::Ignore)).preview(state),
                    controls::button(style, Kind::Ghost,
                        button(text("GHOST").size(14)).padding(10).width(Length::Fill),
                        (state != State::Disabled).then_some(Message::Ignore)).preview(state),
                    controls::field(style, text_input("Placeholder", "").size(14).padding(10),
                        (state != State::Disabled).then_some(|_| Message::Ignore)).preview(state),
                ].spacing(24).width(Length::Fill));
            }
            examples = examples.push(container(column![
                text(caption).size(16).color(style.palette.fg), states,
                text(format!("LIVE — {} clicks", self.clicks)).size(16),
                row![
                    controls::button(style, Kind::Primary, button("PRIMARY").padding(10), Some(Message::Click)),
                    controls::button(style, Kind::Ghost, button("RESET").padding(10), Some(Message::Reset)),
                    controls::field(style, text_input("Type here; Tab changes focus", &self.value).padding(10), Some(Message::Input)),
                    controls::button(style, Kind::Primary, button("DISABLED").padding(10), None),
                ].spacing(24),
            ].spacing(26)).padding(24).width(Length::Fill).style(move |_| container::Style {
                background: Some(style.palette.bg.into()),
                border: Border { color: style.palette.border, width: 1.0, ..Border::default() },
                ..container::Style::default()
            }));
        }
        container(iced::widget::scrollable(examples)).padding(16).into()
    }
}
