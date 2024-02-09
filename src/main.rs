extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use rand::seq::SliceRandom;
use rand::thread_rng;
const WIDTH: i16 = 500;
const HEIGHT: i16 = 500;
const ARRAY_SIZE: usize = 100;
const BAR_WIDTH: i16 = WIDTH / ARRAY_SIZE as i16;

struct Game {
    gl: GlGraphics,
    array: Vec<i8>,
}

impl Game {
    fn new(gl: GlGraphics) -> Game {
        let arr: Vec<i8> = (0..ARRAY_SIZE as i8).collect();
        Game { gl, array: arr }
    }

    fn render(&mut self, arg: &RenderArgs) {
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
            for (i, n) in self.array.iter().enumerate() {
                graphics::rectangle(
                    [1.0, 1.0, 1.0, 1.0],
                    [
                        (i as i16 * BAR_WIDTH) as f64,
                        0.0,
                        BAR_WIDTH as f64,
                        *n as f64 * (HEIGHT as f64 / ARRAY_SIZE as f64),
                    ],
                    _c.transform,
                    gl,
                );
            }
        });
    }

    fn shuffle(&mut self) {
        self.array.shuffle(&mut thread_rng());
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
        if let Some(b) = e.button_args() {
            game.shuffle();
        }
    }
}
