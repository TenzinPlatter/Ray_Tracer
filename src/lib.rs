pub mod vec3;
pub mod ray;
pub mod scene;
pub mod hit;
pub mod camera;
pub mod material;
pub mod pool;
pub mod shapes {
    pub mod sphere;
}

use std::rc::Rc;
use core::f64;
use rand::random;

use vec3::Vec3;
use ray::Ray;
use hit::{Hittable, HittableList};
use material::{Dielectric, Lambertian, Metal, Material};
use shapes::sphere::Sphere;

type Point3 = Vec3;
type Color = Vec3;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = f64::consts::PI;

pub fn generate_world() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Rc::from(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::from(Sphere::new(Point3::new(0, -1000, 0), 1000, ground_material)));
    
    for a in -11..11 {
        let a = a as f64;
        for b in -11..11 {
            let b = b as f64;
            let choose_mat = random_f64();
            let center = Point3::new(a + 0.9 * random_f64(), 0.2, b + 0.9 * random_f64());
            
            let sphere_material: Rc<dyn Material>;

            if (center - Point3::new(4, 0.2, 0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::from(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random_range(0.5, 1);
                    let fuzz = random_range_f64(0, 0.5);
                    sphere_material = Rc::from(Metal::new(albedo, fuzz));
                } else {
                    sphere_material = Rc::from(Dielectric::new(1.5));
                }

                world.add(Rc::from(
                        Sphere::new(center, 0.2, sphere_material.clone())
                ));
            }
        }
    }

    let material1 = Rc::from(Dielectric::new(1.5));
    world.add(Rc::from(Sphere::new(Point3::new(0, 1, 0), 1, material1)));

    let material2 = Rc::from(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::from(Sphere::new(Point3::new(-4, 1, 0), 1, material2)));

    let material3 = Rc::from(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    world.add(Rc::from(Sphere::new(Point3::new(4, 1, 0), 1, material3)));

    world
}

/// random num 0 <= n < 1
pub fn random_f64() -> f64 {
    random()
}

/// random num min <= n < max
pub fn random_range_f64<A: Into<f64>, B: Into<f64>>(min: A, max: B) -> f64 {
    let min: f64 = min.into();
    let max: f64 = max.into();

    random_f64() * (max - min) + min
}

/// Checks if x is between min and max
pub fn surrounds<A: Into<f64>, B: Into<f64>, C: Into<f64>>(min: A, x: B, max: C) -> bool {
    let x = x.into();
    let min = min.into();
    let max = max.into();

    min < x && x < max
}

/// Clamps the value x between min and max
pub fn clamp<A: Into<f64>, B: Into<f64>, C: Into<f64>>(min: A, x: B, max: C) -> f64 {
    let x = x.into();
    let min = min.into();
    let max = max.into();

    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.
}

pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180. / PI
}

/// lerp (linear blend) between two colors
pub fn create_lerp_func(start_color: Color, end_color: Color) -> Box<dyn Fn(&Ray) -> Color> {
    // blendedValue = (1 - a) * startValue + a * endValue
    // Start and end value are colors
    // a is 0 - 1
    Box::from(move |r: &Ray| {
        let unit_direction = r.direction.unit_vector();
        let a = (unit_direction.y() + 1.) * 0.5;
        start_color * (1. - a) + end_color * a
    })
}
