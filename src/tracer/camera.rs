use cgmath;
use cgmath::Vector3;
use cgmath::prelude::InnerSpace;

use super::ray::Ray;

pub struct Camera {
    lower_left: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    origin: Vector3<f32>
}

impl Camera {
    pub fn new(lookfrom:Vector3<f32>, lookat:Vector3<f32>, up:Vector3<f32>, fov:f32, aspect:f32) -> Camera {
        let theta = fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            lower_left: lookfrom - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: lookfrom,
        }
    }

    pub fn get_ray(&self, u:f32, v:f32) -> Ray {
        Ray {
            origin: self.origin.clone(),
            direction: self.lower_left + u * self.horizontal + v * self.vertical - self.origin
        }
    }
}
