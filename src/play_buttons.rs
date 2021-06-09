use iced::{Element, Column, button, Button, Text, Row};
use crate::Message;
use iced::button::State;
use std::convert::TryFrom;

struct PlayButton{
    state: button::State,
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
    AddButtonPressed
}

impl PlayButtons{

    pub fn update(&mut self, msg: ButtonMessage){
        match msg {
            ButtonMessage::PlayButtonPressed(index) => {

            }
            ButtonMessage::AddButtonPressed => {
                self.buttons.push(PlayButton{
                    state: Default::default()
                })
            }
        }
    }



    pub fn view(&mut self) -> Element <'_, Message>{
        let mut children: Vec<Element<'_,_>> = vec![];
        let mut row_children: Vec<Element<'_,_>> = vec![];

        //add play buttons to temp slice
        for (index, button) in self.buttons.iter_mut().enumerate() {
             row_children.push(
                 Button::new(&mut button.state, Text::new(index.to_string()))
                     .min_width(50)
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
                .padding(10)
                .into());

        }
        Column::with_children(children).into()
    }

}