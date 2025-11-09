// Stage 2.2: Quadratic Surfaces.

use std::sync::Arc;

use raytracer::{
    cameras::full_camera::FullCamera,
    core::{
        camera::Camera, environment::Environment, framebuffer::FrameBuffer, light::Light,
        material::Material, object::Object,
    },
    environments::scene::Scene,
    materials::{global_material::GlobalMaterial, phong_material::PhongMaterial},
    objects::{
        csg_object::{Mode, CSG},
        plane_object::Plane,
        polymesh_object::PolyMesh,
        quadratic_object::{Quadratic, QuadraticCoefficients},
    },
    primitives::{colour::Colour, transform::Transform, vector::Vector, vertex::Vertex},
};

fn build_scene(scene: &mut Scene) {
    // Floor.
    let mut floor_plane_object = Box::new(Plane::new(0.0, 1.0, 0.0, 10.0));
    let floor_plane_material = Arc::new(PhongMaterial::new(
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

    let polymesh_material: Arc<dyn Material> = Arc::new(PhongMaterial::new(
        Colour::new(0.1, 0.1, 0.1, 1.0),
        Colour::new(0.0, 0.5, 0.5, 1.0),
        Colour::new(0.5, 0.5, 0.5, 1.0),
        50.0,
    ));
    polymesh_object.set_material(polymesh_material);
    scene.objects.push(polymesh_object);

    // Sphere with radius 3 with centre at [-5, 4, 6]
    // (x + 5)^2 + (y - 4)^2 + (z - 6)^2 = 3^2
    let sphere_object_1 = Box::new(Quadratic::new(QuadraticCoefficients {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 5.0,
        e: 1.0,
        f: 0.0,
        g: -4.0,
        h: 1.0,
        i: -6.0,
        j: 68.0,
    }));

    // Sphere with radius 3 with centre at [-4, 4, 10]
    // (x + 4)^2 + (y - 4)^2 + (z - 10)^2 = 3^2
    let sphere_object_2 = Box::new(Quadratic::new(QuadraticCoefficients {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 4.0,
        e: 1.0,
        f: 0.0,
        g: -4.0,
        h: 1.0,
        i: -10.0,
        j: 123.0,
    }));

    let mut csg_object = Box::new(CSG::new(Mode::CsgDiff, sphere_object_2, sphere_object_1));
    let csg_material = Arc::new(GlobalMaterial::new(
        Colour::new(1.0, 1.0, 1.0, 0.0),
        Colour::new(1.0, 1.0, 1.0, 0.0),
        1.52,
    ));

    csg_object.set_material(csg_material);

    scene.objects.push(csg_object);

    // Lighting.
    scene.add_light(Light::new_directional(
        Vector::new(1.0, -1.0, 1.0),
        Colour::new(1.0, 1.0, 1.0, 0.0),
    ));

    // let mut cylinder = Box::new(Quadratic::new(
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -25.0,
    // ));
    // cylinder.apply_transform(&Transform::new([
    //     [1.0, 0.0, 0.0, 0.0],
    //     [0.0, 1.0, 0.0, 0.0],
    //     [0.0, 0.0, 1.0, -20.0],
    //     [0.0, 0.0, 0.0, 1.0],
    // ]));
    // cylinder.set_material(Arc::new(PhongMaterial::new(
    //     Colour::new(0.1, 0.1, 0.1, 1.0),
    //     Colour::new(0.0, 0.5, 0.5, 1.0),
    //     Colour::new(0.5, 0.5, 0.5, 1.0),
    //     50.0,
    // )));

    // scene.objects.push(cylinder);

    // // Lighting.
    // let directional_light = Box::new(DirectionalLight::new(
    //     Vector::new(1.0, -1.0, 1.0),
    //     Colour::new(1.0, 1.0, 1.0, 0.0),
    // ));
    // scene.lights.push(directional_light);
}

fn main() {
    let width = 2048;
    let height = 2048;

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
        Vertex::new(0.0, 5.0, 0.0, 1.0),
        Vector::new(0.0, 0.0, 20.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    camera.render(&mut scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/6_quadratic_rgb.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };

    if let Err(e) = fb.write_depth_file("./output/6_quadratic_depth.ppm") {
        eprintln!("Error writing Depth file: {}", e);
    };
}
