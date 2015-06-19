#![feature(convert)]

extern crate server;

use server::RpgServer;
use server::WorldState;

use std::io::{self, BufRead, Write};

fn main() {
    let mut server = RpgServer::new("0.0.0.0:1337").unwrap();
    server.start();

    let stdin = io::stdin();

    print!("> ");
    io::stdout().flush();
    for line in stdin.lock().lines() {
        match line {
            Ok(string) => {
                match string.as_str() {
                    "status" => {
                        println!("Status: {:?}", server.status());
                    },
                    "exit"|"quit"|"stop" => {
                        println!("Exiting!");
                        break;
                    },
                    "players" => {
                        // TODO: Print current players!!!
                        let arc_state = server.get_state();
                        let state = arc_state.read().unwrap();
                        let players = state.get_players();
                        println!("{} players currently connected/connecting",
                                 players.len());
                        for player in players.iter() {
                            println!("  {} -> {}", player.0, player.1);
                        }
                    },
                    _ => {
                        println!("Did not recognize your command!");
                    }
                }
            },
            Err(e) => break
        }
        print!("> ");
        io::stdout().flush();
    }

    server.stop();
}
