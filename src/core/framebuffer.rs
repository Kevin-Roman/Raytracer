// The FrameBuffer creates a framebuffer with rgba and depth and can write them to a ppm file.

use std::fs::File;
use std::io::{self, Write};

// Constants for maximum allowed dimensions.
const MAX_WIDTH: i32 = 2048;
const MAX_HEIGHT: i32 = 2048;

#[derive(Clone)]
/// Represents a single pixel with color and depth information.
struct Pixel {
    red: f32,
    green: f32,
    blue: f32,
    depth: f32,
}

pub struct FrameBuffer {
    width: i32,
    height: i32,
    framebuffer: Vec<Pixel>,
}

// The FrameBuffer creates a framebuffer with RGB and depth information
// and provides methods to write them to a PPM file.
impl FrameBuffer {
    /// Creates a new FrameBuffer with the given width and height.
    ///
    /// Returns an error if the dimensions exceed the maximum allowed size.
    pub fn new(w: i32, h: i32) -> Result<Self, &'static str> {
        if w < 0 || w > MAX_WIDTH || h < 0 || h > MAX_HEIGHT {
            return Err("Invalid dimensions: exceeds maximum allowed size.");
        }

        let framebuffer = vec![
            Pixel {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                depth: 0.0,
            };
            (w * h) as usize
        ];

        Ok(Self {
            width: w,
            height: h,
            framebuffer,
        })
    }

    /// Checks if the given coordinates are within the bounds of the framebuffer.
    fn check_bounds(&self, x: i32, y: i32) -> Result<(), &'static str> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            Err("Pixel out of bounds")
        } else {
            Ok(())
        }
    }

    /// Sets the color of a pixel at the specified coordinates.
    ///
    /// Returns an error if the coordinates are out of bounds.
    pub fn plot_pixel(
        &mut self,
        x: i32,
        y: i32,
        red: f32,
        green: f32,
        blue: f32,
    ) -> Result<(), &'static str> {
        self.check_bounds(x, y)?;

        let index = (y * self.width + x) as usize;
        self.framebuffer[index].red = red;
        self.framebuffer[index].green = green;
        self.framebuffer[index].blue = blue;

        Ok(())
    }

    /// Sets the depth value of a pixel at the specified coordinates.
    ///
    /// Returns an error if the coordinates are out of bounds.
    pub fn plot_depth(&mut self, x: i32, y: i32, depth: f32) -> Result<(), &'static str> {
        self.check_bounds(x, y)?;

        let index = (y * self.width + x) as usize;
        self.framebuffer[index].depth = depth;

        Ok(())
    }

    /// Gets the depth value of a pixel at the specified coordinates.
    ///
    /// Returns an error if the coordinates are out of bounds.
    pub fn get_depth(&self, x: i32, y: i32) -> Result<f32, &'static str> {
        self.check_bounds(x, y)?;

        let index = (y * self.width + x) as usize;
        Ok(self.framebuffer[index].depth)
    }

    /// Gets the color of a pixel at the specified coordinates.
    ///
    /// Returns an error if the coordinates are out of bounds.
    pub fn get_pixel(&self, x: i32, y: i32) -> Result<(f32, f32, f32), &'static str> {
        self.check_bounds(x, y)?;

        let index = (y * self.width + x) as usize;
        Ok((
            self.framebuffer[index].red,
            self.framebuffer[index].green,
            self.framebuffer[index].blue,
        ))
    }

    /// Writes RGB data to a PPM file.
    pub fn write_rgb_file(&self, filename: &str) -> io::Result<()> {
        // Compute min and max values.
        let (min, max) = self.framebuffer.iter().fold(
            (f32::INFINITY, f32::NEG_INFINITY),
            |(min, max), pixel| {
                let pixel_min = pixel.red.min(pixel.green).min(pixel.blue);
                let pixel_max = pixel.red.max(pixel.green).max(pixel.blue);

                (min.min(pixel_min), max.max(pixel_max))
            },
        );

        // Calculate the difference and avoid division by zero.
        let diff = if max - min == 0.0 { 1.0 } else { max - min };

        let mut outfile = File::create(filename)?;

        // Write the PPM header.
        writeln!(outfile, "P6")?;
        writeln!(outfile, "{} {}", self.width, self.height)?;
        writeln!(outfile, "255")?;

        // Write pixel data
        for pixel in &self.framebuffer {
            let red = (((pixel.red - min) / diff) * 255.0) as u8;
            let green = (((pixel.green) / diff) * 255.0) as u8;
            let blue = (((pixel.blue) / diff) * 255.0) as u8;
            outfile.write_all(&[red, green, blue])?;
        }

        Ok(())
    }

    /// Writes depth data to a PPM file.
    pub fn write_depth_file(&self, filename: &str) -> io::Result<()> {
        // Compute min and max values.
        let (min, max) = self.framebuffer.iter().fold(
            (f32::INFINITY, f32::NEG_INFINITY),
            |(min, max), pixel| {
                let pixel_min = pixel.red.min(pixel.green).min(pixel.blue);
                let pixel_max = pixel.red.max(pixel.green).max(pixel.blue);

                (min.min(pixel_min), max.max(pixel_max))
            },
        );

        // Calculate the difference and avoid division by zero.
        let diff = if max - min == 0.0 { 1.0 } else { max - min };

        let mut outfile = File::create(filename)?;

        // Write the PPM header.
        writeln!(outfile, "P6")?;
        writeln!(outfile, "{} {}", self.width, self.height)?;
        writeln!(outfile, "255")?;

        // Write pixel data
        for pixel in &self.framebuffer {
            let depth = (((pixel.depth - min) / diff) * 255.0) as u8;
            outfile.write_all(&[depth])?;
        }

        Ok(())
    }
}
