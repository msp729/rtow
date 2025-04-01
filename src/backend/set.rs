use image::Rgb;

use crate::arg::{Cli, Goal};

use super::{
    ray::Ray,
    vec3::{I, J, K},
};

#[derive(Clone, Debug)]
pub enum Set {
    Gradient,
}

impl From<&Cli> for Set {
    fn from(value: &Cli) -> Self {
        match value.render {
            Goal::Gradient { .. } => Set::Gradient,
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
        }
    }
}
