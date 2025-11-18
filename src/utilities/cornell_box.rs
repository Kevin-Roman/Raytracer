use crate::{
    geometry::Plane,
    primitives::{Colour, Vertex},
    Light, Material, SceneBuilder, SceneObject,
};

pub fn setup_cornell_box<S: SceneBuilder>(scene: &mut S) {
    let config = scene.config();
    let width = config.cornell_box.width;
    let length = config.cornell_box.length;
    let height = config.cornell_box.height;

    // Create materials using enum constructors
    let white_material = Material::phong(
        Colour::new(0.1, 0.1, 0.1, 1.0),
        Colour::new(0.6, 0.6, 0.6, 1.0),
        Colour::new(0.3, 0.3, 0.3, 1.0),
        10.0,
    );

    let red_material = Material::phong(
        Colour::new(0.1, 0.0, 0.0, 1.0),
        Colour::new(0.6, 0.0, 0.0, 1.0),
        Colour::new(0.0, 0.0, 0.0, 1.0),
        10.0,
    );

    let blue_material = Material::phong(
        Colour::new(0.0, 0.0, 0.1, 1.0),
        Colour::new(0.0, 0.0, 0.6, 1.0),
        Colour::new(0.3, 0.3, 0.3, 1.0),
        10.0,
    );

    let black_material = Material::phong(
        Colour::new(0.0, 0.0, 0.0, 1.0),
        Colour::new(0.0, 0.0, 0.0, 1.0),
        Colour::new(0.0, 0.0, 0.0, 1.0),
        0.0,
    );

    let floor = Plane::new(0.0, 1.0, 0.0, 0.0, white_material);
    scene.add_object(SceneObject::from(floor));

    let front_wall = Plane::new(0.0, 0.0, -1.0, length, white_material);
    scene.add_object(SceneObject::from(front_wall));

    let back_wall = Plane::new(0.0, 0.0, 1.0, 0.0, black_material);
    scene.add_object(SceneObject::from(back_wall));

    let ceiling = Plane::new(0.0, -1.0, 0.0, height, white_material);
    scene.add_object(SceneObject::from(ceiling));

    let left_wall = Plane::new(1.0, 0.0, 0.0, width / 2.0, red_material);
    scene.add_object(SceneObject::from(left_wall));

    let right_wall = Plane::new(-1.0, 0.0, 0.0, width / 2.0, blue_material);
    scene.add_object(SceneObject::from(right_wall));

    // Add light
    scene.add_light(Light::new_point(
        Vertex::new(0.0, height - 8.0, length * 0.6, 1.0),
        Colour::new(1.0, 1.0, 1.0, 1.0),
    ));
}
