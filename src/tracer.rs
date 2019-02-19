use cgmath;
use cgmath::Vector3;
use cgmath::prelude::InnerSpace;

struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>
}

impl Ray {
    pub fn point_at_parameter(&self, p:f32) -> Vector3<f32> {
        &self.origin + (p * &self.direction)
    }
}

fn hit_sphere(center:&Vector3<f32>, radius:f32, ray:&Ray) -> f32 {
    let oc = ray.origin - center;
    let a = cgmath::dot(ray.direction, ray.direction);
    let b = 2.0 * cgmath::dot(oc, ray.direction);
    let c = cgmath::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn color(ray:&Ray) -> Vector3<f32> {
    let sphere_pos = cgmath::vec3(0.0, 0.0, -1.0);
    let t = hit_sphere(&sphere_pos, 0.5, ray);
    if t > 0.0 {
        let normal = (ray.point_at_parameter(t) - cgmath::vec3(0.0, 0.0, -1.0)).normalize();
        0.5 * cgmath::vec3(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0)
    } else {
        let unit_direction = ray.direction.normalize();
        let t:f32 = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * cgmath::vec3(1.0, 1.0, 1.0) + t * cgmath::vec3(0.5, 0.7, 1.0)
    }
}

pub fn trace(width:u32, height:u32) -> Vec<u32> {
    let mut pixel_data = Vec::new();

    let lower_left = cgmath::vec3(-2.0, -1.0, -1.0);
    let horizontal = cgmath::vec3(4.0, 0.0, 0.0);
    let vertical = cgmath::vec3(0.0, 2.0, 0.0);
    
    let origin = cgmath::vec3(0.0, 0.0, 0.0);

    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f32 / width as f32;
            let v = j as f32 / height as f32;

            let ray:Ray = Ray {
                origin: origin,
                direction: &lower_left + u * &horizontal + v * &vertical
            };

            let col = color(&ray);

            pixel_data.push(pack_colors(
                (col.x * 255.0) as u8, 
                (col.y * 255.0) as u8, 
                (col.z * 255.0) as u8));
        }
    }

    pixel_data
}

pub fn unpack_colors(col:u32) -> (u8, u8, u8) {
    (((col >> 16) & 0xFF) as u8, ((col >> 8) & 0xFF) as u8, (col & 0xFF) as u8)
}

pub fn pack_colors(r:u8, g:u8, b:u8) -> u32 {
    ((r as u32) << 16) + ((g as u32) << 8) + b as u32
}
