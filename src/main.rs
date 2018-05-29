extern crate gl;
extern crate glutin;

use glutin::GlContext;

mod renderer;
use renderer::Renderer;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Pyco8")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let gl = gl::load_with(|ptr| gl_window.get_proc_address(ptr) as *const _);
    let mut running = true;

    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                    _ => ()
                },
                _ => ()
            }
        });
    }   
}