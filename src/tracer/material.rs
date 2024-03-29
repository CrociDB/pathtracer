use super::ray::Ray;
use super::hittable::HitRecord;

extern crate rand;
use rand::prelude::*;

use cgmath::Vector3;
use cgmath::prelude::InnerSpace;

use std::marker::{Sync, Send};

pub trait Material : Sync + Send {
    fn scatter(&self, ray:&Ray, hit_record:&HitRecord, attenuation:&mut Vector3<f32>, scattered:&mut Ray) -> bool;
}

// Helper functions

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

pub fn reflect(v:&Vector3<f32>, n:&Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(*n) * n
}

pub fn refract(v:&Vector3<f32>, n:&Vector3<f32>, ni_over_nt:f32, refracted:&mut Vector3<f32>) -> bool {
    let vnormalized = v.normalize();
    let dt = vnormalized.dot(*n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = ni_over_nt * (vnormalized - n * dt) - n * discriminant.sqrt();
        true
    } else {
        false
    }
}

pub fn schlick(cosine:f32, ref_idx:f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(40.0)
}
