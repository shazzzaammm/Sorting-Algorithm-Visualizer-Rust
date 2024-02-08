extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

const WIDTH: i16 = 500;
const HEIGHT: i16 = 500;
const ARRAY_SIZE: usize = 100;
const BAR_WIDTH: i16 = WIDTH / ARRAY_SIZE as i16;

struct Game {
    gl: GlGraphics,
    array: [i8; ARRAY_SIZE],
}

impl Game {
    fn new(gl: GlGraphics) -> Game {
        let mut arr = [0; ARRAY_SIZE];
        for num in 1..ARRAY_SIZE {
            arr[num - 1] = num as i8;
        }
        Game { gl, array: arr }
    }

    fn render(&mut self, arg: &RenderArgs) {
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
            for n in self.array {
                graphics::rectangle(
                    [1.0, 1.0, 1.0, 1.0],
                    [
                        (n as i16 * BAR_WIDTH) as f64,
                        0.0,
                        BAR_WIDTH as f64,
                        n as f64 * (HEIGHT as f64 / ARRAY_SIZE as f64),
                    ],
                    _c.transform,
                    gl,
                );
            }
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Sort?", [WIDTH as u32, HEIGHT as u32])
        .opengl(opengl)
        .exit_on_esc(true)
        .fullscreen(false)
        .build()
        .unwrap();

    let mut game = Game::new(GlGraphics::new(opengl));

    let e_settings = EventSettings::new();

    let mut events = Events::new(e_settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
    }
}
