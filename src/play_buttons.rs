use iced::{Element, Column, button, Button, Text, Row, Container, Align};
use crate::Message;
use iced::button::State;
use std::convert::TryFrom;
use std::fmt::Alignment;

struct PlayButton{
    play_state: button::State,
    delete_state: button::State,
    //file: ...
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

            }
            ButtonMessage::AddButtonPressed => {
                self.buttons.push(PlayButton{
                    play_state: Default::default(),
                    delete_state: Default::default()
                })
            }
            ButtonMessage::DeleteButtonPressed(index) => {
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
