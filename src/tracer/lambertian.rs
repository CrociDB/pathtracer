use super::material::Material;
use super::ray::Ray;
use super::hittable::HitRecord;

use cgmath::Vector3;
use cgmath::prelude::InnerSpace;

extern crate rand;
use rand::prelude::*;

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

fn random_in_unit_sphere() -> cgmath::Vector3<f32> {
    let mut p:cgmath::Vector3<f32>;
    let mut rng = rand::thread_rng();
    
    loop {
        p = 2.0 * cgmath::vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - cgmath::vec3(1.0, 1.0, 1.0);

        if p.magnitude2() >= 1.0 {
            break;
        }
    }
    
    p
}

impl Material for Lambertian {
    fn scatter(&self, ray:&Ray, hit_record:&HitRecord, attenuation:&mut Vector3<f32>, scattered:&mut Ray) -> bool {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        *scattered = Ray::new(hit_record.p.clone(), target -  hit_record.p);
        *attenuation = self.albedo;
        true
    }
}
