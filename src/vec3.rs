use std::ops;

use crate::{
    clamp,
    random_f64,
    surrounds,
};

#[derive(Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new
        <A: Into<f64>, B: Into<f64>, C: Into<f64>>
        (x: A, y: B, z: C) -> Vec3 {
        Vec3 { e: [x.into(), y.into(), z.into()] }
    }

    pub fn reflect(&self, surface_normal: &Vec3) -> Vec3 {
        *self - *surface_normal * 2. * Vec3::dot(self, surface_normal)
    }

    pub fn near_zero(&self) -> bool {
        let k = 1e-8;

        self.e[0].abs() < k
            && self.e[1].abs() < k
            && self.e[2].abs() < k
    }

    pub fn linear_to_gamma(linear_value: f64) -> f64 {
        if linear_value > 0. {
            linear_value.sqrt()
        } else {
            0.
        }
    }

    pub fn random() -> Vec3 {
        Vec3::new(
            random_f64(),
            random_f64(),
            random_f64(),
        )
    }

    pub fn random_range<A: Into<f64>, B: Into<f64>>(min: A, max: B) -> Vec3 {
        let min: f64 = min.into();
        let max: f64 = max.into();

        Vec3::new(
            random_f64() * (max - min) + min,
            random_f64() * (max - min) + min,
            random_f64() * (max - min) + min,
        )
    }

    pub fn random_unit_vec() -> Vec3 {
        loop {
            let point_in_unit_cube = Vec3::random_range(-1, 1);
            let magnitude = point_in_unit_cube.length_squared();
            let unit_sphere_radius = 1.;

            // 1e-160 is smallest we can get without running risk of 1/0 division
            // when normalising
            if surrounds(1e-160, magnitude, unit_sphere_radius) {
                return point_in_unit_cube / magnitude.sqrt();
            }
        }
    }

    pub fn random_unit_vec_on_hemisphere(surface_normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vec();

        // if surface normal in same direction then unit vec is pointing
        // out of sphere
        // else invert it
        if Vec3::dot(&on_unit_sphere, surface_normal) > 0. {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn x(&self) -> &f64 {
        &self.e[0]
    }

    pub fn y(&self) -> &f64 {
        &self.e[1]
    }

    pub fn z(&self) -> &f64 {
        &self.e[2]
    }

    pub fn get_color_256(&self) -> String {
        let r = (256. * clamp(0, Vec3::linear_to_gamma(*self.x()), 0.999)).floor();
        let g = (256. * clamp(0, Vec3::linear_to_gamma(*self.y()), 0.999)).floor();
        let b = (256. * clamp(0, Vec3::linear_to_gamma(*self.z()), 0.999)).floor();

        format!("{r} {g} {b}")
    }

    pub fn get_color_1(&self) -> String {
        format!("{} {} {}", self.x(), self.y(), self.z())
    }

    pub fn print(&self) {
        println!("Vec: {} {} {}", self.x(), self.y(), self.z());
    }

    pub fn dot(&self, other: &Self) -> f64 {
        return 
            self.x() * other.x()
            + self.y() * other.y()
            + self.z() * other.z()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        let mut len = 0.;

        for i in 0..self.e.len() {
            len += self[i] * self[i]
        }

        len
    }

    pub fn as_string(&self) -> String {
        format!("Vec3: ({}, {}, {})", self.x(), self.y(), self.z())
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3::new(
            *self.x() * -1.,
            *self.y() * -1.,
            *self.z() * -1.,
        )
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
        )
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Vec3::new(
            self.x() + rhs,
            self.y() + rhs,
            self.z() + rhs,
        )
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
        )
    }
}

impl ops::Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Vec3::new(
            self.x() - rhs,
            self.y() - rhs,
            self.z() - rhs,
        )
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z(),
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
        )
    }
}

impl ops::Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x() / rhs.x(),
            self.y() / rhs.y(),
            self.z() / rhs.z(),
        )
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(
            self.x() / rhs,
            self.y() / rhs,
            self.z() / rhs,
        )
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        *self
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.x();
        self.e[1] += rhs.y();
        self.e[2] += rhs.z();
    }
}

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.e[0] += rhs;
        self.e[1] += rhs;
        self.e[2] += rhs;
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..self.e.len() {
            self[i] *= rhs[i];
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..self.e.len() {
            self[i] *= rhs;
        }
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        for i in 0..self.e.len() {
            self[i] /= rhs[i];
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        for i in 0..self.e.len() {
            self[i] /= rhs;
        }
    }
}
