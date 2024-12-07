use std::sync::Arc;

use crate::{
    core::{environment::Environment, material::Material, object::Object},
    lights::point_light::PointLight,
    materials::{
        ambient_occlusion_material::AmbientOcclusionMaterial, compound_material::CompoundMaterial,
        phong_material::PhongMaterial, photon_mapping_material::PhotonMappingMaterial,
    },
    objects::plane_object::Plane,
    primitives::{colour::Colour, vertex::Vertex},
};

pub const WIDTH: f32 = 100.0;
pub const LENGTH: f32 = 150.0;
pub const HEIGHT: f32 = 90.00;

fn create_material(
    mut ambient: Colour,
    diffuse: Colour,
    specular: Colour,
    ambient_occlusion: bool,
    photon_mapping: bool,
) -> Arc<dyn Material> {
    if ambient_occlusion {
        ambient = Colour::default();
    }

    let mut compound_material = CompoundMaterial::new(vec![Box::new(PhongMaterial::new(
        ambient, diffuse, specular, 10.0,
    ))]);

    if ambient_occlusion {
        compound_material.add_material(Box::new(AmbientOcclusionMaterial::new(
            Colour::new(0.1, 0.1, 0.1, 1.0),
            64,
            0.2,
        )));
    }

    if photon_mapping {
        compound_material.add_material(Box::new(PhotonMappingMaterial::new()));
    }

    Arc::new(compound_material)
}

pub fn setup_cornell_box<T: Environment>(
    scene: &mut T,
    ambient_occlusion: bool,
    photon_mapping: bool,
) {
    let white_material = create_material(
        Colour::new(0.1, 0.1, 0.1, 1.0),
        Colour::new(0.6, 0.6, 0.6, 1.0),
        Colour::new(0.3, 0.3, 0.3, 1.0),
        ambient_occlusion,
        photon_mapping,
    );

    let red_material = create_material(
        Colour::new(0.1, 0.0, 0.0, 1.0),
        Colour::new(0.6, 0.0, 0.0, 1.0),
        Colour::new(0.0, 0.0, 0.0, 1.0),
        ambient_occlusion,
        photon_mapping,
    );

    let blue_material = create_material(
        Colour::new(0.0, 0.0, 0.1, 1.0),
        Colour::new(0.0, 0.0, 0.6, 1.0),
        Colour::new(0.3, 0.3, 0.3, 1.0),
        ambient_occlusion,
        photon_mapping,
    );

    let mut floor = Plane::new(0.0, 1.0, 0.0, 0.0);
    floor.set_material(white_material.clone());
    scene.add_object(Box::new(floor));

    let mut front_wall = Plane::new(0.0, 0.0, -1.0, LENGTH);
    front_wall.set_material(white_material.clone());
    scene.add_object(Box::new(front_wall));

    let mut back_wall = Plane::new(0.0, 0.0, 1.0, 0.0);
    back_wall.set_material(white_material.clone());
    scene.add_object(Box::new(back_wall));

    let mut ceiling = Plane::new(0.0, -1.0, 0.0, HEIGHT);
    ceiling.set_material(white_material.clone());
    scene.add_object(Box::new(ceiling));

    let mut left_wall = Plane::new(1.0, 0.0, 0.0, WIDTH / 2.0);
    left_wall.set_material(red_material.clone());
    scene.add_object(Box::new(left_wall));

    let mut right_wall = Plane::new(-1.0, 0.0, 0.0, WIDTH / 2.0);
    right_wall.set_material(blue_material.clone());
    scene.add_object(Box::new(right_wall));

    let point_light = PointLight::new(
        Vertex::new(0.0, HEIGHT - 0.5, LENGTH * 0.7, 1.0),
        Colour::new(1.0, 1.0, 1.0, 1.0),
    );
    scene.add_light(Box::new(point_light));
}
