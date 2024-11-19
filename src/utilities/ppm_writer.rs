use std::fs::File;
use std::io::{Result as IoResult, Write};

use crate::primitives::pixel::Pixel;

pub struct PPMWriter {
    width: u16,
    height: u16,
}

impl PPMWriter {
    pub fn new(width: u16, height: u16) -> Self {
        PPMWriter { width, height }
    }

    /// Writes the PPM header to the provided file.
    fn write_header(&self, outfile: &mut File) -> IoResult<()> {
        writeln!(outfile, "P6")?;
        writeln!(outfile, "{} {}", self.width, self.height)?;
        writeln!(outfile, "255")?;
        Ok(())
    }

    /// Writes pixel data to the file using the provided pixel mapping function.
    pub fn write_file<F>(
        &self,
        filename: &str,
        pixel_data: &[Pixel],
        pixel_mapper: F,
    ) -> IoResult<()>
    where
        F: Fn(&Pixel) -> (u8, u8, u8),
    {
        let mut outfile = File::create(filename)?;
        self.write_header(&mut outfile)?;

        for pixel in pixel_data {
            let (red, green, blue) = pixel_mapper(pixel);
            outfile.write_all(&[red, green, blue])?;
        }

        Ok(())
    }
}
