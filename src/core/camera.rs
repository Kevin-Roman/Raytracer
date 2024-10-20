use super::{environment::Environment, framebuffer::FrameBuffer};

pub trait Camera {
    fn render(&mut self, env: &mut dyn Environment, fb: &mut FrameBuffer);
}
