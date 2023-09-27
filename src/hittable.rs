use crate::materials::Material;
use crate::ray::Ray;
use glam::DVec3;
use std::ops::Range;

pub struct HitRecord {
    pub p: DVec3,
    pub normal: DVec3,
    pub material: Option<&'static mut dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn with_face_normal(p: DVec3, outward_normal: DVec3, t: f64, r: &Ray) -> Self {
        let front_face = r.direction.dot(outward_normal) < 0.;

        HitRecord {
            p,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            material: None,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_range: Range<f64>) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<&'static mut dyn Hittable>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn clear(&mut self) {
        //Convert pointers to Boxes to have them dropped.
        self.objects.drain(..).into_iter().for_each(|obj| unsafe {
            let _ = Box::from_raw(obj);
        });
    }

    pub fn add<T>(&mut self, object: T)
    where
        T: Hittable + 'static,
    {
        self.objects.push(Box::leak(Box::new(object)));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_range: Range<f64>) -> Option<HitRecord> {
        //Find closest hittable object along the path of the ray and return the hit record or
        self.objects
            .iter()
            .fold((ray_range.end, None), |acc, obj| {
                if let Some(temp_rec) = obj.hit(r, ray_range.start..acc.0) {
                    (temp_rec.t, Some(temp_rec))
                } else {
                    acc
                }
            })
            .1
    }
}
