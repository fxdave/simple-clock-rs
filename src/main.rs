mod common;
mod x;

use chrono::prelude::*;
use common::Color;
use schedule_recv::periodic_ms;
use x::Context;

fn main() -> Result<(), String> {
    // Connect to display server
    let ctx = Context::new();
    let screen = ctx
        .get_preferred_screen()
        .expect("Couldn't Open the screen");

    // Properties
    let padding = 10.0;
    let font_size = 28.0;
    let margin = 20;
    let width = 145;
    let height = 40;
    let screen_height: u16 = screen
        .get_geometry()
        .expect("Couldn't get the geometry of your screen")
        .height();
    let pos_x = margin;
    let pos_y = screen_height - margin - height;
    let bg = Color(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0, 0.3);
    let fg = Color(1.0, 1.0, 1.0, 1.0);

    // Show clock
    let mut window_builder = screen.get_window_builder();
    window_builder.set_title("X11SimpleClock");
    window_builder.set_width(width);
    window_builder.set_height(height);
    window_builder.set_x(pos_x as u32);
    window_builder.set_y(pos_y as u32);

    let win = window_builder.build()?; // waits for expose

    let gc = win.get_graphics_context();
    gc.rounded_rectange(
        0 as f64,
        0 as f64,
        width as f64,
        height as f64,
        height as f64 / 5.0,
    );

    let tick = periodic_ms(100);
    loop {
        gc.set_source_rgba(bg.0, bg.1, bg.2, bg.3);
        gc.fill_preserve();
        gc.set_source_rgba(fg.0, fg.1, fg.2, fg.3);
        gc.move_to(padding, padding + 3.0 * font_size / 4.0);
        gc.set_font_size(font_size);
        gc.show_text(&Local::now().time().format("%H:%M:%S").to_string());
        gc.flush();
        tick.recv().unwrap();
    }

    // ctx.wait_for_exit();

    Ok(())
}
