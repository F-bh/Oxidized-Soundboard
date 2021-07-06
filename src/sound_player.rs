use crate::audio_settings::AudioSettings;
use rodio::{Decoder, OutputStream, Sink, Source, DeviceTrait};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};


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
    pub(crate) fn play( &self, settings: Arc<Mutex<AudioSettings>>) -> (Sender<PlayerMessage>, Receiver<PlayState>) {
        let (tx_player_as_receiver, rx_player_as_receiver) = mpsc::channel();
        let (tx_player_as_sender, rx_player_as_sender) = mpsc::channel();

        let path = self.file_path.clone();

        let _thread_handle = thread::spawn(move || {
            //get devices from names
            let out_name = settings.lock().unwrap().out1_dev_name.clone();
            let out_name2 = settings.lock().unwrap().out2_dev_name.clone();
            let mut out_dev1 = rodio::cpal::default_host().default_output_device().unwrap();
            let mut out_dev2 =  rodio::cpal::default_host().default_output_device().unwrap();


            for dev in rodio::cpal::default_host().output_devices().unwrap(){
                if let Ok(name) = dev.name(){
                    let mut out1_set = false;
                    let mut out2_set = false;
                    if name == out_name{
                        out_dev1 = dev;
                        out1_set = true;
                    }
                    else if name == out_name2{
                        out_dev2 = dev;
                        out2_set = true;
                    }
                    if out1_set && out2_set{
                        break;
                    }
                }
            }


            let (_stream, out1_stream_handle) = OutputStream::try_from_device(&out_dev1).unwrap();
            let (_stream, out2_stream_handle) = OutputStream::try_from_device(&out_dev2).unwrap();
            let out1_file_buf = BufReader::new(File::open(path.clone()).unwrap());
            let out2_file_buf = BufReader::new(File::open(path).unwrap());
            let out1_source = Decoder::new(out1_file_buf).unwrap();
            let out2_source = Decoder::new(out2_file_buf).unwrap();
            let out1_sink = Sink::try_new(&out1_stream_handle).unwrap();
            let out2_sink = Sink::try_new(&out2_stream_handle).unwrap();

            let play_duration = match out2_source.total_duration() {
                Some(dur) => dur,
                None => Duration::new(20, 0), //dummy duration
            };

            let start_time = SystemTime::now();

            if !settings.lock().unwrap().output2_muted {
                out2_sink.set_volume(settings.lock().unwrap().output2_slider_value as f32 / 100.0);
            } else {
                out2_sink.set_volume(0.0);
            }

            if !settings.lock().unwrap().output1_muted {
                out1_sink.set_volume(settings.lock().unwrap().output1_slider_value as f32 / 100.0);
            } else {
                out1_sink.set_volume(0.0);
            }

            out2_sink.append(out2_source);
            out1_sink.append(out1_source);
            tx_player_as_sender.send(PlayState::Playing);

            while start_time.elapsed().unwrap() < play_duration && !out1_sink.empty() {
                let msg = rx_player_as_receiver
                    .recv_timeout(play_duration - start_time.elapsed().unwrap());

                if let Ok(msg) = msg {
                    match msg {
                        PlayerMessage::Stop => {
                            out1_sink.stop();
                            out2_sink.stop();
                            tx_player_as_sender.send(PlayState::Stopped);
                        }

                        PlayerMessage::SettingsChange => {
                            let settings = settings.lock().unwrap();
                            if !settings.output1_muted {
                                out1_sink.set_volume(settings.output1_slider_value as f32 / 100.0);
                            } else {
                                out1_sink.set_volume(0.0);
                            }

                            if !settings.output2_muted {
                                out2_sink.set_volume(settings.output2_slider_value as f32 / 100.0);
                            } else {
                                out2_sink.set_volume(0.0);
                            }
                        }
                    }
                }
            }
            out2_sink.stop();
            out1_sink.stop();
            tx_player_as_sender.send(PlayState::Stopped)
        });

        (tx_player_as_receiver, rx_player_as_sender)
    }
}
