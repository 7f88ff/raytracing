use std::f32::consts::PI;

use rand::Rng;

use crate::Vec3d;

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    match x {
        x if x < min => min,
        x if x > max => max,
        _ => x,
    }
}

pub fn rand_unit() -> Vec3d {
    let mut rng = rand::thread_rng();
    let theta = rng.gen_range(0.0..(2. * PI));
    let phi = rng.gen_range(0.0..(2. * PI));
    Vec3d::new(
        theta.sin() * phi.cos(),
        theta.cos() * phi.sin(),
        theta.cos(),
    )
    .unit()
}

pub fn rand_in_unit_disk() -> Vec3d {
    let mut rng = rand::thread_rng();
    let theta = rng.gen_range(0.0..(2. * PI));
    Vec3d::new(theta.cos(), theta.sin(), 0.)
}
