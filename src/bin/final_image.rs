use raytracer::{
    config::RaytracerConfig,
    geometry::{traits::Transformable, PolyMesh, SceneObject, Sphere},
    primitives::{Colour, Transform, Vector, Vertex},
    rendering::{cameras::sampling::SamplingCamera, Camera, FrameBuffer},
    scene::{PhotonScene, SceneBuilder},
    shading::SceneMaterial,
    utilities::cornell_box::setup_cornell_box,
};

fn build_scene<S: SceneBuilder>(scene: &mut S) {
    // Setup cornell box using new idiomatic utility
    setup_cornell_box(scene);

    let config = scene.config();
    let length = config.cornell_box.length;

    let glass_material = SceneMaterial::global(
        Colour::new(1.0, 1.0, 1.0, 1.0),
        Colour::new(1.0, 1.0, 1.0, 1.0),
        1.52,
    );
    let glass_mat_id = scene.add_material(glass_material);

    let sphere =
        Sphere::new(Vertex::new(-20.0, 20.0, length * 0.7, 1.0), 10.0).with_material(glass_mat_id);
    scene.add_object(SceneObject::from(sphere));

    // Create reflective teapot
    let teapot_material = SceneMaterial::global(
        Colour::new(0.5, 0.5, 0.5, 1.0),
        Colour::new(0.5, 0.5, 0.5, 1.0),
        0.0,
    );
    let teapot_mat_id = scene.add_material(teapot_material);

    let mut teapot = match PolyMesh::new(
        "D:/Other Documents/Programming/Raytracer/src/assets/teapot.obj",
        true,
    ) {
        Ok(mesh) => mesh,
        Err(e) => {
            eprintln!("Error reading teapot mesh: {}", e);
            return;
        }
    };

    teapot.geometry.transform(&Transform::new([
        [1.4, 0.0, 0.0, 20.0],
        [0.0, 0.0, 1.4, 0.0],
        [0.0, 1.4, 0.0, length * 0.5],
        [0.0, 0.0, 0.0, 1.0],
    ]));

    let teapot = teapot.with_material(teapot_mat_id);
    scene.add_object(SceneObject::from(teapot));

    // let tree_material = SceneMaterial::ambient_occlusion(Colour::new(0.0, 0.6, 0.0, 1.0), 16, 0.2);
    // let tree_mat_id = scene.add_material(tree_material);

    // let mut tree = match PolyMesh::new(
    //     "D:/Other Documents/Programming/Raytracer/src/assets/tree.obj",
    //     false,
    // ) {
    //     Ok(mesh) => mesh,
    //     Err(e) => {
    //         eprintln!("Error reading tree mesh: {}", e);
    //         return;
    //     }
    // };

    // tree.geometry.transform(&Transform::new([
    //     [6.0, 0.0, 0.0, 10.0],
    //     [0.0, 6.0, 0.0, 0.0],
    //     [0.0, 0.0, 6.0, length * 0.65],
    //     [0.0, 0.0, 0.0, 1.0],
    // ]));

    // let tree = tree.with_material(tree_mat_id);
    // scene.add_object(SceneObject::from(tree));
}

fn main() {
    let config = RaytracerConfig::new();
    println!("{}", config);

    let mut fb = match FrameBuffer::new(&config) {
        Ok(fb) => fb,
        Err(e) => {
            eprintln!("Error creating framebuffer: {}", e);
            return;
        }
    };

    let mut scene = PhotonScene::new(&config);
    build_scene(&mut scene);

    println!(
        "Rendering Cornell Box with {} objects and {} lights...",
        scene.objects.len(),
        scene.lights.len()
    );

    println!("Building photon maps...");
    scene.setup();
    println!("Photon maps built successfully!");

    println!(
        "Using SamplingCamera with {} samples per pixel",
        config.camera.num_camera_ray_samples
    );

    let cornell_height = scene.config.cornell_box.height;
    let cornell_length = scene.config.cornell_box.length;

    let mut camera_front = SamplingCamera::new(
        0.8,
        Vertex::new(0.0, cornell_height / 2.0, 0.05, 1.0),
        Vector::new(0.0, cornell_height / 2.0, cornell_length),
        Vector::new(0.0, 1.0, 0.0),
        config.camera.num_camera_ray_samples,
    );

    camera_front.render(&scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/final_image_rgb_front.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    } else {
        println!("\nRendered: ./output/final_image_rgb_front.ppm");
    }
}
