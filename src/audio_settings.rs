use iced::{slider, button, Column, Element, Align, Row, Text, Button};
use crate::{Message};
use std::ops::RangeInclusive;

#[derive(Default)]
pub struct AudioSettings{
    output_slider_value: i32,
    output_slider: slider::State,
    output_mute_button: button::State,
    output_muted: bool,
    input_slider_value: i32,
    input_slider: slider::State,
    input_mute_button: button::State,
    input_muted: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum AudioType{
    Input,
    Output
}

#[derive(Debug, Clone, Copy)]
pub enum AudioSettingsMessage{
    SliderChange(i32, AudioType),
    MutePressed(AudioType),
}

impl AudioSettings{

   pub fn view(&mut self) -> Element<'_, Message>{
        Column::new()
            .padding(10)
            .push(
                Row::new()
                    .padding(20)
                    .align_items(Align::Start)
                    //add output controls
                    .push(
                        slider::Slider::new(
                            &mut self.input_slider,
                            RangeInclusive::new(0, 100),
                            self.input_slider_value,
                            Self::slider_change(AudioType::Input)
                        )
                            .step(1)
                    )
                    .push(
                        Text::new(self.input_slider_value.to_string())
                    )
                    .push(
                        Button::new(&mut self.input_mute_button, Text::new("mute input"))
                            .on_press(Message::AudioSettings(AudioSettingsMessage::MutePressed(AudioType::Input)))
                    )
            )
            //add output controls
            .push(
                Row::new()
                    .padding(20)
                    .align_items(Align::Start)
                    .push(
                        slider::Slider::new(
                            &mut self.output_slider,
                            RangeInclusive::new(0, 100),
                            self.output_slider_value,
                            Self::slider_change(AudioType::Output)
                        )
                            .step(1)
                    )
                    .push(
                        Text::new(self.output_slider_value.to_string())
                    )
                    .push(
                        Button::new(&mut self.output_mute_button, Text::new("mute output"))
                            .on_press(Message::AudioSettings(AudioSettingsMessage::MutePressed(AudioType::Output)))
                    )
            )
            .into()
    }

    pub fn update (&mut self, msg: AudioSettingsMessage){
        match msg{
            AudioSettingsMessage::SliderChange(val, Type) => {
                match Type {
                    AudioType::Input => {
                         self.input_slider_value = val
                    }
                    AudioType::Output => {
                        self.output_slider_value = val
                    }
                }
            }
            AudioSettingsMessage::MutePressed(Type) => {
                match Type{
                    AudioType::Input => {
                        println!("MutePressed {}","in")
                    }
                    AudioType::Output => {
                        println!("MutePressed {}","out")
                    }
                }
            }
        }
    }

    //function builder that returns an onChanged function depending on the audio_type
    fn slider_change(audio_type: AudioType) -> fn(i32) -> Message {
        return match audio_type {
            AudioType::Input =>
                |val: i32| Message::AudioSettings(AudioSettingsMessage::SliderChange(val, AudioType::Input)),
            AudioType::Output =>
                |val: i32| Message::AudioSettings(AudioSettingsMessage::SliderChange(val, AudioType::Output))
        }

    }

}

