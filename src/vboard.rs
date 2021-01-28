use fltk::*;
use fltk::{app::*, draw, frame::*, window::*};
use std::sync::{Arc, Mutex};
use std::convert::TryFrom;
use crate::rgb::RGB;

pub struct Lights;
/* Had issues with memory scope due to closures
pub struct Redraw; // Marker object to trigger a refresh
pub struct Lights {
    pixels: Arc<Mutex<Vec<RGB>>>,           // TODO lifetime specifier or MOVE to message
    // was use std::sync::mpsc::Receiver as Rec; to send refresh
    receiver: Rec<Redraw>, // Used to trigger a refresh
}*/

// I wanted to do a trait for both the physical device and fltk, but
// I had issues with borrowing/referencing with closures when in struct
//pub trait LightReader {
//  fn watch(&'static self) -> (); 
//}

// TODO move lights to a trait.  Have FL impl and Physical impl
impl Lights {
    pub fn fltk(pixels: Arc<Mutex<Vec<RGB>>>) {
        let app = App::default();
        let mut wind = Window::new(100, 100, 300, 300, "Lights");
        wind.set_color(Color::Dark2);
        wind.set_border(true);
        let mut frame = Frame::new(0, 0, 300, 300, "");

        let fills_square = move|f: &mut Frame| {
            f.set_color(Color::Red);
            if let Ok(pixs) = pixels.lock() {
                for (idx, item) in pixs.iter().enumerate() {
                    let i = i32::try_from(idx).expect("Unable to integer");
                    let x : i32 = i % 10 * 30;
                    let y : i32 = i / 10 * 30;
                    let display = Color::from_rgb(item.r, item.g, item.b);
                    draw::set_draw_color(display);
                    draw::draw_rectf(x,y, 30,30);
                }
            }
        };


        frame.draw2(fills_square);

        wind.end();
        wind.show();

        while app.wait() {
            std::thread::sleep(std::time::Duration::from_millis(30));
        frame.redraw();
        }

    }
}

