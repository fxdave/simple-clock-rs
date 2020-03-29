extern crate xcb;

mod graphics_context;
mod screen;
mod window;
mod window_builder;

pub use screen::Screen;
pub use window_builder::WindowBuilder;

pub struct Context {
    conn: xcb::Connection,
    preferred_screen_number: i32
}

impl Context {
    pub fn new() -> Context {
        let (conn, preferred_screen_number) = xcb::Connection::connect(None).unwrap();
        Context {
            conn,
            preferred_screen_number
        }
    }

    pub fn get_preferred_screen(&self) -> Option<Screen> {
        let setup = self.conn.get_setup();
        let screen = setup.roots().nth(self.preferred_screen_number as usize)?;
        Some(Screen::new(&self.conn, screen))
    }

    pub fn wait_for_exit(&self) {
        loop {
            match self.conn.wait_for_event() {
                Some(_) => (),
                _ => break,
            }
        }
    }
}
