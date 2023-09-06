/*
  Copyright (C) 2023  Tóth Bálint

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use std::ffi::CString;
use std::{fs, path::Path, ptr};
use gl::types::{GLuint, GLint};

#[derive(Default)]
#[derive(Debug)]
pub struct Shader {
    id: GLuint,
    bound: bool,
}

impl Shader {

    pub fn new(vertex: &str, fragment: &str) -> Result<Self,String> {
        let program: GLuint = Self::create_shader(vertex, fragment)?;
        let mut instance = Self {
            id: program, 
            bound: false
        };
        instance.bind();
        Ok(instance)
    }

    fn load_src(src: &str) -> Result<String, String> {
        fs::read_to_string(Path::new(src))
            .map_err(|e| e.to_string())
    }

    fn compile_shader(filename: &str, shader_type: GLuint) -> Result<GLuint, String> {
        let src = Self::load_src(filename);
        let shader: GLuint;
        unsafe {
            shader = gl::CreateShader(shader_type);
            let src_c = CString::new(src?.as_bytes()).map_err(|e| e.to_string());
            gl::ShaderSource(shader,1,&src_c?.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut status: GLint = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // if status is failed, we fail
            if status != gl::TRUE as GLint {
                return Err(format!("Shader compilation failed: {}", filename));
            }
        }
        Ok(shader)
    }

    fn create_shader(vertex: &str, fragment: &str) -> Result<GLuint, String> {
        let vertex_shader: GLuint = Self::compile_shader(vertex, gl::VERTEX_SHADER)?;
        let fragment_shader: GLuint = Self::compile_shader(fragment, gl::FRAGMENT_SHADER)?;

        let shader_program: GLuint;
        unsafe {
            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);

            gl::LinkProgram(shader_program);
            let mut status: GLint = gl::FALSE as GLint;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut status);
            if status != gl::TRUE as GLint {
                return Err(format!("Shader linking failed: {} {}", vertex, fragment));
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Ok(shader_program)
    }

    pub fn bind(&mut self) {
        if !self.bound {
            unsafe {
                gl::UseProgram(self.id);
            }
            self.bound = true;
        }
    }

    pub fn unbind(&mut self) {
        if self.bound {
            unsafe {
                gl::UseProgram(0);
            }
            self.bound = false;
        }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
