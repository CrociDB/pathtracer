use cgmath;
use cgmath::Vector3;
use cgmath::InnerSpace;

use super::hittable::{
    HitRecord,
    Hittable
};

use super::ray::Ray;

pub struct Sphere {
    pub center:Vector3<f32>,
    pub radius:f32
}

impl Sphere {
    pub fn new(c:Vector3<f32>, r:f32) -> Sphere {
        Sphere { center: c, radius: r }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray:&Ray, t_min:f32, t_max:f32, record:&mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = cgmath::dot(ray.direction, ray.direction);
        let b = 2.0 * cgmath::dot(oc, ray.direction);
        let c = cgmath::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b * b - 4.0 * a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = ray.point_at_parameter(temp);
                record.normal = (record.p - self.center).normalize();
                return true;
            }

            let temp = (-b + (b * b - 4.0 * a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                record.t = temp;
                record.p = ray.point_at_parameter(temp);
                record.normal = (record.p - self.center).normalize();
                return true;
            }
        }

        false
    }
}

