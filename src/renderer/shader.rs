extern crate gl;
extern crate find_folder;

use self::find_folder::Search;

use gl::types::*;

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

    pub fn from_folder(shader_path: &str) -> Shader {
        let mut shader = Shader::new();

        let shader_sources = fs::read_dir(shader_path).unwrap();
        for shader_source in shader_sources {
            if let Ok(shader_source) = shader_source {
                let name = shader_source.path();
                let name = name.to_str().expect("file not found!");
                let mut file = File::open(name).expect("file not found!");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("rip");
                if name.ends_with(".vert") {
                    shader.add_shader(&contents, gl::VERTEX_SHADER);
                } else if name.ends_with(".frag") {
                    shader.add_shader(&contents, gl::FRAGMENT_SHADER);
                }
            }
        }
        shader.link();

        shader
    }

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

            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(shader, len, 
                    ptr::null_mut(), 
                    buf.as_mut_ptr() as *mut GLchar
                );
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

    fn get_location(&self, name: &str) -> GLint {
        let name = CString::new(name).unwrap();
        let location;
        unsafe {
            location = gl::GetUniformLocation(self.program, name.as_ptr());
        }
        location
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