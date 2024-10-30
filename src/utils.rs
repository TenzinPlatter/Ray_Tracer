use crate::{Ray, Vec3};

type Point3 = Vec3;

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    /// r is the ray that hit the surface
    /// This function will check if the ray hit inside or outside of surface
    pub fn new(point: Point3, normal: Vec3, t: f64, r: &Ray) -> HitRecord {
        let mut res = HitRecord {
            point,
            normal,
            t,
            front_face: false,
        };

        res.set_face_normal(r, &normal);

        res
    }

    /// Assumes normal vector 'n' is of unit length and is the outward normal
    pub fn set_face_normal(&mut self, r: &Ray, n: &Vec3) {
        // direction of ray and outward facing normal are in same direction
        self.front_face = Vec3::dot(&r.direction, n) < 0.;
        if self.front_face {
            self.normal = -self.normal;
        }
    }
}
