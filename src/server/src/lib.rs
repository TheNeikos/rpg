#![feature(custom_derive)]

extern crate shared;

pub mod servermessage;
pub mod rpgserver;

pub use rpgserver::RpgServer;
