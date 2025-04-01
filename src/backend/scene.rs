use image::Rgb;

use super::{
    ray::Ray,
    set::Set,
    vec3::{Vec3, ZERO},
};
use crate::arg::Cli;
use crate::arg::Goal::{Gradient, Japan, Sky};

#[derive(Clone, Copy, Debug)]
pub struct View {
    pub center: Vec3,
    /// focal length & direction we're looking
    pub focal_length: Vec3,
    pub port_w: Vec3,
    pub port_h: Vec3,
}

impl View {
    pub fn new(center: Vec3, focus: Vec3, dims: (f64, f64)) -> Self {
        let h = Vec3(0.0, -1.0, 0.0); //try to make it point in the negative y-direction
        let h = h.parallel_normal(focus).1.unit();
        let w = focus.cross(h).unit();
        Self {
            center,
            focal_length: focus,
            port_w: w * dims.0,
            port_h: h * dims.1,
        }
    }

    /// x and y are from 0 to 1
    pub fn pixel(self, x: f64, y: f64) -> Vec3 {
        self.center
            + self.focal_length
            + self.port_w * (2.0 * x - 1.0)
            + self.port_h * (2.0 * y - 1.0)
    }

    /// x and y are from 0 to 1
    pub fn pixel_ray(self, x: f64, y: f64) -> Ray {
        Ray {
            orig: self.center,
            dir: self.pixel(x, y) - self.center,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub view: View,
    pub set: Set,
}

impl Scene {
    pub fn color(&self, x: f64, y: f64) -> Rgb<u8> {
        self.set.color_ray(self.view.pixel_ray(x, y))
    }
}

impl From<&Cli> for View {
    fn from(cli: &Cli) -> Self {
        let ratio: f64 = f64::from(cli.width) / f64::from(cli.height);
        let w = 2f64;
        let h = w / ratio;
        let phi = cli.phi;
        let theta = cli.theta;
        match cli.render {
            // both are 0: 0,0,-1
            Gradient | Sky | Japan { .. } => Self::new(
                ZERO,
                Vec3(theta.sin() * phi.cos(), phi.sin(), -theta.cos() * phi.cos()),
                (w, h),
            ),
        }
    }
}

impl From<&Cli> for Scene {
    fn from(cli: &Cli) -> Self {
        Scene {
            view: View::from(cli),
            set: Set::from(cli),
        }
    }
}
