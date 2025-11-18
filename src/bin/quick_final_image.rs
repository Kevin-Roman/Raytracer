use raytracer::{
    config::RaytracerConfig,
    geometry::{traits::Transformable, PolyMesh, SceneObject, Sphere},
    primitives::{Colour, Transform, Vector, Vertex},
    rendering::{cameras::full::FullCamera, Camera, FrameBuffer},
    scene::Scene,
    shading::Material,
    utilities::cornell_box::setup_cornell_box,
    SceneBuilder,
};

fn build_scene(scene: &mut Scene) {
    setup_cornell_box(scene);

    let config = &scene.config;
    let length = config.cornell_box.length;

    let glass_material = Material::global(
        Colour::new(1.0, 1.0, 1.0, 1.0),
        Colour::new(1.0, 1.0, 1.0, 1.0),
        1.52,
    );

    let sphere = Sphere::new(
        Vertex::new(-20.0, 20.0, length * 0.7, 1.0),
        10.0,
        glass_material,
    );
    scene.add_object(SceneObject::from(sphere));

    // Create reflective teapot
    let teapot_material = Material::global(
        Colour::new(0.5, 0.5, 0.5, 1.0),
        Colour::new(0.5, 0.5, 0.5, 1.0),
        0.0,
    );

    let mut teapot = match PolyMesh::new(
        "D:/Other Documents/Programming/Raytracer/src/assets/teapot.obj",
        true,
        teapot_material,
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

    scene.add_object(SceneObject::from(teapot));

    let tree_material = Material::phong(
        Colour::default(),
        Colour::new(0.0, 0.6, 0.0, 1.0),
        Colour::new(0.1, 0.3, 0.1, 1.0),
        20.0,
    );

    let mut tree = match PolyMesh::new(
        "D:/Other Documents/Programming/Raytracer/src/assets/tree.obj",
        false,
        tree_material,
    ) {
        Ok(mesh) => mesh,
        Err(e) => {
            eprintln!("Error reading tree mesh: {}", e);
            return;
        }
    };

    tree.geometry.transform(&Transform::new([
        [6.0, 0.0, 0.0, 10.0],
        [0.0, 6.0, 0.0, 0.0],
        [0.0, 0.0, 6.0, length * 0.65],
        [0.0, 0.0, 0.0, 1.0],
    ]));

    scene.add_object(SceneObject::from(tree));
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

    let mut scene = Scene::new(&config);
    build_scene(&mut scene);

    let cornell_height = scene.config.cornell_box.height;
    let cornell_length = scene.config.cornell_box.length;

    println!(
        "Rendering Cornell Box with {} objects and {} lights...",
        scene.objects.len(),
        scene.lights.len()
    );

    let mut camera_front = FullCamera::new(
        0.8,
        Vertex::new(0.0, cornell_height / 2.0, 0.05, 1.0),
        Vector::new(0.0, cornell_height / 2.0, cornell_length),
        Vector::new(0.0, 1.0, 0.0),
    );

    camera_front.render(&scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/quick_final_image_rgb_front.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    } else {
        println!("\nRendered: ./output/quick_final_image_rgb_front.ppm");
    }
}
