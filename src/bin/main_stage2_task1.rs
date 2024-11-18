// Stage 2.1: Reflection and Refraction.

use std::rc::Rc;

use raytracer::{
    cameras::full_camera::FullCamera,
    core::{camera::Camera, framebuffer::FrameBuffer, material::Material, object::Object},
    environments::scene::Scene,
    lights::directional_light::DirectionalLight,
    materials::{global_material::GlobalMaterial, phong_material::PhongMaterial},
    objects::{plane_object::Plane, polymesh_object::PolyMesh, sphere_object::Sphere},
    primitives::{colour::Colour, transform::Transform, vector::Vector, vertex::Vertex},
};

fn build_scene(scene: &mut Scene) {
    // Floor.
    let mut floor_plane_object = Box::new(Plane::new(0.0, 1.0, 0.0, 10.0));
    let floor_plane_material = Rc::new(PhongMaterial::new(
        Colour::new(0.8, 0.8, 0.8, 1.0),
        Colour::new(0.5, 0.5, 0.5, 1.0),
        Colour::new(0.1, 0.1, 0.1, 1.0),
        20.0,
    ));

    floor_plane_object.set_material(floor_plane_material);

    scene.objects.push(floor_plane_object);

    // Main teapot object.
    let transform: Transform = Transform::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, -10.0],
        [0.0, 1.0, 0.0, 20.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

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

    let polymesh_material: Rc<dyn Material> = Rc::new(PhongMaterial::new(
        Colour::new(0.1, 0.1, 0.1, 1.0),
        Colour::new(0.0, 0.5, 0.5, 1.0),
        Colour::new(0.5, 0.5, 0.5, 1.0),
        50.0,
    ));
    polymesh_object.set_material(polymesh_material);
    scene.objects.push(polymesh_object);

    // Object used for shadow.
    let mut sphere_object = Box::new(Sphere::new(Vertex::new(-4.0, 4.0, 10.0, 1.0), 3.0));
    let sphere_material = Rc::new(GlobalMaterial::new(
        Colour::new(1.0, 1.0, 1.0, 0.0),
        Colour::new(1.0, 1.0, 1.0, 0.0),
        1.52,
    ));

    sphere_object.set_material(sphere_material);

    scene.objects.push(sphere_object);

    // Lighting.
    let directional_light = Box::new(DirectionalLight::new(
        Vector::new(1.0, -1.0, 1.0),
        Colour::new(1.0, 1.0, 1.0, 0.0),
    ));
    scene.lights.push(directional_light);
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

    let mut camera = FullCamera::new(
        0.5,
        Vertex::new(0.0, 7.0, 0.0, 1.0),
        Vector::new(0.0, -3.0, 20.0),
        Vector::new(0.0, 1.0, 1.0),
    );

    camera.render(&mut scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/stage2_task1_rgb.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };

    if let Err(e) = fb.write_depth_file("./output/stage2_task1_depth.ppm") {
        eprintln!("Error writing Depth file: {}", e);
    };

    println!(" Done")
}
