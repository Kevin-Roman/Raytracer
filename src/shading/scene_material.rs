use crate::{
    primitives::{ray::Ray, Colour, Hit, Vector},
    rendering::raytracer::Raytracer,
    shading::traits::{Shader, SurfaceProperties, BRDF},
};

use super::{
    ambient_occlusion::AmbientOcclusionMaterial, global::GlobalMaterial, phong::PhongMaterial,
};

/// Material ID - index into materials vec.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct MaterialId(pub usize);

#[derive(Clone, Copy, Debug)]
pub enum SceneMaterial {
    Phong(PhongMaterial),
    Global(GlobalMaterial),
    AmbientOcclusion(AmbientOcclusionMaterial),
}

impl SceneMaterial {
    pub fn phong(ambient: Colour, diffuse: Colour, specular: Colour, power: f32) -> Self {
        Self::Phong(PhongMaterial::new(ambient, diffuse, specular, power))
    }

    pub fn reflective(reflectivity: f32) -> Self {
        Self::Global(GlobalMaterial::reflective(reflectivity))
    }

    pub fn transparent(transparency: f32, ior: f32) -> Self {
        Self::Global(GlobalMaterial::transparent(transparency, ior))
    }

    pub fn global(reflect: Colour, refract: Colour, ior: f32) -> Self {
        Self::Global(GlobalMaterial::new(reflect, refract, ior))
    }

    pub fn ambient_occlusion(ambient: Colour, num_samples: u32, min_ambient: f32) -> Self {
        Self::AmbientOcclusion(AmbientOcclusionMaterial::new(
            ambient,
            num_samples,
            min_ambient,
        ))
    }

    /// Check if this material is specular (for photon mapping caustics)
    pub fn is_specular(&self) -> bool {
        match self {
            SceneMaterial::Phong(_) => false,
            SceneMaterial::Global(m) => {
                m.reflect_weight.r > 0.5 || m.reflect_weight.g > 0.5 || m.reflect_weight.b > 0.5
            }
            SceneMaterial::AmbientOcclusion(_) => false,
        }
    }

    /// Check if this material is transparent (for photon mapping)
    pub fn is_transparent(&self) -> bool {
        match self {
            SceneMaterial::Phong(_) => false,
            SceneMaterial::Global(m) => {
                m.refract_weight.r > 0.0 || m.refract_weight.g > 0.0 || m.refract_weight.b > 0.0
            }
            SceneMaterial::AmbientOcclusion(_) => false,
        }
    }

    /// Get index of refraction (for photon mapping transmission)
    pub fn index_of_refraction(&self) -> Option<f32> {
        match self {
            SceneMaterial::Phong(_) => None,
            SceneMaterial::Global(m) => {
                if m.refract_weight.r > 0.0 || m.refract_weight.g > 0.0 || m.refract_weight.b > 0.0
                {
                    Some(m.index_of_refraction)
                } else {
                    None
                }
            }
            SceneMaterial::AmbientOcclusion(_) => None,
        }
    }

    /// Get BRDF value (for photon mapping radiance estimation)
    pub fn brdf(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        self.eval(viewer, light_direction, hit)
    }
}

impl BRDF for SceneMaterial {
    fn eval(&self, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        match self {
            SceneMaterial::Phong(m) => m.eval(viewer, light_direction, hit),
            SceneMaterial::Global(m) => m.eval(viewer, light_direction, hit),
            SceneMaterial::AmbientOcclusion(_) => Colour::default(), // AO doesn't use BRDF
        }
    }
}

impl<R: Raytracer> Shader<R> for SceneMaterial {
    fn shade_ambient(&self, ctx: &R, ray: &Ray, hit: &Hit, recurse_depth: u8) -> Colour {
        match self {
            SceneMaterial::Phong(m) => m.shade_ambient(ctx, ray, hit, recurse_depth),
            SceneMaterial::Global(m) => m.shade_ambient(ctx, ray, hit, recurse_depth),
            SceneMaterial::AmbientOcclusion(m) => m.shade_ambient(ctx, ray, hit, recurse_depth),
        }
    }

    fn shade_light(&self, ctx: &R, viewer: &Vector, light_direction: &Vector, hit: &Hit) -> Colour {
        match self {
            SceneMaterial::Phong(m) => m.shade_light(ctx, viewer, light_direction, hit),
            SceneMaterial::Global(m) => m.shade_light(ctx, viewer, light_direction, hit),
            SceneMaterial::AmbientOcclusion(_) => Colour::default(), // AO doesn't use direct lighting
        }
    }

    fn surface_properties(&self) -> SurfaceProperties {
        match self {
            SceneMaterial::Phong(m) => m.get_surface_properties(),
            SceneMaterial::Global(m) => m.get_surface_properties(),
            SceneMaterial::AmbientOcclusion(m) => m.get_surface_properties(),
        }
    }
}

/// Material storage - a Vec that allows material reuse.
#[derive(Default)]
pub struct MaterialStorage {
    materials: Vec<SceneMaterial>,
}

impl MaterialStorage {
    pub fn new() -> Self {
        Self {
            materials: Vec::new(),
        }
    }

    /// Add a material and return its ID
    pub fn add(&mut self, material: SceneMaterial) -> MaterialId {
        let id = self.materials.len();
        self.materials.push(material);
        MaterialId(id)
    }

    /// Get a material by ID
    pub fn get(&self, id: usize) -> Option<&SceneMaterial> {
        self.materials.get(id)
    }

    /// Get all materials
    pub fn materials(&self) -> &[SceneMaterial] {
        &self.materials
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_id_creation() {
        let id = MaterialId(5);
        assert_eq!(id.0, 5);
    }

    #[test]
    fn test_material_id_default() {
        let id = MaterialId::default();
        assert_eq!(id.0, 0);
    }

    #[test]
    fn test_scene_material_phong() {
        let ambient = Colour::new(0.1, 0.1, 0.1, 1.0);
        let diffuse = Colour::new(0.6, 0.6, 0.6, 1.0);
        let specular = Colour::new(0.3, 0.3, 0.3, 1.0);

        let material = SceneMaterial::phong(ambient, diffuse, specular, 32.0);

        match material {
            SceneMaterial::Phong(m) => {
                assert_eq!(m.control_factor, 32.0);
            }
            _ => panic!("Expected Phong material"),
        }
    }

    #[test]
    fn test_scene_material_reflective() {
        let material = SceneMaterial::reflective(0.8);
        assert_eq!(material.is_specular(), true);
    }

    #[test]
    fn test_scene_material_transparent() {
        let material = SceneMaterial::transparent(0.9, 1.5);
        assert_eq!(material.is_transparent(), true);
        assert_eq!(material.index_of_refraction(), Some(1.5));
    }

    #[test]
    fn test_scene_material_global() {
        let reflect = Colour::new(0.6, 0.6, 0.6, 1.0); // > 0.5 so it's specular
        let refract = Colour::new(0.3, 0.3, 0.3, 1.0);
        let material = SceneMaterial::global(reflect, refract, 1.5);

        // Global material is specular if any reflectivity component > 0.5
        assert!(material.is_specular());
        // Should be transparent since refract weight > 0
        assert!(material.is_transparent());
        // Check IOR
        assert_eq!(material.index_of_refraction(), Some(1.5));
    }

    #[test]
    fn test_material_storage_add() {
        let mut storage = MaterialStorage::new();
        let material =
            SceneMaterial::phong(Colour::default(), Colour::default(), Colour::default(), 1.0);

        let id = storage.add(material);
        assert_eq!(id.0, 0);
    }

    #[test]
    fn test_material_storage_get() {
        let mut storage = MaterialStorage::new();
        let material = SceneMaterial::reflective(0.5);

        let id = storage.add(material);
        let retrieved = storage.get(id.0);

        assert!(retrieved.is_some());
    }

    #[test]
    fn test_material_storage_multiple() {
        let mut storage = MaterialStorage::new();

        let id1 = storage.add(SceneMaterial::reflective(0.5));
        let id2 = storage.add(SceneMaterial::transparent(0.8, 1.5));
        let id3 = storage.add(SceneMaterial::phong(
            Colour::default(),
            Colour::default(),
            Colour::default(),
            1.0,
        ));

        assert_eq!(id1.0, 0);
        assert_eq!(id2.0, 1);
        assert_eq!(id3.0, 2);
    }
}
