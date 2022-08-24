use crate::{Color, HitRecord, Material, Ray};

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, crate::Ray)> {
        let reflected = r.dire.unit().reflect(rec.norm);
        if reflected.dot(rec.norm) > 0. {
            Some((self.albedo, Ray::new(rec.p, reflected)))
        } else {
            None
        }
    }
}
