use std::{hash::Hash, collections::HashMap};

use bmp::Pixel;

// Creates a pixel from u32 values.
pub fn pix_from_u32(r: u32, g: u32, b: u32) -> Pixel {
    Pixel::new(r as u8, g as u8, b as u8)
}

// Hashmap get mut or insert default
pub fn hm_get_mut_or_default<'a, K: Hash + Eq + Clone, V>(hm: &'a mut HashMap<K, V>, key: &K, def: V) -> &'a mut V {
    if !hm.contains_key(key) {
        hm.insert(key.clone(), def);
    }
    hm.get_mut(key).unwrap()
}