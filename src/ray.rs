use crate::{
    helpers::RandomExt,
    hittable::{Hittable, HittableList},
};
use glam::DVec3;

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub const fn new(origin: DVec3, direction: DVec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }

    pub fn colour(&self, depth: i32, world: &HittableList) -> DVec3 {
        if depth <= 0 {
            return DVec3::ZERO;
        }

        if let Some(record) = world.hit(&self, (0.001)..f64::INFINITY) {
            if let Some(material) = record.material.as_ref() {
                if let Some((scattered, attenuation)) = material.scatter(&self, &record) {
                    return attenuation * scattered.colour(depth - 1, world);
                }
            }
            let direction = record.normal + DVec3::random_on_hemisphere(&record.normal);
            // return 0.5 * (record.normal + DVec3::ONE);
            return 0.5 * Ray::new(record.p, direction).colour(depth - 1, world);
        }
        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * DVec3::ONE + a * DVec3::new(0.5, 0.7, 1.0)
    }
}
