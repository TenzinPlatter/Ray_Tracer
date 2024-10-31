use crate::{
    hit::HitRecord,
    Color,
    Ray,
    Vec3,
    random_f64,
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        fn reflectance(cos: f64, refraction_index: f64) -> f64 {
            let r0: f64 =
                ((1. - refraction_index) / (1. + refraction_index))
                .powi(2);

            r0 + (1. - r0) * (1. - cos).powi(5)
        }

        let attenuation = Color::new(1, 1, 1);

        let ri = if rec.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = r_in.direction.unit_vector();
        
        let cos_theta = f64::min(Vec3::dot(&-unit_dir, &rec.normal), 1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.; 

        let direction = if cannot_refract || reflectance(cos_theta, ri) > random_f64() {
            // must reflect
            Vec3::reflect(&unit_dir, &rec.normal)
        } else {
            Vec3::refract(unit_dir, rec.normal, ri)
        };

        let scattered = Ray::new(rec.point, direction);

        Some((scattered, attenuation))
    }
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric {
            refraction_index,
        }
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        let fuzz = if fuzz < 1. {
            fuzz
        } else {
            1.
        };

        Metal {
            albedo,
            fuzz,
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
        ) -> Option<(Ray, Color)> where Self: Sized
    {
        let mut reflected = Vec3::reflect(&r_in.direction, &rec.normal);

        // fuzzing
        reflected = reflected.unit_vector() + (Vec3::random_unit_vec() * self.fuzz);

        let scattered = Ray::new(rec.point, reflected);
        let attenuation =  self.albedo;

        // absorbed by surface if fuzziness moves ray inside sphere
        if Vec3::dot(&scattered.direction, &rec.normal) <= 0. {
            return None;
        }

        Some((scattered, attenuation))
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord)
        -> Option<(Ray, Color)> where Self: Sized
    {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vec();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((Ray::new(rec.point, scatter_direction), self.albedo))
    }
}
