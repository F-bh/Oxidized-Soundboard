use iced::{Element, Column, button, Button, Text, Row, Container, Align};
use crate::Message;
use crate::sound_player;
use iced::button::State;
use std::convert::TryFrom;
use std::fmt::Alignment;
use std::fs::{File};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use crate::sound_player::{PlayerMessage, PlayState};

struct PlayButton{
    play_state: button::State,
    delete_state: button::State,
    sound: sound_player::Sound,
    player_handle_sender: Option<Sender<sound_player::PlayerMessage>>,
    player_handle_receiver: Option<Receiver<sound_player::PlayState>>,
}

pub struct PlayButtons{
    buttons: Vec<PlayButton>,
    add_button: button::State,
    button_row_len: usize,
}

impl Default for PlayButtons{
    fn default() -> Self {
        Self{
            buttons: vec![],
            add_button: Default::default(),
            button_row_len: 12
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonMessage{
    PlayButtonPressed(usize),
    AddButtonPressed,
    DeleteButtonPressed(usize)
}

impl PlayButtons{

    pub fn update(&mut self, msg: ButtonMessage){
        match msg {
            ButtonMessage::PlayButtonPressed(index) => {
                let btn = &mut self.buttons[index];

                //check for messages from players
                if let Some(tx) = &btn.player_handle_receiver{
                    let res = tx.try_recv();
                    match res{
                        Ok(msg) => {
                            match msg{
                                PlayState::Stopped => {
                                    btn.sound.state = PlayState::Stopped
                                }
                                PlayState::Playing =>{
                                    btn.sound.state = PlayState::Playing
                                }
                            }
                        }
                        Err(_) => {/*TODO add error handling*/}
                    }
                }

                match btn.sound.state{
                    PlayState::Playing => {
                        btn.player_handle_sender.as_ref().unwrap().send(PlayerMessage::Stop); //unwrap because the handle must exist if the sound is playing
                    }
                    PlayState::Stopped => {
                        let (tx, rx) = btn.sound.play();
                        btn.player_handle_sender = Option::Some(tx);
                        btn.player_handle_receiver = Option::Some(rx);
                    }
                }

            }

            ButtonMessage::AddButtonPressed => {
                self.buttons.push(PlayButton{
                    play_state: Default::default(),
                    delete_state: Default::default(),
                    sound: sound_player::Sound::new(String::from("/home/fbh/Downloads/KEKW.mp3")),
                    player_handle_sender: None,
                    player_handle_receiver: None
                })
            }
            ButtonMessage::DeleteButtonPressed(index) => {
                if let Some(handle) = &self.buttons[index].player_handle_sender{
                    handle.send(PlayerMessage::Stop);
                }
                self.buttons.remove(index);
            }
        }
    }



    pub fn view(&mut self) -> Element <'_, Message>{
        let mut children: Vec<Element<'_,_>> = vec![];
        let mut row_children: Vec<Element<'_,_>> = vec![];
        let button_width = 50;

        //add play buttons to temp slice
        for (index, button) in self.buttons.iter_mut().enumerate() {
            row_children.push(
                //add play + remove buttons
                Column::new()
                    .push(
                        Row::new()
                            .push(
                                Button::new(&mut button.play_state, Text::new(index.to_string()))
                                    .min_width(button_width)
                                    .on_press(Message::PlayButtons(ButtonMessage::PlayButtonPressed(index)))
                            )
                            .push(
                                Button::new(&mut button.delete_state, Text::new("X"))
                                    .on_press(Message::PlayButtons(ButtonMessage::DeleteButtonPressed(index)))
                            )
                    )
                    //TODO: add edit button
                    .into()
            );
        }

        //add "add" button
        row_children.push(
            Button::new(&mut self.add_button, Text::new("add"))
                .on_press(Message::PlayButtons(ButtonMessage::AddButtonPressed))
                .into()
        );

        //calculate amount of rows to draw
        let row_amount = if row_children.len() < self.button_row_len && row_children.len() > 0{
            1
        }
        else{
            row_children.len()/self.button_row_len
        };


        row_children.reverse();

        //move buttons to row's
        for i in 0..row_amount+1 {
            let mut added_buttons = 0;
            let mut kek: Vec<Element<'_,_>> = vec![];
            while added_buttons < self.button_row_len  && 0 < row_children.len() {

                if let Some(x) = row_children.pop(){
                    kek.push(x);
                }
                added_buttons += 1;
            }

            children.push(Row::with_children(kek)
                .spacing(10)
                .padding(5)
                .into());

        }
        Column::with_children(children).into()
    }
}
