mod core;
mod graphics;

use core::framebuffer::FrameBuffer;
use graphics::linedrawer::draw_line;
use std::{
    error::Error,
    f32::consts::PI,
    io::{self, ErrorKind},
    result::Result,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Create a framebuffer
    let mut fb = FrameBuffer::new(512, 512)
        .map_err(|err| Box::new(io::Error::new(ErrorKind::Other, err)))?;

    for i in (0..64).map(|i| i as f32 * PI / 32.0) {
        let x = (i.cos() * 48.0) as i32;
        let y = (i.sin() * 48.0) as i32;
        let ex = (i.cos() * 240.0) as i32;
        let ey = (i.sin() * 240.0) as i32;

        let _ = draw_line(&mut fb, 256 + x, 256 + y, 256 + ex, 256 + ey);
    }

    fb.write_rgb_file("test.ppm")?;

    Ok(())
}
