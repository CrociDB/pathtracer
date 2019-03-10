use super::ray::Ray;
use super::hittable::HitRecord;

extern crate rand;
use rand::prelude::*;

use cgmath::Vector3;
use cgmath::prelude::InnerSpace;

pub fn random_in_unit_sphere() -> cgmath::Vector3<f32> {
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

pub trait Material {
    fn scatter(&self, ray:&Ray, hit_record:&HitRecord, attenuation:&mut Vector3<f32>, scattered:&mut Ray) -> bool;
}