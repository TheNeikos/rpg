use std::fmt;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;
use std::net::TcpStream;

use gfx::ClearData;
use gfx::extra::stream::Stream;
use gfx::device::Factory;
use gfx::device::tex::{Kind, Format, TextureInfo, Components};
use gfx::extra::stream::StreamFactory;
use gfx::render::target::{Plane, Output};
use gfx::attrib::IntSubType;

use piston_window::{PistonWindow, Event, Glyphs, Key, PressEvent, AdvancedWindow, Window};
use piston_window::GenericEvent;

use conrod::*;

pub type SceneId    = usize;

pub trait Scene {
    fn tick(&mut self, &PistonWindow, &[Box<Scene>]) -> SceneModifier;
    fn draw(&self, &PistonWindow, &[Box<Scene>]);
    fn get_id(&self) -> usize;

    fn on_enter(&mut self, &PistonWindow) { }

    fn on_leave(&mut self, &PistonWindow) { }
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
    ui: Rc<RefCell<Ui<Glyphs>>>,
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
            ui: Rc::new(RefCell::new(ui)),
            should_quit: Rc::new(RefCell::new(false)),
            should_start: Rc::new(RefCell::new(false)),
        }
    }
}

impl Scene for MainMenu {
    fn tick(&mut self, window: &PistonWindow, other: &[Box<Scene>]) -> SceneModifier {
        use piston_window::Button;

        self.ui.borrow_mut().handle_event(window);

        if *self.should_quit.borrow() {
            return SceneModifier::Pop;
        }

        if *self.should_start.borrow() {
            *self.should_start.borrow_mut() = false;
            return SceneModifier::Push(Box::new(GameMenu::new(window)));
        }

        SceneModifier::Nothing
    }
    fn draw(&self, window: &PistonWindow, other: &[Box<Scene>]) {
        window.draw_2d(|c, gl| {
            let mut ui = self.ui.borrow_mut();
            Background::new().rgb(1., 1., 1.).draw(&mut ui, gl);

            let sq = self.should_quit.clone();
            Button::new().dimensions(100., 100.).label("Quit").react(|| {
                *sq.borrow_mut() = true;
            })
                .set(0, &mut ui);

            let st = self.should_start.clone();
            Button::new().dimensions(100., 100.).label("Start Game").react(|| {
                *st.borrow_mut() = true;
            })
                .set(1, &mut ui);

            ui.draw(c, gl);
        });
    }

    fn get_id(&self) -> usize { 0 }
}

pub struct GameTest {
    ui: Rc<RefCell<Ui<Glyphs>>>,
    should_quit: Rc<RefCell<bool>>,
    connection: TcpStream
}

impl GameTest {
    pub fn new(window: &PistonWindow, address: &str) -> GameTest {
        let path = Path::new("assets/ShareTechMono-Regular.ttf");
        let glyph_cache = Glyphs::new(&path, window.factory.borrow().clone()).unwrap();
        let mut ui = Ui::new(glyph_cache, Theme::default());

        println!("Trying to connect to: {}", address);
        let stream = TcpStream::connect(address).unwrap();

        GameTest {
            ui: Rc::new(RefCell::new(ui)),
            should_quit: Rc::new(RefCell::new(false)),
            connection: stream
        }
    }
}

impl Scene for GameTest {
    fn tick(&mut self, window: &PistonWindow, other: &[Box<Scene>]) -> SceneModifier {
        use piston_window::Button;

        let mut ui = self.ui.borrow_mut();
        ui.handle_event(window);

        if let Some(Button::Keyboard(Key::Escape)) = window.press_args() {
            return SceneModifier::Push(Box::new(IngameMenu::new(window)));
        }
        SceneModifier::Nothing
    }

    fn draw(&self, window: &PistonWindow, other: &[Box<Scene>]) {
        window.draw_3d(|stream| {
            stream.clear(
                ClearData {
                    color: [0.3, 0.3, 0.9, 1.0],
                    depth: 1.0,
                    stencil: 0
                }
            );
        });
    }

    fn get_id(&self) -> usize { 1 }

    fn on_enter(&mut self, window: &PistonWindow) {
        println!("Entered");
        window.clone().set_capture_cursor(true);
    }

    fn on_leave(&mut self, window: &PistonWindow) {
        println!("Left");
        window.clone().set_capture_cursor(false);
    }
}

pub struct IngameMenu {
    ui: Rc<RefCell<Ui<Glyphs>>>,
    should_quit: Rc<RefCell<bool>>,
    go_back: Rc<RefCell<bool>>,
}

impl IngameMenu {
    pub fn new(window: &PistonWindow) -> IngameMenu {
        let path = Path::new("assets/ShareTechMono-Regular.ttf");
        let glyph_cache = Glyphs::new(&path, window.factory.borrow().clone()).unwrap();
        let mut ui = Ui::new(glyph_cache, Theme::default());

        println!("New Menu!");

        IngameMenu {
            ui: Rc::new(RefCell::new(ui)),
            should_quit: Rc::new(RefCell::new(false)),
            go_back: Rc::new(RefCell::new(false)),
        }
    }
}

impl Scene for IngameMenu {
    fn tick(&mut self, window: &PistonWindow, other: &[Box<Scene>]) -> SceneModifier {
        use piston_window::Button;

        let mut ui = self.ui.borrow_mut();
        ui.handle_event(window);

        if *self.should_quit.borrow() {
            return SceneModifier::PopUntil(0); // TODO: Use constants!!
        }

        if *self.go_back.borrow() {
            return SceneModifier::Pop;
        }

        if let Some(Button::Keyboard(Key::Escape)) = window.press_args() {
            return SceneModifier::Pop;
        }

        SceneModifier::Nothing
    }
    fn draw(&self, window: &PistonWindow, other: &[Box<Scene>]) {

        {
            let len = other.len();
            let (stack, last) = other.split_at(len-1);
            last[0].draw(&window, stack);
        }

        window.draw_2d(|c, gl| {
            let mut ui = self.ui.borrow_mut();
            Background::new().rgba(1., 1., 1., 0.1).draw(&mut ui, gl);

            let gb = self.go_back.clone();
            Button::new().dimensions(100., 100.).label("Back to Game").react(|| {
                *gb.borrow_mut() = true;
            })
                .set(0, &mut ui);

            let sq = self.should_quit.clone();
            Button::new().dimensions(100., 100.).label("Quit").react(|| {
                *sq.borrow_mut() = true;
            })
                .set(1, &mut ui);

            ui.draw(c, gl);
        });
    }

    fn get_id(&self) -> usize { 2 }
}

pub struct GameMenu {
    ui: Rc<RefCell<Ui<Glyphs>>>,
    address: Rc<RefCell<String>>,
    should_go: Rc<RefCell<bool>>,
    go_back: Rc<RefCell<bool>>,
}

impl GameMenu {
    pub fn new(window: &PistonWindow) -> GameMenu {
        let path = Path::new("assets/ShareTechMono-Regular.ttf");
        let glyph_cache = Glyphs::new(&path, window.factory.borrow().clone()).unwrap();
        let mut ui = Ui::new(glyph_cache, Theme::default());

        GameMenu {
            ui: Rc::new(RefCell::new(ui)),
            address: Rc::new(RefCell::new(String::new())),
            should_go: Rc::new(RefCell::new(false)),
            go_back: Rc::new(RefCell::new(false)),
        }
    }
}

impl Scene for GameMenu {
    fn tick(&mut self, window: &PistonWindow, other: &[Box<Scene>]) -> SceneModifier {
        use piston_window::Button;

        let mut ui = self.ui.borrow_mut();
        ui.handle_event(window);

        if *self.should_go.borrow() {
            return SceneModifier::Push(Box::new(GameTest::new(window, &self.address.borrow()[..])));
        }

        if *self.go_back.borrow() {
            return SceneModifier::Pop;
        }

        if let Some(Button::Keyboard(Key::Escape)) = window.press_args() {
            return SceneModifier::Pop;
        }

        SceneModifier::Nothing
    }
    fn draw(&self, window: &PistonWindow, other: &[Box<Scene>]) {

        {
            let len = other.len();
            let (stack, last) = other.split_at(len-1);
            last[0].draw(&window, stack);
        }

        window.draw_2d(|c, gl| {
            let mut ui = self.ui.borrow_mut();
            Background::new().rgba(1., 1., 1., 0.1).draw(&mut ui, gl);

            let gb = self.go_back.clone();
            Button::new().dimensions(100., 100.).label("Back to Menu").react(|| {
                *gb.borrow_mut() = true;
            }).set(0, &mut ui);

            let go = self.should_go.clone();
            Button::new().right_from(0, 30.0).dimensions(100., 100.).label("Go")
            .react(|| {
                *go.borrow_mut() = true;
            }).set(1, &mut ui);

            let go = self.should_go.clone();
            TextBox::new(&mut *self.address.borrow_mut()).up_from(1, 30.0).dimensions(300.0, 40.0).font_size(20)
            .react(|_string: &mut String| {
                *go.borrow_mut() = true;
            }).set(2, &mut ui);

            ui.draw(c, gl);
        });
    }

    fn get_id(&self) -> usize { 3 }
}
