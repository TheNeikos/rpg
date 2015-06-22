
#[derive(RustcEncodable, RustcDecodable)]
pub enum Packet {
    AuthPlayer(String)
}
