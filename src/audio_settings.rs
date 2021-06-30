use crate::sound_player::PlayerMessage;
use crate::{Message};
use iced::{button, slider, Align, Button, Column, Element, Row, Text};
use std::ops::RangeInclusive;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AudioType {
    Input,
    Output,
}

#[derive(Debug, Clone, Copy)]
pub enum AudioSettingsMessage {
    SliderChange(i32, AudioType),
    MutePressed(AudioType),
}

#[derive(Clone)]
pub struct AudioSettings {
    pub output_slider_value: i32,
    pub output_muted: bool,
    pub input_slider_value: i32,
    pub input_muted: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        //TODO save and load Settings
        Self {
            output_slider_value: 0,
            output_muted: false,
            input_slider_value: 0,
            input_muted: false,
        }
    }
}

#[derive(Default)]
pub struct AudioSettingsModel {
    pub settings: Arc<Mutex<AudioSettings>>,
    // pub player_handle_sender: Option<Sender<sound_player::PlayerMessage>>
    output_slider: slider::State,
    output_mute_button: button::State,
    input_slider: slider::State,
    input_mute_button: button::State,
}

impl AudioSettingsModel {
    pub fn view(&mut self) -> Element<'_, Message> {
        let settings = self.settings.lock().unwrap();
        Column::new()
            .padding(10)
            //add input controls
            .push(
                Row::new()
                    .padding(20)
                    .align_items(Align::Start)
                    .push(
                        slider::Slider::new(
                            &mut self.input_slider,
                            RangeInclusive::new(0, 100),
                            settings.input_slider_value,
                            Self::slider_change(AudioType::Input),
                        )
                        .step(1),
                    )
                    .push(Text::new(settings.input_slider_value.to_string()))
                    .push(
                        Button::new(
                            &mut self.input_mute_button,
                            if settings.input_muted {
                                Text::new("unmute input")
                            } else {
                                Text::new("mute input")
                            },
                        )
                        .on_press(Message::AudioSettings(
                            AudioSettingsMessage::MutePressed(AudioType::Input),
                        )),
                    ),
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
                            settings.output_slider_value,
                            Self::slider_change(AudioType::Output),
                        )
                        .step(1),
                    )
                    .push(Text::new(settings.output_slider_value.to_string()))
                    .push(
                        Button::new(
                            &mut self.output_mute_button,
                            if settings.output_muted {
                                Text::new("unmute output")
                            } else {
                                Text::new("mute output")
                            },
                        )
                        .on_press(Message::AudioSettings(
                            AudioSettingsMessage::MutePressed(AudioType::Output),
                        )),
                    ),
            )
            .into()
    }

    pub fn update(
        &mut self,
        msg: AudioSettingsMessage,
        player_update_channels: Vec<Sender<PlayerMessage>>,
    ) {
        let mut settings = self.settings.lock().unwrap();

        //change settings
        match msg {
            AudioSettingsMessage::SliderChange(val, Type) => match Type {
                AudioType::Input => settings.input_slider_value = val,
                AudioType::Output => settings.output_slider_value = val,
            },
            AudioSettingsMessage::MutePressed(Type) => match Type {
                AudioType::Input => settings.input_muted = !settings.input_muted,
                AudioType::Output => settings.output_muted = !settings.output_muted,
            },
        }
        //send settings changed message to players
        for chan in player_update_channels.iter() {
            chan.send(PlayerMessage::SettingsChange);
        }
    }

    //function builder that returns an onChanged function depending on the audio_type
    fn slider_change(audio_type: AudioType) -> fn(i32) -> Message {
        return match audio_type {
            AudioType::Input => |val: i32| {
                Message::AudioSettings(AudioSettingsMessage::SliderChange(val, AudioType::Input))
            },
            AudioType::Output => |val: i32| {
                Message::AudioSettings(AudioSettingsMessage::SliderChange(val, AudioType::Output))
            },
        };
    }
}
