mod audio_settings;
mod play_buttons;
mod sound_player;

use crate::audio_settings::{AudioSettings, AudioSettingsMessage, AudioSettingsModel};
use crate::play_buttons::{ButtonMessage, PlayButtons};
use crate::sound_player::PlayerMessage;
use iced::{Align, Column, Element, Sandbox, Settings};

use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

fn main() -> iced::Result {
    Example::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Default)]
pub struct Example {
    audio_model: AudioSettingsModel,
    play_buttons: PlayButtons,
    settings: Arc<Mutex<AudioSettings>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    AudioSettings(AudioSettingsMessage),
    PlayButtons(ButtonMessage),
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        let mut app = Example::default();
        app.settings = app.audio_model.settings.clone();
        app.play_buttons.settings = Some(app.settings.clone());
        app
    }

    fn title(&self) -> String {
        String::from("Oxidized-Soundboard")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::AudioSettings(msg) => {
                let mut player_update_channels: Vec<Sender<PlayerMessage>> = vec![];

                for btn in &self.play_buttons.buttons {
                    if let Some(tx) = btn.player_handle_sender.clone() {
                        player_update_channels.push(tx);
                    }
                }

                AudioSettingsModel::update(&mut self.audio_model, msg, player_update_channels)
            }
            Message::PlayButtons(msg) => PlayButtons::update(&mut self.play_buttons, msg),
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(self.audio_model.view())
            .push(self.play_buttons.view())
            .into()
    }
}
