use std::net::TcpStream;

pub enum WorldEvent {
    Quit,
}

pub enum ServerEvent {
    Quit,
    ClientConnected(TcpStream),
    ClientAuthed(usize, String),
    ClientDisconnected(usize),
}
