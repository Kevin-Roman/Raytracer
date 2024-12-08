use std::sync::Arc;

use raytracer::{
    cameras::sampling_camera::SamplingCamera,
    core::{camera::Camera, environment::Environment, framebuffer::FrameBuffer, object::Object},
    environments::photon_scene::PhotonScene,
    materials::{compound_material::CompoundMaterial, global_material::GlobalMaterial},
    objects::{polymesh_object::PolyMesh, sphere_object::Sphere},
    primitives::{colour::Colour, transform::Transform, vector::Vector, vertex::Vertex},
    utilities::cornell_box::{setup_cornell_box, HEIGHT, LENGTH},
};

const NUM_CAMERA_RAY_SAMPLES: u32 = 1;

fn build_scene<T: Environment>(scene: &mut T) {
    setup_cornell_box(scene, false, true);

    let mut sphere_object = Box::new(Sphere::new(
        Vertex::new(-20.0, 20.0, LENGTH * 0.7, 1.0),
        10.0,
    ));
    sphere_object.set_material(Arc::new(GlobalMaterial::new(
        Colour::new(1.0, 1.0, 1.0, 1.0),
        Colour::new(1.0, 1.0, 1.0, 1.0),
        1.52,
    )));
    scene.add_object(sphere_object);

    let mut teapot = match PolyMesh::new(
        "D:/Other Documents/Programming/Raytracer/src/assets/teapot.obj",
        true,
    ) {
        Ok(teapot) => Box::new(teapot),
        Err(e) => {
            eprintln!("Error reading poly mesh object: {}", e);
            return;
        }
    };
    teapot.apply_transform(&Transform::new([
        [1.5, 0.0, 0.0, 15.0],
        [0.0, 0.0, 1.5, 0.0],
        [0.0, 1.5, 0.0, LENGTH * 0.6],
        [0.0, 0.0, 0.0, 1.0],
    ]));
    teapot.set_material(Arc::new(CompoundMaterial::new(vec![
        Box::new(GlobalMaterial::new(
            Colour::new(1.0, 1.0, 1.0, 1.0),
            Colour::new(1.0, 1.0, 1.0, 1.0),
            1.52,
        )),
        // Box::new(AmbientOcclusionMaterial::new(
        //     Colour::new(0.2, 0.2, 0.2, 1.0),
        //     64,
        //     0.2,
        // )),
    ])));
    scene.add_object(teapot);
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

    let mut scene = PhotonScene::new();
    build_scene(&mut scene);

    scene.setup();

    let mut camera_front = SamplingCamera::new(
        0.8,
        Vertex::new(0.0, HEIGHT / 2.0, 0.05, 1.0),
        Vector::new(0.0, HEIGHT / 2.0, LENGTH),
        Vector::new(0.0, 1.0, 0.0),
        NUM_CAMERA_RAY_SAMPLES,
    );

    camera_front.render(&mut scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/showcase_rgb_front.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };

    let mut camera_back = SamplingCamera::new(
        0.2,
        Vertex::new(0.0, HEIGHT / 2.0, LENGTH - 0.05, 1.0),
        Vector::new(0.0, HEIGHT / 2.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        NUM_CAMERA_RAY_SAMPLES,
    );

    camera_back.render(&mut scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/showcase_rgb_back.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };
}
