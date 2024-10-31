use crate::Vec3;

type Point3 = Vec3;

pub struct Ray {
    pub origin: Point3,
    pub direction: Point3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Ray {
        Ray { origin, direction }
    } 

    pub fn at(&self, k: f64) -> Vec3 {
        self.origin + self.direction * k
    }
}
