use crate::add_view::AddViewMessage;
use crate::audio_settings::AudioSettings;
use crate::sound_player::{PlayState, PlayerMessage, Sound};
use crate::Message;
use crate::{sound_player, WindowSettings};

use iced::{
    button, text_input, Align, Button, Column, Element, HorizontalAlignment, Length, Row, Text,
    TextInput, VerticalAlignment,
};
use std::fmt::{Debug};


use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub(crate) struct PlayButton {
    pub(crate) player_handle_sender: Option<Sender<sound_player::PlayerMessage>>,
    pub(crate) player_handle_receiver: Option<Receiver<sound_player::PlayState>>,
    play_state: button::State,
    delete_state: button::State,
    sound: sound_player::Sound,
    name: String,
}

impl PlayButton {
    pub(crate) fn new(sound: sound_player::Sound, name: String) -> Self {
        Self {
            name,
            sound,
            play_state: Default::default(),
            delete_state: Default::default(),
            player_handle_sender: None,
            player_handle_receiver: None,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ButtonMessage {
    PlayButtonPressed(usize),
    AddButtonPressed,
    DeleteButtonPressed(usize),
    ButtonAdded(Sound, String), //sound and name
}

pub(crate) struct PlayButtons {
    pub(crate) buttons: Vec<PlayButton>,
    pub(crate) audio_settings: Arc<Mutex<AudioSettings>>,
    pub(crate) video_settings: Arc<Mutex<WindowSettings>>,
    add_button: button::State,
    button_row_len: usize,
    is_being_added: bool,
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
        }
    }
}

impl PlayButtons {
    pub(crate) fn update(&mut self, msg: ButtonMessage) {
        match msg {
            ButtonMessage::PlayButtonPressed(index) => {
                let btn = &mut self.buttons[index];

                //check for messages from players
                if let Some(tx) = &btn.player_handle_receiver {
                    let res = tx.try_recv();
                    if let Ok(msg) = res {
                        match msg {
                            PlayState::Stopped => btn.sound.state = PlayState::Stopped,
                            PlayState::Playing => btn.sound.state = PlayState::Playing,
                        }
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

            ButtonMessage::DeleteButtonPressed(index) => {
                if let Some(handle) = &self.buttons[index].player_handle_sender {
                    handle.send(PlayerMessage::Stop);
                }
                self.buttons.remove(index);
            }
            ButtonMessage::ButtonAdded(sound, name) => {
                self.buttons.push(PlayButton::new(sound, name))
            }
        }
    }

    pub(crate) fn view(&mut self) -> Element<'_, Message> {
        let settings = self.video_settings.lock().unwrap();
        let (width, height) = (settings.width, settings.height);
        let mut children: Vec<Element<'_, _>> = vec![];
        let mut row_children: Vec<Element<'_, _>> = vec![];

        if width >= self.button_row_len && height != 0 {
            let mut button_width =
                (width / self.button_row_len) - ((width / self.button_row_len) / 8);
            if button_width > 15 {
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
                                            .vertical_alignment(VerticalAlignment::Center),
                                    )
                                    .width(Length::from(button_width as u16))
                                    .height(Length::from(button_height as u16))
                                    .on_press(
                                        Message::PlayButtons(ButtonMessage::PlayButtonPressed(
                                            index,
                                        )),
                                    ),
                                )
                                .push(
                                    Button::new(
                                        &mut button.delete_state,
                                        Text::new("X")
                                            .horizontal_alignment(HorizontalAlignment::Center)
                                            .vertical_alignment(VerticalAlignment::Center)
                                            .size((button_height / 4) as u16),
                                    )
                                    .min_height(button_height as u32)
                                    .min_width((button_width / 8) as u32)
                                    .on_press(
                                        Message::PlayButtons(ButtonMessage::DeleteButtonPressed(
                                            index,
                                        )),
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
                            .vertical_alignment(VerticalAlignment::Center),
                    )
                    //.on_press(Message::PlayButtons(ButtonMessage::AddButtonPressed))
                    .on_press(Message::AddView(AddViewMessage::AddPressed))
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
                        .into(),
                );
            }
            Column::with_children(children).into()
        } else {
            Column::new().into()
        }
    }
}
