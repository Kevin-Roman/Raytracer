use std::sync::Arc;

use raytracer::{
    cameras::full_camera::FullCamera,
    config::RaytracerConfig,
    core::{
        camera::Camera, environment::Environment, framebuffer::FrameBuffer, light::Light,
        object::Object,
    },
    environments::scene::Scene,
    materials::phong_material::PhongMaterial,
    objects::{polymesh_object::PolyMesh, sphere_object::Sphere},
    primitives::{colour::Colour, transform::Transform, vector::Vector, vertex::Vertex},
};

fn build_scene(scene: &mut Scene) {
    let transform: Transform = Transform::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, -10.0],
        [0.0, 1.0, 0.0, 20.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    // Main object.
    let mut polymesh_object = match PolyMesh::new(
        "D:/Other Documents/Programming/Raytracer/src/assets/teapot.obj",
        true,
    ) {
        Ok(polymesh_object) => Box::new(polymesh_object),
        Err(e) => {
            eprintln!("Error reading poly mesh object: {}", e);
            return;
        }
    };
    polymesh_object.apply_transform(&transform);

    let polymesh_material = Arc::new(PhongMaterial::new(
        Colour::new(0.1, 0.1, 0.1, 1.0),
        Colour::new(0.0, 0.5, 0.5, 1.0),
        Colour::new(0.5, 0.5, 0.5, 1.0),
        50.0,
    ));
    polymesh_object.set_material(polymesh_material);
    scene.objects.push(polymesh_object);

    // Object used for shadow.
    let mut sphere_object = Box::new(Sphere::new(Vertex::new(-10.0, 0.0, 10.0, 1.0), 3.0));
    let sphere_material = Arc::new(PhongMaterial::new(
        Colour::new(0.1, 0.1, 0.1, 1.0),
        Colour::new(0.0, 0.0, 0.5, 1.0),
        Colour::new(0.3, 0.3, 0.3, 1.0),
        50.0,
    ));
    sphere_object.set_material(sphere_material);

    scene.objects.push(sphere_object);

    // Lighting.
    scene.add_light(Light::new_directional(
        Vector::new(-1.0, -1.0, -1.0),
        Colour::new(1.0, 1.0, 1.0, 1.0),
    ));
}

fn main() {
    let config = RaytracerConfig::default();

    let mut fb = match FrameBuffer::new(&config) {
        Ok(fb) => fb,
        Err(e) => {
            eprintln!("Error creating framebuffer: {}", e);
            return;
        }
    };

    let mut scene = Scene::new();
    build_scene(&mut scene);

    let mut camera = FullCamera::new(
        0.5,
        Vertex::new(0.0, 7.0, 0.0, 1.0),
        Vector::new(0.0, -5.0, 20.0),
        Vector::new(0.0, 1.5, 1.0),
    );

    camera.render(&mut scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/4_phong_material_rgb.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };

    if let Err(e) = fb.write_depth_file("./output/4_phong_material_depth.ppm") {
        eprintln!("Error writing Depth file: {}", e);
    };
}
