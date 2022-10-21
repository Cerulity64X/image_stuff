use std::{collections::HashMap, error::Error, env::Args, cmp::Ordering};

use bmp::{Pixel, Image, consts::BLACK};

use crate::{HPix, utils};

// Sorts by mapping a pixel to a bit field (00000000bbbbbbbbggggggggrrrrrrrr) and comparing
#[allow(dead_code)]
pub fn bgr_sort(l: &Pixel, r: &Pixel) -> Ordering {
    let lv: u32 = l.r as u32 + l.g as u32 * 256 + l.b as u32 * 32768;
    let rv: u32 = r.r as u32 + r.g as u32 * 256 + r.b as u32 * 32768;
    lv.cmp(&rv)
}

// Sorts by standard luminance calculation
#[allow(dead_code)]
pub fn lum_sort(l: &Pixel, r: &Pixel) -> Ordering {
    let lv: f32 = l.r as f32 * 0.2126 + l.g as f32 * 0.7152 + l.b as f32 * 0.0722;
    let rv: f32 = r.r as f32 * 0.2126 + r.g as f32 * 0.7152 + r.b as f32 * 0.0722;
    lv.partial_cmp(&rv).unwrap_or(Ordering::Equal)
}

// Sorts an image by BGR, luminance, and quantity
pub fn img_sort(mut argv: Args) -> Result<(), Box<dyn Error>> {
    let fname = match argv.next() {
        Some(st) => st,
        None => Err("No file name supplied!")?
    };

    let print_quantities = match argv.next() {
        Some(st) => st == String::from("print"),
        None => false
    };

    let img: Image = bmp::open(&fname)?;
    println!("Image loaded.");
    let mut pixel_quantities: HashMap<HPix, usize> = HashMap::new();
    let img_width: u32 = img.get_width();
    let img_height: u32 = img.get_height();
    let mut luminance_sorted: Vec<Pixel> = vec![];

    println!("Extracting pixels...");
    for x in 0..img_width {
        for y in 0..img_height {
            let pix: Pixel = img.get_pixel(x, y);
            luminance_sorted.push(pix);
            let b: &mut usize = utils::hm_get_mut_or_default(&mut pixel_quantities, &HPix::from_pixel(&pix), 0);
            *b = *b + 1;
        }
    }

    println!("Copying pixels...");
    let mut bgr_sorted = luminance_sorted.clone();
    println!("Sorting pixels by BGR...");
    bgr_sorted.sort_by(bgr_sort);
    println!("Sorting pixels by luminance...");
    luminance_sorted.sort_by(lum_sort);

    println!("Injecting BGR and luminance pixels...");
    let mut img_out_luminance: Image = Image::new(img_width, img_height);
    let mut img_out_bgr: Image = Image::new(img_width, img_height);
    let mut img_out_quantities: Image = Image::new(img_width, img_height);

    let mut i = 0;
    for y in 0..img_height {
        for x in 0..img_width {
            img_out_luminance.set_pixel(x, y, *luminance_sorted.get(i).unwrap_or(&BLACK));
            img_out_bgr.set_pixel(x, y, *bgr_sorted.get(i).unwrap_or(&BLACK));
            i += 1;
        }
    }

    println!("Sorting pixels by quantity...");
    let mut quant_sorted: Vec<(HPix, usize)> = pixel_quantities.into_iter().collect::<Vec<(HPix, usize)>>();
    quant_sorted.sort_by(|a, b| a.1.cmp(&b.1));

    println!("Injecting quantity pixels...");
    let mut n: usize = 0;
    for i in quant_sorted {
        for _ in 0..i.1 {
            img_out_quantities.set_pixel(n as u32 % img_width, n as u32 / img_width, i.0.to_pixel());
            n += 1;
        }
        if print_quantities {
            println!("{} pixels are {}", i.1, i.0);
        }
    }

    println!("Saving images...");
    img_out_luminance.save(format!("output/[Luminance sorted] {}", fname))?;
    img_out_bgr.save(format!("output/[BGR sorted] {}", fname))?;
    img_out_quantities.save(format!("output/[Quant sorted] {}", fname))?;
    Ok(())
}