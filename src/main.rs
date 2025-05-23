use std::{
    env,
    time::Instant,
};

use ray_tracer::{
    camera::Camera, generate_world, vec3::Vec3,
};

type Point3 = Vec3;

fn main() {
    // setup logger
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let mut width: u32 = 400;
    let mut image_path = String::from("./imgs/image.ppm");

    (1..args.len()).for_each(|i| {
        if let Ok(x) = args[i].parse::<u32>() {
            width = x;
        } else {
            image_path = args[i].clone();
        }
    });

    let mut camera = Camera::default();
    camera.image_width = 200;
    camera.samples_per_pixel = 100;
    camera.max_ray_bounce_depth = 50;
    camera.aspect_ratio = 16. / 9.;

    camera.look_from = Point3::new(13, 2, 3);
    camera.look_at = Point3::new(0, 0, 0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.;

    let world = generate_world();

    let time_started = Instant::now();

    camera.render(&world, &image_path);

    println!("Render took {} secs", time_started.elapsed().as_secs());
}
