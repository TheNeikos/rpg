use std::io;
use std::net::{TcpListener};
use std::thread::{JoinHandle, Builder};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, RwLock};
use std::net::SocketAddr;
use std::io::Error;

use worldstate::WorldState;
use shared::{game_loop, LoopAction};
use servermessage::{ServerEvent, WorldEvent};
use player::Player;

#[derive(PartialEq, Eq, Debug, Display)]
pub enum ServerStatus {
    Stopped,
    Running {
        world_running: bool,
        server_running: bool,
        socket_running: bool,
    }
}

pub struct RpgServer {
    list: TcpListener,

    world_sender: Option<Sender<WorldEvent>>,
    world_thread: Option<JoinHandle<()>>,

    server_sender: Option<Sender<ServerEvent>>,
    server_thread: Option<JoinHandle<()>>,

    socket_thread: Option<JoinHandle<()>>,

    state: Arc<RwLock<WorldState>>,
}

impl RpgServer {
    pub fn new(address: &str) -> Result<RpgServer, io::Error> {
        let listener = try!(TcpListener::bind(address));

        Ok(RpgServer {
            list: listener,
            world_sender: None,
            world_thread: None,
            server_sender: None,
            server_thread: None,
            socket_thread: None,
            // TODO: Don't actually do this... read it from somewhere
            state: Arc::new(RwLock::new(WorldState::new())),
        })
    }

    pub fn get_state(&self) -> Arc<RwLock<WorldState>> {
        self.state.clone()
    }

    pub fn status(&self) -> ServerStatus {
        let sts = (self.world_thread.is_some(),
        self.server_thread.is_some(),
        self.socket_thread.is_some());

        match sts {
            (false, false,false) => ServerStatus::Stopped,
            (a    , b    ,c    ) => ServerStatus::Running{
                world_running:a,
                server_running:b,
                socket_running:c
            }
        }
    }

    pub fn start(&mut self) {
        // Start the World Handler
        let (tx, rx) = channel();
        self.world_sender = Some(tx);
        self.world_thread = Builder::new().name("World".to_string()).spawn(move||{
            for event in rx.iter() {
                match event {
                    WorldEvent::Quit => break
                }
            }
        }).ok();

        // Start the Server Loop, which is the thread that updates at a fixed
        // tick of 60 Ticks per Second for now
        let (tx, rx) = channel();
        let mut state = self.state.clone();
        let server_tx = tx.clone();
        self.server_sender = Some(tx);
        self.server_thread = Builder::new().name("Server".to_string()).spawn(move||{
            let mut state = state;
            let server_tx = server_tx;

            // 16 million nano seconds should be enough right?!
            // TODO: Check for possible adaptive solution?
            game_loop(16666667, move|tick| {
                loop {
                    use servermessage::ServerEvent::*;
                    match rx.try_recv() {
                        Ok(event) => {
                            match event {
                                Quit => return LoopAction::Quit,
                                ClientConnected(stream) => {
                                    let mut state = (*state).write().unwrap();
                                    let mut players = state.mut_get_players();
                                    let new_player = Player::new(server_tx.clone(),stream);
                                    players.insert(new_player.get_id(), new_player);
                                },
                                ClientDisconnected(id) => {
                                    let mut state = (*state).write().unwrap();
                                    let mut players = state.mut_get_players();
                                    players.remove(&id);
                                },
                                ClientAuthed(id, name) => {
                                    let mut state = (*state).write().unwrap();
                                    let mut players = state.mut_get_players();
                                    let mut player = players.get_mut(&id).unwrap();
                                    player.auth(name);
                                    //TODO: Do something else than auth?
                                }
                            }
                        },
                        Err(err) => {
                            use std::sync::mpsc::TryRecvError::*;
                            match err {
                                Empty => break,
                                Disconnected => return LoopAction::Quit
                            }
                        }
                    };
                }

                LoopAction::Continue
            })
        }).ok();

        let server_sender = self.server_sender.clone().unwrap();
        let socket = self.list.try_clone();
        self.socket_thread = Builder::new().name("Socket".to_string()).spawn(move||{
            for stream in socket.unwrap().incoming() {
                match stream {
                    Ok(stream) => {
                        let _ = server_sender.send(ServerEvent::ClientConnected(stream));
                    }
                    Err(e) => {
                        println!("EEEP ERRORR11");
                        break;
                    }
                }
            }
        }).ok();
    }

    pub fn stop(&mut self) {
        if let Some(world_send) = self.world_sender.take() {
            let _ = world_send.send(WorldEvent::Quit);
        }
        if let Some(server_send) = self.server_sender.take() {
            let _ = server_send.send(ServerEvent::Quit);
        }

        if let Some(world_thr) = self.world_thread.take() {
            let _ = world_thr.join();
        }
        if let Some(server_thr) = self.server_thread.take() {
            let _ = server_thr.join();
        }

        if let Some(socket_thr) = self.socket_thread.take() {
            let _ = socket_thr.join();
        }
    }

    pub fn local_addr(&self) -> Result<SocketAddr, Error> {
        self.list.local_addr()
    }
}


mod tests {
    use super::*;

    #[test]
    fn start() {
        let server = RpgServer::new("127.0.0.0:0");
        assert!(server.is_ok());
    }

    #[test]
    fn status() {
        let mut server = RpgServer::new("127.0.0.0:0").unwrap();

        assert_eq!(server.status(), ServerStatus::Stopped);

        server.start();

        assert_eq!(server.status(), ServerStatus::Running{
            world_running: true, server_running: true, socket_running: true
        });

        server.stop();

        assert_eq!(server.status(), ServerStatus::Stopped);
    }
}
