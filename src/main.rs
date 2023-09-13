use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;

mod camera;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32; //(IMAGE_WIDTH as f32 / ASPECT_RATIO).clamp(1.0, f32::MAX) as u32;

const MAX_VALUE: u8 = 255;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

const FOCAL_LENGTH: f64 = 1.0;
const CAMERA_CENTER: DVec3 = DVec3::ZERO;

const VIEWPORT_U: DVec3 = DVec3::new(VIEWPORT_WIDTH, 0., 0.);
const VIEWPORT_V: DVec3 = DVec3::new(0., -VIEWPORT_HEIGHT, 0.);

struct Ray {
    origin: DVec3,
    direction: DVec3,
}

impl Ray {
    const fn new(origin: DVec3, direction: DVec3) -> Self {
        Ray { origin, direction }
    }

    fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }

    fn colour(&self) -> DVec3 {
        let sphere_center = DVec3::new(0., 0., -1.);
        if let Some(t) = hit_sphere(&sphere_center, 0.5, self) {
            let n = (self.at(t) - sphere_center).normalize();
            return 0.5 * (n + DVec3::ONE);
        }
        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0)
    }
}

fn main() -> std::io::Result<()> {
    let pixel_delta_u: DVec3 = VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v: DVec3 = VIEWPORT_V / IMAGE_HEIGHT as f64;

    let viewport_upper_left =
        CAMERA_CENTER - DVec3::new(0., 0., FOCAL_LENGTH) - VIEWPORT_U / 2. - VIEWPORT_V / 2.;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let pixels = (0..IMAGE_HEIGHT)
        .cartesian_product(0..IMAGE_WIDTH)
        .progress_count(IMAGE_HEIGHT as u64 * IMAGE_WIDTH as u64)
        .map(|(y, x)| {
            let pixel_center =
                pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_direction = pixel_center - CAMERA_CENTER;

            let ray = Ray::new(CAMERA_CENTER, ray_direction);

            let pixel_colour = ray.colour() * 255.;

            format!(
                "{} {} {}",
                pixel_colour.x as u8, pixel_colour.y as u8, pixel_colour.z as u8
            )
        })
        .chunks(IMAGE_WIDTH as usize)
        .into_iter()
        .map(|chunk| chunk.into_iter().join(" "))
        .join("\n");
    // println!("{pixels}");

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
