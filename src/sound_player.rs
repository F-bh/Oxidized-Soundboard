use crate::audio_settings::AudioSettings;
use rodio::{Decoder, OutputStream, Sink, Source, DeviceTrait, Device};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use crate::audio_settings::AudioType::Input;
use std::borrow::Borrow;
use rodio::cpal::traits::HostTrait;

#[derive(Debug, Clone)]
pub(crate) struct Sound {
    pub(crate) file_path: String,
    pub(crate) state: PlayState,
}

#[derive(Clone, Copy)]
pub(crate) enum PlayerMessage {
    SettingsChange,
    Stop,
}

#[derive(Debug, Clone)]
pub(crate) enum PlayState {
    Playing,
    Stopped,
}

impl Sound {
    pub(crate) fn new(p: String) -> Self {
        Self {
            file_path: p,
            state: PlayState::Stopped,
        }
    }

    // plays the sound file associated with the Sound using the default audio output
    // returns a channel Sender to send messages to the player and a receiver to receive messages from the player
    // TODO let user specify output
    // TODO remove unwraps
    pub(crate) fn play( &self, settings: Arc<Mutex<AudioSettings>>) -> (Sender<PlayerMessage>, Receiver<PlayState>) {
        let (tx_player_as_receiver, rx_player_as_receiver) = mpsc::channel();
        let (tx_player_as_sender, rx_player_as_sender) = mpsc::channel();

        let path = self.file_path.clone();

        let _thread_handle = thread::spawn(move || {
            //get devices from names
            let out_name = settings.lock().unwrap().out_dev_name.clone();
            let in_name = settings.lock().unwrap().in_dev_name.clone();
            let mut out_dev = rodio::cpal::default_host().default_output_device().unwrap();
            let mut in_dev=  rodio::cpal::default_host().default_output_device().unwrap();


            for dev in rodio::cpal::default_host().output_devices().unwrap(){
                if let Ok(name) = dev.name(){
                    if name == out_name{
                        out_dev = dev;
                        break;
                    }
                }
            }

            for dev in rodio::cpal::default_host().input_devices().unwrap(){
                if let Ok(name) = dev.name(){
                    if name == in_name{
                        in_dev = dev;
                        break;
                    }
                }
            }


            let (_stream, out_stream_handle) = OutputStream::try_from_device(&out_dev).unwrap();
            let (_stream, in_stream_handle) = OutputStream::try_from_device(&in_dev).unwrap();
            let out_file_buf = BufReader::new(File::open(path.clone()).unwrap());
            let in_file_buf = BufReader::new(File::open(path).unwrap());
            let out_source = Decoder::new(out_file_buf).unwrap();
            let in_source = Decoder::new(in_file_buf).unwrap();
            let out_sink = Sink::try_new(&out_stream_handle).unwrap();
            let in_sink = Sink::try_new(&in_stream_handle).unwrap();

            let play_duration = match in_source.total_duration() {
                Some(dur) => dur,
                None => Duration::new(20, 0), //dummy duration
            };

            let start_time = SystemTime::now();

            if !settings.lock().unwrap().output_muted {
                out_sink.set_volume(settings.lock().unwrap().output_slider_value as f32 / 100.0);
                in_sink.set_volume(settings.lock().unwrap().input_slider_value as f32 / 100.0);
            } else {
                out_sink.set_volume(0.0);
                in_sink.set_volume(0.0);
            }

            if !settings.lock().unwrap().input_muted {
                in_sink.set_volume(settings.lock().unwrap().input_slider_value as f32 / 100.0);
            } else {
                in_sink.set_volume(0.0);
            }

            in_sink.append(in_source);
            out_sink.append(out_source);
            tx_player_as_sender.send(PlayState::Playing);

            while start_time.elapsed().unwrap() < play_duration && !out_sink.empty() {
                let msg = rx_player_as_receiver
                    .recv_timeout(play_duration - start_time.elapsed().unwrap());

                if let Ok(msg) = msg {
                    match msg {
                        PlayerMessage::Stop => {
                            out_sink.stop();
                            in_sink.stop();
                            tx_player_as_sender.send(PlayState::Stopped);
                        }

                        PlayerMessage::SettingsChange => {
                            let settings = settings.lock().unwrap();
                            if !settings.output_muted {
                                out_sink.set_volume(settings.output_slider_value as f32 / 100.0);
                            } else {
                                out_sink.set_volume(0.0);
                            }

                            if !settings.input_muted {
                                in_sink.set_volume(settings.input_slider_value as f32 / 100.0);
                            } else {
                                in_sink.set_volume(0.0);
                            }
                        }
                    }
                }
            }
            in_sink.stop();
            out_sink.stop();
            tx_player_as_sender.send(PlayState::Stopped)
        });

        (tx_player_as_receiver, rx_player_as_sender)
    }
}
