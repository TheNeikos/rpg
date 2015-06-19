use std::net::TcpStream;
use std::thread::{JoinHandle, Builder};
use std::sync::mpsc::Sender;
use std::io::Read;
use std::fmt;
use std::mem;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

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

static GLOBAL_PLAYER_ID: AtomicUsize = ATOMIC_USIZE_INIT;

pub struct Player {
    id: usize,
    thr: JoinHandle<()>,
    stream: TcpStream,
    status: PlayerStatus,
}

impl Player {
    pub fn new(tx: Sender<ServerEvent>, stream: TcpStream) -> Player {
        let stream_clone = stream.try_clone().unwrap();
        let id = GLOBAL_PLAYER_ID.fetch_add(1, Ordering::SeqCst);
        Player {
            id: id,
            stream: stream,
            thr: {
                let name = format!("{}", stream_clone.peer_addr().unwrap().ip());

                Builder::new().name(name).spawn(move|| {
                    let mut stream = stream_clone;
                    let mut stream_iter = stream.bytes();
                    // We now are going to read in a login sequence!
                    // This is a rather simple operation.
                    // Read in 2 bytes, which is representing a u16.
                    // Transmute afterwards to a u16 and read that many
                    // bytes! Parse that many bytes into a JSON Struct
                    // TODO: Think about an upper limit, we don't want them
                    // to spam us with megabytes of data for nothing!
                    let mut slice_buf = [0; 2];
                    let mut idx = 0;
                    for byte in  stream_iter.take(2) {
                        match byte {
                            Ok(n) => {
                                slice_buf[idx] = n;
                                idx += 1;
                            },
                            Err(e) => {
                                tx.send(ServerEvent::ClientDisconnected(id)).unwrap();
                                return;
                            }
                        }
                    }
                    if idx == 0 {
                        tx.send(ServerEvent::ClientDisconnected(id)).unwrap();
                        return;
                    }
                    let to_be_read : u16 = unsafe{ mem::transmute(slice_buf) };

                    // At the end of the thread we always disconnect.
                    // TODO: Make a loop that reads packets
                    tx.send(ServerEvent::ClientDisconnected(id)).unwrap();
                }).unwrap()
            },
            status: PlayerStatus::Connecting
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} - {})",
        self.stream.peer_addr().unwrap().ip(),
        self.status)
    }
}
