pub mod funcs;
pub mod wallpaper;
pub mod image_generator;

use std::error::Error;


pub type Result<T> = std::result::Result<T, Box<dyn Error>>;