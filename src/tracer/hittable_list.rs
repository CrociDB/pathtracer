use super::hittable::{
    HitRecord,
    Hittable
};

use super::ray::Ray;

pub struct HittableList {
    list: Vec<Box<Hittable + 'static>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            list: Vec::new() 
        }
    }

    pub fn add_hittable<T>(&mut self, hittable: T)
        where T: Hittable + 'static {
        self.list.push(Box::new(hittable));
    }
}

impl Hittable for HittableList {
    fn hit<'a, 'b>(&'a self, ray:&Ray, t_min:f32, t_max:f32, record:&'b mut HitRecord<'a>) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for h in &self.list {
            if h.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *record = temp_rec;
            }
        }

        hit_anything
    }
}

