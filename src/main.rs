use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod cpu;
mod gpu;

const UPSCALE_FACTOR: u8 = 4;
const WIDTH: usize = 160 * UPSCALE_FACTOR as usize;
const HEIGHT: usize = 144 * UPSCALE_FACTOR as usize;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect(":( Oopsie Woopsie! Uwu We made a fucky wucky!! A wittle fucko boingo!
    The code monkeys at our headquarters are working VEWY HAWD to fix this!");

    window.limit_update_rate(Some(Duration::from_micros(33200)));

    while window.is_open() {
        for i in buffer.iter_mut() {
            *i = 0;
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("window update error");
    }
}
