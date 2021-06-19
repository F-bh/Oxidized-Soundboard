use crate::audio_settings::{AudioSettings};

use rodio::{Decoder, OutputStream, Sink, Source};

use std::fs::File;
use std::io::BufReader;

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

pub struct Sound {
    pub file_path: String,
    pub state: PlayState,
}

#[derive(Clone, Copy)]
pub enum PlayerMessage {
    SettingsChange,
    Stop,
}

pub enum PlayState {
    Playing,
    Stopped,
}

impl Sound {
    pub fn new(p: String) -> Self {
        Self {
            file_path: p,
            state: PlayState::Stopped,
        }
    }

    // plays the sound file associated with the Sound using the default audio output
    // returns a channel Sender to send messages to the player and a receiver to receive messages from the player
    // TODO let user specify output
    // TODO remove unwraps
    pub fn play(
        &self,
        settings: Arc<Mutex<AudioSettings>>,
    ) -> (Sender<PlayerMessage>, Receiver<PlayState>) {
        let (tx_player_as_receiver, rx_player_as_receiver) = mpsc::channel();
        let (tx_player_as_sender, rx_player_as_sender) = mpsc::channel();

        let path = self.file_path.clone();

        let _thread_handle = thread::spawn(move || {
            println!("started");
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let file_buf = BufReader::new(File::open(path).unwrap());
            let source = Decoder::new(file_buf).unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            let play_duration = match source.total_duration() {
                Some(dur) => dur,
                None => Duration::new(20, 0), //dummy duration
            };

            let start_time = SystemTime::now();

            if !settings.lock().unwrap().output_muted {
                sink.set_volume(settings.lock().unwrap().output_slider_value as f32 / 100.0);
            } else {
                sink.set_volume(0.0);
            }

            sink.append(source);
            tx_player_as_sender.send(PlayState::Playing);

            while start_time.elapsed().unwrap() < play_duration && !sink.empty() {
                let msg = rx_player_as_receiver
                    .recv_timeout(play_duration - start_time.elapsed().unwrap());

                if let Ok(msg) = msg {
                    match msg {
                        PlayerMessage::Stop => {
                            sink.stop();
                            tx_player_as_sender.send(PlayState::Stopped);
                        }

                        PlayerMessage::SettingsChange => {
                            let settings = settings.lock().unwrap();
                            if !settings.output_muted {
                                sink.set_volume(settings.output_slider_value as f32 / 100.0);
                            } else {
                                sink.set_volume(0.0);
                            }
                        }
                    }
                }
            }

            sink.stop();
            tx_player_as_sender.send(PlayState::Stopped)
        });

        (tx_player_as_receiver, rx_player_as_sender)
    }
}
