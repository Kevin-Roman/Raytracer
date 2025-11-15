use crate::{primitives::Colour, rendering::framebuffer::FrameBufferError, FrameBuffer};

/// Draw a line between two points.
/// Using Bresenham's Line Algorithm which doesn't use floating point variables.
pub fn draw_line(
    fb: &mut FrameBuffer,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
) -> Result<(), FrameBufferError> {
    let is_steep = (y1 - y0).abs() > (x1 - x0).abs();

    // Ensure it's shallow.
    let (x0, y0, x1, y1) = if is_steep {
        (y0, x0, y1, x1)
    } else {
        (x0, y0, x1, y1)
    };

    // Ensure it's drawing left to right.
    let (x0, y0, x1, y1) = if x0 > x1 {
        (x1, y1, x0, y0)
    } else {
        (x0, y0, x1, y1)
    };

    let dx = x1 - x0;
    let dy = (y1 - y0).abs();

    // Decision parameter.
    // Accumulated error between the actual line and the ideal line.
    let mut error = 2 * dx;
    let y_step = if y0 < y1 { 1 } else { -1 };

    let mut y = y0;

    for x in x0..=x1 {
        if is_steep {
            fb.plot_pixel(y, x, Colour::new(1.0, 1.0, 1.0, 1.0))?;
        } else {
            fb.plot_pixel(x, y, Colour::new(1.0, 1.0, 1.0, 1.0))?;
        }

        error -= 2 * dy;

        // If line has deviated too far from the ideal line, move vertically.
        if error < 0 {
            y += y_step;
            error += 2 * dx;
        }
    }

    Ok(())
}
