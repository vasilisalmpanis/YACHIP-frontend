use yachip8::CHIP8;
extern crate sdl2;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("CHIP8 Emulator", 800, 600)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGBA8888,
        64, // CHIP-8 width
        32  // CHIP-8 height
    ).unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut chip = CHIP8::new();
    chip.load_rom("../YACHIP-8/roms/games/Cave.ch8");
    'running: loop {
        chip.cycle();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                _ => {}
            }
        }

        texture.with_lock(None, |buffer: &mut [u8], _: usize|{
            for idx in 0..chip.graphics.len() {
                buffer[idx * 4] = if chip.graphics[idx] == 1 { 0xFF } else { 0xFF};
                buffer[idx * 4 + 1] = if chip.graphics[idx] == 1 { 0xFF } else { 0x00};
                buffer[idx * 4 + 2] = if chip.graphics[idx] == 1 { 0xFF } else { 0x00};
                buffer[idx * 4 + 3] = if chip.graphics[idx] == 1 { 0xFF } else { 0x00};
            }
        }).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
