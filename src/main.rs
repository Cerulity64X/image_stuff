use std::{env::args, error::Error, cmp::Ordering, collections::HashMap, hash::Hash, fmt::Display};

use bmp::{Image, Pixel, consts::BLACK};

// Stuff is in different functions so I can add more features later on, and it separates the loading and processing.
fn main() -> Result<(), Box<dyn Error>> {
    let mut argv = args();
    argv.next();
    let manip = &argv.next().expect("No manipulation type supplied")[..];

    let fname = match argv.next() {
        Some(st) => st,
        None => Err("No file name supplied!")?
    };
    let img: Image = bmp::open(&fname)?;
    println!("Image loaded.");

    match manip {
        "sort" => {
            img_sort(img, &fname, match argv.next() {
                Some(st) => st == String::from("print"),
                None => false
            })
        }
        st => panic!("Unknown manipulation type {}!", st)
    }
    /*let filname = match argv.next() {
        Some(st) => st,
        None => {
            println!("No filename specified. Defaulting to filter.bmp");
            "filter.bmp".to_string()
        }
    };
    let filter: Image = bmp::open(&filname).expect(&format!("{} not found!", filname));
    println!("Filter loaded.");*/
}

// Sorts by mapping a pixel to a bit field (00000000bbbbbbbbggggggggrrrrrrrr) and comparing
#[allow(dead_code)]
fn bgr_sort(l: &Pixel, r: &Pixel) -> Ordering {
    let lv: u32 = l.r as u32 + l.g as u32 * 256 + l.b as u32 * 32768;
    let rv: u32 = r.r as u32 + r.g as u32 * 256 + r.b as u32 * 32768;
    lv.cmp(&rv)
}

// Sorts by standard luminance calculation
#[allow(dead_code)]
fn lum_sort(l: &Pixel, r: &Pixel) -> Ordering {
    let lv: f32 = l.r as f32 * 0.2126 + l.g as f32 * 0.7152 + l.b as f32 * 0.0722;
    let rv: f32 = r.r as f32 * 0.2126 + r.g as f32 * 0.7152 + r.b as f32 * 0.0722;
    lv.partial_cmp(&rv).unwrap_or(Ordering::Equal)
}

// Hashable pixel
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct HPix {
    r: u8,
    g: u8,
    b: u8
}
impl HPix {
    fn from_pixel(p: &Pixel) -> HPix {
        HPix { r: p.r, g: p.g, b: p.b }
    }
    fn to_pixel(&self) -> Pixel {
        Pixel { r: self.r, g: self.g, b: self.b }
    }
}
// Printable as well :)
impl Display for HPix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.r, self.g, self.b)
    }
}

// Hashmap get mut or insert default
fn hm_get_mut_or_default<'a, K: Hash + Eq + Clone, V>(hm: &'a mut HashMap<K, V>, key: &K, def: V) -> &'a mut V {
    if !hm.contains_key(key) {
        hm.insert(key.clone(), def);
    }
    hm.get_mut(key).unwrap()
}

// Sorts an image by 
fn img_sort(img: Image, fname: &String, print_quantities: bool) -> Result<(), Box<dyn Error>> {
    let mut pixel_quantities: HashMap<HPix, usize> = HashMap::new();
    let img_width: u32 = img.get_width();
    let img_height: u32 = img.get_height();
    let mut luminance_sorted: Vec<Pixel> = vec![];
    for x in 0..img_width {
        for y in 0..img_height {
            let pix: Pixel = img.get_pixel(x, y);
            luminance_sorted.push(pix);
            let b: &mut usize = hm_get_mut_or_default(&mut pixel_quantities, &HPix::from_pixel(&pix), 0);
            *b = *b + 1;
        }
    }
    let mut bgr_sorted = luminance_sorted.clone();
    bgr_sorted.sort_by(bgr_sort);
    luminance_sorted.sort_by(lum_sort);
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
    let mut quant_sorted: Vec<(HPix, usize)> = pixel_quantities.into_iter().collect::<Vec<(HPix, usize)>>();
    quant_sorted.sort_by(|a, b| a.1.cmp(&b.1));
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
    img_out_luminance.save(format!("output/[Luminance sorted] {}", fname))?;
    img_out_bgr.save(format!("output/[BGR sorted] {}", fname))?;
    img_out_quantities.save(format!("output/[Quant sorted] {}", fname))?;
    Ok(())
}

// Applies a filter over every group of pixels to find the best fit.
#[allow(dead_code)]
fn img_conv(img: Image, filter: Image) -> Result<(), Box<dyn Error>> {
    let img_width: u32 = img.get_width();
    let img_height: u32 = img.get_height();
    let conv_width: u32 = filter.get_width();
    let conv_height: u32 = filter.get_height();
    let mut best_fit: usize = 0;
    let mut processed: usize = 0;
    let mut proc_threshold: usize = 1;
    let mut best_fits: Vec<usize> = vec![];
    let mut filtered: Vec<usize> = vec![];
    println!("Processing {} pixels...", img_width * img_height);
    for x in 0..(img_width - conv_width) {
        for y in 0..(img_height - conv_height) {
            let mut val: usize = 0;
            for w in 0..conv_width {
                for h in 0..conv_height {
                    let pix: Pixel = img.get_pixel(x + w, y + h);
                    let fpix: Pixel = img.get_pixel(w, h);
                    val += pix.r as usize * fpix.r as usize;
                    val += pix.g as usize * fpix.g as usize;
                    val += pix.b as usize * fpix.b as usize;
                    if val > best_fit {
                        best_fit = val;
                        best_fits = vec![processed];
                    } else if val == best_fit {
                        best_fits.push(processed);
                    }
                    filtered.push(val);
                }
            }
            processed += 1;
            if processed >= proc_threshold {
                println!("{} pixels processed.", processed);
                proc_threshold *= 2;
            }
        }
    }
    let mut st: String = String::new();
    st.push_str(&format!("(x: {}, y: {}, i: {})\n", best_fits[0] % img_width as usize, best_fits[0] / img_width as usize, best_fits[0]));
    for _ in best_fits {
        //st.push_str(&format!("(x: {}, y: {}, i: {})\n", i % img_width as usize, i / img_width as usize, i));
    }
    println!("Processed! Best fits are at {} with a strength of {}.", st, best_fit);
    Ok(())
}
