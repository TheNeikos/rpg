#![feature(convert)] // For as_mut_slice on Vector

extern crate piston_window;
extern crate conrod;
extern crate gfx;

mod scene;

use std::thread;
use std::path::Path;
use std::fmt;

use piston_window::*;

use conrod::{Theme, Ui};

use scene::*;

use gfx::Factory;
use gfx::render::RenderFactory;
use gfx::extra::stream::Stream;

pub enum ClientCommand {
    Quit
}

pub struct Client {
    window_thread: thread::JoinHandle<()>
}


impl Client {
    pub fn new() -> Client {
        let thread = thread::Builder::new().name("Client".to_string()).spawn(move || {
            let window: PistonWindow =
            WindowSettings::new(
                    "Rust RPG".to_string(), [1024, 768]
                    ).samples(2).into();

            let mut scenes : scene::SceneBoxes = Vec::with_capacity(8);
            scenes.push(Box::new(scene::MainMenu::new(&window)));
            for event in window {
                let mut post_action = scene::SceneModifier::Nothing;
                {
                    let mut scenes = scenes.as_mut_slice();

                    let len = scenes.len();
                    if len > 0 {
                        let (stack, last) = scenes.split_at_mut(len-1);
                        post_action = last[0].tick(&event, stack);
                        last[0].draw(&event, stack);
                    } else {
                        // The stack is empty! We have thus quit the game
                        break;
                    }
                }

                match post_action {
                    SceneModifier::Quit => break, // TODO: Graceful shutdown
                    SceneModifier::Push(sc) => {
                        scenes.push(sc);
                    },
                    SceneModifier::Pop => {
                        scenes.pop();
                    },
                    _ => ()
                }
            }
        }).unwrap();

        Client {
            window_thread: thread
        }
    }

    pub fn join(self) {
        let _ = self.window_thread.join();
    }
}
