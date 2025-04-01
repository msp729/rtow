use crate::arg::Cli;
use crate::err::Err;
use image::{ImageFormat, RgbImage};
use rayon::prelude::*;
use std::io::{BufWriter, Seek, Write};

mod ray;
mod scene;
mod set;
mod vec3;

pub struct Render {
    pub img: RgbImage,
    pub scene: scene::Scene,
}

pub fn setup(args: &Cli) -> Render {
    Render {
        img: RgbImage::new(args.width, args.height),
        scene: scene::Scene::from(args),
    }
}

pub fn render(trg: &mut Render) {
    let Render { img, scene } = trg;
    let w = img.width();
    let h = img.height();
    img.par_enumerate_pixels_mut().for_each(|(x, y, cell)| {
        *cell = scene.color(
            f64::from(x) / f64::from(w - 1),
            f64::from(y) / f64::from(h - 1),
        );
    });
}

pub fn print<W: Write + Seek>(
    f: &mut BufWriter<W>,
    src: &Render,
    fmt: ImageFormat,
) -> Result<(), Err> {
    let Render { img, scene: _ } = src;

    if !fmt.can_write() {
        eprintln!("Cannot encode into requested format.");
        std::process::exit(65); // 65 DATAERR, user asked for invalid operation
    }

    img.write_to(f, fmt)?;

    Ok(())
}
