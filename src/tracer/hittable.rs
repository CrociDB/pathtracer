use cgmath;
use cgmath::Vector3;

use super::ray;

pub struct HitRecord {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>
}

pub trait Hittable {
    fn hit(ray:&ray::Ray);
}
