use std::fmt;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

pub mod ingamemenu;
pub mod gametest;
pub mod mainmenu;

pub use self::mainmenu::MainMenu;

use gfx::ClearData;
use gfx::extra::stream::Stream;

use piston_window::{PistonWindow, Event, Glyphs, Key, PressEvent};

use conrod::*;

pub type SceneBoxes = Vec<Box<Scene>>;
pub type SceneId    = usize;

pub trait Scene {
    fn tick(&mut self, &PistonWindow, &[Box<Scene>]) -> SceneModifier;
    fn draw(&mut self, &PistonWindow, &[Box<Scene>]);
    fn get_id(&self) -> usize;
}

/// Return value of a Scene Tick
pub enum SceneModifier {
    /// Does really nothing
    Nothing,
    /// Quits the game after unwinding the state stack
    Quit,
    /// Pushes the `Scene` onto the stack
    Push(Box<Scene>),
    /// Pops one Scene
    Pop,
    /// Pops the `Scene` stack until the given id
    PopUntil(SceneId),
}
