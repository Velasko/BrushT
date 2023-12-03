use iced::executor;
use iced::keyboard;
use iced::subscription::{self, Subscription};
use iced::theme;
use iced::widget::{
    self, button, column, container, horizontal_space, pick_list, row, text,
    text_input,
};
pub use iced::{Alignment, Application, Command, Element, Event, Length, Settings};

#[derive(Default)]
pub struct App {
}

#[derive(Debug, Clone)]
pub enum Message {
}

impl Application for App {
	type Executor = executor::Default;
	type Message = Message;
	type Theme = iced::Theme;
	type Flags = ();

	fn new(_flags: ()) -> (Self, Command<Message>) {
		(App{}, Command::none())
	}

	fn title(&self) -> String {
		String::from("BrusT - A shitty photoshop")
	}

	fn update(&mut self, message: Message) -> Command<Message> {
		Command::none()
	}

	fn view(&self) -> Element<Message> {
		let content = container(
			column![
				row![
					text("Top Left")
				]
			]
		);

		content.into()
	}
}
