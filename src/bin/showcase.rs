use std::rc::Rc;

use raytracer::{
    cameras::full_camera::FullCamera,
    core::{camera::Camera, framebuffer::FrameBuffer, object::Object},
    environments::scene::Scene,
    materials::phong_material::PhongMaterial,
    objects::sphere_object::Sphere,
    primitives::{colour::Colour, vector::Vector, vertex::Vertex},
    utilities::cornell_box::{setup_cornell_box, HALF_SIDE_LENGTH},
};

fn build_scene(scene: &mut Scene) {
    setup_cornell_box(scene);

    let mut sphere_object = Box::new(Sphere::new(
        Vertex::new(0.0, 6.0, HALF_SIDE_LENGTH * 1.25, 1.0),
        3.0,
    ));
    let sphere_material = Rc::new(PhongMaterial::new(
        Colour::new(0.2, 0.2, 0.2, 1.0),
        Colour::new(0.8, 0.8, 0.8, 1.0),
        Colour::new(0.1, 0.1, 0.1, 1.0),
        10.0,
    ));
    sphere_object.set_material(sphere_material);
    scene.objects.push(sphere_object);
}

fn main() {
    let width = 256;
    let height = 256;

    let mut fb = match FrameBuffer::new(width, height) {
        Ok(fb) => fb,
        Err(e) => {
            eprintln!("Error creating framebuffer: {}", e);
            return;
        }
    };

    let mut scene = Scene::new(Colour::default());
    build_scene(&mut scene);

    let mut camera = FullCamera::new(
        0.5,
        Vertex::new(0.0, HALF_SIDE_LENGTH, 0.05, 1.0),
        Vector::new(0.0, HALF_SIDE_LENGTH, HALF_SIDE_LENGTH),
        Vector::new(0.0, 1.0, 0.0),
    );

    camera.render(&mut scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/showcase_rgb.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };

    if let Err(e) = fb.write_depth_file("./output/showcase_depth.ppm") {
        eprintln!("Error writing Depth file: {}", e);
    };

    println!(" Done")
}
