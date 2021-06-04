mod audio_settings;
mod play_buttons;

use iced::{
    Align, Column,
    Element, Sandbox, Settings,
};
use crate::audio_settings::{AudioSettingsMessage, AudioSettings};

fn main() -> iced::Result{
    Example::run(Settings{
        antialiasing: true,
        ..Settings::default()
    })
}
#[derive(Default)]
pub struct Example{
    audio: AudioSettings
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    AudioSettings(AudioSettingsMessage)
}

impl Sandbox for Example{
    type Message = Message;

    fn new() -> Self {
        Example::default()
    }

    fn title(&self) -> String {
        String::from("Oxidized-Soundboard")
    }

    fn update(&mut self, message: Self::Message) {
        match message{
           Message::AudioSettings(msg) =>{
               AudioSettings::update(&mut self.audio, msg)
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
       Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                AudioSettings::view(&mut self.audio)
            )
           .into()
    }
}
