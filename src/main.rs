use glam::DVec3;

mod camera;
mod helpers;
mod hittable;
mod materials;
mod ray;
mod shapes;

use camera::Camera;
use hittable::HittableList;
use materials::Lambertian;
use shapes::Sphere;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32; //(IMAGE_WIDTH as f32 / ASPECT_RATIO).clamp(1.0, f32::MAX) as u32;

fn main() -> std::io::Result<()> {
    let (mut rl, thread) = raylib::init()
        .size(IMAGE_WIDTH as _, IMAGE_HEIGHT as _)
        .title("Raytracing in a weekend")
        .build();

    let material_ground = Lambertian::new(DVec3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(DVec3::new(0.7, 0.3, 0.3));

    let mut world = HittableList::new();

    world.add(Sphere::new(
        DVec3::new(0., -100.5, -1.),
        100.,
        material_ground,
    ));
    world.add(Sphere::new(DVec3::new(0., 0., -1.), 0.5, material_center));

    let mut camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        camera.render(&mut d, &world);
    }

    camera.render_to_file(std::path::Path::new("output.ppm"))?;
    world.clear();

    Ok(())
}
