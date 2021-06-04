use iced::{Element, Column};
use crate::Message;

pub struct PlayButtons{

}

impl PlayButtons{

    pub fn update(&mut self){

    }

    pub fn view(&mut self) -> Element <'_, Message>{
        Column::new().into()
    }

}