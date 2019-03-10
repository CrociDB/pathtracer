use super::ray::Ray;
use super::hittable::HitRecord;

use cgmath::Vector3;

pub trait Material {
    fn scatter(&self, ray:&Ray, hit_record:&HitRecord, attenuation:&mut Vector3<f32>, scattered:&mut Ray) -> bool;
}