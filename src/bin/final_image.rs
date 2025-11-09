use std::sync::Arc;

use raytracer::{
    cameras::sampling_camera::SamplingCamera,
    config::RaytracerConfig,
    core::{camera::Camera, environment::Environment, framebuffer::FrameBuffer, object::Object},
    environments::photon_scene::PhotonScene,
    materials::{
        ambient_occlusion_material::AmbientOcclusionMaterial, compound_material::CompoundMaterial,
        global_material::GlobalMaterial, phong_material::PhongMaterial,
    },
    objects::{polymesh_object::PolyMesh, sphere_object::Sphere},
    primitives::{colour::Colour, transform::Transform, vector::Vector, vertex::Vertex},
    utilities::cornell_box::setup_cornell_box,
};

fn build_scene<T: Environment>(scene: &mut T) {
    setup_cornell_box(scene, true, true);

    let config = scene.config();
    let length = config.cornell_box.length;

    let mut sphere_object = Box::new(Sphere::new(
        Vertex::new(-20.0, 20.0, length * 0.7, 1.0),
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
        [1.4, 0.0, 0.0, 20.0],
        [0.0, 0.0, 1.4, 0.0],
        [0.0, 1.4, 0.0, length * 0.5],
        [0.0, 0.0, 0.0, 1.0],
    ]));
    teapot.set_material(Arc::new(GlobalMaterial::new(
        Colour::new(0.5, 0.5, 0.5, 1.0),
        Colour::new(0.5, 0.5, 0.5, 1.0),
        0.0,
    )));
    scene.add_object(teapot);

    let mut tree = match PolyMesh::new(
        "D:/Other Documents/Programming/Raytracer/src/assets/tree.obj",
        false,
    ) {
        Ok(tree) => Box::new(tree),
        Err(e) => {
            eprintln!("Error reading poly mesh object: {}", e);
            return;
        }
    };
    tree.apply_transform(&Transform::new([
        [6.0, 0.0, 0.0, 10.0],
        [0.0, 6.0, 0.0, 0.0],
        [0.0, 0.0, 6.0, length * 0.65],
        [0.0, 0.0, 0.0, 1.0],
    ]));
    tree.set_material(Arc::new(CompoundMaterial::new(vec![
        Box::new(PhongMaterial::new(
            Colour::default(),
            Colour::new(0.0, 0.6, 0.0, 1.0),
            Colour::new(0.1, 0.3, 0.1, 1.0),
            20.0,
        )),
        Box::new(AmbientOcclusionMaterial::new(
            Colour::new(0.0, 0.2, 0.0, 1.0),
            16,
            0.2,
        )),
    ])));
    scene.add_object(tree);
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

    let mut scene = PhotonScene::new();
    build_scene(&mut scene);

    scene.setup();

    let config = scene.config();
    let cornell_height = config.cornell_box.height;
    let cornell_length = config.cornell_box.length;

    let mut camera_front = SamplingCamera::new(
        0.8,
        Vertex::new(0.0, cornell_height / 2.0, 0.05, 1.0),
        Vector::new(0.0, cornell_height / 2.0, cornell_length),
        Vector::new(0.0, 1.0, 0.0),
        config.camera.num_camera_ray_samples,
    );

    camera_front.render(&mut scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/final_image_rgb_front.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };
}
