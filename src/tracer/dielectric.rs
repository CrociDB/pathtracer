use super::material::{Material, random_in_unit_sphere, reflect, refract, schlick};
use super::ray::Ray;
use super::hittable::HitRecord;

extern crate rand;
use rand::prelude::*;

use cgmath::Vector3;
use cgmath::prelude::InnerSpace;

pub struct Dielectric {
    ref_idx: f32,   
}

impl Dielectric {
    pub fn new(ref_idx:f32) -> Dielectric {
        Dielectric {
            ref_idx: ref_idx
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray:&Ray, hit_record:&HitRecord, attenuation:&mut Vector3<f32>, scattered:&mut Ray) -> bool {
        let outward_normal;
        let reflected = reflect(&ray.direction, &hit_record.normal);
        let ni_over_nt;

        *attenuation = cgmath::vec3(1.0, 1.0, 1.0);

        let mut refracted = cgmath::vec3(0.0, 0.0, 0.0);
        let reflect_prob;
        let cosine;

        if ray.direction.dot(hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal.clone();
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * ray.direction.normalize().dot(hit_record.normal) / ray.direction.magnitude();
        } else {
            outward_normal = hit_record.normal.clone();
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -ray.direction.normalize().dot(hit_record.normal) / ray.direction.magnitude();
        }

        if refract(&ray.direction, &outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0
        }

        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < reflect_prob {
            *scattered = Ray::new(hit_record.p, reflected);
        } else {
            *scattered = Ray::new(hit_record.p, refracted);
        }

        true
    }
}
 