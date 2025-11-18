use raytracer::{
    config::RaytracerConfig,
    geometry::{traits::Transformable, Plane, PolyMesh, SceneObject, Sphere},
    primitives::{Colour, Transform, Vector, Vertex},
    rendering::{cameras::full::FullCamera, Camera, FrameBuffer, Light},
    scene::Scene,
    shading::Material,
    SceneBuilder,
};

fn build_scene(scene: &mut Scene) {
    // Floor - Phong material
    let floor_material = Material::phong(
        Colour::new(0.8, 0.8, 0.8, 1.0),
        Colour::new(0.5, 0.5, 0.5, 1.0),
        Colour::new(0.1, 0.1, 0.1, 1.0),
        20.0,
    );
    let floor = Plane::new(0.0, 1.0, 0.0, 10.0, floor_material);
    scene.add_object(SceneObject::Plane(floor));

    // Main teapot object - Phong material
    let transform: Transform = Transform::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, -10.0],
        [0.0, 1.0, 0.0, 20.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let polymesh_material = Material::phong(
        Colour::new(0.1, 0.1, 0.1, 1.0),
        Colour::new(0.0, 0.5, 0.5, 1.0),
        Colour::new(0.5, 0.5, 0.5, 1.0),
        50.0,
    );

    let mut polymesh = match PolyMesh::new(
        "D:/Other Documents/Programming/Raytracer/src/assets/teapot.obj",
        true,
        polymesh_material,
    ) {
        Ok(polymesh) => polymesh,
        Err(e) => {
            eprintln!("Error reading poly mesh object: {}", e);
            return;
        }
    };
    polymesh.transform(&transform);
    let polymesh_obj = SceneObject::PolyMesh(polymesh);
    scene.add_object(polymesh_obj);

    // Glass sphere - Global material for reflection/refraction
    let sphere_material = Material::global(
        Colour::new(1.0, 1.0, 1.0, 0.0),
        Colour::new(1.0, 1.0, 1.0, 0.0),
        1.52,
    );
    let sphere = Sphere::new(Vertex::new(-4.0, 4.0, 10.0, 1.0), 3.0, sphere_material);
    scene.add_object(SceneObject::Sphere(sphere));

    // Lighting.
    scene.add_light(Light::new_directional(
        Vector::new(-1.0, -1.0, -1.0),
        Colour::new(1.0, 1.0, 1.0, 1.0),
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

    if let Err(e) = fb.write_rgb_file("./output/5_global_material_rgb.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };

    if let Err(e) = fb.write_depth_file("./output/5_global_material_depth.ppm") {
        eprintln!("Error writing Depth file: {}", e);
    };
}
