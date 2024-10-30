use indicatif::ProgressBar;

use ray_tracer::vec3::Vec3;
use ray_tracer::ray::Ray;
use ray_tracer::scene::SceneContext;

type Color = Vec3;
type Point3 = Vec3;

fn main() {
     // setup logger
    env_logger::init();

    let scene = SceneContext::new(400, Point3::new(0., 0., 0.));

    ray_tracer::draw_sphere_on_gradient(scene);
}
