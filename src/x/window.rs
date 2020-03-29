extern crate xcb;

use super::graphics_context::GraphicsContext;
use std::mem;

pub enum PropID {
    Atom(xcb::Atom),
    Str(&'static str),
}

impl PropID {
    pub fn get_atom(&self, conn: &xcb::Connection) -> Result<xcb::Atom, xcb::GenericError> {
        let atom = match self {
            PropID::Str(s) => {
                let cookie = xcb::intern_atom(&conn, false, s);
                let atom = cookie.get_reply()?.atom();
                atom
            }
            PropID::Atom(a) => a.clone(),
        };

        Ok(atom)
    }
}

pub struct Window<'c> {
    conn: &'c xcb::Connection,
    screen: &'c xcb::Screen<'c>,
    window: xcb::Window,
    size: (u16, u16),
    pos: (u32, u32),
}

impl<'c> Window<'c> {
    pub fn new(
        conn: &'c xcb::Connection,
        screen: &'c xcb::Screen,
        size: (u16, u16),
        pos: (u32, u32),
    ) -> Window<'c> {
        Window {
            conn,
            screen,
            window: Self::create_window(conn, screen.root(), screen.root_visual(), size),
            size,
            pos,
        }
    }

    pub fn set_name(&self, name: &str) -> Result<(), xcb::GenericError> {
        Self::set_prop(
            self.window,
            &self.conn,
            &PropID::Atom(xcb::ATOM_WM_NAME),
            &PropID::Atom(xcb::ATOM_STRING),
            &name.as_bytes(),
        )?;
        Ok(())
    }

    pub fn get_graphics_context(&self) -> GraphicsContext {
        GraphicsContext::new(
            self.conn,
            self.screen,
            &self.window,
            (self.size.0 as i32, self.size.1 as i32),
        )
    }

    pub fn show(&self) -> Result<(), xcb::GenericError> {
        xcb::map_window(self.conn, self.window).request_check()?;
        // workaround wm's centering behaviour: reposition after show
        xcb::configure_window(
            self.conn,
            self.window,
            &[
                (xcb::CONFIG_WINDOW_X as u16, self.pos.0),
                (xcb::CONFIG_WINDOW_Y as u16, self.pos.1),
            ],
        )
        .request_check()?;

        Ok(())
    }

    fn create_window(
        conn: &xcb::Connection,
        to_where: xcb::Window,
        to_where_visual: xcb::Visualid,
        size: (u16, u16),
    ) -> xcb::Window {
        let win = conn.generate_id();
        xcb::create_window(
            &conn,
            xcb::COPY_FROM_PARENT as u8,
            win,
            to_where,
            1,
            1,
            size.0 as u16,
            size.1 as u16,
            0,
            xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
            to_where_visual,
            &[(
                xcb::CW_EVENT_MASK,
                xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS,
            )],
        );
        win
    }

    fn set_prop<T: Sized>(
        win: xcb::Window,
        conn: &xcb::Connection,
        prop_name: &PropID,
        type_name: &PropID,
        value: &[T],
    ) -> Result<(), xcb::GenericError> {
        let prop_name = prop_name.get_atom(&conn)?;
        let type_name = type_name.get_atom(&conn)?;
        xcb::change_property(
            &conn,
            xcb::PROP_MODE_APPEND as u8,
            win,
            prop_name,
            type_name,
            mem::size_of::<T>() as u8 * 8 as u8,
            value,
        )
        .request_check()?;
        Ok(())
    }
}
