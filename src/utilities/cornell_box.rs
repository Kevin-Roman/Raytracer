use std::rc::Rc;

use crate::{
    core::object::Object,
    environments::scene::Scene,
    lights::point_light::PointLight,
    materials::phong_material::PhongMaterial,
    objects::plane_object::Plane,
    primitives::{colour::Colour, vertex::Vertex},
};

pub const SIDE_LENGTH: f32 = 100.0;
pub const HALF_SIDE_LENGTH: f32 = SIDE_LENGTH / 2.0;

pub fn setup_cornell_box(scene: &mut Scene) {
    let white_material = PhongMaterial::new(
        Colour::new(0.2, 0.2, 0.2, 1.0),
        Colour::new(0.8, 0.8, 0.8, 1.0),
        Colour::new(0.1, 0.1, 0.1, 1.0),
        10.0,
    );
    let red_material = PhongMaterial::new(
        Colour::new(0.2, 0.0, 0.0, 1.0),
        Colour::new(0.8, 0.0, 0.0, 1.0),
        Colour::new(0.0, 0.0, 0.0, 1.0),
        1.0,
    );
    let green_material = PhongMaterial::new(
        Colour::new(0.0, 0.2, 0.0, 1.0),
        Colour::new(0.0, 0.8, 0.0, 1.0),
        Colour::new(0.0, 0.0, 0.0, 1.0),
        1.0,
    );

    let mut floor = Plane::new(0.0, 1.0, 0.0, 0.0);
    floor.set_material(Rc::new(white_material.clone()));
    scene.objects.push(Box::new(floor));

    let mut front_wall = Plane::new(0.0, 0.0, -1.0, SIDE_LENGTH);
    front_wall.set_material(Rc::new(white_material.clone()));
    scene.objects.push(Box::new(front_wall));

    let mut back_wall = Plane::new(0.0, 0.0, 1.0, 0.0);
    back_wall.set_material(Rc::new(white_material.clone()));
    scene.objects.push(Box::new(back_wall));

    let mut ceiling = Plane::new(0.0, -1.0, 0.0, SIDE_LENGTH);
    ceiling.set_material(Rc::new(white_material.clone()));
    scene.objects.push(Box::new(ceiling));

    let mut left_wall = Plane::new(1.0, 0.0, 0.0, HALF_SIDE_LENGTH);
    left_wall.set_material(Rc::new(red_material.clone()));
    scene.objects.push(Box::new(left_wall));

    let mut right_wall = Plane::new(-1.0, 0.0, 0.0, HALF_SIDE_LENGTH);
    right_wall.set_material(Rc::new(green_material.clone()));
    scene.objects.push(Box::new(right_wall));

    let point_light = PointLight::new(
        Vertex::new(0.0, SIDE_LENGTH - 2.0, HALF_SIDE_LENGTH * 1.25, 1.0),
        Colour::new(1.0, 1.0, 1.0, 1.0),
    );
    scene.lights.push(Box::new(point_light));
}
