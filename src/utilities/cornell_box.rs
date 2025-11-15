use crate::{
    geometry::Plane,
    primitives::{Colour, Vertex},
    Light, SceneBuilder, SceneMaterial, SceneObject,
};

pub fn setup_cornell_box<S: SceneBuilder>(scene: &mut S) {
    let config = scene.config();
    let width = config.cornell_box.width;
    let length = config.cornell_box.length;
    let height = config.cornell_box.height;

    // Create materials using enum constructors
    let white_material = SceneMaterial::phong(
        Colour::new(0.1, 0.1, 0.1, 1.0),
        Colour::new(0.6, 0.6, 0.6, 1.0),
        Colour::new(0.3, 0.3, 0.3, 1.0),
        10.0,
    );
    let white_mat_id = scene.add_material(white_material);

    let red_material = SceneMaterial::phong(
        Colour::new(0.1, 0.0, 0.0, 1.0),
        Colour::new(0.6, 0.0, 0.0, 1.0),
        Colour::new(0.0, 0.0, 0.0, 1.0),
        10.0,
    );
    let red_mat_id = scene.add_material(red_material);

    let blue_material = SceneMaterial::phong(
        Colour::new(0.0, 0.0, 0.1, 1.0),
        Colour::new(0.0, 0.0, 0.6, 1.0),
        Colour::new(0.3, 0.3, 0.3, 1.0),
        10.0,
    );
    let blue_mat_id = scene.add_material(blue_material);

    // Create walls using composition pattern
    let floor = Plane::new(0.0, 1.0, 0.0, 0.0).with_material(white_mat_id);
    scene.add_object(SceneObject::from(floor));

    let front_wall = Plane::new(0.0, 0.0, -1.0, length).with_material(white_mat_id);
    scene.add_object(SceneObject::from(front_wall));

    let back_wall = Plane::new(0.0, 0.0, 1.0, 0.0).with_material(white_mat_id);
    scene.add_object(SceneObject::from(back_wall));

    let ceiling = Plane::new(0.0, -1.0, 0.0, height).with_material(white_mat_id);
    scene.add_object(SceneObject::from(ceiling));

    let left_wall = Plane::new(1.0, 0.0, 0.0, width / 2.0).with_material(red_mat_id);
    scene.add_object(SceneObject::from(left_wall));

    let right_wall = Plane::new(-1.0, 0.0, 0.0, width / 2.0).with_material(blue_mat_id);
    scene.add_object(SceneObject::from(right_wall));

    // Add light
    scene.add_light(Light::new_point(
        Vertex::new(0.0, height - 8.0, length * 0.6, 1.0),
        Colour::new(1.0, 1.0, 1.0, 1.0),
    ));
}
