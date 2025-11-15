pub mod csg;
pub mod plane;
pub mod polymesh;
pub mod quadratic;
pub mod scene_object;
pub mod sphere;
pub mod traits;

pub use csg::CSG;
pub use plane::{Plane, PlaneGeometry};
pub use polymesh::{PolyMesh, PolyMeshGeometry};
pub use quadratic::{Quadratic, QuadraticGeometry};
pub use scene_object::SceneObject;
pub use sphere::{Sphere, SphereGeometry};
pub use traits::{Bounded, HitPool, Intersection, Transformable};
