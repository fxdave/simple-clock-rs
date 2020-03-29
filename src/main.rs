mod common;
mod x;

use common::Color;
use x::Context;

fn main() -> Result<(), xcb::GenericError> {
    // Connect to display server
    let ctx = Context::new();
    let screen = ctx
        .get_preferred_screen()
        .expect("Couldn't Open the screen");

    // Properties
    let padding = 5.0;
    let font_size = 14.0;
    let margin = 20;
    let width = 100;
    let height = 40;
    let screen_height: u16 = screen
        .get_geometry()
        .expect("Couldn't get the geometry of your screen")
        .height();
    let pos_x = margin;
    let pos_y = screen_height - margin - height;
    let bg = Color(0.0, 0.0, 0.0, 0.5);
    let fg = Color(1.0, 1.0, 1.0, 1.0);

    // Show clock
    let mut window_builder = screen.get_window_builder();
    window_builder.set_title("X11SimpleClock");
    window_builder.set_width(width);
    window_builder.set_height(height);
    window_builder.set_x(pos_x as u32);
    window_builder.set_y(pos_y as u32);

    let win = window_builder.build()?; // waits for expose then sets props again

    let gc = win.get_graphics_context();
    gc.set_source_rgba(bg.0, bg.1, bg.2, bg.3);
    gc.paint();
    gc.set_source_rgba(fg.0, fg.1, fg.2, fg.3);
    gc.move_to(padding, padding + 3.0*font_size/4.0);
    gc.set_font_size(font_size);
    gc.show_text("19:00");
    gc.flush();

    ctx.wait_for_exit();

    Ok(())
}
