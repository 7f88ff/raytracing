use crate::Vec3d;

pub type Color = Vec3d;

pub fn scale2rgb(c: Color) -> [u8; 3] {
    [
        (255. * c[0]) as u8,
        (255. * c[1]) as u8,
        (255. * c[2]) as u8,
    ]
}
