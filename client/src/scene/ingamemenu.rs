use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

use super::*;
use gfx::extra::stream::Stream;
use piston_window::{PistonWindow, Event, Glyphs, Key, PressEvent};
use conrod::*;

pub struct IngameMenu {
    ui: Ui<Glyphs>,
    should_quit: Rc<RefCell<bool>>,
}

impl IngameMenu {
    pub fn new(window: &PistonWindow) -> IngameMenu {
        let path = Path::new("assets/ShareTechMono-Regular.ttf");
        let glyph_cache = Glyphs::new(&path, window.factory.borrow().clone()).unwrap();
        let mut ui = Ui::new(glyph_cache, Theme::default());

        println!("New Menu!");

        IngameMenu {
            ui: ui,
            should_quit: Rc::new(RefCell::new(false)),
        }
    }
}

impl Scene for IngameMenu {
    fn tick(&mut self, window: &PistonWindow, other: &[Box<Scene>]) -> SceneModifier {
        use piston_window::Button;

        self.ui.handle_event(window);

        if *self.should_quit.borrow() {
            return SceneModifier::PopUntil(0); // TODO: Use constants!!
        }

        if let Some(Button::Keyboard(Key::Escape)) = window.press_args() {
            return SceneModifier::Pop;
        }

        SceneModifier::Nothing
    }
    fn draw(&mut self, window: &PistonWindow, other: &[Box<Scene>]) {
        window.draw_2d(|c, gl| {
            Background::new().rgb(1., 1., 1.).draw(&mut self.ui, gl);

            let sq = self.should_quit.clone();
            Button::new().dimensions(100., 100.).label("Quit").react(|| {
                *sq.borrow_mut() = true;
            })
                .set(0, &mut self.ui);

            self.ui.draw(c, gl);
        });
    }

    fn get_id(&self) -> usize { 2 }
}

