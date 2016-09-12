extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod app;

use app::App;

fn main() {
    App::start();
}
