use std::net::TcpStream;

pub enum WorldEvent {
    Quit,
}

pub enum ServerEvent {
    Quit,
    ClientConnected(TcpStream),
    ClientDisconnected(usize),
}
