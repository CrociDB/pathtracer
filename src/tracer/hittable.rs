use cgmath;
use cgmath::Vector3;

use super::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: cgmath::vec3(0.0, 0.0, 0.0),
            normal: cgmath::vec3(0.0, 0.0, 0.0)
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray:&Ray, t_min:f32, t_max:f32, record:&mut HitRecord) -> bool;
}
