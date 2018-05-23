#[macro_use] extern crate cpython;
extern crate piston_window;

use cpython::{Python, PyResult, PyDict};

use piston_window::*;

use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::{mem, thread};

enum DrawRequest {
  DrawRect { r: f32, g: f32, b: f32, x: f64, y: f64, length: f64, width: f64 },
}

#[derive(Clone)]
struct RequestHolder {
  requests: Arc<Mutex<VecDeque<DrawRequest>>>,
}

// copied from StackOverflow
// background: passing partial calls to py_fun! doesn't work so well
// so, we need a way for draw_rect (and similar functions) to communicate with main
// this is sloppy, but workable
fn get_request_holder() -> RequestHolder {
    // Initialize it to a null value
    static mut SINGLETON: *const RequestHolder = 0 as *const RequestHolder;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = RequestHolder {
                requests: Arc::new(Mutex::new(VecDeque::new())),
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

fn draw_rect(py: Python, r: f32, g: f32, b: f32, x: f64, y: f64, length: f64, width: f64) -> PyResult<bool> {
  let rect_request = DrawRequest::DrawRect {
    r,
    g,
    b,
    x,
    y,
    length,
    width,
  };
  
  let req_holder_wrapper = get_request_holder();
  let mut requests = req_holder_wrapper.requests.lock().unwrap();
  requests.push_back(rect_request);

  Ok(true)
}

fn main() {
  let mut f = File::open("src/test.py").expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("wtf");

  let gil = Python::acquire_gil();
  let py = gil.python();

  let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        // add drawRect to locals, allowing our python to call it
        let dict = PyDict::new(py);
        dict.set_item(py, "draw_rect", py_fn!(py, draw_rect(r: f32, g: f32, b: f32, x: f64, y: f64, length: f64, width: f64))).unwrap();
        update(py, dict, &contents);

        let req_holder_wrapper = get_request_holder();
        let mut requests = req_holder_wrapper.requests.lock().unwrap();
        loop {
          let current_request = requests.pop_front();
          match current_request {
            Some(request) => match request {
              DrawRequest::DrawRect { r, g, b, x, y, length, width } => { 
                window.draw_2d(&event, |context, graphics| {
                  clear([1.0; 4], graphics);
                  rectangle([r, g, b, 1.0], 
                      [x, y, length, width],
                      context.transform,
                      graphics);}); 
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
