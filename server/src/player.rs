use std::net::TcpStream;
use std::thread::{JoinHandle, Builder};
use std::sync::mpsc::Sender;
use std::io::Read;
use std::fmt;
use std::mem;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

use servermessage::ServerEvent;
use shared::net::receive_packet;
use shared::packets::Packet;

#[derive(Debug, Display)]
pub enum PlayerStatus {
    Connecting,
    Authenticated,
    Disconnected
}

impl fmt::Display for PlayerStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            PlayerStatus::Connecting => "Connecting",
            PlayerStatus::Authenticated => "Authenticated",
            PlayerStatus::Disconnected => "Disconnected",
        })
    }
}

static GLOBAL_PLAYER_ID: AtomicUsize = ATOMIC_USIZE_INIT;

pub struct Player {
    id: usize,
    thr: JoinHandle<()>,
    stream: TcpStream,
    status: PlayerStatus,
    name: Option<String>,
}

impl Player {
    pub fn new(tx: Sender<ServerEvent>, stream: TcpStream) -> Player {
        let mut stream_clone = stream.try_clone().unwrap();
        let id = GLOBAL_PLAYER_ID.fetch_add(1, Ordering::SeqCst);
        Player {
            id: id,
            stream: stream,
            thr: {
                let name = format!("{}", stream_clone.peer_addr().unwrap().ip());

                Builder::new().name(name).spawn(move|| {
                    // TODO: Make a loop that reads packets

                    loop {
                        match receive_packet(&mut stream_clone) {
                            Ok(p) => {
                                use shared::packets::Packet::*;
                                match p {
                                    AuthPlayer(s) => {
                                        tx.send(ServerEvent::ClientAuthed(id, s));
                                    }
                                }
                            },
                            Err(e) => {
                                println!("Got error for player({}): {}", id, e);
                                break;
                            }
                        }
                    }

                    // At the end of the thread we always disconnect.
                    tx.send(ServerEvent::ClientDisconnected(id)).unwrap();
                }).unwrap()
            },
            status: PlayerStatus::Connecting,
            name: None
        }
    }

    pub fn auth(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} - {} - Known as: {})",
        self.stream.peer_addr().unwrap().ip(),
        self.status, match self.name { Some(ref s) => &s[..], None => "<Unknown>" })
    }
}
