use crate::audio_settings::{AudioSettings};
use crate::sound_player;
use crate::sound_player::{PlayState, PlayerMessage};
use crate::Message;
use iced::{button, text_input, Button, Column, Element, Row, Text, TextInput, HorizontalAlignment, VerticalAlignment};
use std::fmt::{Debug, Alignment};
use std::path::{Path};

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use iced::button::Style;

pub struct PlayButton {
    play_state: button::State,
    delete_state: button::State,
    sound: sound_player::Sound,
    pub player_handle_sender: Option<Sender<sound_player::PlayerMessage>>,
    player_handle_receiver: Option<Receiver<sound_player::PlayState>>,
    name: String,
}

#[derive(Debug, Clone)]
pub enum ButtonMessage {
    PlayButtonPressed(usize),
    AddButtonPressed,
    DeleteButtonPressed(usize),
    AddConfirmButtonPressed,
    PathOk(String),
    PathNotOk(String),
    NameChange(String),
}

pub struct PlayButtons {
    pub buttons: Vec<PlayButton>,
    pub settings: Option<Arc<Mutex<AudioSettings>>>,
    add_button: button::State,
    add_confirm_button: button::State,
    is_being_added: bool,
    path_input: text_input::State,
    name_input: text_input::State,
    button_row_len: usize,
    temp_path: String,
    temp_name: String,
    allow_confirm: bool,
}

impl Default for PlayButtons {
    fn default() -> Self {
        Self {
            buttons: vec![],
            settings: Default::default(),
            add_button: Default::default(),
            add_confirm_button: Default::default(),
            button_row_len: 6,
            is_being_added: false,
            path_input: Default::default(),
            temp_path: "".to_string(),
            temp_name: "".to_string(),
            allow_confirm: false,
            name_input: Default::default()
        }
    }
}

impl PlayButtons {
    pub fn update(&mut self, msg: ButtonMessage) {
        match msg {
            ButtonMessage::PlayButtonPressed(index) => {
                let btn = &mut self.buttons[index];

                //check for messages from players
                if let Some(tx) = &btn.player_handle_receiver {
                    let res = tx.try_recv();
                    match res {
                        Ok(msg) => match msg {
                            PlayState::Stopped => btn.sound.state = PlayState::Stopped,
                            PlayState::Playing => btn.sound.state = PlayState::Playing,
                        },
                        Err(_) => { /*TODO add error handling*/ }
                    }
                }

                match btn.sound.state {
                    PlayState::Playing => {
                        btn.player_handle_sender
                            .as_ref()
                            .unwrap()
                            .send(PlayerMessage::Stop); //unwrap because the handle must exist if the sound is playing
                    }
                    PlayState::Stopped => {
                        let (tx, rx) = btn.sound.play(self.settings.as_ref().unwrap().clone());
                        btn.player_handle_sender = Option::Some(tx);
                        btn.player_handle_receiver = Option::Some(rx);
                    }
                }
            }

            ButtonMessage::AddButtonPressed => {
                self.is_being_added = true;
            }

            ButtonMessage::AddConfirmButtonPressed => {
                self.buttons.push(PlayButton {
                    play_state: Default::default(),
                    delete_state: Default::default(),
                    sound: sound_player::Sound::new(self.temp_path.clone()),
                    player_handle_sender: None,
                    player_handle_receiver: None,
                    name: self.temp_name.to_owned(),
                });
                self.temp_path = "".to_string();
                self.is_being_added = false;
            }

            ButtonMessage::DeleteButtonPressed(index) => {
                if let Some(handle) = &self.buttons[index].player_handle_sender {
                    handle.send(PlayerMessage::Stop);
                }
                self.buttons.remove(index);
            }

            ButtonMessage::PathOk(val) => {
                self.temp_path = val;
                self.allow_confirm = true;
            }

            ButtonMessage::PathNotOk(val) => {
                self.temp_path = val;
                self.allow_confirm = false;
            }

            ButtonMessage::NameChange(name) =>{
                self.temp_name = name;
            }

        }
    }

    pub fn view(&mut self) -> Element<'_, Message> {
        if self.is_being_added {
            //TODO add winapi filepicker for Windows users
            let add_button = if self.allow_confirm {
                Button::new(&mut self.add_confirm_button, Text::new("confirm"))
                    .on_press(Message::PlayButtons(ButtonMessage::AddConfirmButtonPressed))
            } else {
                Button::new(&mut self.add_confirm_button, Text::new("confirm"))
            };

            Column::new()
                .push(TextInput::new(
                    &mut self.path_input,
                    "enter the filepath here",
                    &self.temp_path,
                    |val| {
                        if Path::exists(Path::new(val.as_str())) {
                            //TODO: add FileType check
                            Message::PlayButtons(ButtonMessage::PathOk(val))
                        } else {
                            Message::PlayButtons(ButtonMessage::PathNotOk(val))
                        }
                    },
                ))
                .push(TextInput::new(
                    &mut self.name_input,
                    "enter button name here",
                    &self.temp_name,
                    |val| Message::PlayButtons(ButtonMessage::NameChange(val))
                ))
                .push(add_button)
                .into()
        } else {
            self.view_button_list()
        }
    }

    fn view_button_list(&mut self) -> Element<'_, Message> {
        let mut children: Vec<Element<'_, _>> = vec![];
        let mut row_children: Vec<Element<'_, _>> = vec![];
        let button_width = 100;
        let button_height = 100;

        //add play buttons to temp slice
        for (index, button) in self.buttons.iter_mut().enumerate() {
            row_children.push(
                //add play + remove buttons
                Column::new()
                    .push(
                        Row::new()
                            .push(
                                Button::new(
                                    &mut button.play_state,
                                    Text::new(&button.name)
                                        .horizontal_alignment(HorizontalAlignment::Center)
                                        .vertical_alignment(VerticalAlignment::Center)
                                    )
                                    .width(button_width.into())
                                    .height(button_height.into())
                                    .on_press(Message::PlayButtons(
                                        ButtonMessage::PlayButtonPressed(index),
                                    )),
                            )
                            .push(
                                Button::new(
                                    &mut button.delete_state,
                                    Text::new("X")
                                        .horizontal_alignment(HorizontalAlignment::Center)
                                        .vertical_alignment(VerticalAlignment::Center)
                                        .size((button_height / 4) as u16)
                                )
                                    .min_height(button_height as u32)
                                    .min_width((button_width / 8) as u32)
                                    .on_press(
                                    Message::PlayButtons(ButtonMessage::DeleteButtonPressed(index)),
                                ),
                            ),
                    )
                    .into(),
            );
        }

        //add "add" button
        row_children.push(
            Button::new(
                &mut self.add_button,
                Text::new("add")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center)
                )
                .on_press(Message::PlayButtons(ButtonMessage::AddButtonPressed))
                .width(button_width.into())
                .height(button_height.into())
                .into(),
        );

        //calculate amount of rows to draw
        let row_amount = if row_children.len() < self.button_row_len && row_children.len() > 0 {
            1
        } else {
            row_children.len() / self.button_row_len
        };

        row_children.reverse();

        //move buttons to row's
        for _i in 0..row_amount + 1 {
            let mut added_buttons = 0;
            let mut temp_buttons: Vec<Element<'_, _>> = vec![];
            while added_buttons < self.button_row_len && 0 < row_children.len() {
                if let Some(x) = row_children.pop() {
                    temp_buttons.push(x);
                }
                added_buttons += 1;
            }

            children.push(
                Row::with_children(temp_buttons)
                    .spacing(10)
                    .padding(5)
                    .into()
            );
        }
        Column::with_children(children).into()
    }
}
