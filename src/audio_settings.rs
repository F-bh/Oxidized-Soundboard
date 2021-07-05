use crate::sound_player::PlayerMessage;
use crate::{Message, WindowSettings};
use iced::{
    button, slider, Align, Button, Column, Element, HorizontalAlignment, Length, Row, Text,
    pick_list, PickList,
};
use std::ops::RangeInclusive;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, PoisonError, MutexGuard};
use rodio::{Device, DeviceTrait};
use rodio::cpal::traits::HostTrait;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum AudioType {
    Input,
    Output,
}

#[derive(Debug, Clone)]
pub(crate) enum AudioSettingsMessage {
    SliderChange(i32, AudioType),
    MutePressed(AudioType),
    InDevSelected(String),
    OutDevSelected(String),
}

#[derive(Clone)]
pub(crate) struct AudioSettings {
    pub(crate) output_slider_value: i32,
    pub(crate) output_muted: bool,
    pub(crate) input_slider_value: i32,
    pub(crate) input_muted: bool,
    pub(crate) out_dev_name: String,
    pub(crate) in_dev_name : String,
}

impl Default for AudioSettings {
    fn default() -> Self {
        //TODO save and load Settings
        Self {
            output_slider_value: 0,
            output_muted: false,
            input_slider_value: 0,
            input_muted: false,
            out_dev_name: "".to_string(),
            in_dev_name: "".to_string()
        }
    }
}

pub(crate) struct AudioSettingsModel {
    pub(crate) audio_settings: Arc<Mutex<AudioSettings>>,
    pub(crate) video_settings: Arc<Mutex<WindowSettings>>,
    output_slider: slider::State,
    output_mute_button: button::State,
    input_slider: slider::State,
    input_mute_button: button::State,
    in_list_state: pick_list::State<String>,
    out_list_state: pick_list::State<String>,
    out_dev_names: Vec<String>,
    out_dev_name: String,
    in_dev_name : String,
}

impl Default for AudioSettingsModel{
    fn default() -> Self {
        let mut out_names = vec![];
        if let Ok(devs) = rodio::cpal::default_host().output_devices() {
            for dev in devs {
                if let Ok(name) = dev.name() {
                    out_names.push(name)
                }
            }
        }

        Self {
            audio_settings: Arc::new(Mutex::new(Default::default())),
            video_settings: Arc::new(Mutex::new(Default::default())),
            output_slider: Default::default(),
            output_mute_button: Default::default(),
            input_slider: Default::default(),
            input_mute_button: Default::default(),
            in_list_state: Default::default(),
            out_list_state: Default::default(),
            out_dev_names: out_names,
            out_dev_name: "".to_string(),
            in_dev_name: "".to_string()
        }
    }
}


impl AudioSettingsModel {

    pub fn view(&mut self) -> Element<'_, Message> {
        let settings = self.video_settings.lock().unwrap();
        let (width, _height) = (settings.width, settings.height);
        let mute_width = (width / 100) * 20;
        let slider_width = (width / 100) * 50;
        let pick_list_width = (width/100) * 20;
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
                    .push(
                        iced::widget::PickList::new(
                            &mut self.in_list_state,
                            &self.out_dev_names,
                            Some(self.in_dev_name.clone()),
                            Message::AudioSettingsInDeviceSelected
                        )
                            .width(Length::from(pick_list_width as u16))
                    )
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
                    .push(
                        iced::widget::PickList::new(
                            &mut self.out_list_state,
                            &self.out_dev_names,
                            Some(self.out_dev_name.clone()),
                            Message::AudioSettingsOutDeviceSelected,
                        )
                            .width(Length::from(pick_list_width as u16))
                    )
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
            }

            AudioSettingsMessage::MutePressed(Type) => match Type {
                AudioType::Input => settings.input_muted = !settings.input_muted,
                AudioType::Output => settings.output_muted = !settings.output_muted,
            }

            AudioSettingsMessage::InDevSelected(name) => {
                self.out_dev_names = vec![];
                if let Ok(devs) = rodio::cpal::default_host().output_devices(){
                    for dev in devs{
                        if let Ok(name) = dev.name(){
                            self.out_dev_names.push(name)
                        }
                    }
                }
                self.in_dev_name = name.clone();
                settings.in_dev_name = name;
            }


            AudioSettingsMessage::OutDevSelected(name) => {
                self.out_dev_names = vec![];
                if let Ok(devs) = rodio::cpal::default_host().output_devices(){
                    for dev in devs{
                        if let Ok(name) = dev.name(){
                            self.out_dev_names.push(name)
                        }
                    }
                }
                self.out_dev_name = name.clone();
                settings.out_dev_name = name;
            }
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
