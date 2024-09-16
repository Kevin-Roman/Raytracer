use crate::core::framebuffer::FrameBuffer;

/// Draws a line on the framebuffer where the X axis is the longer axis.
fn draw_x_line(
    fb: &mut FrameBuffer,
    sx: i32, // Start X coordinate.
    sy: i32, // Start Y coordinate.
    ex: i32, // End X coordinate.
    ey: i32, // End Y coordinate.
) -> Result<(), &'static str> {
    let dir = if sx <= ex { 1 } else { -1 };

    let mut x = sx;
    let mut y = sy as f32;
    let slope = dir as f32 * (ey as f32 - sy as f32) / (ex as f32 - sx as f32);

    while x != ex {
        fb.plot_pixel(x, y as i32, 1.0, 1.0, 1.0)?;
        y += slope;
        x += dir;
    }

    Ok(())
}

/// Draws a line on the framebuffer where the Y axis is the longer axis.
fn draw_y_line(
    fb: &mut FrameBuffer,
    sx: i32, // Start X coordinate.
    sy: i32, // Start Y coordinate.
    ex: i32, // End X coordinate.
    ey: i32, // End Y coordinate.
) -> Result<(), &'static str> {
    let dir = if sy <= ey { 1 } else { -1 };

    let mut y = sy;
    let mut x = sx as f32;
    let slope = dir as f32 * (ex as f32 - sx as f32) / (ey as f32 - sy as f32);

    while y != ey {
        fb.plot_pixel(x as i32, y, 1.0, 1.0, 1.0)?;
        x += slope;
        y += dir;
    }

    Ok(())
}

/// Draws a line on the framebuffer based on the line's orientation.
pub fn draw_line(
    fb: &mut FrameBuffer,
    sx: i32, // Start X coordinate.
    sy: i32, // Start Y coordinate.
    ex: i32, // End X coordinate.
    ey: i32, // End Y coordinate.
) -> Result<(), &'static str> {
    if sx == ex && sy == ey {
        fb.plot_pixel(sx, sy, 1.0, 1.0, 1.0)
    } else if (ex - sx).pow(2) >= (ey - sy).pow(2) {
        draw_x_line(fb, sx, sy, ex, ey)
    } else {
        draw_y_line(fb, sx, sy, ex, ey)
    }
}
