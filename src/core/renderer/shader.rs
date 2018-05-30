extern crate gl;
extern crate find_folder;

use self::find_folder::Search;

use core::gl::types::*;

use std::ffi::CString;
use std::str;
use std::ptr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub struct Shader {
    program: GLuint,
    shaders: Vec<GLuint>,
    pub linked: bool,
}

impl Shader {
    pub fn new() -> Shader {
        let program;
        unsafe {
            program = gl::CreateProgram();
        }
        let shaders: Vec<GLuint> = Vec::new();
        Shader{ program, shaders, linked: false }
    }

    // creates shader by loading all shaders in given folder
    pub fn from_folder(vertex_path: &str, fragment_path: &str) -> Shader {
        let mut shader = Shader::new();

        
        let mut file = File::open(vertex_path).expect("file not found!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("rip");
        shader.add_shader(&contents, gl::VERTEX_SHADER);
        let mut file = File::open(fragment_path).expect("file not found!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("rip");
        shader.add_shader(&contents, gl::FRAGMENT_SHADER);

        shader.link();

        shader
    }

    // adds shader to shader program
    pub fn add_shader(&mut self, source: &str, shader_type: GLenum) {
        if self.linked {
            panic!("Attempted to add a shader to an already linked shader!");
        }

        let shader;
        unsafe {
            shader = gl::CreateShader(shader_type);
            gl::ShaderSource(shader, 1, 
                &CString::new(source.as_bytes()).unwrap().as_ptr(), 
                ptr::null()
            );
            gl::CompileShader(shader);

            // see if it compiled correctly
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
            if status != (gl::TRUE as GLint) {
                // length of log message
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                // read log message into CString-like vector
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(shader, len, 
                    ptr::null_mut(), 
                    buf.as_mut_ptr() as *mut GLchar
                );
                // error time!
                panic!("{}", 
                    str::from_utf8(&buf).ok()
                        .expect("shader log ain't valid utf8")
                );
            }
            self.shaders.push(shader);
            gl::AttachShader(self.program, shader);
        }
    }

    pub fn link(&mut self) {
        if self.linked {
            return;
        }
        
        unsafe {
            gl::LinkProgram(self.program);

            // cleanup
            for shader in &mut self.shaders {
                gl::DetachShader(self.program, *shader);
                gl::DeleteShader(*shader);
            }
        }
        self.shaders = Vec::new();
        self.linked = true;
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
}

// this code is just uniform wrappers
impl Shader {
    // gets location of uniform, since this will come up in all the following functions
    fn get_location(&self, name: &str) -> GLint {
        let name = CString::new(name).unwrap();
        let location;
        unsafe {
            location = gl::GetUniformLocation(self.program, name.as_ptr());
        }
        location
    }

    pub fn add4f(&self, name: &str, f1: f32, f2: f32, f3: f32, f4: f32) {
        unsafe {
            gl::Uniform4f(self.get_location(name), f1, f2, f3, f4);
        }
    }

    pub fn add1f(&self, name: &str, f1: f32) {
        unsafe {
            gl::Uniform1f(self.get_location(name), f1);
        }
    }

    pub fn add1ui(&self, name: &str, ui1: u32) {
        unsafe {
            gl::Uniform1ui(self.get_location(name), ui1);
        }
    }
}


impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            for shader in &mut self.shaders {
                gl::DetachShader(self.program, *shader);
                gl::DeleteShader(*shader);
            }
            gl::UseProgram(0);
            gl::DeleteProgram(self.program);
        }
    }
}