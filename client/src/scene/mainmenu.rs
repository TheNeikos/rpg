use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

use super::*;
use gfx::extra::stream::Stream;
use piston_window::{PistonWindow, Event, Glyphs, Key, PressEvent};
use conrod::*;

use super::ingamemenu::IngameMenu;
use super::gametest::GameTest;

pub struct MainMenu {
    ui: Ui<Glyphs>,
    should_quit: Rc<RefCell<bool>>,
    should_start: Rc<RefCell<bool>>,
}

impl MainMenu {
    pub fn new(window: &PistonWindow) -> MainMenu {
        let path = Path::new("assets/ShareTechMono-Regular.ttf");
        let glyph_cache = Glyphs::new(&path, window.factory.borrow().clone()).unwrap();
        let mut ui = Ui::new(glyph_cache, Theme::default());

        println!("New Menu!");

        MainMenu {
            ui: ui,
            should_quit: Rc::new(RefCell::new(false)),
            should_start: Rc::new(RefCell::new(false)),
        }
    }
}

impl Scene for MainMenu {
    fn tick(&mut self, window: &PistonWindow, other: &[Box<Scene>]) -> SceneModifier {
        use piston_window::Button;

        self.ui.handle_event(window);

        if *self.should_quit.borrow() {
            return SceneModifier::Pop;
        }

        if *self.should_start.borrow() {
            *self.should_start.borrow_mut() = false;
            return SceneModifier::Push(Box::new(GameTest::new(window)));
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

            let st = self.should_start.clone();
            Button::new().dimensions(100., 100.).label("Start Game").react(|| {
                *st.borrow_mut() = true;
            })
                .set(1, &mut self.ui);

            self.ui.draw(c, gl);
        });
    }

    fn get_id(&self) -> usize { 0 }
}

