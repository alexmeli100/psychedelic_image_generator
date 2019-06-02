#[macro_use]
extern crate itertools;

pub mod image_generator;
pub mod funcs;
pub mod input;

use image_generator::ImageGenerator;
use input::*;
use std::io;


use std::fmt::Display;
use std::fs::create_dir_all;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

fn make_image<PA, PB>(img_dir: PA, base: PB, dims: u32) -> Result<(), &'static str>
where PA: AsRef<Path>,
       PB: AsRef<Path> + Display,

{
    let mut img_gen = ImageGenerator::new(dims);

    img_gen.generate(img_dir.as_ref().join(&base));
    println!("Created {}", base);
    Ok(())
}

fn run(n: usize, size: u32, path: PathBuf) -> Result<(), io::Error> {
    create_dir_all(&path)?;

    println!("Saving images into {}", &path.to_str().unwrap());
 
    (0..n).into_par_iter()
        .for_each(|x| {
            make_image(&path, format!("img{}.png", x), size)
                .map_err(|e| println!("{}", e)).unwrap();
        });

    Ok(())
}

fn main() {
    let opt = Opt::from_args();

    run(opt.num, opt.size, opt.path).unwrap();
}
