pub mod vec3;
pub mod ray;
pub mod scene;

use indicatif::ProgressBar;

use scene::SceneContext;
use vec3::Vec3;
use ray::Ray;

pub fn draw_sphere_on_gradient(scene: SceneContext) {
    println!("P3\n{} {}\n255", scene.width, scene.height);

    let bar = ProgressBar::new((scene.height * scene.height) as u64);

    log::info!("Scanlines remaining: ");
    for j in 0..scene.height {
        for i in 0..scene.width {
            bar.inc(1);

            let pixel_center = scene.px00_loc
                + scene.pixel_delta_u * i as f64
                + scene.pixel_delta_v * j as f64;

            // doesn't turn into a unit vector to avoid divisions
            let ray_direction = pixel_center - scene.camera_center;

            let r = Ray::new(scene.camera_center, ray_direction);

            r.color().write_color();
        }
    }

    log::info!("\rDone.                     \r");
    bar.finish();
}
