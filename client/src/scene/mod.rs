use std::fmt;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

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
