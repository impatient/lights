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
                    leds[idx] = [item.r, item.g, item.b, 0];
                }
            }
            controller.render();
        }
    }
}
