use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle, Source};
use std::borrow::BorrowMut;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use crate::Message;


pub struct Sound {
    pub file_path: String,
    pub state: PlayState,
}

pub enum PlayerMessage{
    Stop,
}
pub enum PlayState{
    Playing,
    Stopped,
}


impl Sound{

    pub fn new(p: String) -> Self{
       let out_stream = OutputStream::try_default().unwrap();
        Self{
            file_path: p,
            state: PlayState::Stopped
        }
    }

    //plays the sound file associated with the Sound using the default audio output
    // returns a channel Sender to send messages to the player and a receiver to receive messages from the player
    // TODO let user specify output
    // TODO remove unwraps
    pub fn play(&self) -> (Sender<PlayerMessage>, Receiver<PlayState>){
        let (tx_player_as_receiver, rx_player_as_receiver) = mpsc::channel();
        let (tx_player_as_sender, rx_player_as_sender) = mpsc::channel();

        let path = self.file_path.clone();

        let _thread_handle = thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let file_buf = BufReader::new(File::open(path).unwrap());
            let source = Decoder::new(file_buf).unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            let duration_option = source.total_duration();
            let play_duration= match duration_option {
                None => {
                    Duration::new(20, 0) //if duration couldn't be calculated wait for 20s
                }
                Some(play_duration) => {
                    play_duration
                }
            };

            //TODO set sink volume
            sink.append(source);
            tx_player_as_sender.send(PlayState::Playing);

            let msg: PlayerMessage = rx_player_as_receiver.recv_timeout(play_duration).unwrap();

            match msg{
                PlayerMessage::Stop => {
                   sink.stop()
                }
            }
            sink.sleep_until_end();
            tx_player_as_sender.send(PlayState::Stopped)
        });

        (tx_player_as_receiver, rx_player_as_sender)
    }

}
