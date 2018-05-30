extern crate gl;
extern crate glutin;

use self::glutin::{GlContext, GlWindow, EventsLoop};

mod renderer;
use self::renderer::Renderer;

// create EventsLoop and GlWindow
fn create() -> (EventsLoop, GlWindow) {
    // create stuff
    let events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Pyco8")
        .with_dimensions(512, 512);
    let context = glutin::ContextBuilder::new();
    let window = glutin::GlWindow::new(window_builder, context, &events_loop).unwrap();

    // setup gl instance
    unsafe {
        window.make_current().unwrap();
        gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    (events_loop, window)
}

// start application
pub fn start() {
    let mut running = true;
    let (mut events_loop, window) = create();
    let mut renderer = Renderer::new(4.0, 512.0, "assets/test_cart");

    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(w, h) => {
                         window.resize(w, h);
                    },
                    _ => ()
                },
                _ => ()
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            renderer.draw_pixel(0, 0, 1);
            renderer.draw_pixel(127, 0, 1);
            renderer.draw_pixel(0, 127, 1);
            renderer.draw_pixel(127, 127, 1);

            renderer.draw_sprite(10, 10, 0);
        }

        window.swap_buffers().unwrap();
    }   
}