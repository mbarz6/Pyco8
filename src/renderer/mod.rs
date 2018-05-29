extern crate gl;

mod shader;
use self::shader::Shader;

use gl::types::*;

use std::mem;

pub struct Renderer {
    pallet: Vec<(f32, f32, f32)>,
    scale: f32,
    size: f32,
    shader_program: Shader,
    vao: GLuint,
    vbo_vertices: GLuint,
}

impl Renderer {
    fn to_native_coords(&self, x: u32, y: u32) -> (f32, f32) {
        (x as f32 * self.scale, y as f32 * self.scale)
    }
}

impl Renderer {
    pub fn new(scale: f32, size: f32) -> Renderer {
        let mut pallet: Vec<(f32, f32, f32)> = Vec::new();
        pallet.push((0.0, 0.0, 0.0));
        pallet.push((1.0, 1.0, 1.0));

        let shader_program = Shader::from_folder("assets/shaders/pixel_shader");

        // vertex array of pixel
        // I use uniforms to say which position, but these are basic dimensions
        let ndc_coord: GLfloat = scale / size;
        let vertices: [GLfloat; 8] = [
            0.0, 0.0,
            ndc_coord, 0.0,
            ndc_coord, ndc_coord,
            0.0, ndc_coord,
        ];
        let mut vbo_vertices: u32 = 0;
        let mut vao: u32 = 0;

        unsafe {
            gl::GenBuffers(1, &mut vbo_vertices as *mut u32);
            gl::GenVertexArrays(1, &mut vao as *mut u32);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_vertices);
            gl::BufferData(gl::ARRAY_BUFFER, 
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&vertices[0]), 
                gl::STATIC_DRAW
            );

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 
                    (2 * mem::size_of::<GLfloat>()) as GLint, 
                    0 as *const GLvoid
            );
            gl::EnableVertexAttribArray(0);
        }

        Renderer{ pallet, scale, size, shader_program, vao, vbo_vertices }
    }

    pub fn draw_pixel(&self, x: u32, y: u32, color: usize) {
        let color = self.pallet.get(color).expect("ERROR: TRIED TO DRAW BAD COLOR");
        let (x, y) = self.to_native_coords(x, y);
        let (x, y) = (x as f32 / self.size, y as f32 / self.size);

        unsafe {
            gl::BindVertexArray(self.vao);
            self.shader_program.use_program();
            gl::DrawArrays(gl::QUADS, 0, 4);
        }
    }
}