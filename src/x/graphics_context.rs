extern crate cairo;
extern crate cairo_sys;
extern crate xcb;

use cairo::XCBSurface;

pub struct GraphicsContext<'c> {
    conn: &'c xcb::Connection,
    cr: cairo::Context,
}

impl<'c> GraphicsContext<'c> {
    pub fn new(
        conn: &'c xcb::Connection,
        screen: &xcb::Screen,
        window: &xcb::Window,
        size: (i32, i32),
    ) -> GraphicsContext<'c> {
        GraphicsContext {
            conn,
            cr: Self::create_cairo_context(conn, screen, window, size),
        }
    }

    pub fn set_source_rgba(&self, red: f64, green: f64, blue: f64, alpha: f64) {
        self.cr.set_source_rgba(red, green, blue, alpha);
    }

    pub fn move_to(&self, x: f64, y: f64) {
        self.cr.move_to(x, y);
    }

    pub fn set_font_size(&self, size: f64) {
        self.cr.set_font_size(size);
    }

    pub fn show_text(&self, text: &str) {
        self.cr.show_text(text);
    }

    pub fn paint(&self) {
        self.cr.paint();
    }

    pub fn flush(&self) {
        self.conn.flush();
    }

    fn create_cairo_context(
        conn: &xcb::Connection,
        screen: &xcb::Screen,
        window: &xcb::Window,
        size: (i32, i32),
    ) -> cairo::Context {
        let surface;
        unsafe {
            let cairo_conn = cairo::XCBConnection::from_raw_none(
                conn.get_raw_conn() as *mut cairo_sys::xcb_connection_t
            );
            let visual_ptr: *mut cairo_sys::xcb_visualtype_t =
                &mut Self::get_root_visual_type(&screen).base as *mut _
                    as *mut cairo_sys::xcb_visualtype_t;
            let visual = cairo::XCBVisualType::from_raw_none(visual_ptr);
            let cairo_screen = cairo::XCBDrawable(window.to_owned());
            surface = cairo::Surface::create(&cairo_conn, &cairo_screen, &visual, size.0, size.1);
        }
        cairo::Context::new(&surface)
    }

    fn get_root_visual_type(screen: &xcb::Screen) -> xcb::Visualtype {
        for depth in screen.allowed_depths() {
            for visual in depth.visuals() {
                if screen.root_visual() == visual.visual_id() {
                    return visual;
                }
            }
        }
        panic!("No visual type found");
    }
}
