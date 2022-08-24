mod camera;
mod color;
mod material;
mod object;
mod ray;
mod utils;
mod vec3d;

pub use camera::Camera;
pub use color::{scale2rgb, Color};
pub use material::*;
pub use object::*;
pub use ray::Ray;
pub use utils::{clamp, rand_unit, reflect};
pub use vec3d::{Point3d, Vec3d};
