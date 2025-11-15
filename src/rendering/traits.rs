use super::framebuffer::FrameBuffer;

pub trait Camera<Scene> {
    fn render(&mut self, scene: &Scene, fb: &mut FrameBuffer);
}
