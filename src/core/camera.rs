use super::{environment::Environment, framebuffer::FrameBuffer};

pub const RAYTRACE_RECURSE: u8 = 5;

/// Camera is the trait that renders the scene.
pub trait Camera<T: Environment + Sync> {
    fn render(&mut self, env: &mut T, fb: &mut FrameBuffer);
}
