use super::{environment::Environment, framebuffer::FrameBuffer};

/// Camera is the trait that renders the scene.
pub trait Camera {
    fn render(&mut self, env: &mut dyn Environment, fb: &mut FrameBuffer);
}
