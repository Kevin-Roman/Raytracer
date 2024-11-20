use super::{environment::Environment, framebuffer::FrameBuffer};

/// Camera is the trait that renders the scene.
pub trait Camera<T: Environment> {
    fn render(&mut self, env: &mut T, fb: &mut FrameBuffer);
}
