use crate::backend::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    fn at(self, x: f64) -> Vec3 {
        self.orig + self.dir * x
    }
}
