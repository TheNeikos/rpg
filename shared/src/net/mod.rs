use std::error::Error;
use std::fmt;
use std::io::{Read, Write};
use std::mem;

use packets::Packet;


#[derive(Debug)]
pub enum PacketError {
    TooLarge,
    ZeroRead,
    ErroredOut,
    MismatchedSize,
    DecodeError,
}

impl Error for PacketError {
    fn description(&self) -> &str {
        match *self {
            PacketError::TooLarge => "Client tried to send a packet that was too large.",
            PacketError::ZeroRead => "Stream either hung up, or Client sent 0 bytes.",
            PacketError::ErroredOut => "The client errored out.",
            PacketError::MismatchedSize => "Client said we get X, we received Y.",
            PacketError::DecodeError => "Could not decode Packet",
        }
    }
}

impl fmt::Display for PacketError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        (match *self {
            PacketError::TooLarge => "Client tried to send a packet that was too large.",
            PacketError::ZeroRead => "Stream either hung up, or Client sent 0 bytes.",
            PacketError::ErroredOut => "The client errored out.",
            PacketError::MismatchedSize => "Client said we get X, we received Y.",
            PacketError::DecodeError => "Could not decode Packet",
        }).fmt(fmt)
    }
}

/// Reads in a new `Packet`
///
/// This function blocks on R until either all the required
/// packets have been read, or that an error occured. Note
/// that reading 0 bytes from the `Reader` is considered an
/// error and will abort the try with an error.
///
/// # Examples
///
/// Simple receive loop
///
/// ```
///
/// use std::io::Read;
/// use shared::net::receive_packet;
///
/// fn receive<R: Read>(stream: &mut R) {
///     let should_receive = true;
///
///     while should_receive {
///         match receive_packet(stream) {
///             _ => println!("Do things")
///             // ... Match the packets!
///         }
///     }
/// }
///
/// ```
pub fn receive_packet<R>(reader: &mut R) -> Result<Packet, PacketError> where R: Read {
    use bincode::decode;
    let mut slice_buf = [0; 2];
    let mut idx = 0;
    for byte in reader.bytes().take(2) {
        match byte {
            Ok(n) => {
                slice_buf[idx] = n;
                idx += 1;
            },
            Err(_) => {
                return Err(PacketError::ErroredOut);
            }
        }
    }
    // We did not read any bytes? Error!
    if idx == 0 {
        return Err(PacketError::ZeroRead);
    }

    let buffer_size : u16 = unsafe{ mem::transmute(slice_buf) };

    // If the buffer is above 1024 bytes we reject it.
    if buffer_size > 1024 {
        return Err(PacketError::TooLarge);
    }

    let mut buffer = Vec::<u8>::with_capacity(buffer_size as usize);
    let read = match reader.take(buffer_size as u64).read_to_end(&mut buffer) {
        Ok(b) => b,
        Err(_) => return Err(PacketError::ErroredOut)
    };

    if read != buffer_size as usize {
        println!("Got {} expected {}", read, buffer_size);
        return Err(PacketError::MismatchedSize);
    }

    match decode(&buffer[..]) {
        Ok(p) => Ok(p),
        Err(e) => {
            println!("{:?}", e);
            panic!();
        }
    }
}

pub fn send_packet<W>(writer: &mut W, pack: &Packet) where W: Write {
    use bincode::{encode, SizeLimit};
    let encoded: Vec<u8> = encode(pack, SizeLimit::Bounded(1024)).unwrap();
    let size : [u8; 2] = unsafe{ mem::transmute((encoded.len() as u16)) };
    writer.write(&size).unwrap();
    writer.write(&encoded[..]).unwrap();
}

mod test {
    use super::*;
    use packets::Packet;
    #[test]
    fn test_read_write() {
        let test_packet = Packet::AuthPlayer("Neikos".to_string());

        let mut test = Vec::<u8>::new();
        send_packet(&mut test, &test_packet);

        let result = receive_packet(&mut &test[..]);

        match result.unwrap() {
            Packet::AuthPlayer(name) => {
                assert!(name == "Neikos")
            }
        }
    }
}
