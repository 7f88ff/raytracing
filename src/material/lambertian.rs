use crate::{rand_unit, Color, HitRecord, Material, Ray};

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord) -> Option<(Color, crate::Ray)> {
        let sctr_dire = rec.norm + rand_unit();
        if sctr_dire.near_zero() {
            Some((self.albedo, Ray::new(rec.p, rec.norm)))
        } else {
            Some((self.albedo, Ray::new(rec.p, sctr_dire)))
        }
    }
}
