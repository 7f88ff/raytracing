use std::rc::Rc;

use crate::{HitRecord, Hittable, Material, Point3d, Vec3d};

pub struct Plane {
    pub a: Point3d,
    pub norm: Vec3d,
    pub m: Rc<dyn Material>,
}

impl Plane {
    pub fn new(a: Point3d, norm: Vec3d, m: Rc<dyn Material>) -> Plane {
        Plane { a, norm, m }
    }
    pub fn from(a: Point3d, b: Point3d, c: Point3d, m: Rc<dyn Material>) -> Plane {
        Plane {
            a,
            norm: (a - b).cross(a - c),
            m,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &crate::Ray, t_min: f32, t_max: f32) -> Option<crate::HitRecord> {
        let t = (self.norm.dot(self.a) - self.norm.dot(r.ori)) / self.norm.dot(r.dire);
        if t.is_infinite() || t < t_min || t > t_max {
            None
        } else {
            Some(HitRecord::from(r.at(t), t, self.m.clone(), self.norm, &r))
        }
    }
}
