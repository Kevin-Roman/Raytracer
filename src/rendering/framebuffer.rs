use std::io;
use thiserror::Error as ThiserrorError;

use crate::{
    config::RaytracerConfig,
    primitives::{pixel::Pixel, Colour},
    utilities::ppm_writer::PPMWriter,
};

#[derive(Debug, ThiserrorError)]
pub enum FrameBufferError {
    #[error("Invalid dimensions: {width}x{height}. Exceeds maximum allowed size.")]
    DimensionError { width: u16, height: u16 },

    #[error("Pixel out of bounds at coordinates ({x}, {y}).")]
    PixelOutOfBounds { x: i32, y: i32 },

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

/// Creates a framebuffer with rgba and depth and can write them to a ppm file.
pub struct FrameBuffer {
    pub width: u16,
    pub height: u16,
    framebuffer: Vec<Pixel>,
}

impl FrameBuffer {
    /// Creates a new FrameBuffer with the given width and height.
    ///
    /// Returns an error if the dimensions exceeds the maximum allowed size.
    pub fn new(config: &RaytracerConfig) -> Result<Self, FrameBufferError> {
        let width = config.framebuffer.width;
        let height = config.framebuffer.height;

        let max_width = config.framebuffer.max_width;
        let max_height = config.framebuffer.max_height;

        if width > max_width || height > max_height {
            return Err(FrameBufferError::DimensionError { width, height });
        }

        let framebuffer = vec![Pixel::default(); (width as usize) * (height as usize)];

        Ok(Self {
            width,
            height,
            framebuffer,
        })
    }

    pub fn plot_pixel(&mut self, x: i32, y: i32, colour: Colour) -> Result<(), FrameBufferError> {
        self.check_bounds(x, y)?;

        let index = (y * (self.width as i32) + x) as usize;
        self.framebuffer[index].colour = colour;

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

        println!("min: {}, max: {}", min, max);

        let diff = if max - min == 0.0 { 1.0 } else { max - min };

        let ppm_writer = PPMWriter::new(self.width, self.height);
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

        let ppm_writer = PPMWriter::new(self.width, self.height);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config(width: u16, height: u16) -> RaytracerConfig {
        let mut config = RaytracerConfig::default();
        config.framebuffer.width = width;
        config.framebuffer.height = height;
        config
    }

    #[test]
    fn test_framebuffer_plot_pixel() {
        let config = create_test_config(10, 10);
        let mut fb = FrameBuffer::new(&config).unwrap();

        let colour = Colour::new(0.5, 0.6, 0.7, 1.0);
        let result = fb.plot_pixel(5, 5, colour);
        assert!(result.is_ok());

        let pixel = fb.get_pixel(5, 5).unwrap();
        assert_eq!(pixel.colour.r, 0.5);
    }

    #[test]
    fn test_framebuffer_plot_depth() {
        let config = create_test_config(10, 10);
        let mut fb = FrameBuffer::new(&config).unwrap();

        let result = fb.plot_depth(5, 5, 10.0);
        assert!(result.is_ok());

        let pixel = fb.get_pixel(5, 5).unwrap();
        assert_eq!(pixel.depth, 10.0);
    }

    #[test]
    fn test_framebuffer_out_of_bounds() {
        let config = create_test_config(10, 10);
        let mut fb = FrameBuffer::new(&config).unwrap();

        let colour = Colour::new(0.5, 0.6, 0.7, 1.0);
        let result = fb.plot_pixel(15, 5, colour);
        assert!(result.is_err());

        match result {
            Err(FrameBufferError::PixelOutOfBounds { x, y }) => {
                assert_eq!(x, 15);
                assert_eq!(y, 5);
            }
            _ => panic!("Expected PixelOutOfBounds error"),
        }
    }

    #[test]
    fn test_framebuffer_negative_coordinates() {
        let config = create_test_config(10, 10);
        let fb = FrameBuffer::new(&config).unwrap();

        let result = fb.get_pixel(-1, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_framebuffer_get_pixel() {
        let config = create_test_config(10, 10);
        let mut fb = FrameBuffer::new(&config).unwrap();

        let colour = Colour::new(0.1, 0.2, 0.3, 1.0);
        fb.plot_pixel(3, 4, colour).unwrap();
        fb.plot_depth(3, 4, 5.5).unwrap();

        let pixel = fb.get_pixel(3, 4).unwrap();
        assert_eq!(pixel.colour.r, 0.1);
        assert_eq!(pixel.colour.g, 0.2);
        assert_eq!(pixel.depth, 5.5);
    }
}
