use crate::audio_settings::{AudioSettings};
use crate::{sound_player, WindowSettings};
use crate::sound_player::{PlayState, PlayerMessage};
use crate::Message;
use iced::{button, text_input, Button, Column, Element, Row, Text, TextInput, HorizontalAlignment, VerticalAlignment, Length, Align};
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
    AddConfirmButtonPressed(bool), //ok || not ok
    CancelButtonPressed,
    PathOk(String),
    PathNotOk(String),
    NameChange(String),
}

pub struct PlayButtons {
    pub buttons: Vec<PlayButton>,
    pub audio_settings: Arc<Mutex<AudioSettings>>,
    pub video_settings: Arc<Mutex<WindowSettings>>,
    add_button: button::State,
    cancel_button: button::State,
    is_being_added: bool,
    button_row_len: usize,
    allow_confirm: bool,
    add_confirm_button: button::State,
    path_input: text_input::State,
    name_input: text_input::State,
    temp_path: String,
    temp_name: String,
}


impl Default for PlayButtons {
    fn default() -> Self {
        Self {
            buttons: vec![],
            audio_settings: Default::default(),
            video_settings: Default::default(),
            add_button: Default::default(),
            button_row_len: 5,
            is_being_added: false,
            allow_confirm: false,
            path_input: Default::default(),
            temp_path: "".to_string(),
            temp_name: "".to_string(),
            name_input: Default::default(),
            add_confirm_button: Default::default(),
            cancel_button: Default::default()
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
                        let (tx, rx) = btn.sound.play(self.audio_settings.clone());
                        btn.player_handle_sender = Option::Some(tx);
                        btn.player_handle_receiver = Option::Some(rx);
                    }
                }
            }

            ButtonMessage::AddButtonPressed => {
                self.is_being_added = true;
            }

            ButtonMessage::AddConfirmButtonPressed (ok) => {
                if ok {
                    self.buttons.push(PlayButton {
                        play_state: Default::default(),
                        delete_state: Default::default(),
                        sound: sound_player::Sound::new(self.temp_path.clone()),
                        player_handle_sender: None,
                        player_handle_receiver: None,
                        name: self.temp_name.to_owned(),
                    });
                    self.temp_path = "".to_string();
                    self.temp_name = "".to_string();
                    self.allow_confirm = false;
                    self.is_being_added = false;
                }
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

            ButtonMessage::CancelButtonPressed => {
                self.temp_path = "".to_string();
                self.temp_name = "".to_string();
                self.allow_confirm = false;
                self.is_being_added = false;
            }
        }
    }

    pub fn view(&mut self) -> Element<'_, Message> {
        let mut elements = vec![];
        let settings = self.video_settings.lock().unwrap();
        let (mut width, mut height) = (settings.width, settings.height);
        elements.push({
            let mut children: Vec<Element<'_, _>> = vec![];
            let mut row_children: Vec<Element<'_, _>> = vec![];

            if width >=self.button_row_len && height != 0
            {
                let mut button_width = (width / self.button_row_len) - ((width / self.button_row_len) / 8);
                if button_width > 15{
                    button_width -= 15;
                } // 15 = space + padding

                let button_height = button_width / 2;

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
                                            .width(Length::from(button_width as u16))
                                            .height(Length::from(button_height as u16))
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
                if !self.is_being_added {
                    //add "add" button
                    row_children.push(
                        Button::new(
                            &mut self.add_button,
                            Text::new("add")
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .vertical_alignment(VerticalAlignment::Center)
                        )
                            .on_press(Message::PlayButtons(ButtonMessage::AddButtonPressed))
                            .width(Length::from(button_width as u16))
                            .height(Length::from(button_height as u16))
                            .into(),
                    );
                }

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
                            .spacing(5)
                            .padding(10)
                            .into()
                    );
                }
                Column::with_children(children).into()
            } else {
                Column::new().into()
            }
    });
        if self.is_being_added && width != 0 && height != 0 {
            elements.push(
                if self.is_being_added {
                    //TODO add winapi filepicker for Windows users
                    let add_button = if self.allow_confirm {
                        Button::new(&mut self.add_confirm_button, Text::new("confirm"))
                            .on_press(Message::PlayButtons(ButtonMessage::AddConfirmButtonPressed(true)))
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
                            )
                            .width(Length::from(((width / 100) * 80) as u16))
                            .on_submit(
                                if self.allow_confirm{
                                    Message::PlayButtons(ButtonMessage::AddConfirmButtonPressed(true))
                                }
                                else {
                                    Message::PlayButtons(ButtonMessage::AddConfirmButtonPressed(false))
                                }
                            )                        )
                        .push(TextInput::new(
                                &mut self.name_input,
                                "enter button name here",
                                &self.temp_name,
                                |val| Message::PlayButtons(ButtonMessage::NameChange(val))
                            )
                            .width(Length::from(((width / 100) * 80) as u16))
                            .on_submit(
                                if self.allow_confirm{
                                    Message::PlayButtons(ButtonMessage::AddConfirmButtonPressed(true))
                                }
                                    else {
                                        Message::PlayButtons(ButtonMessage::AddConfirmButtonPressed(false))
                                    }
                            )
                        )
                        .push(Row::new()
                            .push(add_button)
                            .push(Button::new(
                                &mut self.cancel_button,
                                Text::new("cancel")
                            )
                                .on_press(Message::PlayButtons(ButtonMessage::CancelButtonPressed))
                            )
                        )
                        .align_items(Align::Center)
                        .into()
                } else {
                    Column::new().into()
                }
            );
        }
        Column::with_children(elements).into()
    }
}
