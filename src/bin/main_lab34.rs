use raytracer::{
    cameras::full_camera::FullCamera,
    core::{
        camera::Camera, framebuffer::FrameBuffer, object::Object, scene::Scene,
        transform::Transform, vector::Vector, vertex::Vertex,
    },
    materials::falsecolour::FalseColourMaterial,
    objects::polymesh::PolyMesh,
};

fn build_scene(scene: &mut Scene) {
    let transform: Transform = Transform::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, -10.0],
        [0.0, 1.0, 0.0, 20.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let mut pm = match PolyMesh::new(
        "D:/Other Documents/Programming/Raytracer/src/assets/teapot.obj",
        false,
    ) {
        Ok(pm) => Box::new(pm),
        Err(e) => {
            eprintln!("Error reading poly mesh object: {}", e);
            return;
        }
    };
    pm.apply_transform(&transform);

    let material: Box<FalseColourMaterial> = Box::new(FalseColourMaterial::new());
    pm.set_material(material);
    scene.objects.push(pm);
}

fn main() {
    let width = 128;
    let height = 128;

    let mut fb = match FrameBuffer::new(width, height) {
        Ok(fb) => fb,
        Err(e) => {
            eprintln!("Error creating framebuffer: {}", e);
            return;
        }
    };

    let mut scene = Scene::new();
    build_scene(&mut scene);

    // let mut camera = FullCamera::new(
    //     0.5,
    //     Vertex::new(-1.0, 0.0, 1.0, 1.0),
    //     Vector::new(1.0, 0.0, 0.0),
    //     Vector::new(0.0, 1.0, 0.0),
    // );
    let mut camera = FullCamera::new(
        0.5,
        Vertex::new(0.0, 7.0, 0.0, 1.0),
        Vector::new(0.0, -5.0, 20.0),
        Vector::new(0.0, 1.5, 1.0),
    );
    camera.render(&mut scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/lab34_rgb.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };

    if let Err(e) = fb.write_depth_file("./output/lab34_depth.ppm") {
        eprintln!("Error writing Depth file: {}", e);
    };

    println!("Done")
}
