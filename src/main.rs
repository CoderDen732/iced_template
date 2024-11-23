use iced::widget::{button, container, text};
use iced::{alignment, Element, Length, Theme};

pub fn main() -> iced::Result {
    iced::application("A cool counter Iced template", update, view)
    .theme(theme)
    .run()
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
    }
}

fn view(counter: &Counter) -> Element<Message> {
    
    container(
        button(text(counter.value)).on_press(Message::Increment)
    ).height(Length::Fill)
	.width(Length::Fill)
	.align_x(alignment::Horizontal::Center)
	.align_y(alignment::Vertical::Center)
	.into()
}

fn theme(_counter: &Counter) -> Theme {
    Theme::TokyoNight
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct Counter {
    value: u64,
}