use crate::{Color, HitRecord, Ray};

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}
