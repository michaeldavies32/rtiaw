use fastrand::Rng;
use glam::DVec3;
use std::ops::Range;

pub trait RandomExt {
    fn random(rng: &mut Rng, range: Range<f64>) -> DVec3;
    fn random_in_unit_sphere() -> DVec3;
    fn random_unit_vector() -> DVec3;
    fn random_on_hemisphere(normal: &DVec3) -> DVec3;
}

impl RandomExt for DVec3 {
    fn random(rng: &mut Rng, range: Range<f64>) -> DVec3 {
        DVec3::new(
            range.start + (range.end - range.start) * rng.f64(),
            range.start + (range.end - range.start) * rng.f64(),
            range.start + (range.end - range.start) * rng.f64(),
        )
    }

    fn random_in_unit_sphere() -> DVec3 {
        let mut rng = fastrand::Rng::new();
        loop {
            let p = DVec3::random(&mut rng, -1f64..1f64);

            if p.length_squared() < 1f64 {
                return p;
            }
        }
    }

    fn random_unit_vector() -> DVec3 {
        DVec3::random_in_unit_sphere().normalize()
    }

    fn random_on_hemisphere(normal: &DVec3) -> DVec3 {
        match DVec3::random_unit_vector() {
            on_unit_sphere if on_unit_sphere.dot(*normal) > 0.0 => on_unit_sphere,
            on_unit_sphere => -on_unit_sphere,
        }
    }
}
