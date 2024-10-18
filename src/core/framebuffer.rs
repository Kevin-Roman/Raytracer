// The FrameBuffer creates a framebuffer with rgba and depth and can write them to a ppm file.

use std::{
    fs::File,
    io::{self, Write},
};
use thiserror::Error as ThiserrorError;

// Constants for maximum allowed dimensions.
const MAX_WIDTH: i32 = 2048;
const MAX_HEIGHT: i32 = 2048;

#[derive(Clone)]
struct Pixel {
    red: f32,
    green: f32,
    blue: f32,
    depth: f32,
}

#[derive(Debug, ThiserrorError)]
pub enum FrameBufferError {
    #[error("Invalid dimensions: {width}x{height}. Exceeds maximum allowed size.")]
    DimensionError { width: i32, height: i32 },

    #[error("Pixel out of bounds at coordinates ({x}, {y}).")]
    PixelOutOfBounds { x: i32, y: i32 },

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

pub struct FrameBuffer {
    pub width: i32,
    pub height: i32,
    framebuffer: Vec<Pixel>,
}

impl FrameBuffer {
    /// Creates a new FrameBuffer with the given width and height.
    ///
    /// Returns an error if the dimensions exceed the maximum allowed size.
    pub fn new(w: i32, h: i32) -> Result<Self, FrameBufferError> {
        if w < 0 || w > MAX_WIDTH || h < 0 || h > MAX_HEIGHT {
            return Err(FrameBufferError::DimensionError {
                width: w,
                height: h,
            });
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

    pub fn plot_pixel(
        &mut self,
        x: i32,
        y: i32,
        red: f32,
        green: f32,
        blue: f32,
    ) -> Result<(), FrameBufferError> {
        self.check_bounds(x, y)?;

        let index = (y * self.width + x) as usize;
        self.framebuffer[index].red = red;
        self.framebuffer[index].green = green;
        self.framebuffer[index].blue = blue;

        Ok(())
    }

    pub fn plot_depth(&mut self, x: i32, y: i32, depth: f32) -> Result<(), FrameBufferError> {
        self.check_bounds(x, y)?;

        let index = (y * self.width + x) as usize;
        self.framebuffer[index].depth = depth;

        Ok(())
    }

    pub fn get_depth(&self, x: i32, y: i32) -> Result<f32, FrameBufferError> {
        self.check_bounds(x, y)?;

        let index = (y * self.width + x) as usize;
        Ok(self.framebuffer[index].depth)
    }

    /// Gets the color of a pixel at the specified coordinates.
    pub fn get_pixel(&self, x: i32, y: i32) -> Result<(f32, f32, f32), FrameBufferError> {
        self.check_bounds(x, y)?;

        let index = (y * self.width + x) as usize;
        Ok((
            self.framebuffer[index].red,
            self.framebuffer[index].green,
            self.framebuffer[index].blue,
        ))
    }

    /// Writes RGB data to a PPM file.
    pub fn write_rgb_file(&self, filename: &str) -> Result<(), FrameBufferError> {
        let (min, max) = self.compute_min_max_rgb();
        let diff = if max - min == 0.0 { 1.0 } else { max - min };

        let mut outfile = File::create(filename)?;

        // Write the PPM header.
        writeln!(outfile, "P6")?;
        writeln!(outfile, "{} {}", self.width, self.height)?;
        writeln!(outfile, "255")?;

        // Write pixel data
        for pixel in &self.framebuffer {
            // TODO: why subtract min only for red channel?
            let red = (((pixel.red - min) / diff) * 255.0) as u8;
            let green = (((pixel.green - min) / diff) * 255.0) as u8;
            let blue = (((pixel.blue - min) / diff) * 255.0) as u8;
            outfile.write_all(&[red, green, blue])?;
        }

        Ok(())
    }

    /// Writes depth data to a PPM file.
    pub fn write_depth_file(&self, filename: &str) -> Result<(), FrameBufferError> {
        let (min, max) = self.compute_min_max_depth();
        let diff = if max - min == 0.0 { 1.0 } else { max - min };

        let mut outfile = File::create(filename)?;

        // Write the PPM header.
        writeln!(outfile, "P6")?;
        writeln!(outfile, "{} {}", self.width, self.height)?;
        writeln!(outfile, "255")?;

        // Write pixel data
        for pixel in &self.framebuffer {
            let depth = (((pixel.depth - min) / diff) * 255.0) as u8;
            outfile.write_all(&[depth, depth, depth])?;
        }

        Ok(())
    }

    /// Checks if the given coordinates are within the bounds of the framebuffer.
    fn check_bounds(&self, x: i32, y: i32) -> Result<(), FrameBufferError> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            Err(FrameBufferError::PixelOutOfBounds { x, y })
        } else {
            Ok(())
        }
    }

    /// Computes the min and max RGB values for normalisation.
    fn compute_min_max_rgb(&self) -> (f32, f32) {
        self.framebuffer
            .iter()
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), pixel| {
                let pixel_min = pixel.red.min(pixel.green).min(pixel.blue);
                let pixel_max = pixel.red.max(pixel.green).max(pixel.blue);
                (min.min(pixel_min), max.max(pixel_max))
            })
    }

    /// Computes the min and max depth values for normalisation.
    fn compute_min_max_depth(&self) -> (f32, f32) {
        self.framebuffer
            .iter()
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), pixel| {
                (min.min(pixel.depth), max.max(pixel.depth))
            })
    }
}
