use std::ops;

#[derive(Copy)]
pub struct Vec3 {
    e: [f64; 3],
}


impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
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

    pub fn write_color(&self) {
        let r = (255.999 * self.x()).floor();
        let g = (255.999 * self.y()).floor();
        let b = (255.999 * self.z()).floor();

        println!("{r} {g} {b}")
    }

    pub fn dot(&self, other: &Self) -> f64 {
        return 
            self.x() * other.x()
            + self.y() + other.y()
            + self.z() + other.z()
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
            self.x() + rhs,
            self.y() + rhs,
            self.z() + rhs,
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
            self.x() + rhs,
            self.y() + rhs,
            self.z() + rhs,
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
            self.x() + rhs,
            self.y() + rhs,
            self.z() + rhs,
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
            self[i] *= rhs[i]
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..self.e.len() {
            self[i] *= rhs
        }
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        for i in 0..self.e.len() {
            self[i] /= rhs[i]
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] += rhs;
        self.e[1] += rhs;
        self.e[2] += rhs;
    }
}
