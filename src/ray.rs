use crate::{Point3d, Vec3d};

pub struct Ray {
    pub ori: Point3d,
    pub dire: Vec3d,
}

impl Ray {
    pub fn new(ori: Point3d, dire: Vec3d) -> Ray {
        Ray { ori, dire }
    }
    pub fn at(&self, t: f32) -> Point3d {
        self.ori + self.dire * t
    }
}
