use super::material::{Material, random_in_unit_sphere, reflect, refract};
use super::ray::Ray;
use super::hittable::HitRecord;

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

        if ray.direction.dot(hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal.clone();
            ni_over_nt = self.ref_idx;
        } else {
            outward_normal = hit_record.normal.clone();
            ni_over_nt = 1.0 / self.ref_idx;
        }

        if refract(&ray.direction, &outward_normal, ni_over_nt, &mut refracted) {
            *scattered = Ray::new(hit_record.p, refracted);
        } else {
            *scattered = Ray::new(hit_record.p, reflected);
            return false;
        }

        true
    }
}
 