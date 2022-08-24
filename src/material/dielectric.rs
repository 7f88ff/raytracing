use std::cmp::min_by;

use crate::{Color, HitRecord, Material, Ray};

pub struct Dielectric {
    ri: f32,
}

impl Dielectric {
    pub fn new(ri: f32) -> Dielectric {
        Dielectric { ri }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(crate::Color, crate::Ray)> {
        let ratio_of_ri = if rec.isfr { 1. / self.ri } else { self.ri };
        let unit_dire = r.dire.unit();

        let cos_theta = min_by(-unit_dire.dot(rec.norm), 1., |a, b| {
            a.partial_cmp(b).unwrap()
        });
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let out = if ratio_of_ri * sin_theta <= 1. {
            unit_dire.refract(rec.norm, ratio_of_ri)
        } else {
            unit_dire.reflect(rec.norm)
        };

        Some((Color::new(1., 1., 1.), Ray::new(rec.p, out)))
    }
}
