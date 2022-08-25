mod hittable;
mod plane;
mod scene;
mod sphere;

pub use hittable::{HitRecord, Hittable};
pub use plane::Plane;
pub use scene::Scene;
pub use sphere::Sphere;
