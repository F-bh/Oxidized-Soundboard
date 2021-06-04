use iced::{
    button, Align, Button, Column,
    Element, Length, Sandbox, Settings,
    Text, slider, Row,
};
use std::ops::RangeInclusive;
use crate::AudioSettingsMessage::MutePressed;

fn main() -> iced::Result{
    Example::run(Settings{
        antialiasing: true,
        ..Settings::default()
    })
}
#[derive(Default)]
struct Example{
    audio: AudioSettings
}

#[derive(Default)]
struct AudioSettings{
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
enum Message {
    AudioSettings(AudioSettingsMessage)
}

#[derive(Debug, Clone, Copy)]
enum AudioType{
    Input,
    Output
}

#[derive(Debug, Clone, Copy)]
enum AudioSettingsMessage{
    SliderChange(i32, AudioType),
    MutePressed(AudioType),
}


impl Sandbox for Example{
    type Message = Message;

    fn new() -> Self {
        Example::default()
    }

    fn title(&self) -> String {
        String::from("Oxidized-Soundboard")
    }

    fn update(&mut self, message: Self::Message) {
        match message{
           Message::AudioSettings(msg) =>{
                match msg{
                   AudioSettingsMessage::SliderChange(val, Type) => {
                       match Type {
                           AudioType::Input => {
                               self.audio.input_slider_value = val
                           }
                           AudioType::Output => {
                               self.audio.output_slider_value = val
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
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
       Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                audio_settings_view(self)
            )
            .into()
    }
}


fn audio_settings_view(state: &mut Example) -> Element<'_, Message>{
    Column::new()
        .padding(10)
        .push(
        Row::new()
            .padding(20)
            .align_items(Align::Start)
            //add output controls
            .push(
                slider::Slider::new(
                        &mut state.audio.input_slider,
                        RangeInclusive::new(0, 100),
                        state.audio.input_slider_value,
                        audio_slider_change(AudioType::Input)
                    )
                    .step(1)
            )
            .push(
                Text::new(state.audio.input_slider_value.to_string())
            )
            .push(
                Button::new(&mut state.audio.input_mute_button, Text::new("mute input"))
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
                            &mut state.audio.output_slider,
                            RangeInclusive::new(0, 100),
                            state.audio.output_slider_value,
                            audio_slider_change(AudioType::Output)
                        )
                        .step(1)
                )
                .push(
                    Text::new(state.audio.output_slider_value.to_string())
                )
                .push(
                    Button::new(&mut state.audio.output_mute_button, Text::new("mute output"))
                        .on_press(Message::AudioSettings(AudioSettingsMessage::MutePressed(AudioType::Output)))
                )
        )
        .into()
}

//function builder that returns an onChanged function depending on the audio_type
fn audio_slider_change(audio_type: AudioType) -> fn(i32) -> Message {
    return match audio_type {
        AudioType::Input =>
            |val: i32| Message::AudioSettings(AudioSettingsMessage::SliderChange(val, AudioType::Input)),
        AudioType::Output =>
            |val: i32| Message::AudioSettings(AudioSettingsMessage::SliderChange(val, AudioType::Output))
    }

}