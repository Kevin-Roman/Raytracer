use std::io;

use crate::{
    geometry::traits::{Bounded, HitPool, Intersection, Transformable},
    primitives::{ray::Ray, Hit, Transform, Vector, Vertex},
    shading::scene_material::MaterialId,
    utilities::obj_reader::{ObjReader, Triangle},
};

#[derive(Debug)]
struct IntersectionData {
    t: f32,
    u: f32,
    v: f32,
    entering: bool,
}

#[derive(Debug)]
pub struct PolyMeshGeometry {
    pub smooth: bool,
    pub vertices: Vec<Vertex>,
    pub vertex_normals: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

impl PolyMeshGeometry {
    pub fn from_obj(file_path: &str, smooth: bool) -> io::Result<Self> {
        let obj_reader = ObjReader::new(file_path)?;

        Ok(Self {
            smooth,
            vertices: obj_reader.vertices().to_vec(),
            vertex_normals: obj_reader.vertex_normals().to_vec(),
            triangles: obj_reader.triangles(),
        })
    }

    fn add_hit(
        &self,
        hitpool: &mut HitPool,
        triangle_index: usize,
        ray: &Ray,
        intersection: IntersectionData,
    ) {
        let hit_position = ray.position + intersection.t * ray.direction;

        let mut hit_normal = if self.smooth {
            let triangle = &self.triangles[triangle_index];

            // Interpolate using N(t) = (1 - u - v)N_0 + uN_1 + vN_2
            // to get the normal at the specific point inside the triangle.
            (1.0 - intersection.u - intersection.v)
                * self.vertex_normals[triangle.vertex_normal_indices[0]].vector
                + intersection.u * self.vertex_normals[triangle.vertex_normal_indices[1]].vector
                + intersection.v * self.vertex_normals[triangle.vertex_normal_indices[2]].vector
        } else {
            self.triangles[triangle_index].face_normal
        };

        hit_normal = hit_normal.normalise();

        // Flip normal if pointing away from the surface we are looking at.
        if hit_normal.dot(ray.direction) > 0.0 {
            hit_normal = hit_normal.negate();
        }

        hitpool.insert(Hit::new(
            intersection.t,
            intersection.entering,
            hit_position,
            hit_normal,
        ));
    }

    /// Triangle intersection using the `Möller–Trumbore intersection algorithm`.
    /// Non-culling.
    ///
    /// Tomas Möller and Ben Trumbore. 2005. Fast, minimum storage ray/triangle intersection.
    /// In ACM SIGGRAPH 2005 Courses (SIGGRAPH '05). Association for Computing Machinery,
    /// New York, NY, USA, 7–es. https://doi.org/10.1145/1198555.1198746
    fn triangle_intersection(&self, ray: &Ray, triangle_index: usize) -> Option<IntersectionData> {
        // Epsilon for floating point comparison
        const EPSILON: f32 = 0.000001;

        // Retrieve the triangle and its vertices.
        let triangle = &self.triangles[triangle_index];
        let vert0 = &self.vertices[triangle.vertex_indices[0]];
        let vert1 = &self.vertices[triangle.vertex_indices[1]];
        let vert2 = &self.vertices[triangle.vertex_indices[2]];

        // Calculate the edges of the triangle.
        let edge1 = vert1.vector - vert0.vector;
        let edge2 = vert2.vector - vert0.vector;

        // Calculate the determinant.
        let p_vec = ray.direction.cross(edge2);
        let det = edge1.dot(p_vec);

        // If the determinant is near zero, the ray lies in the plane of the triangle.
        if -EPSILON < det && det < EPSILON {
            return None;
        }

        // Calculate the inverse of the determinant.
        let inv_det = 1.0 / det;

        let t_vec = ray.position.vector - vert0.vector;

        // Test bounds.
        let u = t_vec.dot(p_vec) * inv_det;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q_vec = t_vec.cross(edge1);

        // Test bounds for the barycentric coordinate.
        let v = ray.direction.dot(q_vec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // Calculate the distance from the ray origin to the intersection point.
        let t = edge2.dot(q_vec) * inv_det;

        // Determine if the intersection is entering or exiting the triangle.
        let entering = det < 0.0;

        Some(IntersectionData { t, u, v, entering })
    }
}

impl Intersection for PolyMeshGeometry {
    fn intersect(&self, ray: &Ray, hitpool: &mut HitPool) {
        // For each triangle in the model.
        for i in 0..self.triangles.len() {
            if let Some(intersection) = self.triangle_intersection(ray, i) {
                self.add_hit(hitpool, i, ray, intersection);
            }
        }
    }
}

impl Transformable for PolyMeshGeometry {
    fn transform(&mut self, trans: &Transform) {
        for vertex in &mut self.vertices {
            trans.apply_to_vertex(vertex);
        }

        for vertex_normal in &mut self.vertex_normals {
            // 1. Inverse - undo scaling.
            // 2. Transpose - preserve perpendicular relationship to the surface.
            trans.inverse().transpose().apply_to_vertex(vertex_normal);
        }

        for triangle in &mut self.triangles {
            trans
                .inverse()
                .transpose()
                .apply_to_vector(&mut triangle.face_normal)
        }
    }
}

impl Bounded for PolyMeshGeometry {
    fn bounding_sphere(&self) -> Option<(Vertex, f32)> {
        if self.vertices.is_empty() {
            return None;
        }

        // Get min and max for each all components:
        let min = self
            .vertices
            .iter()
            .fold(self.vertices[0].vector, |acc, vertex| {
                Vector::new(
                    acc.x.min(vertex.vector.x),
                    acc.y.min(vertex.vector.y),
                    acc.z.min(vertex.vector.z),
                )
            });

        let max = self
            .vertices
            .iter()
            .fold(self.vertices[0].vector, |acc, vertex| {
                Vector::new(
                    acc.x.max(vertex.vector.x),
                    acc.y.max(vertex.vector.y),
                    acc.z.max(vertex.vector.z),
                )
            });

        let center = (min + max) / 2.0;

        Some((
            Vertex::new(center.x, center.y, center.z, 1.0),
            self.vertices
                .iter()
                .map(|vertex| (vertex.vector - center).length())
                .fold(0.0, |acc, length| acc.max(length)),
        ))
    }
}

#[derive(Debug)]
pub struct PolyMesh {
    pub geometry: PolyMeshGeometry,
    pub material_id: MaterialId,
}

impl PolyMesh {
    pub fn new(file_path: &str, smooth: bool) -> io::Result<Self> {
        Ok(Self {
            geometry: PolyMeshGeometry::from_obj(file_path, smooth)?,
            material_id: MaterialId::default(),
        })
    }

    pub fn with_material(mut self, material_id: MaterialId) -> Self {
        self.material_id = material_id;
        self
    }
}

impl Intersection for PolyMesh {
    fn intersect(&self, ray: &Ray, hitpool: &mut HitPool) {
        self.geometry.intersect(ray, hitpool)
    }
}

impl Transformable for PolyMesh {
    fn transform(&mut self, trans: &Transform) {
        self.geometry.transform(trans)
    }
}

impl Bounded for PolyMesh {
    fn bounding_sphere(&self) -> Option<(Vertex, f32)> {
        self.geometry.bounding_sphere()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn create_triangle() -> PolyMeshGeometry {
        let vertices = vec![
            Vertex::new(0.0, 0.0, 0.0, 1.0),
            Vertex::new(1.0, 0.0, 0.0, 1.0),
            Vertex::new(0.5, 1.0, 0.0, 1.0),
        ];

        let vertex_normals = vec![
            Vertex::new(0.0, 0.0, 1.0, 1.0),
            Vertex::new(0.0, 0.0, 1.0, 1.0),
            Vertex::new(0.0, 0.0, 1.0, 1.0),
        ];

        let triangles = vec![Triangle {
            vertex_indices: [0, 1, 2],
            vertex_normal_indices: [0, 1, 2],
            face_normal: Vector::new(0.0, 0.0, 1.0),
        }];

        PolyMeshGeometry {
            smooth: false,
            vertices,
            vertex_normals,
            triangles,
        }
    }

    #[test]
    fn test_triangle_intersection_hit() {
        let mesh = create_triangle();

        // Ray pointing straight at the triangle
        let ray = Ray::new(Vertex::new(0.5, 0.5, -1.0, 1.0), Vector::new(0.0, 0.0, 1.0));

        let intersection = mesh.triangle_intersection(&ray, 0);
        assert!(intersection.is_some());
    }

    #[test]
    fn test_triangle_intersection_miss() {
        let mesh = create_triangle();

        // Ray pointing away from triangle
        let ray = Ray::new(Vertex::new(5.0, 5.0, -1.0, 1.0), Vector::new(0.0, 0.0, 1.0));

        let intersection = mesh.triangle_intersection(&ray, 0);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_triangle_intersection_parallel() {
        let mesh = create_triangle();

        // Ray parallel to triangle plane
        let ray = Ray::new(Vertex::new(0.5, 0.5, 0.0, 1.0), Vector::new(1.0, 0.0, 0.0));

        let intersection = mesh.triangle_intersection(&ray, 0);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_add_hit_normal_calculation() {
        let mesh = create_triangle();
        let mut hitpool = HitPool::new();

        let ray = Ray::new(Vertex::new(0.5, 0.5, -1.0, 1.0), Vector::new(0.0, 0.0, 1.0));

        let intersection = IntersectionData {
            t: 1.0,
            u: 0.3,
            v: 0.3,
            entering: true,
        };

        mesh.add_hit(&mut hitpool, 0, &ray, intersection);

        assert_eq!(hitpool.len(), 1);
        let hits = hitpool.flatten();
        assert_eq!(hits[0].entering, true);
        // Normal is flipped to face the incoming ray
        assert_relative_eq!(hits[0].normal.z, -1.0, epsilon = 1e-5);
    }

    #[test]
    fn test_smoothness() {
        let mut flat_mesh = create_triangle();
        flat_mesh.smooth = false;

        let mut smooth_mesh = create_triangle();
        smooth_mesh.smooth = true;

        let ray = Ray::new(Vertex::new(0.5, 0.5, -1.0, 1.0), Vector::new(0.0, 0.0, 1.0));

        let mut flat_hitpool = HitPool::new();
        let mut smooth_hitpool = HitPool::new();

        flat_mesh.intersect(&ray, &mut flat_hitpool);
        smooth_mesh.intersect(&ray, &mut smooth_hitpool);

        assert_eq!(flat_hitpool.len(), 1);
        assert_eq!(smooth_hitpool.len(), 1);
    }

    #[test]
    fn test_bounding_sphere_calculation() {
        let mesh = create_triangle();
        let bounding = mesh.bounding_sphere();

        assert!(bounding.is_some());

        let (center, radius) = bounding.unwrap();

        assert!(center.vector.x >= 0.0 && center.vector.x <= 1.0);
        assert!(center.vector.y >= 0.0 && center.vector.y <= 1.0);

        for vertex in &mesh.vertices {
            let distance = (vertex.vector - center.vector).length();
            assert!(distance <= radius + 1e-5);
        }
    }

    #[test]
    fn test_bounding_sphere_empty_mesh() {
        let empty_mesh = PolyMeshGeometry {
            smooth: false,
            vertices: Vec::new(),
            vertex_normals: Vec::new(),
            triangles: Vec::new(),
        };

        let bounding = empty_mesh.bounding_sphere();
        assert!(bounding.is_none());
    }

    #[test]
    fn test_transform_vertices() {
        let mut mesh = create_triangle();
        let original_vertices = mesh.vertices.clone();

        let translation = Transform::new([
            [1.0, 0.0, 0.0, 5.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        mesh.transform(&translation);

        // Vertices should be translated
        for i in 0..mesh.vertices.len() {
            assert_relative_eq!(
                mesh.vertices[i].vector.x,
                original_vertices[i].vector.x + 5.0,
                epsilon = 1e-5
            );
        }
    }

    #[test]
    fn test_normal_flipping() {
        let mesh = create_triangle();

        // Ray from behind (should flip normal)
        let ray = Ray::new(Vertex::new(0.5, 0.5, 1.0, 1.0), Vector::new(0.0, 0.0, -1.0));

        let mut hitpool = HitPool::new();
        mesh.intersect(&ray, &mut hitpool);

        if hitpool.len() > 0 {
            let hits = hitpool.flatten();
            // Normal should be flipped to face the ray
            assert!(hits[0].normal.dot(ray.direction) <= 0.0);
        }
    }

    #[test]
    fn test_multiple_triangles_intersection() {
        let mut mesh = create_triangle();

        mesh.vertices.push(Vertex::new(0.0, 0.0, 1.0, 1.0));
        mesh.vertices.push(Vertex::new(1.0, 0.0, 1.0, 1.0));
        mesh.vertices.push(Vertex::new(0.5, 1.0, 1.0, 1.0));
        mesh.vertex_normals.push(Vertex::new(0.0, 0.0, 1.0, 1.0));
        mesh.vertex_normals.push(Vertex::new(0.0, 0.0, 1.0, 1.0));
        mesh.vertex_normals.push(Vertex::new(0.0, 0.0, 1.0, 1.0));
        mesh.triangles.push(Triangle {
            vertex_indices: [3, 4, 5],
            vertex_normal_indices: [3, 4, 5],
            face_normal: Vector::new(0.0, 0.0, 1.0),
        });

        let ray = Ray::new(Vertex::new(0.5, 0.5, -1.0, 1.0), Vector::new(0.0, 0.0, 1.0));

        let mut hitpool = HitPool::new();
        mesh.intersect(&ray, &mut hitpool);

        assert!(hitpool.len() > 0);
    }
}
