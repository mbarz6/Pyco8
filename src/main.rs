extern crate cpython;
extern crate piston_window;

use cpython::{Python, PyResult};

use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

use piston_window::*;

fn draw_rect(py: Python, r: f64, g: f64, b: f64) {
  println!("r: {}, g: {}, b: {}", r, g, b);  
}

fn main() {
  let gil = Python::acquire_gil();
  
  let mut f = File::open("test.py").expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("wtf");

  let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }
}

fn update(py: Python, contents: &str) {
  py.run(&format!("{}\n_update()", contents), None, None);
}
