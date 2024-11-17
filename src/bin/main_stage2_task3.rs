// Stage 2.1: Reflection and Refraction.

use std::rc::Rc;

use raytracer::{
    cameras::full_camera::FullCamera,
    core::{
        camera::Camera, colour::Colour, framebuffer::FrameBuffer, object::Object, scene::Scene,
        vector::Vector, vertex::Vertex,
    },
    lights::directional_light::DirectionalLight,
    materials::ambient_occlusion_material::AmbientOcclusionMaterial,
    objects::{plane_object::Plane, sphere_object::Sphere},
};

fn build_scene(scene: &mut Scene) {
    // Floor.
    let mut floor_plane_object = Box::new(Plane::new(0.0, 1.0, 0.0, 3.0));
    let floor_plane_material = Rc::new(AmbientOcclusionMaterial::new(
        Colour::new(1.0, 1.0, 1.0, 1.0),
        64,
        0.1,
    ));

    floor_plane_object.set_material(floor_plane_material);
    scene.objects.push(floor_plane_object);

    // Object used for shadow.
    let mut sphere_object = Box::new(Sphere::new(Vertex::new(0.0, 0.0, 10.0, 1.0), 3.0));
    let sphere_material = Rc::new(AmbientOcclusionMaterial::new(
        Colour::new(1.0, 1.0, 0.0, 1.0),
        64,
        0.1,
    ));
    sphere_object.set_material(sphere_material);
    scene.objects.push(sphere_object);

    // Lighting.
    let directional_light = Box::new(DirectionalLight::new(
        Vector::new(0.0, -1.0, 0.0),
        Colour::new(1.0, 0.75, 0.75, 1.0),
    ));

    scene.lights.push(directional_light);
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

    let mut camera = FullCamera::new(
        0.5,
        Vertex::new(0.0, 7.0, 0.0, 1.0),
        Vector::new(0.0, -3.0, 20.0),
        Vector::new(0.0, 1.0, 1.0),
    );

    camera.render(&mut scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/stage2_task3_rgb.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };

    if let Err(e) = fb.write_depth_file("./output/stage2_task3_depth.ppm") {
        eprintln!("Error writing Depth file: {}", e);
    };

    println!(" Done")
}
