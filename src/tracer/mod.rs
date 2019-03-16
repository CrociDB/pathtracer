pub mod ray;
pub mod hittable;
pub mod hittable_list;
pub mod sphere;
pub mod camera;

pub mod material;
pub mod metal;
pub mod lambertian;
pub mod dielectric;


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

extern crate scoped_threadpool;
use scoped_threadpool::Pool;

extern crate rand;
use rand::prelude::*;
use std::thread;

static SAMPLINGS_PER_PIXEL:u32 = 100;

fn vec_mul(a:&Vector3<f32>, b:&Vector3<f32>) -> Vector3<f32> {
    cgmath::vec3(a.x * b.x, a.y * b.y, a.z * b.z)
}


fn color(ray:&Ray, world:&impl Hittable, depth:i32) -> Vector3<f32> {
    let mut record = HitRecord::new();

    if world.hit(ray, 0.001, 10_000_000.0, &mut record) {
        let mut scattered:Ray = Ray::new(cgmath::vec3(0.0, 0.0, 0.0), cgmath::vec3(0.0, 0.0, 0.0));
        let mut attenuation:Vector3<f32> = cgmath::vec3(0.0, 0.0, 0.0);

        if depth < 50 && record.material.unwrap().scatter(ray, &mut record, &mut attenuation, &mut scattered) {
            let c = color(&scattered, world, depth + 1);
            vec_mul(&attenuation, &c)
        } else {
            cgmath::vec3(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let t:f32 = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * cgmath::vec3(1.0, 1.0, 1.0) + t * cgmath::vec3(0.1, 0.7, 1.0)
    }
}

pub fn trace(width:u32, height:u32) -> Vec<u32> {
    let size = width * height;
    let mut pixel_data = Vec::new();
    
    // let mut world = create_world();
    let world = create_random_world();

    // Camera settings
    let lookfrom = cgmath::vec3(7.0, 1.0, 2.5);
    let lookat = cgmath::vec3(0.0, 0.0, -1.0);
    let up = cgmath::vec3(0.0, 1.0, 0.0);
    let apperture = 0.2;
    let focusdist = (lookfrom - lookat).magnitude();

    let camera = Camera::new(lookfrom, lookat, up, 50.0, (width as f32) / (height as f32), apperture, focusdist);

    let iterations = 12;
    let mut pool = Pool::new(12);

    let ds = size / iterations as u32;

    let mut slices = Vec::new();
    for i in 0..iterations { slices.push(Vec::new()); }

    pool.scoped(|scope| {
        let mut c = 0;
        let ref_camera = &camera;
        let ref_world = &world;

        for i in &mut slices {
            scope.execute(move || {
                *i = trace_slice((width, height), &c * ds, (&c + 1) * ds, ref_camera, ref_world);
            });
            c += 1;
        }
    });

    for s in slices {
        for p in s {
            pixel_data.push(p);
        }
    }

    pixel_data
}

fn trace_slice(dim:(u32, u32), start:u32, end:u32, camera:&Camera, world:&impl Hittable) -> Vec<u32> {
    let mut pixel_data = Vec::new();
    let ns = SAMPLINGS_PER_PIXEL;
    let mut rng = rand::thread_rng();
    let (width, height) = dim;
    for i in start..end {
        let x = i % width;
        let y = height - 1 - (i / width);

        let mut col = cgmath::vec3(0.0, 0.0, 0.0);
        for _ in 0..ns {
            let r1:f32 = rng.gen();
            let r2:f32 = rng.gen();

            let u = ((x as f32) + r1) / width as f32;
            let v = ((y as f32) + r2) / height as f32;
            
            let ray = camera.get_ray(u, v);
            col += color(&ray, world, 0);
        }

        col /= ns as f32;
        col = cgmath::vec3(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

        // pixel_data[i as usize] = pack_colors(
        //     (col.x * 255.0) as u8,  
        //     (col.y * 255.0) as u8, 
        //     (col.z * 255.0) as u8);
        pixel_data.push(pack_colors(
            (col.x * 255.0) as u8,  
            (col.y * 255.0) as u8, 
            (col.z * 255.0) as u8));
    }
    pixel_data
}

fn create_world() -> HittableList {
    let mut world = HittableList::new();
    world.add_hittable(Box::new(Sphere::<lambertian::Lambertian>::new::<lambertian::Lambertian>(Vector3::new(0.4, 0.0, -1.2), 0.5, lambertian::Lambertian::new(Vector3::new(0.4, 0.1, 0.3)))));
    world.add_hittable(Box::new(Sphere::<lambertian::Lambertian>::new::<lambertian::Lambertian>(Vector3::new(-0.6, -0.1, -1.4), 0.4, lambertian::Lambertian::new(Vector3::new(0.34, 0.245, 0.312)))));
    world.add_hittable(Box::new(Sphere::<dielectric::Dielectric>::new::<dielectric::Dielectric>(Vector3::new(-2.4, 0.2, -2.4), 0.6, dielectric::Dielectric::new(1.5))));
    
    world.add_hittable(Box::new(Sphere::<metal::Metal>::new::<metal::Metal>(Vector3::new(1.2, 0.5, -2.6), 1.0, metal::Metal::new(Vector3::new(0.4, 0.2, 0.6), 0.55))));
    world.add_hittable(Box::new(Sphere::<metal::Metal>::new::<metal::Metal>(Vector3::new(-1.2, 0.5, -4.6), 1.0, metal::Metal::new(Vector3::new(0.5, 0.4, 0.2), 0.04))));
    world.add_hittable(Box::new(Sphere::<metal::Metal>::new::<metal::Metal>(Vector3::new(-3.6, 1.0, -8.6), 2.0, metal::Metal::new(Vector3::new(0.67, 0.622, 0.68), 0.04))));
    world.add_hittable(Box::new(Sphere::<dielectric::Dielectric>::new::<dielectric::Dielectric>(Vector3::new(-1.3, -0.2, -1.0), 0.3, dielectric::Dielectric::new(1.5))));
    world.add_hittable(Box::new(Sphere::<metal::Metal>::new::<metal::Metal>(Vector3::new(1.4, 0.0, -0.7), 0.5, metal::Metal::new(Vector3::new(0.415, 0.62, 0.34), 0.14))));
    world.add_hittable(Box::new(Sphere::<dielectric::Dielectric>::new::<dielectric::Dielectric>(Vector3::new(-0.2, -0.3, -0.4), 0.2, dielectric::Dielectric::new(1.5))));

    world.add_hittable(Box::new(Sphere::<lambertian::Lambertian>::new::<lambertian::Lambertian>(Vector3::new(0.0, -100.5, -1.0), 100.0, lambertian::Lambertian::new(Vector3::new(0.5, 0.5, 0.5)))));

    world
}

fn create_random_world() -> HittableList {
    let mut world = HittableList::new();
    world.add_hittable(Box::new(Sphere::<lambertian::Lambertian>::new::<lambertian::Lambertian>(Vector3::new(0.0, -1000.0, 0.0), 1000.0, lambertian::Lambertian ::new(Vector3::new(0.5, 0.5, 0.5)))));

    let mut rng = rand::thread_rng();
    let n = 6;

    for a in -n..n {
        for b in -n..n {
            if b > -1 && b < 1 { continue; }

            let choose_mat = rng.gen::<f32>();
            let center = cgmath::vec3((a as f32) + 0.7 * rng.gen::<f32>(), 0.2, (b as f32) + 0.7 * rng.gen::<f32>());
            if (center - cgmath::vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.4 {
                    world.add_hittable(Box::new(Sphere::<lambertian::Lambertian>::new::<lambertian::Lambertian>(
                            center, 
                            0.2, 
                            lambertian::Lambertian ::new(Vector3::new(rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>())))));
                } else if choose_mat < 0.85 {
                    world.add_hittable(Box::new(Sphere::<metal::Metal>::new::<metal::Metal>(
                            center, 
                            0.2, 
                            metal::Metal::new(Vector3::new(rng.gen::<f32>(), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>())), 0.5 * rng.gen::<f32>()))));
                } else {
                    world.add_hittable(Box::new(Sphere::<dielectric::Dielectric>::new::<dielectric::Dielectric>(
                            center, 
                            0.2, 
                            dielectric::Dielectric::new(1.5))));
                }
            }
        } 
    } 

    world.add_hittable(Box::new(Sphere::<dielectric::Dielectric>::new::<dielectric::Dielectric>(Vector3::new(0.0, 1.0, 0.0), 1.0, dielectric::Dielectric::new(1.5))));
    world.add_hittable(Box::new(Sphere::<lambertian::Lambertian>::new::<lambertian::Lambertian>(Vector3::new(-4.0, 1.0, 0.0), 1.0, lambertian::Lambertian::new(Vector3::new(0.34, 0.245, 0.312)))));
    world.add_hittable(Box::new(Sphere::<metal::Metal>::new::<metal::Metal>(Vector3::new(4.0, 1.0, 0.0), 1.0, metal::Metal::new(Vector3::new(0.4, 0.2, 0.6), 0.05))));

    world
}

pub fn unpack_colors(col:u32) -> (u8, u8, u8) {
    (((col >> 16) & 0xFF) as u8, ((col >> 8) & 0xFF) as u8, (col & 0xFF) as u8)
}

pub fn pack_colors(r:u8, g:u8, b:u8) -> u32 {
    ((r as u32) << 16) + ((g as u32) << 8) + b as u32
}


