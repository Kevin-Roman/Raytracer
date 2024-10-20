use raytracer::{
    cameras::{full_camera::FullCamera, simple_camera::SimpleCamera},
    core::{
        camera::Camera, framebuffer::FrameBuffer, object::Object, scene::Scene, vertex::Vertex,
    },
    materials::falsecolour::FalseColourMaterial,
    objects::sphere::Sphere,
};

fn build_scene(scene: &mut Scene) {
    let mut sphere = Box::new(Sphere::new(Vertex::new(0.0, 0.0, 2.0, 1.0), 1.0));
    let material = Box::new(FalseColourMaterial::new());
    sphere.set_material(material);
    scene.objects.push(sphere);
}

fn main() {
    let width = 512;
    let height = 512;

    let mut fb = match FrameBuffer::new(width, height) {
        Ok(fb) => fb,
        Err(e) => {
            eprintln!("Error creating framebuffer: {}", e);
            return;
        }
    };

    let mut scene = Scene::new();
    build_scene(&mut scene);

    let mut camera = FullCamera::default();
    camera.render(&mut scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/lab34_rgb.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };

    if let Err(e) = fb.write_depth_file("./output/lab34_depth.ppm") {
        eprintln!("Error writing Depth file: {}", e);
    };

    println!("Done")
}
