use raytracer::{
    config::RaytracerConfig,
    core::{framebuffer::FrameBuffer, object::Object},
    objects::polymesh_object::PolyMesh,
    primitives::transform::Transform,
    utilities::linedrawer::draw_line,
};

fn main() {
    let config = RaytracerConfig::default();
    // Create a framebuffer.
    let mut fb = match FrameBuffer::new(&config) {
        Ok(fb) => fb,
        Err(e) => {
            eprintln!("Error creating framebuffer: {}", e);
            return;
        }
    };

    let transform: Transform = Transform::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, -10.0],
        [0.0, 1.0, 0.0, 20.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let mut pm: PolyMesh = match PolyMesh::new(
        "D:/Other Documents/Programming/Raytracer/src/assets/teapot.obj",
        false,
    ) {
        Ok(pm) => pm,
        Err(e) => {
            eprintln!("Error reading poly mesh object: {}", e);
            return;
        }
    };
    pm.apply_transform(&transform);

    let vertices = &pm.vertices;
    // For each triangle in the model.
    for triangle in pm.triangles.iter() {
        // The following lines project the point onto the 2D image from 3D space.
        let x0 = (vertices[triangle.vertex_indices[0]].vector.x
            / vertices[triangle.vertex_indices[0]].vector.z)
            * 256.0
            + 256.0;
        let y0 = -(vertices[triangle.vertex_indices[0]].vector.y
            / vertices[triangle.vertex_indices[0]].vector.z)
            * 256.0
            + 256.0;
        let x1 = (vertices[triangle.vertex_indices[1]].vector.x
            / vertices[triangle.vertex_indices[1]].vector.z)
            * 256.0
            + 256.0;
        let y1 = -(vertices[triangle.vertex_indices[1]].vector.y
            / vertices[triangle.vertex_indices[1]].vector.z)
            * 256.0
            + 256.0;
        let x2 = (vertices[triangle.vertex_indices[2]].vector.x
            / vertices[triangle.vertex_indices[2]].vector.z)
            * 256.0
            + 256.0;
        let y2 = -(vertices[triangle.vertex_indices[2]].vector.y
            / vertices[triangle.vertex_indices[2]].vector.z)
            * 256.0
            + 256.0;

        // Draw the three edges.
        let _ = draw_line(&mut fb, x0 as i32, y0 as i32, x1 as i32, y1 as i32);
        let _ = draw_line(&mut fb, x1 as i32, y1 as i32, x2 as i32, y2 as i32);
        let _ = draw_line(&mut fb, x2 as i32, y2 as i32, x0 as i32, y0 as i32);

        // Print a dot to indicate progress.
        print!(".");
    }

    if let Err(e) = fb.write_rgb_file("./output/2_object_reader_rgb.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };
}
