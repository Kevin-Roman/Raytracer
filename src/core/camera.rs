use super::{environment::Environment, framebuffer::FrameBuffer};

/// Camera is the trait that renders the scene.
pub trait Camera<T: Environment + Sync> {
    fn render(&mut self, env: &T, fb: &mut FrameBuffer);
}
