use raytracer::{
    config::RaytracerConfig,
    geometry::{traits::Transformable, PolyMesh, SceneObject, Sphere},
    primitives::{Colour, Transform, Vector, Vertex},
    rendering::{cameras::sampling::SamplingCamera, Camera, FrameBuffer},
    scene::{PhotonScene, SceneBuilder},
    shading::Material,
    utilities::cornell_box::setup_cornell_box,
};

fn build_scene(scene: &mut PhotonScene) {
    setup_cornell_box(scene);

    let config = scene.config();
    let length = config.cornell_box.length;

    let sphere_material = Material::global(
        Colour::new(1.0, 1.0, 1.0, 1.0),
        Colour::new(1.0, 1.0, 1.0, 1.0),
        1.52,
    );
    let sphere = Sphere::new(
        Vertex::new(-20.0, 20.0, length * 0.7, 1.0),
        10.0,
        sphere_material,
    );
    scene.add_object(SceneObject::Sphere(sphere));

    // Teapot - Phong material
    let teapot_material = Material::phong(
        Colour::new(0.1, 0.1, 0.1, 1.0),
        Colour::new(0.0, 0.5, 0.5, 1.0),
        Colour::new(0.5, 0.5, 0.5, 1.0),
        10.0,
    );

    let mut teapot = match PolyMesh::new(
        "D:/Other Documents/Programming/Raytracer/src/assets/teapot.obj",
        true,
        teapot_material,
    ) {
        Ok(teapot) => teapot,
        Err(e) => {
            eprintln!("Error reading poly mesh object: {}", e);
            return;
        }
    };
    teapot.transform(&Transform::new([
        [1.5, 0.0, 0.0, 15.0],
        [0.0, 0.0, 1.5, 0.0],
        [0.0, 1.5, 0.0, length * 0.6],
        [0.0, 0.0, 0.0, 1.0],
    ]));
    let teapot_obj = SceneObject::PolyMesh(teapot);
    scene.add_object(teapot_obj);
}

fn main() {
    let config = RaytracerConfig::new();

    let mut fb = match FrameBuffer::new(&config) {
        Ok(fb) => fb,
        Err(e) => {
            eprintln!("Error creating framebuffer: {}", e);
            return;
        }
    };

    let mut scene = PhotonScene::new(&config);
    build_scene(&mut scene);

    scene.setup();

    let config = *scene.config();
    let cornell_height = config.cornell_box.height;
    let cornell_length = config.cornell_box.length;

    let mut camera_front = SamplingCamera::new(
        0.8,
        Vertex::new(0.0, cornell_height / 2.0, 0.05, 1.0),
        Vector::new(0.0, cornell_height / 2.0, cornell_length),
        Vector::new(0.0, 1.0, 0.0),
        config.camera.num_camera_ray_samples,
    );

    camera_front.render(&scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/8_photon_mapping_rgb_front.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };

    let mut camera_back = SamplingCamera::new(
        0.2,
        Vertex::new(0.0, cornell_height / 2.0, cornell_length - 0.05, 1.0),
        Vector::new(0.0, cornell_height / 2.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        config.camera.num_camera_ray_samples,
    );

    camera_back.render(&scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/8_photon_mapping_rgb_back.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };
}
