#![feature(never_type, try_trait_v2)]
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use clap::Parser;

mod arg;
mod backend;
mod err;

fn main() -> Result<(), err::Err> {
    let args = arg::Cli::parse();

    let mut out_path = args.out_dir.clone().unwrap_or(".".into());
    let out_fmt = args.out_fmt;
    let ext = out_fmt.ext();
    let default_path = PathBuf::from("out.".to_owned() + &ext);
    out_path.push(args.out_name.clone().unwrap_or(default_path));

    let file = File::create(&out_path)?;
    let mut file = BufWriter::new(file);

    let mut rays = backend::setup(&args);
    backend::render(&mut rays, args.render);
    backend::print(&mut file, rays, out_fmt.0)?;

    return Ok(());
}
