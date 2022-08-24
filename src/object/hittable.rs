use std::rc::Rc;

use crate::{Material, Point3d, Ray, Vec3d};

pub struct HitRecord {
    pub p: Point3d,
    pub norm: Vec3d,
    pub t: f32,
    pub isfr: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        p: Point3d,
        norm: Vec3d,
        t: f32,
        isfr: bool,
        material: Rc<dyn Material>,
    ) -> HitRecord {
        HitRecord {
            p,
            norm,
            t,
            isfr,
            material,
        }
    }
    pub fn from(
        p: Point3d,
        t: f32,
        material: Rc<dyn Material>,
        out_norm: Vec3d,
        r: &Ray,
    ) -> HitRecord {
        let isfr = r.dire.dot(out_norm) < 0.;
        let norm = if isfr { out_norm } else { -out_norm };
        HitRecord::new(p, norm, t, isfr, material)
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
