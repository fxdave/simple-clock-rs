extern crate xcb;

use super::WindowBuilder;
use xcb::xproto;

pub struct Screen<'a> {
    conn: &'a xcb::Connection,
    screen: xcb::Screen<'a>,
}

impl<'a> Screen<'a> {
    pub fn new(conn: &'a xcb::Connection, screen: xcb::Screen<'a>) -> Self {
        Self {
            conn,
            screen
        }
    }

    pub fn get_geometry(&self) -> Result<xcb::xproto::GetGeometryReply, xcb::GenericError> {
        xproto::get_geometry(&self.conn, self.screen.root()).get_reply()
    }


    pub fn get_window_builder(&self) -> WindowBuilder {
        WindowBuilder::new(
            self.conn,
            &self.screen
        )
    }
}
