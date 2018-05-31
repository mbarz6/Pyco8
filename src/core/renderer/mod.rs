extern crate gl;

mod shader;
use self::shader::Shader;

use core::gl::types::*;

use std::mem;
use std::fs::File;
use std::io::prelude::*;

// fat pixel = 'pixel' that someone using .s8 to draw sees
// so, when one .s8 file asks to draw one pixel, they actually draw a small square called a fat pixel

pub struct Renderer {
    scale: f32, // size of fat pixels
    size: f32,
    shader_program: Shader, 
    vao: GLuint,
    vbo_vertices: GLuint,
    sprites: Vec<[u8; 64]>,
    ndc_coord: GLfloat,
    width: u32, 
    height: u32,
}

impl Renderer {
    // fat pixels to NDC
    fn fat_to_normalized(&self, x: f32, y: f32) -> (f32, f32) {
        (-1.0 + x * self.ndc_coord, 1.0 - y * self.ndc_coord)
    }
}

impl Renderer {
    pub fn new(scale: f32, size: f32, cart_data: &str) -> Renderer {
        let shader_program = Shader::from_folder("assets/shaders/pixel.vert", "assets/shaders/color_picker.frag");
        // vertex array for fat pixel square
        // I use uniforms to say which position, but these are basic dimensions
        let ndc_coord: GLfloat = 2.0 * scale / size;
        let vertices: [GLfloat; 8] = [
            0.0, 0.0,
            ndc_coord, 0.0,
            ndc_coord, -ndc_coord,
            0.0, -ndc_coord,
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
                    (2 * mem::size_of::<GLfloat>()) as GLint, // distance between points in RAM
                    0 as *const GLvoid
            );
            gl::EnableVertexAttribArray(0);
        }

        // now, create sprite sheet
        let mut sprites: Vec<[u8; 64]> = Vec::with_capacity(128);
        let mut sprite_file = File::open(format!("{}/sprites.s8d", cart_data)).expect("rip2");
        let mut sprite_data = String::new();
        sprite_file.read_to_string(&mut sprite_data).expect("rip3: rip again!");
        
        for sprite in sprite_data.lines() {
            let mut data: [u8; 64] = [0; 64];
            data.copy_from_slice(&sprite.as_bytes());
            sprites.push(data);
        }

        // finally we're done
        Renderer{ scale, size, shader_program, vao, vbo_vertices, sprites, ndc_coord, width: size as u32, height: size as u32 }
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        let min = if w < h { w } else { h }; 
        self.size = min as f32;
        // always keep screen 128x128 fat pixels
        self.scale = self.size / 128.0;
        self.width = w;
        self.height = h;

        unsafe {
            // change viewport to reflext this
            // the first two args are for margins in case we're not perfect square
            // the last two are just width/height
            gl::Viewport(((w - self.size as u32) / 2) as i32, ((h - self.size as u32) / 2) as i32, self.size as i32, self.size as i32);
        }
    }
}

// s8 visible drawing api
impl Renderer {
    // draws a fat pixel to screen
    pub fn draw_pixel(&self, x: u32, y: u32, color: u32) {
        let (x, y) = self.fat_to_normalized(x as f32, y as f32);

        unsafe {
            gl::BindVertexArray(self.vao);
            self.shader_program.use_program();

            // set uniforms so shader correctly draws
            self.shader_program.add1ui("color", color);
            self.shader_program.add1f("offsetx", x);
            self.shader_program.add1f("offsety", y);

            gl::DrawArrays(gl::QUADS, 0, 4);
        }
    }

    // TODO: optimize this to be raw opengl
    pub fn draw_sprite(&self, x: u32, y: u32, sprite: usize) {
        let sprite_data = self.sprites.get(sprite).expect("invalid sprite my dude");
        
        for i in 0..8 {
            for j in 0..8 {
                // we do - '0' from sprite data so that char 0 represents color 0
                self.draw_pixel(i + x, j + y, (sprite_data[(i + 8 * j) as usize] - ('0' as u8)) as u32);
            }
        }
    }
}