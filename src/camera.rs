use crate::hittable::HittableList;
use crate::ray::Ray;
use glam::DVec3;
use itertools::Itertools;
use raylib::prelude::*;

// const ASPECT_RATIO: f64 = 16.0 / 9.0;
// const IMAGE_WIDTH: u32 = 800;
// const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32; //(IMAGE_WIDTH as f32 / ASPECT_RATIO).clamp(1.0, f32::MAX) as u32;
//
// const MAX_VALUE: u8 = 255;
//
// const VIEWPORT_HEIGHT: f64 = 2.0;
// const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO; //(IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
//
// const FOCAL_LENGTH: f64 = 1.0;
// const CAMERA_CENTER: DVec3 = DVec3::ZERO;
//
// const VIEWPORT_U: DVec3 = DVec3::new(VIEWPORT_WIDTH, 0., 0.);
// const VIEWPORT_V: DVec3 = DVec3::new(0., -VIEWPORT_HEIGHT, 0.);

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    max_value: u8,
    center: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    pixel00_loc: DVec3,
    frame: String,
}

impl Camera {
    pub fn new(image_width: u32, aspect_ratio: f64) -> Self {
        let max_value = 255u8;
        let focal_length: f64 = 1.0;
        let image_height = (image_width as f64 / aspect_ratio) as u32;

        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = DVec3::new(viewport_width, 0., 0.);
        let viewport_v = DVec3::new(0., -viewport_height, 0.);

        let pixel_delta_u: DVec3 = viewport_u / image_width as f64;
        let pixel_delta_v: DVec3 = viewport_v / image_height as f64;

        let center = DVec3::ZERO;

        let viewport_upper_left =
            center - DVec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            image_height,
            image_width,
            max_value,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            frame: String::new(),
        }
    }
    pub fn render(&mut self, d: &mut RaylibDrawHandle, world: &HittableList) {
        self.frame = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            // .progress_count(IMAGE_HEIGHT as u64 * IMAGE_WIDTH as u64)
            .map(|(y, x)| {
                let pixel_center = self.pixel00_loc
                    + (x as f64 * self.pixel_delta_u)
                    + (y as f64 * self.pixel_delta_v);

                let ray_direction = pixel_center - self.center;

                let ray = Ray::new(self.center, ray_direction);

                let pixel_colour = ray.colour(&world) * 255.;

                d.draw_pixel(
                    x as _,
                    y as _,
                    Color {
                        r: pixel_colour.x as _,
                        g: pixel_colour.y as _,
                        b: pixel_colour.z as _,
                        a: 255u8,
                    },
                );

                format!(
                    "{} {} {}",
                    pixel_colour.x as u8, pixel_colour.y as u8, pixel_colour.z as u8
                )
            })
            .chunks(self.image_width as usize)
            .into_iter()
            .map(|chunk| chunk.into_iter().join(" "))
            .join("\n");
    }

    pub fn render_to_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        std::fs::write(
            path,
            format!(
                "P3
{} {}
{}
{}
",
                self.image_width, self.image_height, self.max_value, self.frame
            ),
        )?;

        Ok(())
    }
}
