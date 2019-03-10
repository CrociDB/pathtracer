use cgmath;
use cgmath::Vector3;

use super::material::Material;

use super::hittable::{
    HitRecord,
    Hittable
};

use super::ray::Ray;

pub struct Sphere<T>
    where T : Material {
    pub center:Vector3<f32>,
    pub radius:f32,
    pub material: T
}

impl<T> Sphere<T>
    where T : Material {
    pub fn new<Y>(c:Vector3<f32>, r:f32, m:Y) -> Sphere<Y>
        where Y : Material {
        Sphere { center: c, radius: r, material: m }
    }
}

impl<T> Hittable for Sphere<T>
    where T : Material {
    fn hit<'a, 'b>(&'a self, ray:&Ray, t_min:f32, t_max:f32, record:&'b mut HitRecord<'a>) -> bool{
        let oc = ray.origin - self.center;
        let a = cgmath::dot(ray.direction, ray.direction);
        let b = cgmath::dot(oc, ray.direction);
        let c = cgmath::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 1.0 * a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b * b - 1.0 * a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = ray.point_at_parameter(record.t);
                record.normal = (record.p - self.center) / self.radius;
                record.material = Some(&self.material);
                return true;
            }

            let temp = (-b + (b * b - 1.0 * a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = ray.point_at_parameter(record.t);
                record.normal = (record.p - self.center) / self.radius;
                record.material = Some(&self.material);
                return true;
            }
        }

        false
    }
}

