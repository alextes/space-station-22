extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{LoadTexture, INIT_JPG, INIT_PNG};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::path::Path;

fn main() {
    let tilesheet_path = Path::new("resources/AutoReiv.png");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let window = video_subsystem
        .window("Space Station 22", 160, 160)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(tilesheet_path).unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.copy(&texture, None, None).expect("Render failed");

    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {
                    canvas.present();
                }
            }
        }
    }
}
