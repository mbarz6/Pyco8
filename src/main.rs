#[macro_use] extern crate cpython;
extern crate piston_window;

use cpython::{Python, PyDict, PyTuple};

use piston_window::*;

use std::fs::File;
use std::io::prelude::*;

mod pyapi;
use pyapi::*;

enum State {
  Game,
  ArtEditor { sprite: usize },
}

const SCALE: u32 = 4;
const BASE: u32 = 128;

fn main() {
  let mut f = File::open("assets/test.py").expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("wtf");

  let gil = Python::acquire_gil();
  let py = gil.python();

  let opengl = OpenGL::V3_2;

  let mut state = State::ArtEditor{ sprite: 0 };
  let mut window: PistonWindow =
    WindowSettings::new("Pyco8", [BASE * SCALE, BASE * SCALE])
    .exit_on_esc(true).resizable(false).build().unwrap();

  while let Some(event) = window.next() {
    
    match state {
      State::Game => {  
        // if it's a render event...
        if let Some(_args) = event.render_args() { 
          // "why make a new dict every loop, mr. michael?"
          // BECAUSE PYDICT CAN'T COPY, SO WE NEEDA MOVE IT
          // (and where's it gonna go once it gets a-moved?)
          let dict = PyDict::new(py);
          dict.set_item(py, "draw_rect", py_fn!(py, draw_rect_py(r: f32, g: f32, b: f32, x: f64, y: f64, length: f64, width: f64))).unwrap();
          dict.set_item(py, "clear_screen", py_fn!(py, clear_screen_py(r: f32, g: f32, b: f32))).unwrap();
          dict.set_item(py, "draw_sprite", py_fn!(py, draw_sprite_py(x: i32, y: i32, sprite: usize))).unwrap(); 
          render(py, dict, &contents);

          let req_holder_wrapper = get_request_holder();
          let mut requests = req_holder_wrapper.requests.lock().unwrap();
          loop {
            let current_request = requests.pop_front();
            match current_request {
              Some(request) => match request {
                DrawRequest::ClearScreen { r, g, b } => {
                  window.draw_2d(&event, |_context, graphics| {
                    clear([r, g, b, 1.0], graphics);
                  }); 
                },
                DrawRequest::DrawRect { r, g, b, x, y, length, width } => { 
                  window.draw_2d(&event, |context, graphics| {
                    rectangle([r, g, b, 1.0], 
                        [x * (SCALE as f64), y * (SCALE as f64), length * (SCALE as f64), width * (SCALE as f64)],
                        context.transform,
                        graphics);
                  }); 
                },
                DrawRequest::DrawSprite { x, y, sprite_data } => {
                  window.draw_2d(&event, |context, graphics| {
                    for i in x..(x+8) {
                      for j in y..(y+8) {
                        if sprite_data[((i-x) + 8*(j-y)) as usize] == 0 {
                          rectangle([0.0, 0.0, 0.0, 1.0], 
                            [(i * (SCALE as i32)) as f64, (j * (SCALE as i32)) as f64, SCALE as f64, SCALE as f64],
                            context.transform,
                            graphics);
                        } else {
                          rectangle([1.0, 1.0, 1.0, 1.0], 
                            [(i * (SCALE as i32)) as f64, (j * (SCALE as i32)) as f64, SCALE as f64, SCALE as f64],
                            context.transform,
                            graphics);
                        }
                      }     
                    }
                  });
                },  
              },
              None => break,
            }
          }
        }
      },
      State::ArtEditor { sprite } => {
        // render sprite
        let sprite_data = get_sprite_data(sprite);
        let x: i32 = 10;
        let y: i32 = 0;
        let editor_scale: i32 = 4;
        window.draw_2d(&event, |context, graphics| {
          for i in x..(x+8) {
            for j in y..(y+8) {
              if sprite_data[((i-x) + 8*(j-y))  as usize] == 0 {
                rectangle([0.0, 0.0, 0.0, 1.0], 
                  [(i * (SCALE as i32) * editor_scale) as f64, (j * (SCALE as i32) * editor_scale) as f64, 
                    ((SCALE as i32) * editor_scale) as f64, ((SCALE as i32) * editor_scale) as f64],
                  context.transform,
                  graphics);
              } else {
                rectangle([1.0, 1.0, 1.0, 1.0], 
                  [(i * (SCALE as i32) * editor_scale) as f64, (j * (SCALE as i32) * editor_scale) as f64, 
                    ((SCALE as i32) * editor_scale) as f64, ((SCALE as i32) * editor_scale) as f64],
                  context.transform,
                  graphics);
              }
            }     
          }
        });

      }
    }
  }
}

fn update(py: Python, lib: PyDict, contents: &str) {
  py.run(&format!("{}\n_update()", contents), Some(&lib), None).unwrap();
}

fn render(py: Python, lib: PyDict, contents: &str) {
  py.run(&format!("{}\n_draw()", contents), Some(&lib), None).unwrap();
}