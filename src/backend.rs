use crate::arg::{Cli, Goal};
use crate::err::Err;
use image::{ImageFormat, RgbImage};
use std::io::{BufWriter, Seek, Write};
use std::path::PathBuf;

pub fn setup(args: &Cli) -> RgbImage {
    return RgbImage::new(args.width, args.height);
}

fn ifrac(numerator: u32, denominator: u32) -> u8 {
    let big = numerator << 8;
    let scaled = big.saturating_sub(1);
    (scaled / denominator) as u8
}

pub fn render(img: &mut RgbImage, goal: Goal) {
    let h = img.height();
    let w = img.width();
    match goal {
        Goal::RedGreen => img
            .enumerate_pixels_mut()
            .for_each(|(x, y, cell)| cell.0 = [ifrac(x, w), ifrac(y, h), 0]),
        Goal::RedBlue => img
            .enumerate_pixels_mut()
            .for_each(|(x, y, cell)| cell.0 = [ifrac(x, w), 0, ifrac(y, h)]),
        Goal::BlueGreen => img
            .enumerate_pixels_mut()
            .for_each(|(x, y, cell)| cell.0 = [0, ifrac(y, h), ifrac(x, w)]),
    }
}

pub fn print<W: Write + Seek>(
    f: &mut BufWriter<W>,
    img: RgbImage,
    name: PathBuf,
    fmt: ImageFormat,
) -> Result<(), Err> {
    if !fmt.can_write() {
        eprintln!("Cannot encode into requested format.");
        std::process::exit(65); // 65 DATAERR, user asked for invalid operation
    }

    img.write_to(f, fmt)?;

    Ok(())
}
