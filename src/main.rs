use std::{
    env,
    rc::Rc,
};

use ray_tracer::{
    hit::HittableList,
    vec3::Vec3,
    camera::Camera,
    shapes::sphere::Sphere,
};

type Point3 = Vec3;

fn main() {
    // setup logger
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let mut width: u32 = 1920;
    let mut image_path = String::from("./imgs/image.ppm");

    (1..args.len()).for_each(|i| {
        if let Ok(x) = args[i].parse::<u32>() {
            width = x;
        } else {
            image_path = args[i].clone();
        }
    });

    let mut camera = Camera::default();
    camera.image_width = width;
    camera.samples_per_pixel = 100;

    let mut world = HittableList::new();
    world.add(Rc::from(Sphere::new(Point3::new(0, 0, -1), 0.5)));
    world.add(Rc::from(Sphere::new(Point3::new(0, -100.5, -1), 100)));

    camera.render(&world, &image_path);
}
