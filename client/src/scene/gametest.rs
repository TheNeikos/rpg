use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

use super::*;
use gfx::extra::stream::Stream;
use gfx::ClearData;
use piston_window::{PistonWindow, Event, Glyphs, Key, PressEvent};
use conrod::*;

use super::ingamemenu::IngameMenu;

pub struct GameTest {
    ui: Ui<Glyphs>,
    should_quit: Rc<RefCell<bool>>
}

impl GameTest {
    pub fn new(window: &PistonWindow) -> GameTest {
        let path = Path::new("assets/ShareTechMono-Regular.ttf");
        let glyph_cache = Glyphs::new(&path, window.factory.borrow().clone()).unwrap();
        let mut ui = Ui::new(glyph_cache, Theme::default());

        println!("New Menu!");

        GameTest {
            ui: ui,
            should_quit: Rc::new(RefCell::new(false))
        }
    }
}

impl Scene for GameTest {
    fn tick(&mut self, window: &PistonWindow, other: &[Box<Scene>]) -> SceneModifier {
        use piston_window::Button;

        self.ui.handle_event(window);

        if let Some(Button::Keyboard(Key::Escape)) = window.press_args() {
            return SceneModifier::Push(Box::new(IngameMenu::new(window)));
        }
        SceneModifier::Nothing
    }

    fn draw(&mut self, window: &PistonWindow, other: &[Box<Scene>]) {
        window.draw_3d(|stream| {
            stream.clear(
                ClearData {
                    color: [0.3, 0.3, 0.3, 1.0],
                    depth: 1.0,
                    stencil: 0
                }
            );
        });
    }

    fn get_id(&self) -> usize { 1 }
}


