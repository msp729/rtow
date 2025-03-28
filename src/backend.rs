use crate::arg::{Cli, Color, Goal};
use crate::err::Err;
use image::{ImageFormat, Rgb, RgbImage};
use rayon::prelude::*;
use std::io::{BufWriter, Seek, Write};

pub fn setup(args: &Cli) -> RgbImage {
    return RgbImage::new(args.width, args.height);
}

fn ifrac(numerator: u32, denominator: u32) -> u8 {
    let big = numerator << 8;
    let scaled = big.saturating_sub(1);
    (scaled / denominator) as u8
}

fn color(cell: &mut Rgb<u8>, x: u32, y: u32, w: u32, h: u32, xcolor: Color, ycolor: Color) {
    cell.0[0] &= 0;
    cell.0[1] &= 0;
    cell.0[2] &= 0;
    cell.0[xcolor as usize] |= ifrac(x, w);
    cell.0[ycolor as usize] |= ifrac(y, h);
}

pub fn render(img: &mut RgbImage, goal: Goal) {
    let w = img.width();
    let h = img.height();
    match goal {
        Goal::Gradient { xcolor, ycolor } => img
            .par_enumerate_pixels_mut()
            .for_each(|(x, y, cell)| color(cell, x, y, w, h, xcolor, ycolor)),
    }
}

pub fn print<W: Write + Seek>(
    f: &mut BufWriter<W>,
    img: RgbImage,
    fmt: ImageFormat,
) -> Result<(), Err> {
    if !fmt.can_write() {
        eprintln!("Cannot encode into requested format.");
        std::process::exit(65); // 65 DATAERR, user asked for invalid operation
    }

    img.write_to(f, fmt)?;

    Ok(())
}
