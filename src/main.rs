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

const HEIGHT: i16 = 600;
const WIDTH: i16 = 600;
const ARRAY_SIZE: usize = 75;
const BAR_WIDTH: i16 = WIDTH / ARRAY_SIZE as i16;

fn to_rgba(h: f32) -> [f32; 4] {
    let (h, s, v) = (h, 1.0, 1.0);
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h >= 0.0 && h < 60.0 {
        (c, x, 0.0)
    } else if h >= 60.0 && h < 120.0 {
        (x, c, 0.0)
    } else if h >= 120.0 && h < 180.0 {
        (0.0, c, x)
    } else if h >= 180.0 && h < 240.0 {
        (0.0, x, c)
    } else if h >= 240.0 && h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    [(r + m), (g + m), (b + m), 1.0]
}

fn map(n: i16, start_1: i16, end_1: i16, start_2: f32, end_2: f32) -> f32 {
    ((n - start_1) as f32 / (end_1 - start_1) as f32) * (end_2 - start_2) + start_2
}

struct Game {
    gl: GlGraphics,
    array: Vec<i16>,
    current_index: usize,
    compared_index: usize,
    sorted_index: usize,
    sorted: bool,
}

impl Game {
    fn new(gl: GlGraphics) -> Game {
        let mut arr: Vec<i16> = (0..ARRAY_SIZE as i16).collect();
        arr.shuffle(&mut thread_rng());
        Game {
            gl,
            array: arr,
            current_index: 0,
            compared_index: 0,
            sorted_index: ARRAY_SIZE,
            sorted: false,
        }
    }

    fn render(&mut self, arg: &RenderArgs) {
        self.test_sorted();
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
            for (i, n) in self.array.iter().enumerate() {
                let mut rect_color = to_rgba(map(self.array[i], 0, ARRAY_SIZE as i16, 0.0, 325.0));
                if !self.sorted && (i == self.current_index || i == self.compared_index) {
                    rect_color = [0.9, 0.9, 0.9, 1.0];
                }

                graphics::rectangle(
                    rect_color,
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

    fn process_input(&mut self, arg: &ButtonArgs) {
        if arg.state == ButtonState::Press {
            match arg.button {
                Button::Keyboard(Key::R) => self.shuffle(),
                _ => (),
            }
        }
    }

    fn shuffle(&mut self) {
        self.sorted_index = ARRAY_SIZE;
        self.array.shuffle(&mut thread_rng());
    }

    fn test_sorted(&mut self) {
        let mut sorted = self.array.clone();
        sorted.sort();

        self.sorted = self.array == sorted;
    }

    fn bubble(&mut self) {
        if self.sorted {
            return;
        }
        self.current_index = self.current_index + 1;
        if self.sorted_index <= self.current_index {
            self.current_index = 0;
            self.sorted_index -= 1;
        }
        if self.current_index == self.array.len() - 1 {
            return;
        }
        self.compared_index = self.current_index + 1;
        if self.array[self.current_index] > self.array[self.current_index + 1] {
            self.array.swap(self.current_index, self.current_index + 1);
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Sort?", [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true)
        .fullscreen(false)
        .resizable(false)
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
            game.process_input(&b);
        }
        game.bubble();
    }
}
