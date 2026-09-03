//! The working mail client, in any era.
//!
//! [`crate::screens::mail`] is the *design target*: the mailbox exactly
//! as the four reference sets draw it, display-only, and golden-tested
//! in every era. This is the other half -- the same screen with
//! selection, focus, scrolling and deletion wired up, which is what the
//! `cp-eras-ui-mail` binary runs.
//!
//! It predates the era generalisation and used to be a neo-militarism
//! app: `crate::colors` at every call site, the hardcoded `top_bar`,
//! the glow `background`, and the chamfered `text_box`. It now takes a
//! [`Style`] like everything else and draws from the shared vocabulary,
//! so `--era` re-dresses it and a desktop `switch` is enough to move it.
//! Nothing below branches on era.

use crate::palette::Palette;
use crate::style::{Chrome, Metrics, Style};
use crate::widgets::surface::{backdrop, surface, Surface};
use crate::widgets::{footer, ground, text, top_bar};
use iced::widget::{column, container, mouse_area, row, scrollable, stack, Space};
use iced::{Alignment, Color, Element, Length, Padding};

/// One reply in a thread.
#[derive(Debug, Clone)]
pub struct ThreadMessage {
    pub sender: String,
    pub body: String,
    pub timestamp: String,
}

/// A message, with the replies hanging off it.
#[derive(Debug, Clone)]
pub struct Email {
    pub id: usize,
    pub title: String,
    pub sender: String,
    pub body: String,
    pub is_new: bool,
    pub timestamp: String,
    pub thread: Vec<ThreadMessage>,
}

/// Which pane has the keyboard. The unfocused one recedes rather than
/// disappearing, which is the treatment every era's references use for
/// an inactive panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MailFocus {
    List,
    Content,
}

/// How far an unfocused pane fades.
const UNFOCUSED: f32 = 0.35;

const ACTIONS: [&str; 3] = ["REPLY", "ARCHIVE", "CLOSE"];

/// The mail panel: list on the left, thread on the right.
pub fn mail_panel<'a, Message: 'static + Clone>(
    style: &Style,
    emails: &'a [Email],
    selected_id: Option<usize>,
    on_select: impl Fn(usize) -> Message + Clone + 'static,
    on_delete: impl Fn(usize) -> Message + Clone + 'static,
    list_scrollable_id: iced::widget::Id,
    content_scrollable_id: iced::widget::Id,
    focus: MailFocus,
) -> Element<'a, Message> {
    let s = style;
    let fade = |c: Color, focused: bool| {
        if focused {
            c
        } else {
            Palette::faded(c, UNFOCUSED)
        }
    };
    let list_ink = fade(s.palette.fg, focus == MailFocus::List);
    let list_line = fade(s.palette.border, focus == MailFocus::List);
    let content_ink = fade(s.palette.fg, focus == MailFocus::Content);
    let content_line = fade(s.palette.border, focus == MailFocus::Content);

    // --- Left: the message list ---
    let mut list = column![].spacing(s.metrics.gap * LIST_GAP_FACTOR).width(Length::Fill);
    for email in emails {
        list = list.push(message_row(
            s,
            email,
            Some(email.id) == selected_id,
            (on_select.clone())(email.id),
            list_ink,
            list_line,
        ));
    }

    let left = column![
        pane_heading(s, "MESSAGES", list_ink, list_line),
        Space::new().height(s.metrics.gap),
        scrollable(container(list).padding(Padding {
            top: 0.0,
            right: 14.0,
            bottom: 0.0,
            left: 0.0,
        }))
        .id(list_scrollable_id)
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::new()
                .width(4.0)
                .scroller_width(4.0)
                .margin(5.0),
        ))
        .style(rail(list_line))
        .height(Length::Fill)
        .width(Length::Fill),
    ]
    .width(Length::FillPortion(4))
    .height(Length::Fill);

    // --- Right: the selected thread ---
    let selected = selected_id.and_then(|id| emails.iter().find(|e| e.id == id));

    let body: Element<'a, Message> = match selected {
        Some(email) => column![
            container(surface(
                Surface::outlined(s).stroke(content_line),
                s.metrics.pad,
                scrollable(thread(s, email, content_ink))
                    .id(content_scrollable_id)
                    .direction(scrollable::Direction::Vertical(
                        scrollable::Scrollbar::new()
                            .width(4.0)
                            .scroller_width(4.0)
                            .margin(5.0),
                    ))
                    .style(rail(content_line))
                    .height(Length::Fill),
            ))
            .height(Length::Fill),
            Space::new().height(s.metrics.gap),
            actions(s, (on_delete.clone())(email.id)),
        ]
        .into(),
        // The empty state is a well, not a panel: nothing is loaded, so
        // nothing is raised.
        None => container(surface(
            Surface::outlined(s).stroke(Palette::faded(content_line, 0.4)),
            s.metrics.pad,
            container(
                text::body(s, "SELECT A MESSAGE TO VIEW CONTENT")
                    .color(Palette::faded(content_ink, 0.5)),
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill),
        ))
        .height(Length::Fill)
        .into(),
    };

    let right = column![
        pane_heading(s, "CONTENT", content_ink, content_line),
        Space::new().height(s.metrics.gap),
        body,
    ]
    .width(Length::FillPortion(6))
    .height(Length::Fill);

    let screen = column![
        top_bar(
            s,
            ["PERSONAL LINK SOFTWAREV2", "MAIL BOX", "FLAIR TRS 5MMP"],
        ),
        row![
            left,
            Space::new().width(s.metrics.gap * 2.0),
            right,
        ]
        .height(Length::Fill),
        footer(
            s,
            "INTERFACE LOADED",
            "PROVIDED BY NEXUS NETWORK V10.8",
            "BUILD 6.47.48441.R15",
        ),
    ]
    .spacing(s.metrics.gap);

    stack![ground(s), container(screen).padding(40)].into()
}

/// The height the message-list scrollable is laid out at inside a window
/// of `window_height`, reconstructed from this panel's own chrome.
///
/// The list scrollable is `Length::Fill` (above), so iced gives it exactly
/// what the fixed chrome leaves: the 40px screen padding top and bottom,
/// the era's top bar and footer (fixed heights from
/// [`crate::widgets::top_bar`] / [`crate::widgets::footer`], both in
/// `widgets/chrome.rs`), the MESSAGES pane heading ([`pane_heading`]), and
/// the three `metrics.gap` steps between them. Text lines count as
/// `size * 1.3`, iced's default relative line height.
///
/// Single source for the `cp-eras-ui-mail` example's `scroll_to_selected`
/// clamp, so it lives next to the layout it summarises: if the chrome
/// above or below the list changes, this function is the number that must
/// move with it. [`message_row_pitch`] is the matching single source for
/// the row pitch.
pub fn mail_list_viewport_height(style: &Style, window_height: f32) -> f32 {
    let m = &style.metrics;
    let line = |size: u16| f32::from(size) * 1.3;
    let top_bar = match style.chrome {
        Chrome::Segmented => 28.0,
        Chrome::Tape => 24.0,
        Chrome::Caption | Chrome::DeviceFrame => line(m.text_caption),
    };
    let footer = match style.chrome {
        Chrome::Segmented => 1.0 + 16.0 + line(m.text_body),
        Chrome::Tape => 16.0 + line(m.text_body),
        Chrome::Caption => line(m.text_caption + 2),
        Chrome::DeviceFrame => 30.0 + 8.0 + line(m.text_body),
    };
    // `pane_heading`: its 5px vertical padding twice, plus a body line.
    let heading = 10.0 + line(m.text_body);
    window_height - 80.0 - top_bar - footer - heading - 3.0 * m.gap
}

/// The boxed caption that heads each pane.
fn pane_heading<'a, Message: 'static>(
    style: &Style,
    label: &'a str,
    ink: Color,
    line: Color,
) -> Element<'a, Message> {
    row![backdrop(
        Surface::outlined(style).stroke(line),
        Padding::from([5, 15]),
        text::body(style, label).color(ink),
    )]
    .into()
}

/// One message row is a fixed-height card ([`message_row`] pins it) and
/// the list column spaces sibling rows by half the era's gap. Both
/// builders read these same numbers, so anything that maps a message
/// index to a y-offset cannot drift from the panel.
pub const MESSAGE_ROW_HEIGHT: f32 = 52.0;

/// Fraction of the era's gap that sits between two message rows.
const LIST_GAP_FACTOR: f32 = 0.5;

/// Vertical pitch of the message list: one fixed-height row plus the
/// half-gap below it. Single source for scroll geometry -- the mail
/// example's `scroll_to_selected` -- alongside
/// [`mail_list_viewport_height`].
pub fn message_row_pitch(metrics: &Metrics) -> f32 {
    MESSAGE_ROW_HEIGHT + metrics.gap * LIST_GAP_FACTOR
}

/// One row of the list. Selection is the era's own idiom, so a
/// neokitsch row is veneered and a kitsch one is yellow without this
/// function knowing either.
fn message_row<'a, Message: 'static + Clone>(
    style: &Style,
    email: &'a Email,
    selected: bool,
    on_press: Message,
    ink: Color,
    line: Color,
) -> Element<'a, Message> {
    let s = style;

    let bg = if selected {
        Surface::selected(s)
    } else {
        Surface::outlined(s).stroke(line)
    };

    let (title_ink, meta_ink) = if selected {
        (s.palette.on_select, s.palette.on_select)
    } else {
        (ink, Palette::faded(ink, 0.6))
    };

    let flag: Element<'a, Message> = if email.is_new {
        text::caption(s, "NEW").color(title_ink).into()
    } else {
        Space::new().into()
    };

    let content = row![
        column![
            text::body(s, email.title.as_str()).color(title_ink),
            row![
                text::caption(s, email.sender.as_str()).color(meta_ink),
                Space::new().width(Length::Fill).height(Length::Shrink),
                text::caption(s, email.timestamp.as_str()).color(meta_ink),
            ]
            .width(Length::Fill),
        ]
        .spacing(2)
        .width(Length::Fill),
        Space::new().width(8.0),
        flag,
    ]
    .align_y(Alignment::Center);

    mouse_area(
        container(surface(bg, Padding::from([6, 10]), content))
            .width(Length::Fill)
            .height(Length::Fixed(MESSAGE_ROW_HEIGHT)),
    )
    .on_press(on_press)
    .into()
}

/// The action row. The destructive one is the only filled control, in
/// the era's alert colour rather than its selection colour -- the same
/// distinction `screens::mail` draws, and the reason `alert` and
/// `select` are separate roles.
fn actions<'a, Message: 'static + Clone>(
    style: &Style,
    on_delete: Message,
) -> Element<'a, Message> {
    let s = style;
    let mut bar = row![].spacing(s.metrics.gap * 0.5).height(Length::Fixed(34.0));

    bar = bar.push(
        mouse_area(
            container(surface(
                Surface::filled(s, s.palette.alert).no_stroke(),
                Padding::from([5, 8]),
                container(text::on_select(s, "DELETE")).center_x(Length::Fill),
            ))
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .on_press(on_delete),
    );

    for label in ACTIONS {
        bar = bar.push(
            container(surface(
                Surface::outlined(s),
                Padding::from([5, 8]),
                container(text::body(s, label)).center_x(Length::Fill),
            ))
            .width(Length::Fill)
            .height(Length::Fill),
        );
    }

    bar.into()
}

/// The root message and its replies.
fn thread<'a, Message: 'static>(
    style: &Style,
    email: &'a Email,
    ink: Color,
) -> Element<'a, Message> {
    let s = style;
    let quiet = Palette::faded(ink, 0.55);

    let head = |sender: &'a str, stamp: &'a str| {
        row![
            text::body(s, "FROM: ").color(ink),
            text::body(s, sender).color(ink),
            Space::new().width(Length::Fill).height(Length::Shrink),
            text::caption(s, stamp).color(quiet),
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill)
    };

    let mut col = column![].spacing(s.metrics.gap).width(Length::Fill);

    col = col.push(
        column![
            text::title(s, email.title.as_str())
                .size(f32::from(s.metrics.text_title - 3))
                .color(ink),
            head(email.sender.as_str(), email.timestamp.as_str()),
            Space::new().height(8.0),
            markdown(s, email.body.as_str(), ink),
        ]
        .width(Length::Fill),
    );

    for reply in &email.thread {
        let card = container(surface(
            Surface::outlined(s).stroke(Palette::faded(ink, 0.25)),
            s.metrics.pad * 0.75,
            column![
                head(reply.sender.as_str(), reply.timestamp.as_str()),
                Space::new().height(8.0),
                markdown(s, reply.body.as_str(), ink),
            ]
            .width(Length::Fill),
        ))
        .width(Length::Fill);

        // Replies are indented, the way a quoted chain is in every
        // reference set.
        col = col.push(row![Space::new().width(s.metrics.gap), card].width(Length::Fill));
    }

    col.into()
}

// --- A very small markdown subset ---
//
// The reference mail bodies are plain paragraphs, bullet lists and
// pipe tables. Nothing else is recognised, and anything unrecognised
// falls through as a paragraph rather than erroring.

fn markdown<'a, Message: 'static>(
    style: &Style,
    source: &'a str,
    ink: Color,
) -> Element<'a, Message> {
    let mut col = column![].spacing(12).width(Length::Fill);

    for block in source.split("\n\n") {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        if is_table(block) {
            col = col.push(table(style, block, ink));
        } else if is_list(block) {
            col = col.push(list(style, block, ink));
        } else {
            col = col.push(text::body(style, block).color(ink));
        }
    }

    col.into()
}

fn is_table(block: &str) -> bool {
    let lines: Vec<&str> = block.lines().map(str::trim).collect();
    if lines.len() < 2 {
        return false;
    }
    let sep = lines[1];
    let has_separator = sep.contains('|')
        && sep
            .chars()
            .all(|c| c == '|' || c == '-' || c == ':' || c == '+' || c.is_whitespace());
    lines[0].contains('|') && has_separator
}

fn is_list(block: &str) -> bool {
    block.lines().any(|l| {
        let t = l.trim_start();
        t.starts_with("* ") || t.starts_with("- ") || t.starts_with("• ")
    })
}

fn cells(line: &str) -> Vec<&str> {
    line.split('|')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect()
}

fn table<'a, Message: 'static>(
    style: &Style,
    block: &'a str,
    ink: Color,
) -> Element<'a, Message> {
    let s = style;
    let mut grid = column![].width(Length::Fill);
    let lines: Vec<&str> = block.lines().map(str::trim).collect();

    let headers = match lines.first() {
        Some(first) => cells(first),
        None => return grid.into(),
    };
    let columns = headers.len();
    if columns == 0 {
        return grid.into();
    }

    // A wash of the era's own ink, not `palette.emphasis` (the whole
    // band). That band
    // is kitsch's mint stat strip and degrades to `panel` -- which is
    // the desktop's *bar* colour, and lands a blue-grey header row in
    // the middle of a three-red era. Seen in a neomil render.
    let band = Palette::faded(ink, 0.14);
    let mut head = row![].width(Length::Fill);
    for header in headers {
        head = head.push(
            container(surface(
                Surface::filled(s, band).stroke(Palette::faded(ink, 0.35)),
                Padding::from([5, 6]),
                text::caption(s, header)
                    .size(f32::from(s.metrics.text_caption + 2))
                    .color(ink),
            ))
            .width(Length::FillPortion(1))
            .height(Length::Fixed(26.0)),
        );
    }
    grid = grid.push(head);

    for line in lines.iter().skip(2) {
        let values = cells(line);
        if values.is_empty() {
            continue;
        }
        let mut data = row![].width(Length::Fill);
        for i in 0..columns {
            data = data.push(
                container(surface(
                    Surface::outlined(s).stroke(Palette::faded(ink, 0.15)),
                    Padding::from([5, 6]),
                    text::caption(s, values.get(i).copied().unwrap_or(""))
                        .size(f32::from(s.metrics.text_caption + 2))
                        .color(ink),
                ))
                .width(Length::FillPortion(1))
                .height(Length::Fixed(26.0)),
            );
        }
        grid = grid.push(data);
    }

    grid.into()
}

fn list<'a, Message: 'static>(
    style: &Style,
    block: &'a str,
    ink: Color,
) -> Element<'a, Message> {
    let mut col = column![].spacing(6).width(Length::Fill);

    for line in block.lines() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        let content = trimmed
            .strip_prefix("* ")
            .or_else(|| trimmed.strip_prefix("- "))
            .or_else(|| trimmed.strip_prefix("• "))
            .unwrap_or(trimmed);

        col = col.push(
            row![
                text::body(style, "•").color(ink),
                Space::new().width(8.0),
                text::body(style, content).color(ink),
            ]
            .align_y(Alignment::Center),
        );
    }

    col.into()
}

/// Scrollbar styling in the era's line colour. A closure factory rather
/// than a widget: `scrollable`'s style takes an `Fn`, and the colour has
/// to be copied out before it crosses into one.
fn rail(
    line: Color,
) -> impl Fn(&iced::Theme, scrollable::Status) -> scrollable::Style {
    // `auto_scroll` is iced 0.14's middle-click autoscroll overlay and
    // has no era reading, so it is left at whatever the built-in theme
    // draws rather than invented here; everything this crate has an
    // opinion about is still written out.
    move |theme, status| scrollable::Style {
        container: container::Style::default(),
        vertical_rail: scrollable::Rail {
            background: Some(Palette::faded(line, 0.05).into()),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            scroller: scrollable::Scroller {
                background: Palette::faded(line, 0.5).into(),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 2.0.into(),
                },
            },
        },
        horizontal_rail: scrollable::Rail {
            background: None,
            border: iced::Border::default(),
            scroller: scrollable::Scroller {
                background: Color::TRANSPARENT.into(),
                border: iced::Border::default(),
            },
        },
        gap: None,
        ..scrollable::default(theme, status)
    }
}
