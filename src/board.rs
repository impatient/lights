use crate::rgb::RGB;
use rs_ws281x::{ChannelBuilder, ControllerBuilder, StripType};
pub use crate::vboard::Lights;
use std::sync::{Arc, Mutex};
impl Lights {
    pub fn rpi(pixels: Arc<Mutex<Vec<RGB>>>) {
        let mut controller = ControllerBuilder::new()
            // default
            .freq(800_000)
            // default
            .dma(10)
            .channel(
                0,
                ChannelBuilder::new()
                    .pin(18)
                    .count(100)
                    .strip_type(StripType::Ws2811Rgb)
                    .brightness(255)
                    .build(),
            )
            .build()
            .expect("Unable to connect to lights");

        loop {
            std::thread::sleep(std::time::Duration::from_millis(30));
            let leds = controller.leds_mut(0);
            if let Ok(pixs) = pixels.lock() {
                for (idx, item) in pixs.iter().enumerate() {
                    // this chicanery is due to the board actually being BGR and the zig-zag layout
                    // of pixels
                    if idx / 10 % 2 == 1 {
                        let base = idx / 10 * 10;
                        let offset = idx - base;
                        let placement = base + 9 - offset;
                        leds[placement] = [item.b, item.g, item.r, 0];
                    }
                    else {
                        leds[idx] = [item.b, item.g, item.r, 0];
                    }
                }
            }
            controller.render();
        }
    }
}

