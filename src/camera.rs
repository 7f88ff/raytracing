use crate::{Point3d, Ray, Vec3d};

pub struct Camera {
    pub origin: Point3d,
    pub base: Point3d,
    pub horizontal: Vec3d,
    pub vertical: Vec3d,
}

impl Default for Camera {
    fn default() -> Self {
        const ASPECT_RATIO: f32 = 16. / 9.;
        const VP_HEIGHT: f32 = 2.;
        const VP_WIDTH: f32 = ASPECT_RATIO * VP_HEIGHT;
        const FOCAL_LEN: f32 = 1.;

        let origin = Point3d::new(0., 0., 0.);
        let horizontal = Vec3d::new(VP_WIDTH, 0., 0.);
        let vertical = Vec3d::new(0., VP_HEIGHT, 0.);
        let base = origin - horizontal / 2. - vertical / 2. - Vec3d::new(0., 0., FOCAL_LEN);

        Camera::new(origin, base, horizontal, vertical)
    }
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
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.base + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
