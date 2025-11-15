use raytracer::{
    config::RaytracerConfig,
    geometry::{Plane, SceneObject, Sphere},
    primitives::{Colour, Vector, Vertex},
    rendering::{cameras::full::FullCamera, Camera, FrameBuffer, Light},
    scene::Scene,
    shading::SceneMaterial,
    SceneBuilder,
};

fn build_scene(scene: &mut Scene) {
    let floor_material = SceneMaterial::ambient_occlusion(Colour::new(1.0, 1.0, 1.0, 1.0), 64, 0.1);
    let floor_mat_id = scene.add_material(floor_material);
    let floor = Plane::new(0.0, 1.0, 0.0, 3.0).with_material(floor_mat_id);
    scene.add_object(SceneObject::Plane(floor));

    let sphere_material =
        SceneMaterial::ambient_occlusion(Colour::new(1.0, 1.0, 0.0, 1.0), 64, 0.1);
    let sphere_mat_id = scene.add_material(sphere_material);
    let sphere = Sphere::new(Vertex::new(0.0, 0.0, 10.0, 1.0), 3.0).with_material(sphere_mat_id);
    scene.add_object(SceneObject::Sphere(sphere));

    scene.add_light(Light::new_directional(
        Vector::new(0.0, -1.0, 0.0),
        Colour::new(1.0, 0.75, 0.75, 1.0),
    ));
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

    let mut scene = Scene::new(&config);
    build_scene(&mut scene);

    let mut camera = FullCamera::new(
        0.5,
        Vertex::new(0.0, 7.0, 0.0, 1.0),
        Vector::new(0.0, -3.0, 20.0),
        Vector::new(0.0, 1.0, 1.0),
    );

    camera.render(&scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/7_ambient_occlusion_rgb.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };

    if let Err(e) = fb.write_depth_file("./output/7_ambient_occlusion_depth.ppm") {
        eprintln!("Error writing Depth file: {}", e);
    };
}
