mod audio_settings;
mod play_buttons;
mod sound_player;

use crate::audio_settings::{AudioSettings, AudioSettingsMessage, AudioSettingsModel};
use crate::play_buttons::{ButtonMessage, PlayButtons};
use crate::sound_player::PlayerMessage;
use iced::{Align, Column, Element, Sandbox, Settings, Application, Clipboard, Executor, executor, Command, Scrollable, scrollable};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use iced_native::{Subscription, Event};

fn main() -> iced::Result {
    Example::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Default)]
pub struct WindowSettings{
    pub height: usize,
    pub width: usize,
}

#[derive(Default)]
pub struct Example {
    scroll_state: scrollable::State,
    audio_model: AudioSettingsModel,
    play_buttons: PlayButtons,
    audio_settings: Arc<Mutex<AudioSettings>>,
    window_settings: Arc<Mutex<WindowSettings>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    AudioSettings(AudioSettingsMessage),
    PlayButtons(ButtonMessage),
    WindowResized(usize, usize)
}

impl Application for Example {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut app = Example::default();
        app.audio_settings = app.audio_model.audio_settings.clone();
        app.play_buttons.audio_settings = app.audio_settings.clone();
        app.play_buttons.video_settings = app.window_settings.clone();
        app.audio_model.video_settings = app.window_settings.clone();
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Oxidized-Soundboard")
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message>{
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

            Message::WindowResized(width, height) =>{
                let mut settings =  self.window_settings.lock().unwrap();
                settings.width = width;
                settings.height = height;
            }
        }
        Command::none()
    }

    //TODO Sounddateien per drag and drop hinzufügen
    fn subscription(& self) -> Subscription<Message>{
        iced_native::subscription::events_with(
            |event, _status |
            match event{
                Event::Window(event) => {
                    match event {
                        iced_native::window::Event::Resized { width, height } => {
                            Some(Message::WindowResized(width as usize, height as usize))
                        }
                        //Event::FileHovered(_) => {}
                        //Event::FileDropped(_) => {}
                        //Event::FilesHoveredLeft => {}
                        _ => None
                    }
                }
                _ => None
            }
        )
    }


    fn view(&mut self) -> Element<'_, Self::Message> {
        Scrollable::new(&mut self.scroll_state)
            .push(
                Column::new()
                    .padding(20)
                    .align_items(Align::Center)
                    .push(self.audio_model.view())
                    .push(self.play_buttons.view())
            ).into()
    }

}
