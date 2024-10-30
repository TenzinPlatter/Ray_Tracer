pub mod vec3;
pub mod ray;
pub mod scene;
pub mod utils;
pub mod shapes {
    pub mod sphere;
}

use indicatif::ProgressBar;

use scene::SceneContext;
use vec3::Vec3;
use ray::Ray;

type Color = Vec3;

pub fn draw_sphere_on_gradient(scene: SceneContext) -> String {
    let mut res = String::new();

    res.push_str(&format!("P3\n{} {}\n255\n", scene.width, scene.height));

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

            res.push_str(&(r.color().get_color_256() + "\n "));
        }
    }

    log::info!("\rDone.                     \r");
    bar.finish();

    res
}

pub fn draw_red_green_gradient(scene: SceneContext) -> String {
    let mut res = String::new();
    res.push_str(&format!("P3\n{} {}\n255\n", scene.width, scene.height));

    for j in 0..scene.height {
        for i in 0..scene.width {
            let color = Color::new(
                i as f32 / (scene.width - 1) as f32,
                j as f32 / (scene.height - 1) as f32,
                0.,
            );

            res.push_str(&(color.get_color_256() + "\n"));
        }
    }


    res
}
