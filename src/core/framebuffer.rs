// The FrameBuffer creates a framebuffer with rgba and depth and can write them to a ppm file.

use std::io;
use thiserror::Error as ThiserrorError;

use crate::{
    primitives::{colour::Colour, pixel::Pixel},
    utilities::ppm_writer::PPMWriter,
};

const MAX_WIDTH: u16 = 2048;
const MAX_HEIGHT: u16 = 2048;

#[derive(Debug, ThiserrorError)]
pub enum FrameBufferError {
    #[error("Invalid dimensions: {width}x{height}. Exceeds maximum allowed size.")]
    DimensionError { width: u16, height: u16 },

    #[error("Pixel out of bounds at coordinates ({x}, {y}).")]
    PixelOutOfBounds { x: i32, y: i32 },

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

pub struct FrameBuffer {
    pub width: u16,
    pub height: u16,
    framebuffer: Vec<Pixel>,
}

impl FrameBuffer {
    /// Creates a new FrameBuffer with the given width and height.
    ///
    /// Returns an error if the dimensions exceeds the maximum allowed size.
    pub fn new(w: u16, h: u16) -> Result<Self, FrameBufferError> {
        if w > MAX_WIDTH || h > MAX_HEIGHT {
            return Err(FrameBufferError::DimensionError {
                width: w,
                height: h,
            });
        }

        let framebuffer = vec![Pixel::default(); (w as usize) * (h as usize)];

        Ok(Self {
            width: w,
            height: h,
            framebuffer,
        })
    }

    pub fn plot_pixel(&mut self, x: i32, y: i32, colour: Colour) -> Result<(), FrameBufferError> {
        self.check_bounds(x, y)?;

        let index = (y * (self.width as i32) + x) as usize;
        self.framebuffer[index].colour.r = colour.r;
        self.framebuffer[index].colour.g = colour.g;
        self.framebuffer[index].colour.b = colour.b;

        Ok(())
    }

    pub fn plot_depth(&mut self, x: i32, y: i32, depth: f32) -> Result<(), FrameBufferError> {
        self.check_bounds(x, y)?;

        let index = (y * (self.width as i32) + x) as usize;
        self.framebuffer[index].depth = depth;

        Ok(())
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Result<&Pixel, FrameBufferError> {
        self.check_bounds(x, y)?;

        let index = (y * (self.width as i32) + x) as usize;
        Ok(&self.framebuffer[index])
    }

    /// Writes RGB data to a PPM file.
    pub fn write_rgb_file(&self, filename: &str) -> Result<(), FrameBufferError> {
        let (min, max) = self.framebuffer.iter().fold(
            (f32::INFINITY, f32::NEG_INFINITY),
            |(min, max), pixel| {
                let pixel_min = pixel.colour.r.min(pixel.colour.g).min(pixel.colour.b);
                let pixel_max = pixel.colour.r.max(pixel.colour.g).max(pixel.colour.b);
                (min.min(pixel_min), max.max(pixel_max))
            },
        );
        let diff = if max - min == 0.0 { 1.0 } else { max - min };

        let ppm_writer = PPMWriter::new(self.width as u16, self.height as u16);
        ppm_writer.write_file(filename, &self.framebuffer, |pixel| {
            let red = (((pixel.colour.r - min) / diff) * 255.0) as u8;
            let green = (((pixel.colour.g - min) / diff) * 255.0) as u8;
            let blue = (((pixel.colour.b - min) / diff) * 255.0) as u8;
            (red, green, blue)
        })?;

        Ok(())
    }

    /// Writes depth data to a PPM file.
    pub fn write_depth_file(&self, filename: &str) -> Result<(), FrameBufferError> {
        let (min, max) = self
            .framebuffer
            .iter()
            .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), pixel| {
                (min.min(pixel.depth), max.max(pixel.depth))
            });
        let diff = if max - min == 0.0 { 1.0 } else { max - min };

        let ppm_writer = PPMWriter::new(self.width as u16, self.height as u16);
        ppm_writer.write_file(filename, &self.framebuffer, |pixel| {
            let depth = (((pixel.depth - min) / diff) * 255.0) as u8;
            (depth, depth, depth)
        })?;

        Ok(())
    }

    /// Checks if the given coordinates are within the bounds of the framebuffer.
    fn check_bounds(&self, x: i32, y: i32) -> Result<(), FrameBufferError> {
        if x < 0 || x >= (self.width as i32) || y < 0 || y >= (self.height as i32) {
            Err(FrameBufferError::PixelOutOfBounds { x, y })
        } else {
            Ok(())
        }
    }
}
