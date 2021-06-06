use iced::{Element, Column, button, Button, Text, Row};
use crate::Message;
use std::borrow::BorrowMut;
use std::ops::{DerefMut, Deref};
use iced::image::viewer::Renderer;
use std::cell::RefCell;
use iced::button::State;

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


       for (index, button) in self.buttons.iter_mut().enumerate() {
            row_children.push(
                Button::new(&mut button.state, Text::new(index.to_string())).into()
            );
        }

        let row_amount = if row_children.len() < self.button_row_len && row_children.len() > 0{
            1
        }
        else{
            row_children.len()/self.button_row_len
        };


        row_children.reverse();

        for i in 0..row_amount+1 {
            let mut added_buttons = 0;
            let mut kek: Vec<Element<'_,_>> = vec![];
            while added_buttons < self.button_row_len  && 0 < row_children.len() {

                if let Some(x) = row_children.pop(){
                    kek.push(x);
                }
                added_buttons += 1;
            }

            children.push(Row::with_children(kek).into());
        }


        children.push(Button::new(&mut self.add_button, Text::new("add"))
            .on_press(Message::PlayButtons(ButtonMessage::AddButtonPressed))
            .into()
        );

        Column::with_children(children).into()
    }


}