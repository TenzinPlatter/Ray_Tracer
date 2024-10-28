use indicatif::ProgressBar;

pub mod vec3;
pub mod ray;

use vec3::Vec3;

pub fn setup_window() {
    let aspect_ratio = 16. / 9.;
    let image_width: i32 = 400;

    let mut image_height: i32 = (image_width as f32 / aspect_ratio) as i32;

    if image_height < 1 {
        image_height = 1
    }

    let viewport_height = 2.;
    let viewport_width
}

pub fn draw_gradient() {
    // setup logger
    env_logger::init();

    let image_width = 256;
    let image_height = 256;

    println!("P3\n{image_width} {image_height}\n255");

    let bar = ProgressBar::new(image_height * image_width);

    log::info!("Scanlines remaining: ");
    for j in 0..image_height {
        for i in 0..image_width {
            bar.inc(1);

            let color = Vec3::new(
                (i as f64) / (image_width - 1) as f64,
                (j as f64) / (image_height - 1) as f64,
                0.,
            );

            color.write_color();
        }
    }

    log::info!("\rDone.                     \r");
    bar.finish();
}
