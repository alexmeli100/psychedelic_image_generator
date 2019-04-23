use crate::funcs::*;

use std::f64;
use image::{ImageBuffer, RgbImage};
use std::path::Path;

pub struct ImageGenerator {
    img: RgbImage,
    dims: u32,
    red_exp: Box<EvalFunc>,
    green_exp: Box<EvalFunc>,
    blue_exp: Box<EvalFunc>
}

impl ImageGenerator {
    pub fn new(n: u32) -> ImageGenerator {
        ImageGenerator{
            img: ImageBuffer::new(n, n),
            dims: n,
            red_exp: build_expr(0.99),
            green_exp: build_expr(0.99),
            blue_exp: build_expr(0.99)
        }
    }

    pub fn generate<PA>(&mut self, p: PA)
        where PA: AsRef<Path>
    {
        let ppu = self.dims as usize/2;
        let red_plane = get_intensities(&self.red_exp, ppu);
        let green_plane = get_intensities(&self.green_exp, ppu);
        let blue_plane = get_intensities(&self.blue_exp, ppu);

        for (i, (r, g, b)) in izip!(red_plane, green_plane, blue_plane).enumerate() {
            let y = i / self.dims as usize;
            let x = i % self.dims as usize;

            let pixel = self.img.get_pixel_mut(x as u32, y as u32);
            *pixel = image::Rgb([r, g, b]);
        }

        self.img.save(p).unwrap();
    }
}



fn get_intensities(exp: &Box<EvalFunc>, ppu: usize) -> Vec<u8> {
    let width = 2 * ppu;
    let mut intensity_vec = Vec::with_capacity(width * width);

    for i in 0..width*width {
        let y_coord = i / width;
        let x_coord = i % width;

        let y = (y_coord as isize - ppu as isize) as f64 / ppu as f64;
        let x = -(x_coord as isize - ppu as isize) as f64 / ppu as f64;
        let z = exp.eval(x, y);
        let intensity = (z * 127.5 + 127.5) as u8;

        intensity_vec.push(intensity);
    }
    intensity_vec
}