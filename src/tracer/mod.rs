pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod camera;
pub mod hittable_list;

use sphere::Sphere;
use hittable::{
    HitRecord,
    Hittable
};
use ray::Ray;
use camera::Camera;
use hittable_list::HittableList;

use cgmath;
use cgmath::Vector3;
use cgmath::prelude::InnerSpace;


fn color(ray:&Ray, world:&impl Hittable) -> Vector3<f32> {
    let mut record = HitRecord::new();

    if world.hit(ray, 0.0, 10_000_000.0, &mut record) {
        0.5 * cgmath::vec3(record.normal.x + 1.0, record.normal.y + 1.0, record.normal.z + 1.0)
    } else {
        let unit_direction = ray.direction.normalize();
        let t:f32 = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * cgmath::vec3(1.0, 1.0, 1.0) + t * cgmath::vec3(0.5, 0.7, 1.0)
    }
}

pub fn trace(width:u32, height:u32) -> Vec<u32> {
    let mut pixel_data = Vec::new();

    let mut world = HittableList::new();
    world.add_hittable(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5));
    world.add_hittable(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new();

    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f32 / width as f32;
            let v = j as f32 / height as f32;

            let ray = camera.get_ray(u, v);

            let col = color(&ray, &world);

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


