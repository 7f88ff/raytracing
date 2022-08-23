mod camera;
mod color;
mod hittable;
mod ray;
mod scene;
mod sphere;
mod utils;
mod vec3d;

pub use camera::Camera;
pub use color::{scale2rgb, Color};
pub use hittable::{HitRecord, Hittable};
pub use ray::Ray;
pub use scene::Scene;
pub use sphere::Sphere;
pub use utils::clamp;
pub use vec3d::{Point3d, Vec3d};
