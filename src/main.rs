use std::{env::{args, Args}, error::Error, hash::Hash, fmt::Display};

use bmp::{Pixel};
use gen::GenerationType;

mod gen;
mod utils;
mod sort;

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut argv: Args = args();
    argv.next();
    let manip = &argv.next().expect("No manipulation type supplied")[..];

    match manip {
        "sort" => {
            sort::img_sort(argv)
        },
        "gen" => {
            gen::img_gen(
            match &argv.next().expect("Generation type not supplied!")[..] {
                "lut" => GenerationType::LUT,
                er => {panic!("Generation type {} not found!", er)}
            },
            argv)
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
