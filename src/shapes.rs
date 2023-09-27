use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use glam::DVec3;
use std::ops::Range;

pub struct Sphere {
    center: DVec3,
    radius: f64,
    material: &'static mut dyn Material,
}

impl Sphere {
    pub fn new<M>(center: DVec3, radius: f64, material: M) -> Self
    where
        M: Material + 'static,
    {
        Sphere {
            center,
            radius,
            material: Box::leak(Box::new(material)),
        }
    }
}

impl Drop for Sphere {
    fn drop(&mut self) {
        println!("Sphere dropped.");
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_range: Range<f64>) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt = discriminant.sqrt();

        let mut root = (-half_b - sqrt) / a;

        if !ray_range.contains(&root) {
            root = (-half_b + sqrt) / a;
            if !ray_range.contains(&root) {
                return None;
            }
        }

        let p = r.at(root);
        let normal = (p - self.center) / self.radius;
        Some(HitRecord::with_face_normal(p, normal, root, &r))
    }
}
