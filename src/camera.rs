use crate::{rand_in_unit_disk, Point3d, Ray, Vec3d};

pub struct Camera {
    pub origin: Point3d,
    pub base: Point3d,
    pub horizontal: Vec3d,
    pub vertical: Vec3d,
    pub u: Vec3d,
    pub v: Vec3d,
    pub w: Vec3d,
    pub lens_r: f32,
}

impl Camera {
    pub fn new(
        origin: Point3d,
        base: Point3d,
        horizontal: Vec3d,
        vertical: Vec3d,
        u: Vec3d,
        v: Vec3d,
        w: Vec3d,
        lens_r: f32,
    ) -> Camera {
        Camera {
            origin,
            base,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_r,
        }
    }
    pub fn from(
        lookfrom: Point3d,
        lookat: Vec3d,
        vup: Vec3d,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let h = (vfov / 2.).tan();
        let vp_height = 2. * h;
        let vp_width = aspect_ratio * vp_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * vp_width * focus_dist;
        let vertical = v * vp_height * focus_dist;
        let base = origin - horizontal / 2. - vertical / 2. - w * focus_dist;
        let lens_r = aperture / 2.;

        Camera::new(origin, base, horizontal, vertical, u, v, w, lens_r)
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = rand_in_unit_disk() * self.lens_r;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.base + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
