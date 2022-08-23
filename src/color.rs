use crate::{clamp, Vec3d};

pub type Color = Vec3d;

pub fn scale2rgb(c: Color) -> [u8; 3] {
    [
        (255. * clamp(c[0], 0., 1.)) as u8,
        (255. * clamp(c[1], 0., 1.)) as u8,
        (255. * clamp(c[2], 0., 1.)) as u8,
    ]
}
