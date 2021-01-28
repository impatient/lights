use std::convert::TryFrom;

// TODO move request to ISS module
use reqwest;
use reqwest::Error;
mod iss;
use iss::{IssLocation, ISS_ENDPOINT};

mod rgb;
use rgb::RGB;

use std::env;

mod vboard;
mod board;
mod gif;

use gif::GifSrc;

use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {

    let args_vec : Vec<String> = env::args().collect();
    let fltk = !args_vec.contains(&String::from("pi"));
    let pi = args_vec.contains(&String::from("pi"));
    // look shadowing intentional first class.
    // vec macro, size 100, initialized "off"
    let pixels: Vec<RGB> = vec![RGB { r: 0, g: 0, b: 0 }; 100];
    // make sure can only be updated by one owner at a time
    let pixels = Mutex::new(pixels);
    // acr = multi thread reference counter
    let pixels = Arc::new(pixels);

    println!("Hi Mazie!!!")

    // Was planning on triggering reload through this channel. 
    // Switched to just letting each read at their leisure
    // This is synchronous. Was having to span 2 threads for this pattern in fltk
    // let (sendChannel, receiveChannel) = channel::<vboard::Redraw>();
    if fltk {
    // clone makes a copy of the reference
    // per my understanding. Pointer + size.
    // We don't update p_clone, just pixels
    let p_clone = pixels.clone();
    // We should keep a track of the threads and join to current
    // before ending
    thread::spawn(|| {
        vboard::Lights::fltk(p_clone);
    });
    }
    if pi {
      let pi_clone = pixels.clone();
      thread::spawn(|| { board::Lights::rpi(pi_clone);
      });
    }

    let src = GifSrc::new(String::from("/home/scott/sample.gif"));

    src.animate(&pixels);
    //loop {
        //random(&pixels);
    //}
}

fn random(pixels: &Arc<Mutex<Vec<RGB>>>) {
        let delay = u64::try_from(rand::random::<u8>())
            .expect("I have to be doing this wrong");

        thread::sleep(time::Duration::from_millis(delay));
        if let Ok(mut p) = pixels.lock() {
            // Should do one random and bit twiddle
            for pixel in p.iter_mut() {
                pixel.r = rand::random::<u8>();
                pixel.g = rand::random::<u8>();
                pixel.b = rand::random::<u8>();
            }
        }

}
// TODO move this into ISS
#[tokio::main]
async fn mains() -> Result<(), Error> {
    let response = reqwest::get(ISS_ENDPOINT).await?;
    let body: IssLocation = response.json().await?;
    println!("Current location {:?}", body);
    Ok(())
}
