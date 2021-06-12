mod audio_settings;
mod play_buttons;
mod sound_player;

use iced::{
    Align, Column,
    Element, Sandbox, Settings,
};
use crate::audio_settings::{AudioSettingsMessage, AudioSettings};
use crate::play_buttons::{PlayButtons, ButtonMessage};

fn main() -> iced::Result{
    Example::run(Settings{
        antialiasing: true,
        ..Settings::default()
    })
}
#[derive(Default)]
pub struct Example{
    audio: AudioSettings,
    play_buttons: PlayButtons
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    AudioSettings(AudioSettingsMessage),
    PlayButtons(ButtonMessage)
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
            Message::PlayButtons(msg) => {
                PlayButtons::update(&mut self.play_buttons, msg)
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
       Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                self.audio.view()
            )
           .push(self.play_buttons.view())
           .into()
    }
}
