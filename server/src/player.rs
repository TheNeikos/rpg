use std::net::TcpStream;
use std::thread::{JoinHandle, Builder};
use std::sync::mpsc::Sender;
use std::io::Read;
use std::fmt;

use servermessage::ServerEvent;

#[derive(Debug, Display)]
pub enum PlayerStatus {
    Connecting,
    Connected,
    Disconnected
}

impl fmt::Display for PlayerStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            PlayerStatus::Connecting => "Connecting",
            PlayerStatus::Connected => "Connected",
            PlayerStatus::Disconnected => "Disconnected",
        })
    }
}

pub struct Player {
    thr: JoinHandle<()>,
    stream: TcpStream,
    status: PlayerStatus
}

impl Player {
    pub fn new(tx: Sender<ServerEvent>, stream: TcpStream) -> Player {
        let stream_clone = stream.try_clone().unwrap();
        Player {
            stream: stream,
            thr: {
                let name = format!("{}", stream_clone.peer_addr().unwrap().ip());

                Builder::new().name(name).spawn(move|| {
                    for byte in stream_clone.bytes() {
                        match byte {
                            Ok(b) => {
                                println!("Got a byte ;3: {}", b);
                            },
                            Err(e) => {
                                println!("There was an error: {}", e);
                                break;
                            }
                        }
                    }
                }).unwrap()
            },
            status: PlayerStatus::Connecting
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} - {})",
               self.stream.peer_addr().unwrap().ip(),
               self.status)
    }
}
