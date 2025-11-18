use crate::{
    geometry::traits::{Bounded, HitPool, Intersection, Transformable},
    primitives::{ray::Ray, Transform, Vertex},
    shading::scene_material::MaterialId,
};

use super::{csg::CSG, plane::Plane, polymesh::PolyMesh, quadratic::Quadratic, sphere::Sphere};

#[derive(Debug)]
pub enum SceneObject {
    Sphere(Sphere),
    Plane(Plane),
    Quadratic(Quadratic),
    PolyMesh(PolyMesh),
    CSG(Box<CSG>), // Boxed because CSG is recursive
}

impl SceneObject {
    pub fn material_id(&self) -> MaterialId {
        match self {
            SceneObject::Sphere(s) => s.material_id,
            SceneObject::Plane(p) => p.material_id,
            SceneObject::Quadratic(q) => q.material_id,
            SceneObject::PolyMesh(pm) => pm.material_id,
            SceneObject::CSG(csg) => csg.material_id,
        }
    }

    pub fn set_material_id(&mut self, material_id: MaterialId) {
        match self {
            SceneObject::Sphere(s) => s.material_id = material_id,
            SceneObject::Plane(p) => p.material_id = material_id,
            SceneObject::Quadratic(q) => q.material_id = material_id,
            SceneObject::PolyMesh(pm) => pm.material_id = material_id,
            SceneObject::CSG(csg) => csg.material_id = material_id,
        }
    }
}

impl From<Sphere> for SceneObject {
    fn from(sphere: Sphere) -> Self {
        SceneObject::Sphere(sphere)
    }
}

impl From<Plane> for SceneObject {
    fn from(plane: Plane) -> Self {
        SceneObject::Plane(plane)
    }
}

impl From<Quadratic> for SceneObject {
    fn from(quadratic: Quadratic) -> Self {
        SceneObject::Quadratic(quadratic)
    }
}

impl From<PolyMesh> for SceneObject {
    fn from(polymesh: PolyMesh) -> Self {
        SceneObject::PolyMesh(polymesh)
    }
}

impl From<CSG> for SceneObject {
    fn from(csg: CSG) -> Self {
        SceneObject::CSG(Box::new(csg))
    }
}

impl Intersection for SceneObject {
    fn intersect(&self, ray: &Ray, hitpool: &mut HitPool) {
        match self {
            SceneObject::Sphere(s) => s.intersect(ray, hitpool),
            SceneObject::Plane(p) => p.intersect(ray, hitpool),
            SceneObject::Quadratic(q) => q.intersect(ray, hitpool),
            SceneObject::PolyMesh(pm) => pm.intersect(ray, hitpool),
            SceneObject::CSG(csg) => csg.intersect(ray, hitpool),
        }
    }
}

impl Transformable for SceneObject {
    fn transform(&mut self, trans: &Transform) {
        match self {
            SceneObject::Sphere(s) => s.transform(trans),
            SceneObject::Plane(p) => p.transform(trans),
            SceneObject::Quadratic(q) => q.transform(trans),
            SceneObject::PolyMesh(pm) => pm.transform(trans),
            SceneObject::CSG(csg) => csg.transform(trans),
        }
    }
}

impl Bounded for SceneObject {
    fn bounding_sphere(&self) -> Option<(Vertex, f32)> {
        match self {
            SceneObject::Sphere(s) => s.bounding_sphere(),
            SceneObject::Plane(_) => None,
            SceneObject::Quadratic(_) => None,
            SceneObject::PolyMesh(pm) => pm.bounding_sphere(),
            SceneObject::CSG(_) => None,
        }
    }
}
