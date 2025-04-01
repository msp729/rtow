use std::path::PathBuf;

use clap::{builder::PossibleValue, Parser, Subcommand, ValueEnum};

use image::ImageFormat;
use image::ImageFormat::{
    Avif, Bmp, Dds, Farbfeld, Gif, Hdr, Ico, Jpeg, OpenExr, Pcx, Png, Pnm, Qoi, Tga, Tiff, WebP,
};

#[derive(Clone, Copy, Debug)]
pub struct Format(pub ImageFormat);

const FORMAT_VARIANTS: &[Format] = &[
    Format(Png),
    Format(Jpeg),
    Format(Gif),
    Format(WebP),
    Format(Pnm),
    Format(Tiff),
    Format(Tga),
    Format(Dds),
    Format(Bmp),
    Format(Ico),
    Format(Hdr),
    Format(OpenExr),
    Format(Farbfeld),
    Format(Avif),
    Format(Qoi),
    Format(Pcx),
];

impl ValueEnum for Format {
    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Format(Png) => Some(PossibleValue::from("png")),
            Format(Jpeg) => Some(PossibleValue::from("jpeg").alias("jpg")),
            Format(Gif) => Some(PossibleValue::from("gif")),
            Format(WebP) => Some(PossibleValue::from("webp")),
            Format(Pnm) => Some(PossibleValue::from("pnm")),
            Format(Tiff) => Some(PossibleValue::from("tiff")),
            Format(Tga) => Some(PossibleValue::from("tga")),
            Format(Dds) => Some(PossibleValue::from("dds")),
            Format(Bmp) => Some(PossibleValue::from("bmp")),
            Format(Ico) => Some(PossibleValue::from("ico")),
            Format(Hdr) => Some(PossibleValue::from("hdr")),
            Format(OpenExr) => Some(PossibleValue::from("openexr")),
            Format(Farbfeld) => Some(PossibleValue::from("farbfeld")),
            Format(Avif) => Some(PossibleValue::from("avif")),
            Format(Qoi) => Some(PossibleValue::from("qoi")),
            Format(Pcx) => Some(PossibleValue::from("pcx")),
            _ => todo!(),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        FORMAT_VARIANTS
    }
}

impl Format {
    pub fn ext(self) -> String {
        match self {
            Format(Png) => "png".into(),
            Format(Jpeg) => "jpeg".into(),
            Format(Gif) => "gif".into(),
            Format(WebP) => "webp".into(),
            Format(Pnm) => "pnm".into(),
            Format(Tiff) => "tiff".into(),
            Format(Tga) => "tga".into(),
            Format(Dds) => "dds".into(),
            Format(Bmp) => "bmp".into(),
            Format(Ico) => "ico".into(),
            Format(Hdr) => "hdr".into(),
            Format(OpenExr) => "openexr".into(),
            Format(Farbfeld) => "farbfeld".into(),
            Format(Avif) => "avif".into(),
            Format(Qoi) => "qoi".into(),
            Format(Pcx) => "pcx".into(),
            _ => String::new(),
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'd', long = "dir", name = "out-dir")]
    pub out_dir: Option<PathBuf>,

    #[arg(short = 'o', long = "out", name = "out")]
    pub out_name: Option<PathBuf>,

    #[arg(short = 'f', long = "format", default_value = "png")]
    pub out_fmt: Format,

    #[arg(short = 'x', long, default_value_t = 1080)]
    pub width: u32,

    #[arg(short = 'y', long, default_value_t = 720)]
    pub height: u32,

    #[command(subcommand)]
    pub render: Goal,
}

#[derive(Clone, Copy, Debug, Subcommand)]
pub enum Goal {
    #[command()]
    Gradient {
        #[arg(default_value_t = 0.0)]
        theta: f64,
        #[arg(default_value_t = 0.0)]
        phi: f64,
    },
}
