use std::{
    env,
    rc::Rc,
    time::Instant,
};

use ray_tracer::{
    camera::Camera, hit::HittableList, material::{Dielectric, Lambertian, Metal}, shapes::sphere::Sphere, vec3::Vec3
};

type Point3 = Vec3;
type Color = Vec3;

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
    camera.image_width = width;
    camera.samples_per_pixel = 100;
    camera.max_ray_bounce_depth = 50;

    let mut world = HittableList::new();
    let ground = Rc::from(Lambertian::new(Color::new(0.8, 0.8, 0)));
    let blue_solid = Rc::from(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let _material_left = Rc::from(Metal::new(Color::new(0.8, 0.33, 0.), 0.3));
    let colored_metal = Rc::from(Metal::new(Color::new(0.8, 0.6, 0.2), 1.));
    let _shiny_metal = Rc::from(Metal::new(Color::new(1, 1, 1), 0.1));
    let glass = Rc::from(Dielectric::new(1. / 1.33));
    let glass_to_air = Rc::from(Dielectric::new(1. / 1.5));

    // world.add(Rc::from(
    //         Sphere::new(
    //             Point3::new(-1, 0, -1),
    //             0.4,
    //             glass_to_air.clone(),
    //             )
    //         ));

    world.add(Rc::from(
            Sphere::new(
                Point3::new(0, 0, -1.2),
                0.5,
                blue_solid.clone(),
                )
            ));

    world.add(Rc::from(
            Sphere::new(
                Point3::new(0, -100.5, -1),
                100,
                ground.clone(),
                )
            ));

    world.add(Rc::from(
            Sphere::new(
                Point3::new(-1, 0, -1),
                0.5,
                glass.clone(),
                )
            ));

    world.add(Rc::from(
            Sphere::new(
                Point3::new(1, 0, -1),
                0.5,
                colored_metal.clone(),
                )
            ));

    let time_started = Instant::now();

    camera.render(&world, &image_path);

    println!("Render took {} secs", time_started.elapsed().as_secs());
}
