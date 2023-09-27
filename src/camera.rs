use crate::hittable::HittableList;
use crate::ray::Ray;
use glam::DVec3;
use itertools::Itertools;
use raylib::prelude::*;

pub struct Camera {
    _aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    max_value: u8,
    center: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    pixel00_loc: DVec3,
    frame: String,
    samples_per_pixel: u32,
    max_depth: u32,
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
            _aspect_ratio: aspect_ratio,
            image_height,
            image_width,
            max_value,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            frame: String::new(),
            samples_per_pixel: 100,
            max_depth: 50,
        }
    }
    pub fn render(&mut self, d: &mut RaylibDrawHandle, world: &HittableList) {
        let mut rng = fastrand::Rng::new();

        self.frame = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            // .progress_count(IMAGE_HEIGHT as u64 * IMAGE_WIDTH as u64)
            .map(|(y, x)| {
                let pixel_center = self.pixel00_loc
                    + (x as f64 * self.pixel_delta_u)
                    + (y as f64 * self.pixel_delta_v);

                let pixel_colour =
                    (0..=self.samples_per_pixel).fold(DVec3::default(), |mut pixel_colour, _| {
                        // let ray_direction = pixel_center - self.pixel_sample_square();
                        let pixel_sample = pixel_center + self.pixel_sample_square(&mut rng);
                        let ray_direction = pixel_sample - self.center;

                        let new_colour = Ray::new(self.center, ray_direction)
                            .colour(self.max_depth as i32, &world)
                            * 255.0
                            * (self.samples_per_pixel as f64).recip();

                        pixel_colour
                            + DVec3 {
                                x: new_colour.x.sqrt(),
                                y: new_colour.y.sqrt(),
                                z: new_colour.z.sqrt(),
                            }
                    });

                d.draw_pixel(
                    x as _,
                    y as _,
                    Color {
                        r: pixel_colour.x as _,
                        g: pixel_colour.y as _,
                        b: pixel_colour.z as _,
                        a: u8::MAX,
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

    fn pixel_sample_square(&self, rng: &mut fastrand::Rng) -> DVec3 {
        (self.pixel_delta_u * (-0.5 + rng.f64())) + (self.pixel_delta_v * (-0.5 + rng.f64()))
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
