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

const WIDTH: i16 = 1280;
const HEIGHT: i16 = 620;
const ARRAY_SIZE: usize = (WIDTH / 1) as usize;
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

trait Visualizer {
    fn new(gl: GlGraphics) -> Self;

    fn render(&mut self, arg: &RenderArgs);

    fn step_sort(&mut self);

    fn process_input(&mut self, arg: &ButtonArgs) {
        if arg.state == ButtonState::Press {
            match arg.button {
                Button::Keyboard(Key::R) => self.shuffle(),
                _ => (),
            }
        }
    }

    fn shuffle(&mut self);

    fn test_sorted(&mut self);
}

#[allow(dead_code)]
struct BubbleVisualizer {
    gl: GlGraphics,
    speed: u32,
    array: Vec<i16>,
    current_index: usize,
    compared_index: usize,
    sorted_index: usize,
    sorted: bool,
}

#[allow(dead_code)]
struct CocktailVisualizer {
    gl: GlGraphics,
    speed: u32,
    array: Vec<i16>,
    current_index: usize,
    compared_index: usize,
    sorted_max: usize,
    sorted_min: usize,
    direction: i32,
    sorted: bool,
}

#[allow(dead_code)]
struct QuickVisualizer {
    array: Vec<i16>,
    speed: u32,
    stack: Vec<(usize, usize)>,
    gl: GlGraphics,
    sorted: bool,
}

#[allow(dead_code)]
struct BogoVisualizer {
    array: Vec<i16>,
    speed: u32,
    gl: GlGraphics,
    sorted: bool,
}

impl Visualizer for QuickVisualizer {
    fn new(g_graphics: GlGraphics) -> Self {
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut arr: Vec<i16> = (0..ARRAY_SIZE as i16).collect();
        arr.shuffle(&mut thread_rng());
        let low = 0;
        let high = arr.len() - 1;
        stack.push((low, high));
        QuickVisualizer {
            array: arr,
            stack,
            gl: g_graphics,
            sorted: false,
            speed: 1,
        }
    }

    fn render(&mut self, arg: &RenderArgs) {
        self.test_sorted();
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
            for (i, n) in self.array.iter().enumerate() {
                let hue = map(self.array[i], 0, ARRAY_SIZE as i16, 0.0, 360.0);
                let mut rect_color = to_rgba(hue);
                if let Some(j) = self.stack.last() {
                    if j.0 == i || j.1 == i {
                        rect_color = [1.0, 1.0, 1.0, 1.0];
                    }
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

    fn step_sort(&mut self) {
        if self.sorted {
            return;
        }
        if let Some((low, high)) = self.stack.pop() {
            if let Some(pi) = self.partition_step(low, high) {
                if pi > 0 {
                    self.stack.push((low, pi - 1));
                }
                self.stack.push((pi + 1, high));
            }
        }
    }
    fn shuffle(&mut self) {
        self.sorted = false;
        self.stack = Vec::new();
        let low = 0;
        let high = self.array.len() - 1;
        self.stack.push((low, high));
        self.array.shuffle(&mut thread_rng());
    }
    fn test_sorted(&mut self) {
        let mut sorted = self.array.clone();
        sorted.sort();

        self.sorted = self.array == sorted;
    }
}

impl QuickVisualizer {
    fn partition_step(&mut self, low: usize, high: usize) -> Option<usize> {
        if low >= high {
            return None;
        }

        let pivot = self.array[high];
        let mut i = low;
        for j in low..high {
            if self.array[j] < pivot {
                self.array.swap(i, j);
                i += 1;
            }
        }
        self.array.swap(i, high);
        Some(i)
    }
}

impl Visualizer for BubbleVisualizer {
    fn new(gl: GlGraphics) -> BubbleVisualizer {
        let mut arr: Vec<i16> = (0..ARRAY_SIZE as i16).collect();
        arr.shuffle(&mut thread_rng());
        BubbleVisualizer {
            gl,
            array: arr,
            current_index: 0,
            compared_index: 0,
            sorted_index: ARRAY_SIZE,
            sorted: false,
            speed: 1,
        }
    }

    fn render(&mut self, arg: &RenderArgs) {
        self.test_sorted();
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
            for (i, n) in self.array.iter().enumerate() {
                let hue = map(self.array[i], 0, ARRAY_SIZE as i16, 0.0, 360.0);
                let mut rect_color = to_rgba(hue);
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

    fn step_sort(&mut self) {
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

    fn shuffle(&mut self) {
        self.sorted_index = ARRAY_SIZE;
        self.array.shuffle(&mut thread_rng());
    }

    fn test_sorted(&mut self) {
        let mut sorted = self.array.clone();
        sorted.sort();

        self.sorted = self.array == sorted;
    }

    fn process_input(&mut self, arg: &ButtonArgs) {
        if arg.state == ButtonState::Press {
            match arg.button {
                Button::Keyboard(Key::R) => self.shuffle(),
                _ => (),
            }
        }
    }
}

impl Visualizer for CocktailVisualizer {
    fn new(gl: GlGraphics) -> CocktailVisualizer {
        let mut arr: Vec<i16> = (0..ARRAY_SIZE as i16).collect();
        arr.shuffle(&mut thread_rng());
        CocktailVisualizer {
            gl,
            array: arr,
            current_index: 0,
            compared_index: 0,
            sorted_max: ARRAY_SIZE,
            sorted_min: 0,
            direction: 1,
            sorted: false,
            speed: 1,
        }
    }

    fn render(&mut self, arg: &RenderArgs) {
        self.test_sorted();
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
            for (i, n) in self.array.iter().enumerate() {
                let hue = map(self.array[i], 0, ARRAY_SIZE as i16, 0.0, 360.0);
                let mut rect_color = to_rgba(hue);
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

    fn step_sort(&mut self) {
        if self.sorted {
            return;
        }

        self.current_index = (self.current_index as i32 + self.direction) as usize;
        if self.current_index == self.array.len() - 1 || self.current_index == 0 {
            self.direction *= -1;
        }

        if self.sorted_max <= self.current_index {
            self.sorted_min -= 1;
        } else if self.sorted_min >= self.current_index {
            self.sorted_max += 1;
        }

        self.compared_index = (self.current_index as i32 + self.direction) as usize;

        if self.direction > 0 {
            if self.array[self.current_index] > self.array[self.current_index + 1] {
                self.array.swap(self.current_index, self.current_index + 1);
            }
        } else {
            if self.array[self.current_index] < self.array[self.current_index - 1] {
                self.array.swap(self.current_index, self.current_index - 1);
            }
        }
    }

    fn shuffle(&mut self) {
        self.sorted_max = ARRAY_SIZE;
        self.sorted_min = 0;
        self.array.shuffle(&mut thread_rng());
    }

    fn test_sorted(&mut self) {
        let mut sorted = self.array.clone();
        sorted.sort();

        self.sorted = self.array == sorted;
    }
}

impl Visualizer for BogoVisualizer {
    fn new(gl: GlGraphics) -> Self {
        let mut arr: Vec<i16> = (0..ARRAY_SIZE as i16).collect();
        arr.shuffle(&mut thread_rng());
        BogoVisualizer {
            array: arr,
            speed: 1,
            gl: gl,
            sorted: false,
        }
    }

    fn shuffle(&mut self) {
        self.array.shuffle(&mut thread_rng());
    }

    fn step_sort(&mut self) {
        self.test_sorted();
        if !self.sorted {
            self.shuffle();
        }
    }

    fn test_sorted(&mut self) {
        let mut sorted = self.array.clone();
        sorted.sort();

        self.sorted = self.array == sorted;
    }

    fn render(&mut self, arg: &RenderArgs) {
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
            for (i, n) in self.array.iter().enumerate() {
                let hue = map(self.array[i], 0, ARRAY_SIZE as i16, 0.0, 360.0);
                graphics::rectangle(
                    to_rgba(hue),
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
}
fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Sort?", [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // let mut visualizer = BubbleVisualizer::new(GlGraphics::new(opengl));
    // let mut visualizer = CocktailVisualizer::new(GlGraphics::new(opengl));
    // let mut visualizer = QuickVisualizer::new(GlGraphics::new(opengl));
    let mut visualizer = BogoVisualizer::new(GlGraphics::new(opengl));

    let e_settings = EventSettings::new();

    let mut events = Events::new(e_settings);
    let mut event_cycles = 0;
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            visualizer.render(&r);
        }
        if let Some(b) = e.button_args() {
            visualizer.process_input(&b);
        }
        if event_cycles % visualizer.speed == 0 {
            visualizer.step_sort();
        }
        event_cycles += 1;
    }
}
