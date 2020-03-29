use super::window::Window;

pub struct WindowBuilder<'c> {
    conn: &'c xcb::Connection,
    screen: &'c xcb::Screen<'c>,
    title: &'static str,
    width: u16,
    height: u16,
    x: u32,
    y: u32,
}

impl<'c> WindowBuilder<'c> {
    pub fn new(conn: &'c xcb::Connection, screen: &'c xcb::Screen<'c>) -> WindowBuilder<'c> {
        WindowBuilder {
            conn,
            screen,
            title: "Unnamed",
            width: 100,
            height: 100,
            x: 0,
            y: 0,
        }
    }

    pub fn set_title(&mut self, title: &'static str) {
        self.title = title;
    }

    pub fn set_width(&mut self, width: u16) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u16) {
        self.height = height;
    }

    pub fn set_x(&mut self, x: u32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: u32) {
        self.y = y;
    }

    fn wait_for_expose(&self) {
        loop {
            match self.conn.wait_for_event() {
                Some(event) => {
                    let r = event.response_type() & !0x80;
                    match r {
                        xcb::EXPOSE => break,
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }

    pub fn build(&self) -> Result<Window, xcb::GenericError> {
        let win = Window::new(
            self.conn,
            self.screen,
            (self.width, self.height),
            (self.x, self.y),
        );

        win.set_name(self.title)?;
        win.show()?;
        self.conn.flush();
        self.wait_for_expose();

        Ok(win)
    }

    
}
