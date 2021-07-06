mod add_view;
mod audio_settings;
mod play_buttons;
mod sound_player;

use crate::add_view::{AddView, AddViewMessage};
use crate::audio_settings::{AudioSettings, AudioSettingsMessage, AudioSettingsModel};
use crate::play_buttons::{ButtonMessage, PlayButtons, PlayButton};
use crate::sound_player::{PlayerMessage, Sound, PlayState};
use iced::{
    executor, scrollable, Align, Application, Clipboard, Column, Command, Element,
    Scrollable, Settings,
};
use iced_native::{Event, Subscription};
use std::sync::mpsc::{Sender};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use std::io::{Write, BufReader};
use std::ops::Deref;

use std::collections::HashMap;
use home::home_dir;

fn main() -> iced::Result {
    if cfg!(target_os = "windows"){
        Example::run(Settings {
            ..Settings::default()})
    }
    else{
         Example::run(Settings {
             antialiasing: true,
             ..Settings::default()})
    }
}

#[derive(Default)]
pub(crate) struct WindowSettings {
    pub(crate) height: usize,
    pub(crate) width: usize,
}

#[derive(Default)]
pub(crate) struct Example {
    scroll_state: scrollable::State,
    audio_model: AudioSettingsModel,
    play_buttons: PlayButtons,
    add_view: AddView,
    audio_settings: Arc<Mutex<AudioSettings>>,
    window_settings: Arc<Mutex<WindowSettings>>,
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Save,
    AudioSettings(AudioSettingsMessage),
    PlayButtons(ButtonMessage),
    AddView(AddViewMessage),
    WindowResized(usize, usize),
    AudioSettingsOutDev1Selected(String),//not an elegant solution
    AudioSettingsOutDev2Selected(String), //not an elegant solution
}

#[derive(Serialize, Deserialize)]
struct SaveSettings{
    audio: AudioSettings,
    sound_paths: HashMap<String,String>,
}

fn save(settings: &SaveSettings){
    #[cfg(target_os = "linux")]
        let file_name = "/.oxidized_soundboard";
    #[cfg(target_os = "windows")]
        let file_name = "\\oxidized_soundboard.yaml";
    if let Some(dir) = home_dir() {
        if let Some(dir) = dir.to_str() {
            if let Ok(mut file) = std::fs::File::create(String::from(dir) + file_name) {
                if let Ok(yaml) = serde_yaml::to_string(settings) {
                    file.write(yaml.as_bytes());
                }
            }
        }
    }
}

fn load_save() -> Option<SaveSettings>{
    #[cfg(target_os = "linux")]
        let file_name = "/.oxidized_soundboard";
    #[cfg(target_os = "windows")]
        let file_name = "\\oxidized_soundboard.yaml";

    let yaml = std::fs::File::open(String::from(String::from(home_dir()?.to_str()?)) + file_name).ok()?;
    let reader = BufReader::new(yaml);
    let mut settings: SaveSettings = serde_yaml::from_reader(reader).ok()?;
    let mut temp: HashMap<String, String> = HashMap::new();
    for (k,v) in settings.sound_paths.iter().filter(
        |path|
            crate::add_view::check_filetype(path.1)
    ){
          temp.insert(k.clone(),v.clone());
    }
    settings.sound_paths = temp;
    Some(settings)
}


impl Application for Example {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut app = Example::default();
        //load settings
        if let Some(settings) = load_save(){
            app.audio_model.audio_settings = Arc::new(Mutex::new(settings.audio));
            for (name, path) in settings.sound_paths.iter(){
                app.play_buttons.buttons.push(PlayButton{
                    player_handle_sender: None,
                    player_handle_receiver: None,
                    play_state: Default::default(),
                    delete_state: Default::default(),
                    sound: Sound {
                        file_path: path.clone(),
                        state: PlayState::Stopped
                    },
                    name: name.clone()
                })
            }
        }

        //enable memory sharing between components
        app.audio_settings = app.audio_model.audio_settings.clone();
        app.play_buttons.audio_settings = app.audio_settings.clone();
        app.play_buttons.video_settings = app.window_settings.clone();
        app.audio_model.video_settings = app.window_settings.clone();
        app.add_view.video_settings = app.window_settings.clone();
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Oxidized-Soundboard")
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message> {
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

            Message::AddView(msg) => {
                let btn_msg = AddView::update(&mut self.add_view, msg);
                if let Some(msg) = btn_msg {
                    PlayButtons::update(&mut self.play_buttons, msg);
                }
            }

            Message::PlayButtons(msg) => PlayButtons::update(&mut self.play_buttons, msg),

            Message::WindowResized(width, height) => {
                let mut settings = self.window_settings.lock().unwrap();
                settings.width = width;
                settings.height = height;
            }

            Message::AudioSettingsOutDev2Selected(name) => {
                let mut player_update_channels: Vec<Sender<PlayerMessage>> = vec![];

                for btn in &self.play_buttons.buttons {
                    if let Some(tx) = btn.player_handle_sender.clone() {
                        player_update_channels.push(tx);
                    }
                }
                AudioSettingsModel::update(&mut self.audio_model, AudioSettingsMessage::OutDev2Selected(name), player_update_channels);
            }

            Message::AudioSettingsOutDev1Selected(name) => {
                let mut player_update_channels: Vec<Sender<PlayerMessage>> = vec![];

                for btn in &self.play_buttons.buttons {
                    if let Some(tx) = btn.player_handle_sender.clone() {
                        player_update_channels.push(tx);
                    }
                }
                AudioSettingsModel::update(&mut self.audio_model, AudioSettingsMessage::OutDev1Selected(name), player_update_channels);
            }

            Message::Save => {
                //save current settings and buttons
                let mut sound_paths: HashMap<String, String> = Default::default();
                for btn in  self.play_buttons.buttons.iter(){
                sound_paths.insert(btn.name.clone(),btn.sound.file_path.clone());
                }
                let audio = self.audio_settings.clone().lock().unwrap().deref().clone();
                save(&SaveSettings{
                audio ,
                sound_paths,
                });
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events_with(|event, _status| match event {
            Event::Window(event) => {
                match event {
                    iced_native::window::Event::Resized { width, height } => {
                        Some(Message::WindowResized(width as usize, height as usize))
                    }
                    iced_native::window::Event::FileDropped(path_buf) => {
                        Some(Message::AddView(AddViewMessage::FileDropped(path_buf)))
                    }
                    _ => None,
                }
            }
            _ => None,
        })
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Scrollable::new(&mut self.scroll_state)
            .push(
                Column::new()
                    .padding(20)
                    .align_items(Align::Center)
                    .push(self.audio_model.view())
                    .push(self.play_buttons.view())
                    .push(self.add_view.view()),
            )
            .into()
    }
}
