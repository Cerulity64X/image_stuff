//! This module is for code not yet completed or code that needs to be fixed.
//! DO NOT USE ANY CODE INSIDE THIS MODULE. IT IS HIGHLY VOLATILE.
//! CODE IN THIS MODULE SHOULD BE MARKED AS DEPRECATED.
//! Code in this module may be moved, deleted, changed or renamed AT ANY TIME.

// Applies a filter over every group of pixels to find the best fit. CURRENTLY INCOMPLETE
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