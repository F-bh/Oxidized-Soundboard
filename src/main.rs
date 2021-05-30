use iced::{
    button, Align, Button, Column, Element, Length, Sandbox, Settings, Text
};

fn main() -> iced::Result{
    Example::run(Settings{
        antialiasing: true,
        ..Settings::default()
    })
}

struct Example{
    button_state: button::State,
    button_text: String,
}

enum Message{
    Proggers
}

impl Sandbox for Example{
    type Message = Message;

    fn new() -> Self {
        Example::default()
    }

    fn title(&self) -> String {
        String::from("Hello World")
    }

    fn update(&mut self, message: Self::Message) {
        self.button_text = String::from("KEKW");
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.button_state, &self.button_text)
                    .on_press(Message::Proggers),
            )
            .into()
    }
}