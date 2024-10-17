use super::framebuffer::{FrameBuffer, FrameBufferError};

/// Draws a line on the framebuffer where the X axis is the longer axis.
fn draw_x_line(
    fb: &mut FrameBuffer,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
) -> Result<(), FrameBufferError> {
    let direction = if y0 <= y1 { 1 } else { -1 };
    let dx = x1 - x0;
    let dy = direction * (y1 - y0);

    // Difference parameter.
    let mut p = 2 * dy - dx;
    let mut y = y0;

    for x in x0..x1 {
        fb.plot_pixel(x, y, 1.0, 1.0, 1.0)?;

        if p > 0 {
            y += direction;
            p += 2 * (dy - dx);
        } else {
            p += 2 * dy;
        }
    }

    Ok(())
}

/// Draws a line on the framebuffer where the Y axis is the longer axis.
fn draw_y_line(
    fb: &mut FrameBuffer,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
) -> Result<(), FrameBufferError> {
    let direction = if x0 <= x1 { 1 } else { -1 };
    let dx = direction * (x1 - x0);
    let dy = y1 - y0;

    // Difference parameter.
    let mut p = 2 * dx - dy;
    let mut x = x0;

    for y in y0..y1 {
        fb.plot_pixel(x, y, 1.0, 1.0, 1.0)?;

        if p > 0 {
            x += direction;
            p += 2 * (dx - dy);
        } else {
            p += 2 * dx;
        }
    }

    Ok(())
}

/// Draws a line on the framebuffer based on the line's orientation.
pub fn draw_line(
    fb: &mut FrameBuffer,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
) -> Result<(), FrameBufferError> {
    if (y1 - y0).abs() < (x1 - x0).abs() {
        if x0 > x1 {
            draw_x_line(fb, x1, y1, x0, y0)
        } else {
            draw_x_line(fb, x0, y0, x1, y1)
        }
    } else {
        if y0 > y1 {
            draw_y_line(fb, x1, y1, x0, y0)
        } else {
            draw_y_line(fb, x0, y0, x1, y1)
        }
    }
}
