use std::rc::Rc;

use crate::{HitRecord, Hittable, Ray};

pub struct Scene {
    pub objs: Vec<Rc<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { objs: Vec::new() }
    }
    pub fn clear(&mut self) {
        self.objs.clear();
    }
    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objs.push(obj);
    }
}

impl Hittable for Scene {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut cur_clostest = t_max;

        for obj in &self.objs {
            if let Some(trec) = obj.hit(r, t_min, cur_clostest) {
                cur_clostest = trec.t;
                rec = Some(trec);
            }
        }

        rec
    }
}
