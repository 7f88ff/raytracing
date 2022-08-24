use std::{cmp::min_by, ops};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3d {
    pub c: [f32; 3],
}

impl Vec3d {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3d {
        Vec3d { c: [x, y, z] }
    }
    pub fn x(&self) -> f32 {
        self[0]
    }
    pub fn y(&self) -> f32 {
        self[1]
    }
    pub fn z(&self) -> f32 {
        self[2]
    }
    pub fn slen(&self) -> f32 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }
    pub fn len(&self) -> f32 {
        self.slen().sqrt()
    }
    pub fn unit(self) -> Vec3d {
        self / self.len()
    }
    pub fn dot(&self, v: Vec3d) -> f32 {
        self[0] * v[0] + self[1] * v[1] + self[2] * v[2]
    }
    pub fn cross(&self, v: Vec3d) -> Vec3d {
        Vec3d::new(
            self[1] * v[2] - self[2] * v[1],
            self[2] * v[0] - self[0] * v[2],
            self[0] * v[1] - self[1] * v[0],
        )
    }
    pub fn sqrt(&self) -> Vec3d {
        Vec3d::new(self[0].sqrt(), self[1].sqrt(), self[2].sqrt())
    }
    pub fn near_zero(&self) -> bool {
        const EPS: f32 = 1e-8;
        self[0].abs() < EPS && self[1].abs() < EPS && self[2].abs() < EPS
    }
    pub fn reflect(&self, norm: Vec3d) -> Vec3d {
        *self - norm * 2. * self.dot(norm)
    }
    pub fn refract(&self, norm: Vec3d, ratio_of_ri: f32) -> Vec3d {
        let cos_theta = min_by(-self.dot(norm), 1., |a, b| a.partial_cmp(b).unwrap());
        let out_perp = (*self + norm * cos_theta) * ratio_of_ri;
        let out_para = norm * -(1. - out_perp.slen()).abs().sqrt();
        out_para + out_perp
    }
}

impl ops::Index<usize> for Vec3d {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.c[index]
    }
}

impl Into<(f32, f32, f32)> for Vec3d {
    fn into(self) -> (f32, f32, f32) {
        (self[0], self[1], self[2])
    }
}

impl ops::Neg for Vec3d {
    type Output = Vec3d;
    fn neg(self) -> Self::Output {
        Vec3d::new(-self[0], -self[1], -self[2])
    }
}

impl ops::Add<Vec3d> for Vec3d {
    type Output = Vec3d;
    fn add(self, rhs: Vec3d) -> Self::Output {
        Vec3d::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl ops::Sub<Vec3d> for Vec3d {
    type Output = Vec3d;
    fn sub(self, rhs: Vec3d) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Mul<f32> for Vec3d {
    type Output = Vec3d;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3d::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl ops::Mul<Vec3d> for Vec3d {
    type Output = Vec3d;
    fn mul(self, rhs: Vec3d) -> Self::Output {
        Vec3d::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl ops::Div<f32> for Vec3d {
    type Output = Vec3d;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1. / rhs)
    }
}

pub type Point3d = Vec3d;
