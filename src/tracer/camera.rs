use cgmath;
use cgmath::Vector3;
use cgmath::prelude::InnerSpace;

extern crate rand;
use rand::prelude::*;

use super::ray::Ray;

pub struct Camera {
    lower_left: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    origin: Vector3<f32>,
    lens_radius: f32
}

fn random_in_unit_disk() -> Vector3<f32> {
    let mut p:cgmath::Vector3<f32>;
    let mut rng = rand::thread_rng();
    
    loop {
        p = 2.0 * cgmath::vec3(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - cgmath::vec3(1.0, 1.0, 0.0);

        if p.dot(p) < 1.0 {
            break;
        }
    }
    
    p
}

impl Camera {
    pub fn new(lookfrom:Vector3<f32>, lookat:Vector3<f32>, up:Vector3<f32>, fov:f32, aspect:f32, apperture:f32, focus_dist:f32) -> Camera {
        let theta = fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        Camera {
            lower_left: lookfrom - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: lookfrom,
            lens_radius: apperture / 2.0
        }
    }

    pub fn get_ray(&self, u:f32, v:f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = cgmath::vec3(u * rd.x, v * rd.y, rd.z);

        Ray {
            origin: offset + self.origin,
            direction: self.lower_left + u * self.horizontal + v * self.vertical - self.origin - offset
        }
    }
}
