use std::{
    env,
    rc::Rc,
};

use ray_tracer::{
    hit::HittableList,
    vec3::Vec3,
    camera::Camera,
    shapes::sphere::Sphere,
    material::{Lambertian, Metal},
};

type Point3 = Vec3;
type Color = Vec3;

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
    camera.max_ray_bounce_depth = 50;

    let mut world = HittableList::new();
    let material_ground = Rc::from(Lambertian::new(Color::new(0.8, 0.8, 0)));
    let material_center = Rc::from(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::from(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::from(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add( Rc::from(
            Sphere::new(
                Point3::new(0, 0, -1.2),
                0.5,
                material_center.clone(),
                )
            ));

    world.add(Rc::from(
            Sphere::new(
                Point3::new(0, -100.5, -1),
                100,
                material_ground.clone(),
                )
            ));

    world.add(Rc::from(
            Sphere::new(
                Point3::new(-1, 0, -1),
                0.5,
                material_left.clone(),
                )
            ));

    world.add(Rc::from(
            Sphere::new(
                Point3::new(1, 0, -1),
                0.5,
                material_right.clone(),
                )
            ));

    camera.render(&world, &image_path);
}
