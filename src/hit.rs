use std::vec::Vec;
use std::rc::Rc;
use crate::{
    Ray,
    Vec3,
    material::Material,
};

type Point3 = Vec3;

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// Moves object
    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_t_so_far = ray_tmax;

        for object in &self.objects {
            match object.hit(r, ray_tmin, closest_t_so_far) {
                None => continue,
                Some(record) => {
                    closest_t_so_far = record.t;
                    hit_record = Some(record);
                }
            }
        }

        hit_record
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HitRecord {
    /// r is the ray that hit the surface
    /// This function will check if the ray hit inside or outside of surface
    pub fn new(point: Point3, normal: Vec3, t: f64, r: &Ray, material: Rc<dyn Material>) -> HitRecord {
        let mut res = HitRecord {
            point,
            normal,
            t,
            // default value to allow function creation
            front_face: false,
            material,
        };

        res.set_face_normal(r, &normal);

        res
    }

    /// Assumes normal vector 'n' is of unit length and is the outward normal
    pub fn set_face_normal(&mut self, r: &Ray, n: &Vec3) {
        // direction of ray and outward facing normal are in same direction
        self.front_face = Vec3::dot(&r.direction, n) < 0.;
        if !self.front_face {
            self.normal = -self.normal;
        }
    }
}
