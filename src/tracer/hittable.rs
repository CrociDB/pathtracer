use cgmath;
use cgmath::Vector3;

use super::ray::Ray;
use super::material::Material;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub material: Option<&'a Material>
}

impl<'a> HitRecord<'a> {
    pub fn new() -> HitRecord<'a> {
        HitRecord {
            t: 0.0,
            p: cgmath::vec3(0.0, 0.0, 0.0),
            normal: cgmath::vec3(0.0, 0.0, 0.0),
            material: None
        }
    }
}

pub trait Hittable {
    fn hit<'a, 'b>(&'a self, ray:&Ray, t_min:f32, t_max:f32, record:&'b mut HitRecord<'a>) -> bool;
}
