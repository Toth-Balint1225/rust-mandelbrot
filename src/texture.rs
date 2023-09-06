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
use gl::types::{GLuint, GLint, GLsizei, GLenum};
use std::fs::File;
use std::mem;

use crate::uniform::Uniform;
use crate::shader::Shader;

pub struct Texture {
    id: GLuint,
    unit: GLint,

    bound: bool,
}

#[allow(unused)]
pub enum TextureType {
    RGB,
    RGBA,
}

#[allow(unused)]
pub enum InterpolationType {
    LINEAR,
    NEAREST, 
}

#[allow(unused)]
pub enum MapType {
    REPEAT,
    MIRRORED,
}

#[allow(unused)]
impl Texture {
    pub fn new(file: &str, unit_: GLint, ty: TextureType) -> Result<Self, String> {
        let (buffer, width, height) = Self::read_png(file)?;
        let mut texture: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
        }
        Self::generate(texture, &buffer, width, height, unit_, ty);
        let mut instance = Self { 
            id: texture, 
            unit: unit_,
            bound: false,
        };
        instance.settings(InterpolationType::NEAREST, InterpolationType::NEAREST, MapType::REPEAT, MapType::REPEAT);
        instance.bind();
        Ok(instance)
    }   

    pub fn link(&self, shader_program: &mut Shader, sampler: &str) {
        let tex0 = Uniform::new(sampler, shader_program);
        shader_program.bind();
        tex0.seti(self.unit, shader_program);  
        shader_program.unbind();
    }

    fn read_png(filename: &str) -> Result<(Vec<u8>, u32, u32), String> {
        let decoder = png::Decoder::new(File::open(filename).map_err(|e| e.to_string())?);
        let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let texture_info = reader.next_frame(&mut buf).unwrap();
        let texture_bytes = &mut buf[..texture_info.buffer_size()]; // these are the bytes of the png
        Ok((texture_bytes.to_owned(), texture_info.width, texture_info.height))
    }

    fn generate(texture: GLuint, buf: &[u8], width: u32, height: u32, unit: GLint, ty: TextureType) {
        unsafe {
            // select, then bind
            gl::ActiveTexture(((gl::TEXTURE0) as GLint + unit) as GLenum);
            gl::BindTexture(gl::TEXTURE_2D, texture);
        }
        let format = match ty {
            TextureType::RGB => gl::RGB,
            TextureType::RGBA => gl::RGBA,
        };
        // texture generation
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D, 
                0, 
                gl::RGBA as GLint, 
                width as GLsizei, 
                height as GLsizei, 
                0, 
                format,
                gl::UNSIGNED_BYTE, 
                mem::transmute(&buf[0]));

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    fn gl_to_enum_inter(ty: InterpolationType) -> GLint {
        match ty {
            InterpolationType::LINEAR => gl::LINEAR as i32,
            InterpolationType::NEAREST => gl::NEAREST as i32,
        }
    }

    fn gl_to_enum_map(ty: MapType) -> GLint {
        match ty {
            MapType::REPEAT => gl::LINEAR as i32,
            MapType::MIRRORED => gl::MIRRORED_REPEAT as i32,
        }
    }

    pub fn settings(&mut self, min: InterpolationType, mag: InterpolationType, wrap_s: MapType, wrap_t: MapType) {
        self.bind();

        unsafe {
            // size
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, Self::gl_to_enum_inter(min));
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, Self::gl_to_enum_inter(mag));

            // repetion / wrapping
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, Self::gl_to_enum_map(wrap_s));
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, Self::gl_to_enum_map(wrap_t));
        }

        self.unbind();
    }


    pub fn bind(&mut self) {
        if !self.bound {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, self.id);
            }
            self.bound = true;
        }
    }

    pub fn unbind(&mut self) {
        if self.bound {
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
            self.bound = false;
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
