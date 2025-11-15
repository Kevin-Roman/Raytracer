use raytracer::{
    config::RaytracerConfig,
    geometry::{
        csg::{Mode, CSG},
        quadratic::QuadraticCoefficients,
        traits::Transformable,
        Plane, PolyMesh, Quadratic, SceneObject,
    },
    primitives::{Colour, Transform, Vector, Vertex},
    rendering::{cameras::full::FullCamera, Camera, FrameBuffer, Light},
    scene::Scene,
    shading::SceneMaterial,
    SceneBuilder,
};

fn build_scene(scene: &mut Scene) {
    let floor_material = SceneMaterial::phong(
        Colour::new(0.8, 0.8, 0.8, 1.0),
        Colour::new(0.5, 0.5, 0.5, 1.0),
        Colour::new(0.1, 0.1, 0.1, 1.0),
        20.0,
    );
    let floor_mat_id = scene.add_material(floor_material);
    let floor = Plane::new(0.0, 1.0, 0.0, 10.0).with_material(floor_mat_id);
    scene.add_object(SceneObject::Plane(floor));

    let transform: Transform = Transform::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, -10.0],
        [0.0, 1.0, 0.0, 20.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let polymesh_material = SceneMaterial::phong(
        Colour::new(0.1, 0.1, 0.1, 1.0),
        Colour::new(0.0, 0.5, 0.5, 1.0),
        Colour::new(0.5, 0.5, 0.5, 1.0),
        50.0,
    );
    let polymesh_mat_id = scene.add_material(polymesh_material);

    let mut polymesh = match PolyMesh::new(
        "D:/Other Documents/Programming/Raytracer/src/assets/teapot.obj",
        true,
    ) {
        Ok(polymesh) => polymesh,
        Err(e) => {
            eprintln!("Error reading poly mesh object: {}", e);
            return;
        }
    };
    polymesh.transform(&transform);
    let polymesh_obj = SceneObject::PolyMesh(polymesh.with_material(polymesh_mat_id));
    scene.add_object(polymesh_obj);

    let csg_material = SceneMaterial::global(
        Colour::new(1.0, 1.0, 1.0, 0.0),
        Colour::new(1.0, 1.0, 1.0, 0.0),
        1.52,
    );
    let csg_mat_id = scene.add_material(csg_material);

    // Sphere with radius 3 with centre at [-5, 4, 6]
    // (x + 5)^2 + (y - 4)^2 + (z - 6)^2 = 3^2
    let sphere_1 = Quadratic::new(QuadraticCoefficients {
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
    });

    // Sphere with radius 3 with centre at [-4, 4, 10]
    // (x + 4)^2 + (y - 4)^2 + (z - 10)^2 = 3^2
    let sphere_2 = Quadratic::new(QuadraticCoefficients {
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
    });

    let csg = CSG::new(
        Mode::CsgDiff,
        SceneObject::Quadratic(sphere_2),
        SceneObject::Quadratic(sphere_1),
    )
    .with_material(csg_mat_id);
    scene.add_object(SceneObject::CSG(Box::new(csg)));

    // Lighting.
    scene.add_light(Light::new_directional(
        Vector::new(1.0, -1.0, 1.0),
        Colour::new(1.0, 1.0, 1.0, 0.0),
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
        Vertex::new(0.0, 5.0, 0.0, 1.0),
        Vector::new(0.0, 0.0, 20.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    camera.render(&scene, &mut fb);

    if let Err(e) = fb.write_rgb_file("./output/6_quadratic_rgb.ppm") {
        eprintln!("Error writing RGB file: {}", e);
    };

    if let Err(e) = fb.write_depth_file("./output/6_quadratic_depth.ppm") {
        eprintln!("Error writing Depth file: {}", e);
    };
}
