#[macro_use] extern crate cpython;
use cpython::{Python, PyResult, PyErr, PyDict};
use cpython::{exc};

use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

fn draw_rect(py: Python, r: f64, g: f64, b: f64) -> PyResult<bool> {
  println!("r: {}, g: {}, b: {}", r, g, b);  
  
  Ok(true)
}

fn main() {
  let mut f = File::open("src/test.py").expect("file not found");
  let mut contents = String::new();
  f.read_to_string(&mut contents).expect("wtf");

  let gil = Python::acquire_gil();
  let py = gil.python();
  update(py, &contents);
}

fn update(py: Python, contents: &str) {
  // add drawRect to locals, allowing our python to call it
  let dict = PyDict::new(py);
  dict.set_item(py, "draw_rect", py_fn!(py, draw_rect(r: f64, g: f64, b: f64))).unwrap();

  py.run(&format!("{}\n_update()", contents), Some(&dict), None).unwrap();
}
