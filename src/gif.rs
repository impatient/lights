use image::gif::GifDecoder;
use image::AnimationDecoder;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::{thread,time};

use crate::rgb::RGB;
use image::Pixel;

pub struct GifSrc {
    frames: Vec<Vec<RGB>>,
}

impl GifSrc {
    pub fn new(file_name: String) -> GifSrc {
        let gif = File::open(file_name).expect("Unable to open file");
        let decoder = GifDecoder::new(gif).unwrap();
        let frames = decoder.into_frames();
        let frames = frames.collect_frames().expect("Error decoding");

        let mut pix_frames = Vec::new();

        for f in frames.iter() {
            println!("New frame");
            let mut current_frame = Vec::new();
            for p in f.buffer().pixels() {
                let rgb = p.to_rgb();
                current_frame.push(RGB {
                    r: rgb[0],
                    g: rgb[1],
                    b: rgb[2],
                });
            }
            pix_frames.push(current_frame);
        }
        GifSrc { frames: pix_frames }
    }

    pub fn animate(&self, pixels: &Arc<Mutex<Vec<RGB>>>) {
        loop {
            for i in 0..self.frames.len() {
                if let Ok(mut p) = pixels.lock() {
                    // mutable deref allows us to replace with copy of frame data
                    *p = self.frames[self.frames.len() - 1 - i ].to_owned();
                    
                }
                    thread::sleep(time::Duration::from_millis(1000));

            }
        }
    }
}
