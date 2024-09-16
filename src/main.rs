mod core;

use core::framebuffer::FrameBuffer;
use std::io::{Error, ErrorKind, Result};

fn main() -> Result<()> {
    let fb = FrameBuffer::new(512, 512).map_err(|err| Error::new(ErrorKind::Other, err))?;

    fb.write_rgb_file("test.ppm")?;

    Ok(())
}
