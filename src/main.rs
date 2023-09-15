use std::ops::Range;

use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use raylib::{ffi::Color, prelude::*};

mod camera;
mod hittable;
mod ray;
mod shapes;

use hittable::HittableList;
use ray::Ray;
use shapes::Sphere;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32; //(IMAGE_WIDTH as f32 / ASPECT_RATIO).clamp(1.0, f32::MAX) as u32;

const MAX_VALUE: u8 = 255;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO; //(IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

const FOCAL_LENGTH: f64 = 1.0;
const CAMERA_CENTER: DVec3 = DVec3::ZERO;

const VIEWPORT_U: DVec3 = DVec3::new(VIEWPORT_WIDTH, 0., 0.);
const VIEWPORT_V: DVec3 = DVec3::new(0., -VIEWPORT_HEIGHT, 0.);

fn main() -> std::io::Result<()> {
    let (mut rl, thread) = raylib::init()
        .size(IMAGE_WIDTH as _, IMAGE_HEIGHT as _)
        .title("Raytracing in a weekend")
        .build();
    let pixel_delta_u: DVec3 = VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v: DVec3 = VIEWPORT_V / IMAGE_HEIGHT as f64;

    let viewport_upper_left =
        CAMERA_CENTER - DVec3::new(0., 0., FOCAL_LENGTH) - VIEWPORT_U / 2. - VIEWPORT_V / 2.;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut world = HittableList::new();

    world.add(Sphere::new(DVec3::new(0., 0., -1.), 0.5));
    world.add(Sphere::new(DVec3::new(0., -100.5, -1.), 100.));

    let mut pixels = String::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        pixels = (0..IMAGE_HEIGHT)
            .cartesian_product(0..IMAGE_WIDTH)
            // .progress_count(IMAGE_HEIGHT as u64 * IMAGE_WIDTH as u64)
            .map(|(y, x)| {
                let pixel_center =
                    pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
                let ray_direction = pixel_center - CAMERA_CENTER;

                let ray = Ray::new(CAMERA_CENTER, ray_direction);

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
            .chunks(IMAGE_WIDTH as usize)
            .into_iter()
            .map(|chunk| chunk.into_iter().join(" "))
            .join("\n");
    }

    std::fs::write(
        "output.ppm",
        format!(
            "P3
{IMAGE_WIDTH} {IMAGE_HEIGHT}
{MAX_VALUE}
{pixels}
"
        ),
    )?;

    world.clear();

    Ok(())
}

fn hit_sphere(center: &DVec3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin - *center;
    // let a = ray.direction.dot(ray.direction);
    let a = ray.direction.length_squared();
    // let b = 2.0 * oc.dot(ray.direction);
    let half_b = oc.dot(ray.direction);
    // let c = oc.dot(oc) - radius.powi(2);
    let c = oc.length_squared() - radius.powi(2);
    // let discriminant = b.powi(2) - 4. * a * c;
    let discriminant = half_b.powi(2) - a * c;

    if discriminant < 0. {
        None
    } else {
        // Some((-b - discriminant.sqrt()) / (2.0 * a))
        Some((-half_b - discriminant.sqrt()) / a)
    }
}
