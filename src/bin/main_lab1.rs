use raytracer::{core::framebuffer::FrameBuffer, graphics::linedrawer::draw_line};
use std::f32::consts::PI;

fn main() {
    // Create a framebuffer.
    let mut fb = match FrameBuffer::new(512, 512) {
        Ok(fb) => fb,
        Err(e) => {
            eprintln!("Error creating framebuffer: {}", e);
            return;
        }
    };

    for i in (0..64).map(|i| i as f32 * PI / 32.0) {
        let x0 = 256 + (i.cos() * 48.0) as i32;
        let y0 = 256 + (i.sin() * 48.0) as i32;
        let x1 = 256 + (i.cos() * 240.0) as i32;
        let y1 = 256 + (i.sin() * 240.0) as i32;

        let _ = draw_line(&mut fb, x0, y0, x1, y1);
    }

    if let Err(e) = fb.write_rgb_file("./output/lab1.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };
}
