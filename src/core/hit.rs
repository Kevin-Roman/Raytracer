// Hit is stores and manipulates information about an intersection
// between a ray and an object.

use super::{object::Object, tex_coords::TexCoords, vector::Vector, vertex::Vertex};
use std::{fmt, rc::Rc};

pub struct Hit {
    pub t: f32,                       // The intersection distance.
    pub entering: bool,               // True if entering an object, false if leaving.
    pub next: Option<Rc<Hit>>,        // The next hit in a list along a ray.
    pub what: Option<Rc<dyn Object>>, // The primitive object that has been hit.
    pub position: Vertex,             // The position of intersection.
    pub normal: Vector,               // The normal at the point of intersection.
    pub coordinates: TexCoords,       // The texture coordinates at the point of intersection.
}

impl Hit {
    pub fn new(
        t: f32,
        entering: bool,
        position: Vertex,
        normal: Vector,
        coordinates: TexCoords,
        what: Option<Rc<dyn Object>>,
    ) -> Rc<Self> {
        Rc::new(Self {
            t,
            entering,
            next: None,
            what,
            position,
            normal,
            coordinates,
        })
    }
}

impl fmt::Display for Hit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Hit{{ position: [{:.2}, {:.2}, {:.2}], normal: [{:.2}, {:.2}, {:.2}] }}",
            self.position.vector.x,
            self.position.vector.y,
            self.position.vector.z,
            self.normal.x,
            self.normal.y,
            self.normal.z
        )
    }
}

pub struct HitPool {
    pool: Vec<Rc<Hit>>,
}

impl HitPool {
    pub fn new() -> Self {
        let mut pool = Vec::with_capacity(100);
        for _ in 0..100 {
            pool.push(Hit::new(
                0.0,
                true,
                Vertex::origin(),
                Vector::zero(),
                TexCoords::new(),
                None,
            ));
        }
        Self { pool }
    }

    pub fn allocate(&mut self) -> Rc<Hit> {
        self.pool.pop().unwrap_or_else(|| {
            Hit::new(
                0.0,
                true,
                Vertex::origin(),
                Vector::zero(),
                TexCoords::new(),
                None,
            )
        })
    }

    pub fn deallocate(&mut self, hit: Rc<Hit>) {
        self.pool.push(hit);
    }
}
