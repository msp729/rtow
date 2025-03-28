use crate::arg::{Cli, Goal};
use crate::err::Err;
use image::{ImageFormat, Rgb, RgbImage};
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

fn color(x: u32, y: u32, w: u32, h: u32, goal: Goal) -> [u8; 3] {
    match goal {
        Goal::RedGreen => [ifrac(x, w), ifrac(y, h), ifrac(0, 1)],
        Goal::RedBlue => [ifrac(x, w), ifrac(0, 1), ifrac(y, h)],
        Goal::BlueGreen => [ifrac(0, 1), ifrac(y, h), ifrac(x, w)],
    }
}

pub fn render(img: &mut RgbImage, goal: Goal) {
    let w = img.width();
    let h = img.height();
    match goal {
        Goal::RedGreen | Goal::RedBlue | Goal::BlueGreen => {
            img.enumerate_rows_mut().for_each(|(y, row)| {
                eprint!("\rCurrently on row {y} of {h}...");
                row.for_each(|(x, y, cell)| cell.0 = color(x, y, w, h, goal));
            });
            eprintln!("Done!");
        }
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
