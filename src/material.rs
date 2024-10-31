use crate::{
    hit::HitRecord,
    Color,
    Ray,
    Vec3,
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Ray, Color);
}

pub struct Lambertian {
    albedo: Color,
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal {
            albedo,
        }
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord
        ) -> (Ray, Color) where Self: Sized
    {
        let reflected = Vec3::reflect(&r_in.direction, &rec.normal);
        (Ray::new(rec.point, reflected), self.albedo)
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord)
        -> (Ray, Color) where Self: Sized
    {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vec();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        (Ray::new(rec.point, scatter_direction), self.albedo)
    }
}
