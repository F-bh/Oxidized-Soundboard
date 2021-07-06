use crate::sound_player::PlayerMessage;
use crate::{Message, WindowSettings};
use iced::{
    button, slider, Align, Button, Column, Element, HorizontalAlignment, Length, Row, Text,
    pick_list,
};
use std::ops::RangeInclusive;
use std::thread;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use rodio::{DeviceTrait};
use rodio::cpal::traits::HostTrait;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum AudioType {
    Output1,
    Output2,
}

#[derive(Debug, Clone)]
pub(crate) enum AudioSettingsMessage {
    SliderChange(i32, AudioType),
    MutePressed(AudioType),
    InDevSelected(String),
    OutDevSelected(String),
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct AudioSettings {
    pub(crate) output2_slider_value: i32,
    pub(crate) output2_muted: bool,
    pub(crate) output1_slider_value: i32,
    pub(crate) output1_muted: bool,
    pub(crate) out2_dev_name: String,
    pub(crate) out1_dev_name: String,
}

impl Default for AudioSettings {
    fn default() -> Self {
        //TODO save and load Settings
        Self {
            output2_slider_value: 0,
            output2_muted: false,
            output1_slider_value: 0,
            output1_muted: false,
            out2_dev_name: "".to_string(),
            out1_dev_name: "".to_string()
        }
    }
}

pub(crate) struct AudioSettingsModel {
    pub(crate) audio_settings: Arc<Mutex<AudioSettings>>,
    pub(crate) video_settings: Arc<Mutex<WindowSettings>>,
    output2_slider: slider::State,
    output2_mute_button: button::State,
    output1_slider: slider::State,
    output1_mute_button: button::State,
    out1_list_state: pick_list::State<String>,
    out2_list_state: pick_list::State<String>,
    out2_dev_names: Vec<String>,
    out2_dev_name: String,
    out1_dev_name: String,
}

fn get_audio_device_names() -> Vec<String>{
    let handle = thread::spawn(|| -> Vec<String>{
            let mut out_names = vec![];
            if let Ok(devs) = rodio::cpal::default_host().output_devices() {
                for dev in devs {
                    if let Ok(name) = dev.name() {
                        out_names.push(name)
                    }
                }
            }
        out_names
    });
    handle.join().unwrap()
}

impl Default for AudioSettingsModel{
    fn default() -> Self {
        Self {
            audio_settings: Arc::new(Mutex::new(Default::default())),
            video_settings: Arc::new(Mutex::new(Default::default())),
            output2_slider: Default::default(),
            output2_mute_button: Default::default(),
            output1_slider: Default::default(),
            output1_mute_button: Default::default(),
            out1_list_state: Default::default(),
            out2_list_state: Default::default(),
            out2_dev_names: get_audio_device_names(),
            out2_dev_name: "".to_string(),
            out1_dev_name: "".to_string()
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
            //add output1 controls
            .push(
                Row::new()
                    .spacing(spacing)
                    .padding(padding)
                    .align_items(Align::Start)
                    .push(
                        Button::new(
                            &mut self.output1_mute_button,
                            if settings.output1_muted {
                                Text::new("unmute output 1")
                                    .horizontal_alignment(HorizontalAlignment::Center)
                            } else {
                                Text::new("mute output 1")
                                    .horizontal_alignment(HorizontalAlignment::Center)
                            },
                        )
                            .on_press(Message::AudioSettings(AudioSettingsMessage::MutePressed(
                                AudioType::Output1,
                            )))
                            .width(Length::from(mute_width as u16)),
                    )
                    .push(
                        slider::Slider::new(
                            &mut self.output1_slider,
                            RangeInclusive::new(0, 100),
                            settings.output1_slider_value,
                            Self::slider_change(AudioType::Output1),
                        )
                        .step(1)
                        .width(Length::from(slider_width as u16)),
                    )
                    .push(Text::new(settings.output1_slider_value.to_string()))
                    .push(
                        iced::widget::PickList::new(
                            &mut self.out1_list_state,
                            &self.out2_dev_names,
                            Some(self.out1_dev_name.clone()),
                            Message::AudioSettingsInDeviceSelected
                        )
                            .width(Length::from(pick_list_width as u16))
                    )
            )
            //add output2 controls
            .push(
                Row::new()
                    .spacing(spacing)
                    .padding(padding)
                    .align_items(Align::Start)
                    .push(
                        Button::new(
                            &mut self.output2_mute_button,
                            if settings.output2_muted {
                                Text::new("unmute output 2")
                                    .horizontal_alignment(HorizontalAlignment::Center)
                            } else {
                                Text::new("mute output 2")
                                    .horizontal_alignment(HorizontalAlignment::Center)
                            },
                        )
                            .on_press(Message::AudioSettings(AudioSettingsMessage::MutePressed(
                                AudioType::Output2,
                            )))
                            .width(Length::from(mute_width as u16))
                    )
                    .push(
                        slider::Slider::new(
                            &mut self.output2_slider,
                            RangeInclusive::new(0, 100),
                            settings.output2_slider_value,
                            Self::slider_change(AudioType::Output2),
                        )
                        .step(1)
                        .width(Length::from(slider_width as u16)),
                    )
                    .push(Text::new(settings.output2_slider_value.to_string()))
                    .push(
                        iced::widget::PickList::new(
                            &mut self.out2_list_state,
                            &self.out2_dev_names,
                            Some(self.out2_dev_name.clone()),
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
            AudioSettingsMessage::SliderChange(val, audio_type) => match audio_type {
                AudioType::Output1 => settings.output1_slider_value = val,
                AudioType::Output2 => settings.output2_slider_value = val,
            }

            AudioSettingsMessage::MutePressed(audio_type) => match audio_type {
                AudioType::Output1 => settings.output1_muted = !settings.output1_muted,
                AudioType::Output2 => settings.output2_muted = !settings.output2_muted,
            }

            AudioSettingsMessage::InDevSelected(name) => {
                self.out2_dev_names = get_audio_device_names();
                self.out1_dev_name = name.clone();
                settings.out2_dev_name = name;
            }


            AudioSettingsMessage::OutDevSelected(name) => {
                self.out2_dev_names = get_audio_device_names();
                self.out2_dev_name = name.clone();
                settings.out2_dev_name = name;
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
            AudioType::Output1 => |val: i32| {
                Message::AudioSettings(AudioSettingsMessage::SliderChange(val, AudioType::Output1))
            },
            AudioType::Output2 => |val: i32| {
                Message::AudioSettings(AudioSettingsMessage::SliderChange(val, AudioType::Output2))
            },
        };
    }
}
