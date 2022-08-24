use crate::{Point3d, Ray, Vec3d};

pub struct Camera {
    pub origin: Point3d,
    pub base: Point3d,
    pub horizontal: Vec3d,
    pub vertical: Vec3d,
}

impl Camera {
    pub fn new(origin: Point3d, base: Point3d, horizontal: Vec3d, vertical: Vec3d) -> Camera {
        Camera {
            origin,
            base,
            horizontal,
            vertical,
        }
    }
    pub fn from(vfov: f32, aspect_ratio: f32) -> Camera {
        let h = (vfov / 2.).tan();
        let vp_height = 2. * h;
        let vp_width = aspect_ratio * vp_height;
        let focal_len = 1.;

        let origin = Point3d::new(0., 0., 0.);
        let horizontal = Vec3d::new(vp_width, 0., 0.);
        let vertical = Vec3d::new(0., vp_height, 0.);
        let base = origin - horizontal / 2. - vertical / 2. - Vec3d::new(0., 0., focal_len);

        Camera::new(origin, base, horizontal, vertical)
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.base + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
