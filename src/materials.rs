use crate::helpers::RandomExt;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use glam::DVec3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, DVec3)>; //Scatter, Attenuation
}

pub struct Lambertian {
    albedo: DVec3,
}

impl Lambertian {
    pub fn new(colour: DVec3) -> Self {
        Lambertian { albedo: colour }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, DVec3)> {
        let mut scatter_direction = record.normal + DVec3::random_unit_vector();
        if scatter_direction.abs_diff_eq(DVec3::ZERO, 1e-8) {
            scatter_direction = record.normal;
        }
        Some((Ray::new(record.p, scatter_direction), self.albedo))
    }
}
