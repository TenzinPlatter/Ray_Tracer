use crate::Vec3;

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn at(&self, k: f64) -> Vec3 {
        self.origin + self.direction * k
    }
}
