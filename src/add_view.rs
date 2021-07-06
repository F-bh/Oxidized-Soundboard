use crate::play_buttons::{ButtonMessage};
use crate::sound_player::{Sound};

use crate::{sound_player, Message, WindowSettings};
use iced::{
    button, text_input, Align, Button, Column, Element, Length, Row, Text, TextInput,
};
use std::path::{Path, PathBuf};

use std::sync::{Arc, Mutex};

#[derive(Default)]
pub(crate) struct AddView {
    pub(crate) video_settings: Arc<Mutex<WindowSettings>>,
    cancel_button: button::State,
    is_being_added: bool,
    allow_confirm: bool,
    add_confirm_button: button::State,
    path_output1: text_input::State,
    name_output1: text_input::State,
    temp_path: String,
    temp_name: String,
}

#[derive(Debug, Clone)]
pub(crate) enum AddViewMessage {
    CancelButtonPressed,
    PathOk(String),
    PathNotOk(String),
    NameChange(String),
    ButtonAdded(Sound, String, bool), //sound, name, ok
    FileDropped(PathBuf),
    AddPressed,
}


fn check_filetype(path: &str) -> bool{
     if Path::exists(Path::new(path)){
         if let Some(ending) = path.split(".").last() {
             if ["mp3","wav","ogg","flac"].contains(&ending){
                 return true
             }
         }
     }
     return false
}

impl AddView {

    pub fn update(&mut self, msg: AddViewMessage) -> Option<ButtonMessage> {
        let mut ret_val = None;
        match msg {
            AddViewMessage::ButtonAdded(sound, name, ok) => {
                if ok {
                    ret_val = Some(ButtonMessage::ButtonAdded(sound, name));
                    self.temp_path = "".to_string();
                    self.temp_name = "".to_string();
                    self.allow_confirm = false;
                    self.is_being_added = false;
                }
            }

            AddViewMessage::PathOk(val) => {
                self.temp_path = val;
                self.allow_confirm = true;
            }

            AddViewMessage::PathNotOk(val) => {
                self.temp_path = val;
                self.allow_confirm = false;
            }

            AddViewMessage::NameChange(name) => {
                self.temp_name = name;
            }

            AddViewMessage::CancelButtonPressed => {
                self.temp_path = "".to_string();
                self.temp_name = "".to_string();
                self.allow_confirm = false;
                self.is_being_added = false;
            }
            AddViewMessage::AddPressed => {
                self.is_being_added = true;
            }
            AddViewMessage::FileDropped(path_buf) => {
                if let Some(str) = path_buf.to_str(){
                    self.temp_path = String::from(str);
                    self.allow_confirm = check_filetype(str);
                    self.is_being_added = true;
                }
            }
        }
        ret_val
    }

    pub(crate) fn view(&mut self) -> Element<'_, Message> {
        let settings = self.video_settings.lock().unwrap();
        let (width, height) = (settings.width, settings.height);

        if self.is_being_added && width != 0 && height != 0 {
            if self.is_being_added {
                let add_button = if self.allow_confirm {
                    Button::new(&mut self.add_confirm_button, Text::new("confirm")).on_press(
                        Message::AddView(AddViewMessage::ButtonAdded(
                            sound_player::Sound::new(self.temp_path.clone()),
                            self.temp_name.to_owned(),
                            self.allow_confirm,
                        )),
                    )
                } else {
                    Button::new(&mut self.add_confirm_button, Text::new("please enter the path to a supported sound file"))
                };

                Column::new()
                    .push(
                        TextInput::new(
                            &mut self.path_output1,
                            "enter the filepath here",
                            &self.temp_path,
                            |val| {
                                if check_filetype(&val){
                                    Message::AddView(AddViewMessage::PathOk(val))
                                } else {
                                    Message::AddView(AddViewMessage::PathNotOk(val))
                                }
                            },
                        )
                        .width(Length::from(((width / 100) * 80) as u16))
                        .on_submit(Message::AddView(
                            AddViewMessage::ButtonAdded(
                                sound_player::Sound::new(self.temp_path.clone()),
                                self.temp_name.to_owned(),
                                self.allow_confirm,
                            ),
                        )),
                    )
                    .push(
                        TextInput::new(
                            &mut self.name_output1,
                            "enter button name here",
                            &self.temp_name,
                            |val| Message::AddView(AddViewMessage::NameChange(val)),
                        )
                        .width(Length::from(((width / 100) * 80) as u16))
                        .on_submit(Message::AddView(
                            AddViewMessage::ButtonAdded(
                                sound_player::Sound::new(self.temp_path.clone()),
                                self.temp_name.to_owned(),
                                self.allow_confirm,
                            ),
                        )),
                    )
                    .push(
                        Row::new().push(add_button).push(
                            Button::new(&mut self.cancel_button, Text::new("cancel"))
                                .on_press(Message::AddView(AddViewMessage::CancelButtonPressed)),
                        ),
                    )
                    .align_items(Align::Center)
                    .into()
            } else {
                Column::new().into()
            }
        } else {
            Column::new().into()
        }
    }
}
