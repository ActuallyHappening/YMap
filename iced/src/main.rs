use iced::{alignment::Horizontal, widget::{button, column, container, text}, Element, Length::Fill, Theme};

fn main() -> impl std::process::Termination {
	console_error_panic_hook::set_once();
	iced::application("YMap Iced frontend", State::update, State::view).theme(theme).run()
}

#[derive(Default)]
struct State {
	counter: u8,
}

#[derive(Debug, Clone)]
enum Message {
	Increment,
}

impl State {
	pub fn update(&mut self, message: Message) {
		match message {
			Message::Increment => self.counter += 1
		}
	}
}

impl State {
	pub fn view(&self) -> Element<Message> {
		let col = column![
			text("Hey from iced!"),
			text(self.counter),
			button("+").on_press(Message::Increment)
		].align_x(Horizontal::Center);
		container(col).padding(10).center_x(Fill).center_y(Fill).into()
	}
}

fn theme(_state: &State) -> Theme {
	Theme::TokyoNightStorm
}
