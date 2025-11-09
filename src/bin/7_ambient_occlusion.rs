// Stage 2.3: Ambient Occlusion.

use std::sync::Arc;

use raytracer::{
    cameras::full_camera::FullCamera,
    config::RaytracerConfig,
    core::{
        camera::Camera, environment::Environment, framebuffer::FrameBuffer, light::Light,
        object::Object,
    },
    environments::scene::Scene,
    materials::ambient_occlusion_material::AmbientOcclusionMaterial,
    objects::{plane_object::Plane, sphere_object::Sphere},
    primitives::{colour::Colour, vector::Vector, vertex::Vertex},
};

fn build_scene(scene: &mut Scene) {
    // Floor.
    let mut floor_plane_object = Box::new(Plane::new(0.0, 1.0, 0.0, 3.0));
    let floor_plane_material = Arc::new(AmbientOcclusionMaterial::new(
        Colour::new(1.0, 1.0, 1.0, 1.0),
        64,
        0.1,
    ));

    floor_plane_object.set_material(floor_plane_material);
    scene.objects.push(floor_plane_object);

    // Object used for shadow.
    let mut sphere_object = Box::new(Sphere::new(Vertex::new(0.0, 0.0, 10.0, 1.0), 3.0));
    let sphere_material = Arc::new(AmbientOcclusionMaterial::new(
        Colour::new(1.0, 1.0, 0.0, 1.0),
        64,
        0.1,
    ));
    sphere_object.set_material(sphere_material);
    scene.objects.push(sphere_object);

    // Lighting.
    scene.add_light(Light::new_directional(
        Vector::new(0.0, -1.0, 0.0),
        Colour::new(1.0, 0.75, 0.75, 1.0),
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
        Vector::new(0.0, -3.0, 20.0),
        Vector::new(0.0, 1.0, 1.0),
    );

    camera.render(&mut scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/7_ambient_occlusion_rgb.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };

    if let Err(e) = fb.write_depth_file("./output/7_ambient_occlusion_depth.ppm") {
        eprintln!("Error writing Depth file: {}", e);
    };
}
