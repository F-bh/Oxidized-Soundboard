use crate::sound_player::PlayerMessage;
use crate::{Message, WindowSettings};
use iced::{
    button, slider, Align, Button, Column, Element, HorizontalAlignment, Length, Row, Text,
};
use std::ops::RangeInclusive;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum AudioType {
    Input,
    Output,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum AudioSettingsMessage {
    SliderChange(i32, AudioType),
    MutePressed(AudioType),
}

#[derive(Clone)]
pub(crate) struct AudioSettings {
    pub(crate) output_slider_value: i32,
    pub(crate) output_muted: bool,
    pub(crate) input_slider_value: i32,
    pub(crate) input_muted: bool,
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
pub(crate) struct AudioSettingsModel {
    pub(crate) audio_settings: Arc<Mutex<AudioSettings>>,
    pub(crate) video_settings: Arc<Mutex<WindowSettings>>,
    output_slider: slider::State,
    output_mute_button: button::State,
    input_slider: slider::State,
    input_mute_button: button::State,
}

impl AudioSettingsModel {
    pub fn view(&mut self) -> Element<'_, Message> {
        let settings = self.video_settings.lock().unwrap();
        let (width, _height) = (settings.width, settings.height);
        let mute_width = (settings.width / 100) * 20;
        let slider_width = (width / 100) * 60;
        let padding: u16 = 5;
        let spacing: u16 = 10;
        let settings = self.audio_settings.lock().unwrap();
        Column::new()
            .padding(padding as u16)
            //add input controls
            .push(
                Row::new()
                    .spacing(spacing)
                    .padding(padding)
                    .align_items(Align::Start)
                    .push(
                        Button::new(
                            &mut self.input_mute_button,
                            if settings.input_muted {
                                Text::new("unmute input")
                                    .horizontal_alignment(HorizontalAlignment::Center)
                            } else {
                                Text::new("mute input")
                                    .horizontal_alignment(HorizontalAlignment::Center)
                            },
                        )
                            .on_press(Message::AudioSettings(AudioSettingsMessage::MutePressed(
                                AudioType::Input,
                            )))
                            .width(Length::from(mute_width as u16)),
                    )
                    .push(
                        slider::Slider::new(
                            &mut self.input_slider,
                            RangeInclusive::new(0, 100),
                            settings.input_slider_value,
                            Self::slider_change(AudioType::Input),
                        )
                        .step(1)
                        .width(Length::from(slider_width as u16)),
                    )
                    .push(Text::new(settings.input_slider_value.to_string()))
            )
            //add output controls
            .push(
                Row::new()
                    .spacing(spacing)
                    .padding(padding)
                    .align_items(Align::Start)
                    .push(
                        Button::new(
                            &mut self.output_mute_button,
                            if settings.output_muted {
                                Text::new("unmute output")
                                    .horizontal_alignment(HorizontalAlignment::Center)
                            } else {
                                Text::new("mute output")
                                    .horizontal_alignment(HorizontalAlignment::Center)
                            },
                        )
                            .on_press(Message::AudioSettings(AudioSettingsMessage::MutePressed(
                                AudioType::Output,
                            )))
                            .width(Length::from(mute_width as u16))
                    )
                    .push(
                        slider::Slider::new(
                            &mut self.output_slider,
                            RangeInclusive::new(0, 100),
                            settings.output_slider_value,
                            Self::slider_change(AudioType::Output),
                        )
                        .step(1)
                        .width(Length::from(slider_width as u16)),
                    )
                    .push(Text::new(settings.output_slider_value.to_string()))
            )
            .into()
    }

    pub fn update(
        &mut self,
        msg: AudioSettingsMessage,
        player_update_channels: Vec<Sender<PlayerMessage>>,
    ) {
        let mut settings = self.audio_settings.lock().unwrap();

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
