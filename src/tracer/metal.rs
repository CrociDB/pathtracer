use super::material::{Material, random_in_unit_sphere};
use super::ray::Ray;
use super::hittable::HitRecord;

use cgmath::Vector3;
use cgmath::prelude::InnerSpace;

pub struct Metal {
    albedo: Vector3<f32>,
    fuzz: f32
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, fuzz:f32) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: fuzz
        }
    }
}

fn reflect(v:&Vector3<f32>, n:&Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(*n) * n
}

impl Material for Metal {
    fn scatter(&self, ray:&Ray, hit_record:&HitRecord, attenuation:&mut Vector3<f32>, scattered:&mut Ray) -> bool {
        let reflected = reflect(&ray.direction.normalize(), & hit_record.normal);
        *scattered = Ray::new(hit_record.p, reflected + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo;
        (cgmath::dot(scattered.direction, hit_record.normal) > 0.0)
    }
}
 