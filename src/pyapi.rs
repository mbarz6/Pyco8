use cpython::{Python, PyResult};

use std::fs::File;
use std::io::prelude::*;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::{mem};

pub enum DrawRequest {
  DrawRect { r: f32, g: f32, b: f32, x: f64, y: f64, length: f64, width: f64 },
  ClearScreen { r: f32, g: f32, b: f32 },
  DrawSprite { x: i32, y: i32, sprite_data: [u8; 64] },
}

#[derive(Clone)]
pub struct RequestHolder {
  pub requests: Arc<Mutex<VecDeque<DrawRequest>>>,
  pub sprite_data: Arc<Mutex<[[u8; 64]; 1]>>,
}

// copied from StackOverflow
// background: passing partial calls to py_fun! doesn't work so well
// so, we need a way for draw_rect (and similar functions) to communicate with main
// this is sloppy, but workable
pub fn get_request_holder() -> RequestHolder {
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

pub fn make_request(request: DrawRequest) {
  let req_holder_wrapper = get_request_holder();
  let mut requests = req_holder_wrapper.requests.lock().unwrap();
  requests.push_back(request);
}

pub fn get_sprite_data(sprite: usize) -> [u8; 64] {
  let req_holder_wrapper = get_request_holder();
  let sprite_data = req_holder_wrapper.sprite_data.lock().unwrap();
  sprite_data[sprite]
}

pub fn set_sprite_data(sprite: usize, new_data: [u8; 64]) {
  let req_holder_wrapper = get_request_holder();
  let mut sprite_data = req_holder_wrapper.sprite_data.lock().unwrap();
  sprite_data[sprite] = new_data;
}

pub fn draw_rect_py(_py: Python, r: f32, g: f32, b: f32, x: f64, y: f64, length: f64, width: f64) -> PyResult<bool> {
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

pub fn draw_sprite_py(_py: Python, x: i32, y: i32, sprite: usize) -> PyResult<bool> {
  let req_holder_wrapper = get_request_holder();
  let sprite_data = req_holder_wrapper.sprite_data.lock().unwrap();
  let sprite_data = sprite_data[sprite];

  let sprite_request = DrawRequest::DrawSprite {
    x, y, sprite_data
  };
  make_request(sprite_request);

  Ok(true)
}

pub fn clear_screen_py(_py: Python, r: f32, g: f32, b: f32) -> PyResult<bool> {
  let clear_screen_request = DrawRequest::ClearScreen {
    r, g, b,
  };
  make_request(clear_screen_request);

  Ok(true)
}