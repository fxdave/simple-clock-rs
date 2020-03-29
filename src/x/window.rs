extern crate xcb;

use super::graphics_context::GraphicsContext;
use std::mem;

pub enum PropID {
    Atom(xcb::Atom),
    Str(&'static str),
}

impl PropID {
    pub fn get_atom(&self, conn: &xcb::Connection) -> Result<xcb::Atom, String> {
        let atom = match self {
            PropID::Str(s) => {
                let cookie = xcb::intern_atom(&conn, false, s);
                let atom = cookie
                    .get_reply()
                    .or(Err(String::from("Unable to get atom: ") + s))?
                    .atom();
                atom
            }
            PropID::Atom(a) => a.clone(),
        };

        Ok(atom)
    }
}

enum PropVal<'l, T: Sized> {
    List(&'l [T]),
    PropID(PropID),
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
            window: Self::create_window(conn, screen.root(), screen.root_visual(), size, pos),
            size,
            pos,
        }
    }

    pub fn set_name(&self, name: &str, class: &str) -> Result<(), String> {
        let wm_name = name.to_owned();
        Self::set_prop(
            self.window,
            &self.conn,
            &PropID::Atom(xcb::ATOM_WM_NAME),
            &PropID::Atom(xcb::ATOM_STRING),
            PropVal::List(&name.as_bytes()),
        )?;
        Self::set_prop(
            self.window,
            &self.conn,
            &PropID::Atom(xcb::ATOM_WM_CLASS),
            &PropID::Atom(xcb::ATOM_STRING),
            PropVal::List(&(wm_name + "\0" + class).as_bytes()),
        )?;
        Ok(())
    }

    pub fn set_decoration_disabled(&self) -> Result<(), String> {
        self.set_decoration_disabled_motif()?;
        self.set_decoration_disabled_net()?;
        self.set_wm_type_normal()?;
        Ok(())
    }

    fn set_wm_type_normal(&self) -> Result<(), String> {
        Self::set_prop::<u32>(
            self.window,
            self.conn,
            &PropID::Str("_NET_WM_WINDOW_TYPE"),
            &PropID::Atom(xcb::ATOM_ATOM),
            PropVal::PropID(PropID::Str("_NET_WM_WINDOW_TYPE_NORMAL")),
        )?;
        Ok(())
    }

    fn set_decoration_disabled_net(&self) -> Result<(), String> {
        Self::set_prop(
            self.window,
            self.conn,
            &PropID::Str("_NET_WM_BYPASS_COMPOSITOR"),
            &PropID::Atom(xcb::ATOM_CARDINAL),
            PropVal::List(&[2]),
        )?;
        Ok(())
    }

    fn set_decoration_disabled_motif(&self) -> Result<(), String> {
        Self::set_prop(
            self.window,
            self.conn,
            &PropID::Str("_MOTIF_WM_HINTS"),
            &PropID::Str("_MOTIF_WM_HINTS"),
            PropVal::List(&[2u32, 0u32, 0u32, 0u32, 0u32]),
        )?;
        Ok(())
    }

    pub fn set_normal_hints(&self) -> Result<(), String> {
        Self::set_prop(
            self.window,
            self.conn,
            &PropID::Atom(xcb::ATOM_WM_NORMAL_HINTS),
            &PropID::Atom(xcb::ATOM_WM_SIZE_HINTS),
            PropVal::List(&[
                1 << 2 | 1 << 4 | 1 << 5,
                self.pos.0 as i32,
                self.pos.1 as i32,
                self.size.0 as i32,
                self.size.1 as i32,
                self.size.0 as i32,
                self.size.1 as i32,
                self.size.0 as i32,
                self.size.1 as i32,
                0,
                0,
                0,
                0,
                0,
                0,
                self.size.0 as i32,
                self.size.1 as i32,
                1i32,
            ]),
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

    pub fn show(&self) -> Result<(), String> {
        xcb::map_window_checked(self.conn, self.window)
            .request_check()
            .or(Err("Unable to show window."))?;

        // workaround wm's centering behaviour: relocation after show
        self.set_position()?;

        Ok(())
    }

    fn set_position(&self) -> Result<(), String> {
        xcb::configure_window_checked(
            self.conn,
            self.window,
            &[
                (xcb::CONFIG_WINDOW_X as u16, self.pos.0),
                (xcb::CONFIG_WINDOW_Y as u16, self.pos.1),
            ],
        )
        .request_check()
        .or(Err("Unable to set window to the desired positon."))?;
        Ok(())
    }

    fn create_window(
        conn: &xcb::Connection,
        to_where: xcb::Window,
        to_where_visual: xcb::Visualid,
        size: (u16, u16),
        pos: (u32, u32),
    ) -> xcb::Window {
        let win = conn.generate_id();
        xcb::create_window_checked(
            &conn,
            xcb::COPY_FROM_PARENT as u8,
            win,
            to_where,
            pos.0 as i16,
            pos.1 as i16,
            size.0 as u16,
            size.1 as u16,
            0, // TODO: border size
            xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
            to_where_visual,
            &[(
                xcb::CW_EVENT_MASK,
                xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS,
            )],
        );
        win
    }

    fn set_prop<'l, T: Sized>(
        win: xcb::Window,
        conn: &xcb::Connection,
        prop_name: &PropID,
        type_name: &PropID,
        value: PropVal<'l, T>,
    ) -> Result<(), String> {
        let prop_name = prop_name.get_atom(&conn)?;
        let type_name = type_name.get_atom(&conn)?;
        match value {
            PropVal::List(l) => xcb::change_property_checked(
                &conn,
                xcb::PROP_MODE_REPLACE as u8,
                win,
                prop_name,
                type_name,
                mem::size_of::<T>() as u8 * 8 as u8,
                l,
            ),
            PropVal::PropID(p) => xcb::change_property_checked(
                &conn,
                xcb::PROP_MODE_REPLACE as u8,
                win,
                prop_name,
                type_name,
                32,
                &[p.get_atom(&conn)?],
            ),
        }
        .request_check()
        .or(Err("Unable to set property"))?;
        Ok(())
    }
}
