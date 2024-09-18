use raytracer::core::{framebuffer::FrameBuffer, object::Object, transform::Transform};
use raytracer::graphics::linedrawer::draw_line;
use raytracer::objects::polymesh::PolyMesh;
use std::{
    error::Error,
    io::{self, ErrorKind},
    result::Result,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Create a framebuffer.
    let mut fb = FrameBuffer::new(512, 512)
        .map_err(|err| Box::new(io::Error::new(ErrorKind::Other, err)))?;

    let transform: Transform = Transform::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, -55.0],
        [0.0, 1.0, 0.0, 20.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let mut pm: PolyMesh = PolyMesh::new("teapot.obj", false);
    pm.apply_transform(&transform);

    // For each triangle in the model.
    for i in 0..pm.triangle_count {
        // The following lines project the point onto the 2D image from 3D space.
        let triangle = &pm.triangle[i];
        let vertices = &pm.vertex;

        let x0 = (vertices[triangle[0]].x / vertices[triangle[0]].z) * 256.0 + 256.0;
        let y0 = -(vertices[triangle[0]].y / vertices[triangle[0]].z) * 256.0 + 256.0;
        let x1 = (vertices[triangle[1]].x / vertices[triangle[1]].z) * 256.0 + 256.0;
        let y1 = -(vertices[triangle[1]].y / vertices[triangle[1]].z) * 256.0 + 256.0;
        let x2 = (vertices[triangle[2]].x / vertices[triangle[2]].z) * 256.0 + 256.0;
        let y2 = -(vertices[triangle[2]].y / vertices[triangle[2]].z) * 256.0 + 256.0;

        // Draw the three edges.
        let _ = draw_line(&mut fb, x0 as i32, y0 as i32, x1 as i32, y1 as i32);
        let _ = draw_line(&mut fb, x1 as i32, y1 as i32, x2 as i32, y2 as i32);
        let _ = draw_line(&mut fb, x2 as i32, y2 as i32, x0 as i32, y0 as i32);

        // Print a dot to indicate progress.
        print!(".");
    }

    fb.write_rgb_file("test.ppm")?;

    Ok(())
}
