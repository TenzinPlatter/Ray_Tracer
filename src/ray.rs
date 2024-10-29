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
        if self.hit_sphere(Point3::new(0., 0., -1.), 0.5) {
            return Color::new(1., 0., 0.);
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

    fn hit_sphere(&self, center: Point3, radius: f64) -> bool {
        let oc = center - self.origin;

        let a = Vec3::dot(&self.direction, &self.direction);
        let b = -2. * Vec3::dot(&self.direction, &oc);
        let c = Vec3::dot(&oc, &oc) - radius * radius;

        let discriminant = b * b - 4. * a * c;

        discriminant >= 0.
    }
}
