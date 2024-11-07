mod comp;
mod opcode;
mod utils;

use crate::comp::Chip8;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
use std::thread::sleep;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("chip 8", 640, 320)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    // canvas.window().
    let mut chip8 = Chip8::default();
    chip8.load_rom("./roms/airplane.ch8");

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        chip8.tick();
        // for i in 0..32 {
        //     for j in 0..64 {
        //         chip8.display.set(j, i)
        //     }
        // }

        chip8.display.show(&mut canvas);

        // Show it on the screen

        sleep(Duration::from_millis(16));
    }

    Ok(())
}
