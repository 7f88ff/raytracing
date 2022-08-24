use std::rc::Rc;

use crate::{HitRecord, Hittable, Material, Point3d, Ray};

pub struct Sphere {
    pub c: Point3d,
    pub r: f32,
    pub m: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(c: Point3d, r: f32, m: Rc<dyn Material>) -> Sphere {
        Sphere { c, r, m }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.ori - self.c;

        let a = r.dire.slen();
        let h = oc.dot(r.dire);
        let c = oc.slen() - self.r * self.r;
        let d = h * h - a * c;

        if d < 0. {
            None
        } else {
            let rt1 = (-h - d.sqrt()) / a;
            let rt2 = (-h + d.sqrt()) / a;
            if let Some(root) = match (rt1, rt2) {
                (rt1, _) if t_min <= rt1 && rt1 <= t_max => Some(rt1),
                (_, rt2) if t_min <= rt2 && rt2 <= t_max => Some(rt2),
                _ => None,
            } {
                Some(HitRecord::from(
                    r.at(root),
                    root,
                    self.m.clone(),
                    (r.at(root) - self.c) / self.r,
                    &r,
                ))
            } else {
                None
            }
        }
    }
}
