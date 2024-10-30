use std::{env, fs};

use ray_tracer::vec3::Vec3;
use ray_tracer::scene::SceneContext;

type Point3 = Vec3;

fn main() {
     // setup logger
    env_logger::init();

    let scene = SceneContext::new(400, 16. / 9., Point3::new(0., 0., 0.));

    let picture = ray_tracer::draw_sphere_on_gradient(scene);
    // let picture = ray_tracer::draw_red_green_gradient(scene);


    let args: Vec<String> = env::args().collect();

    let image_path: String = if args.len() > 1 {
        args[1].clone()
    } else {
        String::from("./imgs/image.ppm")
    };

    fs::write(image_path, picture).expect("Unable to write to file");
}
