use indicatif::ProgressBar;
use std::sync::{Arc, Mutex};
use std::{fs, cell::RefCell};

use crate::{
    create_lerp_func,
    degrees_to_radians,
    pool::ThreadPool,
    random_f64,
    Color,
    Hittable,
    Point3,
    Ray,
    Vec3,
    INFINITY
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub center: Point3,
    pub samples_per_pixel: u32,
    pub max_ray_bounce_depth: u32,
    pub vfov: u32,
    pub vup: Vec3,
    pub look_from: Point3,
    pub look_at: Point3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    image_height: u32,
    px00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        // default values to be over written in initialize
        Camera {
            image_width: 100,
            image_height: 0,
            vfov: 90,
            vup: Vec3::new(0, 1, 0),
            look_from: Point3::new(0, 0, 0),
            look_at: Point3::new(0, 0, -1),
            aspect_ratio: 43. / 18.,
            samples_per_pixel: 10,
            center: Point3::new(0, 0, 0),
            px00_loc: Point3::new(0, 0, 0),
            pixel_delta_u: Vec3::new(0, 0, 0),
            pixel_delta_v: Vec3::new(0, 0, 0),
            pixel_samples_scale: 0.,
            max_ray_bounce_depth: 10,
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_angle: 0.,
            focus_dist: 10.,
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
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

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p[0]) + (self.defocus_disk_v * p[1])
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {

        let offset = Camera::sample_square();

        let pixel_sample = self.px00_loc
            + self.pixel_delta_u * (i as f64 + offset.x())
            + self.pixel_delta_v * (j as f64 + offset.y());

        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_dir = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_dir)
    }
    
    pub fn initialize(&mut self) {
        self.center = self.look_from;

        let image_width = self.image_width;
        let aspect_ratio = self.aspect_ratio;
        let camera_center = self.center;

        let mut image_height: i32 = (image_width as f32 / aspect_ratio) as i32;

        // image must be at least 1 pixel high
        if image_height < 1 {
            image_height = 1
        }

        // height calculated from width/height rather than aspect ratio as ratio
        // is an ideal not actual
        let actual_aspect_ratio: f64 = image_width as f64 / image_height as f64;

        let theta = degrees_to_radians(self.vfov as f64);
        let h = f64::tan(theta / 2.);
        let viewport_height = 2. * h * self.focus_dist;
        let viewport_width = viewport_height * actual_aspect_ratio;

        self.w = (self.look_from - self.look_at).unit_vector();
        self.u = (Vec3::cross(&self.vup, &self.w)).unit_vector();
        self.v = Vec3::cross(&self.w, &self.u);

        let viewport_u = self.u * viewport_width;
        // invert as real y and internal y coords are opposites
        let viewport_v = -self.v * viewport_height;

        // u -> x distance between pixels on viewport
        // v -> y distance between pixels on viewport
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // don't really get why this does what it does
        let viewport_upper_left = camera_center
            - self.w * self.focus_dist
            - viewport_u/2.
            - viewport_v/2.;

        // (0, 0) pixel on viewport, as it is inset by 1/2 a pixel on the x and y
        // axis, and focal length away from camera

        let pixel00_loc = viewport_upper_left
            + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius = self.focus_dist
            * f64::tan(degrees_to_radians(self.defocus_angle / 2.));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

        self.image_height = image_height as u32;
        self.px00_loc = pixel00_loc;
        self.pixel_delta_u = pixel_delta_u;
        self.pixel_delta_v = pixel_delta_v;
        self.pixel_samples_scale = 1. / self.samples_per_pixel as f32;
    }

    pub fn render(&mut self, world: &dyn Hittable, image_path: &str) {
        self.initialize();

        println!("Generating {} by {} image...", self.image_width, self.image_height);
        let bar = ProgressBar::new((self.image_width * self.image_height) as u64);

        log::info!("Scanlines remaining: ");

        let pool = ThreadPool::new(10);
        let mut pixel_colors = Arc::new(Mutex::new(
                vec![String::new(); (self.image_height * self.image_width) as usize]
                ));

        let world: Arc<&dyn Hittable> = Arc::new(world);
        let image_width = Arc::new(self.image_width);
        let cam_arc = Arc::new(&mut *self);

        for y in 0..cam_arc.image_height {
            for x in 0..cam_arc.image_width {
                bar.inc(1);

                let world = Arc::clone(&world);
                let pixel_colors: Arc<Mutex<Vec<String>>> = Arc::clone(&pixel_colors);
                let i: u32 = x.clone();
                let j: u32 = y.clone();
                let image_width = Arc::clone(&image_width);
                let cam = cam_arc.clone();

                pool.execute(move || {
                    let color = get_pixel_color(cam, i, j, world);
                    let mut colors_array = pixel_colors.lock().unwrap();
                    let idx: usize = (j * *image_width + i).try_into().unwrap();

                    colors_array[idx] = color.get_color_256();
                });
            }
        }

        log::info!("\rDone.                     \r");
        bar.finish();

        let mut res = String::new();

        res.push_str(&format!("P3\n{} {}\n255\n", cam_arc.image_width, cam_arc.image_height));

        for color in pixel_colors.lock().unwrap().iter() {
            res.push_str(&(color.to_owned() + "\n"));
        }

        fs::write(image_path, res).expect("Unable to write to file");
    }
}

fn get_pixel_color(
    cam: Arc<&mut Camera>,
    x: u32,
    y: u32,
    world: Arc<&dyn Hittable>
    ) -> Color {

    let mut pixel_color = Color::new(0, 0, 0);

    // color will become total sum of all samples and then
    // be scaled down
    for _sample in 0..cam.samples_per_pixel {
        let r = cam.get_ray(x, y);
        pixel_color += ray_color(
            &r,
            &world,
            cam.max_ray_bounce_depth,
        );
    }

    pixel_color *= cam.pixel_samples_scale as f64;

    pixel_color
}

fn ray_color(
    r: &Ray,
    world: &Arc<&dyn Hittable>,
    depth: u32,
)
    -> Color
{
        // if max bounces reached collect no more color
        // depth cannot accidentally be less than 0 as it is u32
        if depth == 0 {
            return Color::new(0, 0, 0);
        }

        // if ray collides with an object in hittable world, return color
        if let Some(rec) = world.hit(r, 0.001, INFINITY) {

            if let Some((scattered, attenuation)) = (*rec.material).scatter(r, &rec) {
                return ray_color(&scattered, world, depth - 1)
                    * attenuation;
            } else {
                return Color::new(0, 0, 0);
            }
        }

        let start_color = Color::new(1, 1, 1);
        let end_color = Color::new(1, 0.5, 0.7);

        let unit_direction = r.direction.unit_vector();
        let a = (unit_direction.y() + 1.) * 0.5;
        start_color * (1. - a) + end_color * a
    }
