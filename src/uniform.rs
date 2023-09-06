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
use gl::types::{GLint, GLfloat};
use std::ffi::CString;
use crate::shader::Shader;

pub struct Uniform {
    id: GLint,
}

#[allow(unused)]
impl Uniform {

    pub fn new(name: &str, shader: &mut Shader) -> Self {
        shader.bind();
        let name_c = CString::new(name.as_bytes()).unwrap();
        let uniform: GLint;
        unsafe {
            uniform = gl::GetUniformLocation(shader.id(),name_c.as_ptr());
        }
        shader.unbind();
        Self { id: uniform }
    }

    pub fn setf(&self, value: GLfloat, shader: &mut Shader) {
        shader.bind();
        unsafe {
            gl::Uniform1f(self.id, value);
        }
    }

    pub fn seti(&self, value: GLint, shader: &mut Shader) {
        shader.bind();
        unsafe {
            gl::Uniform1i(self.id, value);
        }
    }

    pub fn set_m4f(&self, value: &glm::Mat4, shader: &mut Shader) {
        shader.bind();
        unsafe {
            gl::UniformMatrix4fv(self.id, 1, gl::FALSE, std::ptr::addr_of!(value.data[0]));
        }
    }

    pub fn set_3f(&self, x: f32, y: f32, z: f32, shader: &mut Shader) {
        shader.bind();
        unsafe {
            gl::Uniform3f(self.id, x,y,z);
        }
    }

    pub fn set_4f(&self, x0: f32, x1: f32, x2: f32, x3: f32, shader: &mut Shader) {
        shader.bind();
        unsafe {
            gl::Uniform4f(self.id, x0, x1, x2, x3);
        }
    }

    pub fn set_v4f(&self, value: &glm::Vec4, shader: &mut Shader) {
        shader.bind();
        unsafe {
            gl::Uniform4f(self.id, value.x, value.y, value.z, value.w);
        }
    }

    pub fn set_v3f(&self, value: &glm::Vec3, shader: &mut Shader) {
        shader.bind();
        unsafe {
            gl::Uniform3f(self.id, value.x, value.y, value.z);
        }
    }

    pub fn set_m2f(&self, value: &glm::Mat2, shader: &mut Shader) {
        shader.bind();
        unsafe {
            gl::UniformMatrix2fv(self.id, 1, gl::FALSE, std::ptr::addr_of!(value.data[0]));
        }
    }

    pub fn set_m3f(&self, value: &glm::Mat3, shader: &mut Shader) {
        shader.bind();
        unsafe {
            gl::UniformMatrix3fv(self.id, 1, gl::FALSE, std::ptr::addr_of!(value.data[0]));
        }
    }

    pub fn set_m3d(&self, value: &glm::DMat3, shader: &mut Shader) {
        shader.bind();
        unsafe {
            gl::UniformMatrix3dv(self.id, 1, gl::FALSE, std::ptr::addr_of!(value.data[0]));
        }
    }
}
