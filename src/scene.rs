use crate::vec3::Vec3;

type Point3 = Vec3;

pub struct SceneContext {
    pub px00_loc: Point3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub camera_center: Point3,
    pub width: i32,
    pub height: i32,
}

impl SceneContext {
    pub fn new(image_width: i32, camera_center: Point3) -> SceneContext {
        let aspect_ratio = 16. / 9.;

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

        SceneContext {
            px00_loc: pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            camera_center,
            width: image_width,
            height: image_height,
        }
    }
}
