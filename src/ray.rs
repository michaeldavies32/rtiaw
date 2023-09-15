use crate::hittable::{Hittable, HittableList};
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

    pub fn colour(&self, world: &HittableList) -> DVec3 {
        if let Some(record) = world.hit(&self, (0.)..f64::INFINITY) {
            return 0.5 * (record.normal + DVec3::ONE);
        }
        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);

        (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0)
    }
}
