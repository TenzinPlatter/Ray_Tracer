use indicatif::ProgressBar;

mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

type Color = Vec3;
type Point3 = Vec3;

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width: i32 = 400;

    let mut image_height: i32 = (image_width as f32 / aspect_ratio) as i32;

    // image must be at least 1 pixel high
    if image_height < 1 {
        image_height = 1
    }

    // viewport height is at least 1px + 1/2px * for inset
    let viewport_height = 2.;

    // height calculated from width/height rather than aspect ratio as ratio
    // is an ideal not actual
    let viewport_width: f64 = viewport_height * (image_width / image_height) as f64;

    // distance between camera and viewport on z axis
    // from camera towards viewport is -z
    let focal_length = 1.;

    let camera_center = Vec3::new(0., 0., 0.);

    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // u -> x distance between pixels on viewport
    // v -> y distance between pixels on viewport
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // don't really get why this does what it does
    let viewport_upper_left = camera_center
        - Vec3::new(0., 0., focal_length)
        - viewport_u/2.
        - viewport_v/2.;
    
    // (0, 0) pixel on viewport, as it is inset by 1/2 a pixel on the x and y
    // axis, and focal length away from camera

    let pixel00_loc = viewport_upper_left
        + (pixel_delta_u + pixel_delta_v) * 0.5;

     // setup logger
    env_logger::init();

    let pixel_center = pixel00_loc
        + pixel_delta_u * 100.
        + pixel_delta_v * 100.;
    let ray_dir = pixel_center - camera_center;
    let r = Ray::new(camera_center, ray_dir);
    r.origin.eprint();
    r.direction.eprint();


    // let image_width = 256;
    // let image_height = 256;

    // println!("P3\n{image_width} {image_height}\n255");

    // let bar = ProgressBar::new(image_height * image_width);

    // log::info!("Scanlines remaining: ");
    // for j in 0..image_height {
    //     for i in 0..image_width {
    //         bar.inc(1);

    //         let pixel_center = pixel00_loc
    //             + pixel_delta_u * i as f64
    //             + pixel_delta_v * j as f64;

    //         // doesn't turn into a unit vector to avoid divisions
    //         let ray_direction = pixel_center - camera_center;

    //         let r = Ray::new(camera_center, ray_direction);

    //         r.color().write_color();
    //     }
    // }

    // log::info!("\rDone.                     \r");
    // bar.finish();
}
