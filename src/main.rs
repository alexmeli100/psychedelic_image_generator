#[macro_use]
extern crate itertools;

pub mod image_generator;
pub mod funcs;

use image_generator::ImageGenerator;
use std::io;
use std::fmt::Display;
use std::fs::create_dir_all;
use rayon::prelude::*;
use std::path::Path;

fn make_image<PA, PB>(img_dir: PA, base: PB, dims: u32) -> Result<(), &'static str>
where PA: AsRef<Path>,
       PB: AsRef<Path> + Display,

{
    let mut img_gen = ImageGenerator::new(dims);

    img_gen.generate(img_dir.as_ref().join(&base));
    println!("Created {}", base);
    Ok(())
}

fn run(n: u32) -> Result<(), io::Error> {
    let img_dir = "images";
    create_dir_all(img_dir)?;

    println!("Saving images into /{}", img_dir);
 
    (0..n).into_par_iter()
        .for_each(|x| {
            make_image(img_dir, format!("img{}.png", x), 600)
                .map_err(|e| println!("{}", e)).unwrap();
        });

    Ok(())
}

fn main() {
    run(30u32).unwrap();
}
