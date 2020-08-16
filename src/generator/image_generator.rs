use crate::generator::funcs::*;

use std::f64;
use image::{ImageBuffer, RgbImage};
use std::path::Path;

pub struct ImageGenerator {
    img: RgbImage,
    dims: (usize, usize),
    red_exp: Box<dyn EvalFunc>,
    green_exp: Box<dyn EvalFunc>,
    blue_exp: Box<dyn EvalFunc>
}

impl ImageGenerator {
    pub fn new(w: u32, h: u32) -> ImageGenerator {
        ImageGenerator{
            img: ImageBuffer::new(w, h),
            dims: (w as usize, h as usize),
            red_exp: build_expr(0.99),
            green_exp: build_expr(0.99),
            blue_exp: build_expr(0.99)
        }
    }

    pub fn generate<PA>(&mut self, p: PA)
        where PA: AsRef<Path>
    {
        let red_plane = get_intensities(&self.red_exp, self.dims);
        let green_plane = get_intensities(&self.green_exp, self.dims);
        let blue_plane = get_intensities(&self.blue_exp, self.dims);

        for (i, (r, g, b)) in izip!(red_plane, green_plane, blue_plane).enumerate() {
            let y = i / self.dims.0 as usize;
            let x = i % self.dims.0 as usize;

            let pixel = self.img.get_pixel_mut(x as u32, y as u32);
            *pixel = image::Rgb([r, g, b]);
        }

        self.img.save(p).unwrap();
    }
}



fn get_intensities(exp: &Box<dyn EvalFunc>, dims: (usize, usize)) -> Vec<u8> {
    let ppu = dims.0 / 2;
    let w = dims.0;
    let h = dims.1;
    let mut intensity_vec = Vec::with_capacity(w * h);

    for i in 0..w*h {
        let y_coord = i / w;
        let x_coord = i % w;

        let y = (y_coord as isize - ppu as isize) as f64 / ppu as f64;
        let x = -(x_coord as isize - ppu as isize) as f64 / ppu as f64;
        let z = exp.eval(x, y);
        let intensity = (z * 127.5 + 127.5) as u8;

        intensity_vec.push(intensity);
    }
    intensity_vec
}