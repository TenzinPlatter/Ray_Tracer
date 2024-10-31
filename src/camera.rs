use indicatif::ProgressBar;
use std::fs;

use crate::{
    Hittable,
    Ray,
    Vec3,
    Color,
    Point3,
    INFINITY,
    create_lerp_func,
    random_f64,
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub center: Point3,
    pub samples_per_pixel: u32,
    image_height: u32,
    px00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f32,
}

impl Default for Camera {
    fn default() -> Self {
        // default values to be over written in initialize
        Camera {
            image_width: 100,
            image_height: 0,
            aspect_ratio: 16. / 9.,
            samples_per_pixel: 10,
            center: Point3::new(0, 0, 0),
            px00_loc: Point3::new(0, 0, 0),
            pixel_delta_u: Vec3::new(0, 0, 0),
            pixel_delta_v: Vec3::new(0, 0, 0),
            pixel_samples_scale: 0.,
        }
    }
}

impl Camera {
    /// Returns the vector to a random point in the [-.5, -.5] - [.5, .5] unit square
    fn sample_square() -> Vec3 {
        Vec3::new(
            random_f64() - 0.5,
            random_f64() - 0.5,
            0,
        )

    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Camera::sample_square();

        let pixel_sample = self.px00_loc
            + self.pixel_delta_u * (i as f64 + offset.x())
            + self.pixel_delta_v * (j as f64 + offset.y());

        let ray_origin = self.center;
        let ray_dir = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_dir)
    }
    
    pub fn initialize(&mut self) {
        let image_width = self.image_width;
        let aspect_ratio = self.aspect_ratio;
        let camera_center = self.center;

        let mut image_height: i32 = (image_width as f32 / aspect_ratio) as i32;

        // image must be at least 1 pixel high
        if image_height < 1 {
            image_height = 1
        }

        // viewport height is at least 1px + 1/2px * for inset
        let viewport_height = 2.;

        // height calculated from width/height rather than aspect ratio as ratio
        // is an ideal not actual
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        // distance between camera and viewport on z axis
        // from camera towards viewport is -z
        let focal_length = 1.;

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // u -> x distance between pixels on viewport
        // v -> y distance between pixels on viewport
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // don't really get why this does what it does
        let viewport_upper_left = camera_center
            - Vec3::new(0., 0., focal_length)
            - viewport_u/2.
            - viewport_v/2.;

        // (0, 0) pixel on viewport, as it is inset by 1/2 a pixel on the x and y
        // axis, and focal length away from camera

        let pixel00_loc = viewport_upper_left
            + (pixel_delta_u + pixel_delta_v) * 0.5;


        self.image_height = image_height as u32;
        self.px00_loc = pixel00_loc;
        self.pixel_delta_u = pixel_delta_u;
        self.pixel_delta_v = pixel_delta_v;
        self.pixel_samples_scale = 1. / self.samples_per_pixel as f32;
    }

    pub fn render(&mut self, world: &dyn Hittable, image_path: &str) {
        self.initialize();

        let mut res = String::new();

        res.push_str(&format!("P3\n{} {}\n255\n", self.image_width, self.image_height));

        println!("Generating {} by {} image...", self.image_width, self.image_height);
        let bar = ProgressBar::new((self.image_width * self.image_height) as u64);

        log::info!("Scanlines remaining: ");

        let background_func = create_lerp_func(
            Color::new(1, 1, 1),
            Color::new(0, 0.5, 0.7)
        );

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                bar.inc(1);
                let mut pixel_color = Color::new(0, 0, 0);

                // color will become total sum of all samples and then
                // be scaled down
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(&r, world, &background_func);
                }

                pixel_color *= self.pixel_samples_scale as f64;
                res.push_str(&(pixel_color.get_color_256() + "\n"))
            }
        }

        log::info!("\rDone.                     \r");
        bar.finish();

        fs::write(image_path, res).expect("Unable to write to file");
    }
}

fn ray_color(
    r: &Ray,
    world: &dyn Hittable,
    background_func: &dyn Fn(&Ray) -> Color
)
    -> Color {
        // if ray collides with an object in hittable world, return color
        match world.hit(r, 0., INFINITY) {
            None => (),
            Some(rec) => {
                return (rec.normal + Color::new(1, 1, 1)) * 0.5;
            }
        }

        // else return background
        background_func(r)
    }
