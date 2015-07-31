#![feature(convert)] // For as_mut_slice on Vector

extern crate piston_window;
extern crate conrod;
#[macro_use]
extern crate gfx;
extern crate camera_controllers;

mod scene;
mod graphics;

use std::thread;
use std::path::Path;
use std::fmt;

use piston_window::*;

use conrod::{Theme, Ui};

use scene::{Scene, SceneModifier};

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

            let mut scenes : Vec<Box<Scene>> = Vec::with_capacity(8);
            scenes.push(Box::new(scene::MainMenu::new(&window)));
            for event in window {
                let post_action;
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
                    SceneModifier::Push(mut sc) => {
                        scenes.last_mut().unwrap().on_leave(&event);
                        sc.on_enter(&event);
                        scenes.push(sc);
                    },
                    SceneModifier::Pop => {
                        let mut sc = scenes.pop().unwrap();
                        sc.on_leave(&event);
                        if let Some(ref mut nsc) = scenes.last_mut() {
                            nsc.on_enter(&event);
                        }
                    },
                    SceneModifier::PopUntil(id) => {
                        loop {
                            if let Some(scene) = scenes.last() {
                                println!("{:#?} =?= {:#?}", scene.get_id(), id);
                                if scene.get_id() == id {
                                    break;
                                }
                            }
                            scenes.pop();

                            println!("{:#?}", scenes.len());
                        }
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
