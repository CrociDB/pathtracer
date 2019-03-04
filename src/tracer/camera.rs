use cgmath;
use cgmath::Vector3;

use super::ray::Ray;

pub struct Camera {
    lower_left: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    origin: Vector3<f32>
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left: cgmath::vec3(-2.0, -1.0, -1.0),
            horizontal: cgmath::vec3(4.0, 0.0, 0.0),
            vertical: cgmath::vec3(0.0, 2.0, 0.0),
            origin: cgmath::vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u:f32, v:f32) -> Ray {
        Ray {
            origin: self.origin.clone(),
            direction: self.lower_left + u * self.horizontal + v * self.vertical - self.origin
        }
    }
}
