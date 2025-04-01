use image::Rgb;

use crate::arg::{Cli, Goal};

use super::{
    ray::Ray,
    vec3::{Vec3, I, J, K},
};

#[derive(Clone, Debug)]
pub enum Set {
    Gradient,
    Sky,
    Japan(Object),
}

#[derive(Clone, Copy, Debug)]
pub enum Object {
    Sphere { center: Vec3, radius: f64 },
}

impl From<&Cli> for Set {
    fn from(value: &Cli) -> Self {
        match value.render {
            Goal::Gradient => Set::Gradient,
            Goal::Sky => Set::Sky,
            Goal::Japan { dist, radius } => Set::Japan(Object::Sphere {
                center: -K * dist,
                radius,
            }),
        }
    }
}

impl Set {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn color_ray(&self, ray: Ray) -> Rgb<u8> {
        match self {
            Set::Gradient => {
                let dir = ray.dir.unit();
                let upshifted = dir + I + J + K;
                let upscaled = upshifted * 127.999;
                Rgb([upscaled.0 as u8, upscaled.1 as u8, upscaled.2 as u8])
            }
            Set::Sky => {
                let dir = ray.dir.unit();
                let y = 1.0 - dir.1;
                let v = (y * 127.999) as u8;
                Rgb([v, v, 255])
            }
            Set::Japan(dot) => {
                if !dot.intersections(ray).is_empty() {
                    return Rgb([255, 0, 0]);
                }
                let dir = ray.dir.unit();
                let y = 1.0 - dir.1;
                let v = (y * 127.999) as u8;
                Rgb([v, v, 255])
            }
        }
    }
}

impl Object {
    #[allow(clippy::many_single_char_names)] // quadratic formula <3
    pub fn intersections(self, r: Ray) -> Vec<f64> {
        match self {
            Object::Sphere { center, radius } => {
                let v = center - r.orig;
                let a = r.dir.length_squared();
                let b = r.dir.dot(v) * 2.0;
                let c = v.length_squared() - radius * radius;
                let m = -b / (a + a);
                let p = c / a;
                let d = m * m - p;
                if d < 0.0 {
                    Vec::new()
                } else if d == 0.0 {
                    vec![m]
                } else {
                    vec![m - d.sqrt(), m + d.sqrt()]
                }
            }
        }
    }
}
