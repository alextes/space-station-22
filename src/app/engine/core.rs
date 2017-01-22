use piston::input::*;
use opengl_graphics::{GlGraphics, Texture};
use graphics::rectangle::square;
use std::path::Path;

pub fn render(gl: &mut GlGraphics, args: &RenderArgs) {
    use graphics::*;

    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

    //Create the image object and attach a square Rectangle object inside.
    let image   = Image::new().rect(square(0.0, 0.0, 160.0));
    //A texture to use with the image
    let texture = Texture::from_path(Path::new("/Users/alexander/code/space-station-22/res/AutoReiv.png")).unwrap();

    gl.draw(args.viewport(), |c, gl| {
        // Clear the screen.
        clear(BLACK, gl);

        //Draw the image with the texture
        image.draw(&texture, &DrawState::default(), c.transform, gl);
    });
}

pub fn update(gl: &mut GlGraphics, args: &UpdateArgs) {}
