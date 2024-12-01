use std::sync::Arc;

use crate::{
    core::{environment::Environment, material::Material, object::Object},
    lights::point_light::PointLight,
    materials::{
        ambient_occlusion_material::AmbientOcclusionMaterial, compound_material::CompoundMaterial,
        phong_material::PhongMaterial,
    },
    objects::plane_object::Plane,
    primitives::{colour::Colour, vertex::Vertex},
};

pub const SIDE_LENGTH: f32 = 100.0;
pub const HALF_SIDE_LENGTH: f32 = SIDE_LENGTH / 2.0;

pub fn setup_cornell_box<T: Environment>(scene: &mut T, ambient_occlusion: bool) {
    let white_material: Arc<dyn Material> = if !ambient_occlusion {
        Arc::new(PhongMaterial::new(
            Colour::new(0.2, 0.2, 0.2, 1.0),
            Colour::new(0.8, 0.8, 0.8, 1.0),
            Colour::new(0.1, 0.1, 0.1, 1.0),
            10.0,
        ))
    } else {
        Arc::new(CompoundMaterial::new(vec![
            Box::new(PhongMaterial::new(
                Colour::default(),
                Colour::new(0.8, 0.8, 0.8, 1.0),
                Colour::new(0.1, 0.1, 0.1, 1.0),
                10.0,
            )),
            Box::new(AmbientOcclusionMaterial::new(
                Colour::new(0.2, 0.2, 0.2, 1.0),
                64,
                0.2,
            )),
        ]))
    };

    let red_material: Arc<dyn Material> = if !ambient_occlusion {
        Arc::new(PhongMaterial::new(
            Colour::new(0.2, 0.0, 0.0, 1.0),
            Colour::new(0.8, 0.0, 0.0, 1.0),
            Colour::new(0.0, 0.0, 0.0, 1.0),
            1.0,
        ))
    } else {
        Arc::new(CompoundMaterial::new(vec![
            Box::new(PhongMaterial::new(
                Colour::default(),
                Colour::new(0.8, 0.0, 0.0, 1.0),
                Colour::new(0.0, 0.0, 0.0, 1.0),
                1.0,
            )),
            Box::new(AmbientOcclusionMaterial::new(
                Colour::new(0.2, 0.0, 0.0, 1.0),
                64,
                0.2,
            )),
        ]))
    };

    let green_material: Arc<dyn Material> = if !ambient_occlusion {
        Arc::new(PhongMaterial::new(
            Colour::new(0.0, 0.2, 0.0, 1.0),
            Colour::new(0.0, 0.8, 0.0, 1.0),
            Colour::new(0.0, 0.0, 0.0, 1.0),
            1.0,
        ))
    } else {
        Arc::new(CompoundMaterial::new(vec![
            Box::new(PhongMaterial::new(
                Colour::default(),
                Colour::new(0.0, 0.8, 0.0, 1.0),
                Colour::new(0.0, 0.0, 0.0, 1.0),
                1.0,
            )),
            Box::new(AmbientOcclusionMaterial::new(
                Colour::new(0.0, 0.2, 0.0, 1.0),
                64,
                0.2,
            )),
        ]))
    };

    let mut floor = Plane::new(0.0, 1.0, 0.0, 0.0);
    floor.set_material(white_material.clone());
    scene.add_object(Box::new(floor));

    let mut front_wall = Plane::new(0.0, 0.0, -1.0, SIDE_LENGTH);
    front_wall.set_material(white_material.clone());
    scene.add_object(Box::new(front_wall));

    let mut back_wall = Plane::new(0.0, 0.0, 1.0, 0.0);
    back_wall.set_material(white_material.clone());
    scene.add_object(Box::new(back_wall));

    let mut ceiling = Plane::new(0.0, -1.0, 0.0, SIDE_LENGTH);
    ceiling.set_material(white_material.clone());
    scene.add_object(Box::new(ceiling));

    let mut left_wall = Plane::new(1.0, 0.0, 0.0, HALF_SIDE_LENGTH);
    left_wall.set_material(red_material.clone());
    scene.add_object(Box::new(left_wall));

    let mut right_wall = Plane::new(-1.0, 0.0, 0.0, HALF_SIDE_LENGTH);
    right_wall.set_material(green_material.clone());
    scene.add_object(Box::new(right_wall));

    let point_light = PointLight::new(
        Vertex::new(0.0, SIDE_LENGTH - 2.0, HALF_SIDE_LENGTH * 1.5, 1.0),
        Colour::new(1.0, 1.0, 1.0, 1.0),
    );
    scene.add_light(Box::new(point_light));
}
