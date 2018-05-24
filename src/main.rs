#[macro_use] extern crate cpython;
extern crate piston_window;

use cpython::{Python, PyResult, PyDict};

use piston_window::*;

use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::{mem};

enum DrawRequest {
  DrawRect { r: f32, g: f32, b: f32, x: f64, y: f64, length: f64, width: f64 },
  ClearScreen { r: f32, g: f32, b: f32 },
  DrawSprite { x: i32, y: i32, sprite_data: [u8; 64] },
}

#[derive(Clone)]
struct RequestHolder {
  requests: Arc<Mutex<VecDeque<DrawRequest>>>,
  sprite_data: Arc<Mutex<[[u8; 64]; 1]>>,
}

// copied from StackOverflow
// background: passing partial calls to py_fun! doesn't work so well
// so, we need a way for draw_rect (and similar functions) to communicate with main
// this is sloppy, but workable
fn get_request_holder() -> RequestHolder {
    // Initialize it to a null value
    static mut REQ_HOLDER: *const RequestHolder = 0 as *const RequestHolder;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // read sprite data
            let mut f = File::open("assets/images.py8").expect("file not found");
            let mut contents = String::new();
            f.read_to_string(&mut contents).expect("wtf");
            let mut sprite_data: [[u8; 64]; 1] = [[0; 64]; 1];
            for (i, line) in contents.lines().enumerate() {
              for (j, c) in line.chars().enumerate() {
                (sprite_data[i])[j] = c.to_digit(10).unwrap() as u8;
              }
            }

            // now, make the thing
            let req_holder_wrapper = RequestHolder {
                requests: Arc::new(Mutex::new(VecDeque::new())),                
                sprite_data: Arc::new(Mutex::new(sprite_data)),
            };

            // Put it in the heap so it can outlive this call
            REQ_HOLDER = mem::transmute(Box::new(req_holder_wrapper));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*REQ_HOLDER).clone()
    }
}

fn make_request(request: DrawRequest) {
  let req_holder_wrapper = get_request_holder();
  let mut requests = req_holder_wrapper.requests.lock().unwrap();
  requests.push_back(request);
}

fn draw_rect(_py: Python, r: f32, g: f32, b: f32, x: f64, y: f64, length: f64, width: f64) -> PyResult<bool> {
  let rect_request = DrawRequest::DrawRect {
    r,
    g,
    b,
    x,
    y,
    length,
    width,
  };
  
  make_request(rect_request);

  Ok(true)
}

fn draw_sprite(_py: Python, x: i32, y: i32, sprite: usize) -> PyResult<bool> {
  let req_holder_wrapper = get_request_holder();
  let sprite_data = req_holder_wrapper.sprite_data.lock().unwrap();
  let sprite_data = sprite_data[sprite];

  let sprite_request = DrawRequest::DrawSprite {
    x, y, sprite_data
  };
  make_request(sprite_request);

  Ok(true)
}

fn clear_screen(_py: Python, r: f32, g: f32, b: f32) -> PyResult<bool> {
  let clear_screen_request = DrawRequest::ClearScreen {
    r, g, b,
  };
  make_request(clear_screen_request);

  Ok(true)
}

const SCALE: u32 = 9;
const BASE: u32 = 128;

fn main() {
  let mut f = File::open("src/test.py").expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("wtf");

  let gil = Python::acquire_gil();
  let py = gil.python();

  let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [BASE * SCALE, BASE * SCALE])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        // add drawRect to locals, allowing our python to call it
        let dict = PyDict::new(py);
        dict.set_item(py, "draw_rect", py_fn!(py, draw_rect(r: f32, g: f32, b: f32, x: f64, y: f64, length: f64, width: f64))).unwrap();
        dict.set_item(py, "clear_screen", py_fn!(py, clear_screen(r: f32, g: f32, b: f32))).unwrap();
        dict.set_item(py, "draw_sprite", py_fn!(py, draw_sprite(x: i32, y: i32, sprite: usize))).unwrap();
        update(py, dict, &contents);

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
                      [x, y, length, width],
                      context.transform,
                      graphics);
                }); 
              },
              DrawRequest::DrawSprite { x, y, sprite_data } => {
                window.draw_2d(&event, |context, graphics| {
                  for i in x..(x+8) {
                    for j in y..(y+8) {
                      if sprite_data[(i + 8*j) as usize] == 0 {
                        rectangle([0.0, 0.0, 0.0, 1.0], 
                          [(i*(SCALE as i32)) as f64, (j*(SCALE as i32)) as f64, SCALE as f64, SCALE as f64],
                          context.transform,
                          graphics);
                      } else {
                        rectangle([1.0, 1.0, 1.0, 1.0], 
                          [(i*(SCALE as i32)) as f64, (j*(SCALE as i32)) as f64, SCALE as f64, SCALE as f64],
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
}

fn update(py: Python, lib: PyDict, contents: &str) {
  py.run(&format!("{}\n_update()", contents), Some(&lib), None).unwrap();
}
