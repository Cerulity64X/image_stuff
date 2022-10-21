use std::{env::Args, error::Error};

use bmp::Image;

use crate::utils;

pub enum GenerationType {
    LUT
}

pub fn img_gen(gt: GenerationType, mut argv: Args) -> Result<(), Box<dyn Error>> {
    match gt {
        GenerationType::LUT => {
            let r_gran: u32 = argv.next().expect("No red count found!").parse::<u32>().expect("Red count must be positive!");
            let g_gran: u32 = argv.next().expect("No green count found!").parse::<u32>().expect("Green count must be positive!");
            let b_width: u32 = argv.next().expect("No blue width supplied!").parse::<u32>().expect("Blue width must be positive!");
            let b_height: u32 = argv.next().expect("No blue height supplied!").parse::<u32>().expect("Blue height must be positive!");
            let mut lut: Image = Image::new(r_gran * b_width, g_gran * b_height);
            for w in 0..b_width {
                for h in 0..b_height {
                    for x in 0..r_gran {
                        for y in 0..g_gran {
                            lut.set_pixel(x + w * r_gran, y + h * g_gran, utils::pix_from_u32(x * 256 / r_gran, y * 256 / g_gran, (w + h * b_width) * 256 / (b_width * b_height)));
                        }
                    }
                }
            }
            lut.save(format!("output/LUT {}R {}G {}B {}BW {}BH {}x{}.bmp", r_gran, g_gran, b_height * b_width, b_width, b_height, r_gran * b_width, g_gran * b_height))?
        }
    }
    Ok(())
}