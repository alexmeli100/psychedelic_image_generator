mod generator;

#[macro_use]
extern crate itertools;
pub mod options;

use generator::image_generator::ImageGenerator;
use generator::wallpaper::set_wallpaper;
use options::*;
use uuid::Uuid;
use log::info;
use indicatif::ParallelProgressIterator;
use std::fs::canonicalize;
use std::fs::create_dir_all;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use std::error::Error;

fn generate_image<PA>(img_path: PA, w: u32, h: u32)
where PA: AsRef<Path>,
{
    let mut img_gen = ImageGenerator::new(w, h);

    img_gen.generate(img_path);
}

fn create_images(n: usize, w: u32, h: u32, path: PathBuf) -> generator::Result<()> {
    info!("Saving images into {}", &path.to_str().unwrap());
 
    (0..n).into_par_iter()
        .progress()
        .for_each(|_| {
            create_image(&path, w, h);
            info!("created {:?}", path);
        });

    info!("Finished saving images");
    Ok(())
}

fn create_image(path: &PathBuf, w: u32, h: u32) -> PathBuf {
    let filename = format!("PI_{}.png", Uuid::new_v4());
    let img_path = path.join(filename);
    generate_image(&img_path, w, h);

    img_path
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    env_logger::init();

    create_dir_all(&opt.path)?;

    let abs_path = canonicalize(&opt.path)?;

    if opt.wallpaper {
        info!("creating wallpaper image");
        let img_path = create_image(&abs_path, opt.width, opt.height);
        info!("setting wallpaper");
        set_wallpaper(img_path.to_str().unwrap())?;
        info!("finished setting wallpaper");
        info!("the image can be found in {:?}", img_path);
    } else {
        create_images(opt.num, opt.width, opt.height, abs_path)?
    }

    Ok(())
}

