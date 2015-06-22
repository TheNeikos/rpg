#![feature(custom_derive)]
#![feature(ip_addr)]

extern crate shared;

mod player;
pub mod worldstate;

pub mod servermessage;
pub mod rpgserver;

pub use rpgserver::{RpgServer, ServerStatus};
pub use player::Player;
pub use worldstate::WorldState;
