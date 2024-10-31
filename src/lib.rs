pub mod vec3;
pub mod ray;
pub mod scene;
pub mod hit;
pub mod camera;
pub mod shapes {
    pub mod sphere;
}

use core::f64;
use rand::random;

use vec3::Vec3;
use ray::Ray;
use hit::{Hittable, HittableList};
use shapes::sphere::Sphere;

type Point3 = Vec3;
type Color = Vec3;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = f64::consts::PI;

/// random num 0 <= n < 1
pub fn random_f64() -> f64 {
    random()
}

pub fn surrounds<A: Into<f64>, B: Into<f64>, C: Into<f64>>(min: A, x: B, max: C) -> bool {
    let x = x.into();
    let min = min.into();
    let max = max.into();

    min < x && x < max
}

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
