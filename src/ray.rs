use crate::Vec3;

type Point3 = Vec3;
type Color = Vec3;

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

    pub fn color(&self) -> Color {
        let sphere_center = Point3::new(0, 0, -1);

        let t = self.hit_sphere(sphere_center, 0.5);
        // hit circle
        if t > 0.0 {
            let n = (self.at(t) - sphere_center).unit_vector();
            return Color::new(n.x() + 1., n.y() + 1., n.z() + 1.) * 0.5;
        }

        // lerp (linear blend) between two colors
        // blendedValue = (1 - a) * startValue + a * endValue
        // Start and end value are colors
        // a is 0 - 1

        // white
        let start_color = Color::new(1., 1., 1.);

        // blue
        let end_color = Color::new(0.5, 0.7, 1.);

        let unit_direction = self.direction.unit_vector();
        let a = (unit_direction.y() + 1.) * 0.5;

        start_color * (1. - a) + end_color * a
    }

    fn hit_sphere(&self, center: Point3, radius: f64) -> f64 {
        let oc = center - self.origin;

        // math for this in section 6.2

        let a = self.direction.length_squared();
        let h = Vec3::dot(&self.direction, &oc);
        let c = oc.length_squared() - radius.powi(2);

        let discriminant = h.powi(2) - a * c;

        if discriminant < 0. {
            // no intersections
            -1.
        } else {
            (h - discriminant.sqrt()) / a
        }
    }
}
