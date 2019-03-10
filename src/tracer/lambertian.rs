use super::material::{Material, random_in_unit_sphere};
use super::ray::Ray;
use super::hittable::HitRecord;

use cgmath::Vector3;

pub struct Lambertian {
    albedo: Vector3<f32>
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Lambertian {
        Lambertian {
            albedo: albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray:&Ray, hit_record:&HitRecord, attenuation:&mut Vector3<f32>, scattered:&mut Ray) -> bool {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        *scattered = Ray::new(hit_record.p.clone(), target -  hit_record.p);
        *attenuation = self.albedo;
        true
    }
}
