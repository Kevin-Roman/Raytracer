use std::sync::Arc;

use raytracer::{
    cameras::full_camera::FullCamera,
    core::{camera::Camera, framebuffer::FrameBuffer, object::Object},
    environments::scene::Scene,
    materials::{
        ambient_occlusion_material::AmbientOcclusionMaterial, compound_material::CompoundMaterial,
        global_material::GlobalMaterial, phong_material::PhongMaterial,
    },
    objects::{polymesh_object::PolyMesh, sphere_object::Sphere},
    primitives::{colour::Colour, transform::Transform, vector::Vector, vertex::Vertex},
    utilities::cornell_box::{setup_cornell_box, HALF_SIDE_LENGTH},
};

fn build_scene(scene: &mut Scene) {
    setup_cornell_box(scene, true);

    let mut sphere_object = Box::new(Sphere::new(
        Vertex::new(-20.0, 15.0, HALF_SIDE_LENGTH * 1.25, 1.0),
        15.0,
    ));
    sphere_object.set_material(Arc::new(GlobalMaterial::new(
        Colour::new(1.0, 1.0, 1.0, 1.0),
        Colour::new(1.0, 1.0, 1.0, 1.0),
        1.52,
    )));
    scene.objects.push(sphere_object);

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
        [0.0, 1.5, 0.0, HALF_SIDE_LENGTH * 1.5],
        [0.0, 0.0, 0.0, 1.0],
    ]));
    teapot.set_material(Arc::new(CompoundMaterial::new(vec![
        Box::new(PhongMaterial::new(
            Colour::default(),
            Colour::new(0.0, 0.5, 0.5, 1.0),
            Colour::new(0.5, 0.5, 0.5, 1.0),
            50.0,
        )),
        Box::new(AmbientOcclusionMaterial::new(
            Colour::new(0.2, 0.2, 0.2, 1.0),
            64,
            0.2,
        )),
    ])));
    scene.objects.push(teapot);
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
}
