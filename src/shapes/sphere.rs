use crate::utils::{Hittable, HitRecord};
use crate::Vec3;
use crate::Ray;

type Point3 = Vec3;

struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new<T: Into<f64>>(center: Point3, radius: T) -> Sphere {
        let mut r = radius.into();
        if r < 0. {
            r = 0.
        }

        Sphere {
            center,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - r.origin;

        // math for this in section 6.2

        let a = r.direction.length_squared();
        let h = Vec3::dot(&r.direction, &oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;

        if discriminant < 0. {
            return None;
        };

        let sqrtd = discriminant.sqrt();

        // find nearest hit in acceptable range

        // a value of t
        let mut root = (h - sqrtd) / a;

        fn invalid_root(tmin: f64, tmax: f64, t: f64) -> bool{
            tmin < t && t < tmax 
        }

        if invalid_root(ray_tmin, ray_tmax, root) {
            root = (h + sqrtd) / a;
            if invalid_root(ray_tmin, ray_tmax, root){
                return None;
            }
        };

        let t = root;
        let point = r.at(root);
        let normal = (point - self.center) / self.radius;

        Some(HitRecord::new(point, normal, t, r))
    }
}
