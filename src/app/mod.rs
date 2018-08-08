use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

mod engine;

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    pub fn start() {
        let opengl = OpenGL::V3_2;

        // Create an Glutin window.
        let mut window: Window = WindowSettings::new("space station 22", [160, 160])
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        // Create a new game and run it.
        let mut app = App {
            gl: GlGraphics::new(opengl),
        };

        let mut events = window.events();
        while let Some(e) = events.next(&mut window) {
            if let Some(r) = e.render_args() {
                engine::core::render(&mut app.gl, &r);
            }

            if let Some(u) = e.update_args() {
                engine::core::update(&mut app.gl, &u);
            }
        }
    }
}
