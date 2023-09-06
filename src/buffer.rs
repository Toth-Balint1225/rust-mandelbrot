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
use gl::types::{GLuint, GLint, GLfloat, GLsizeiptr, GLsizei, GLenum};
use std::{mem};


pub struct VertexArray {
    id: GLuint,

    bound: bool,
}

pub struct VertexBuffer {
    id: GLuint,
    
    bound: bool,
}

pub struct ElementBuffer {
    id: GLuint,

    bound: bool,
}

pub struct VertexAttribDescriptor {
    items: std::collections::BTreeMap<u8, LayoutItem>
}

#[allow(unused)]
pub enum LayoutItem {
    Integer(u8),
    Float(u8),
}

impl VertexAttribDescriptor {
    pub fn new() -> Self {
        Self {items: std::collections::BTreeMap::new() }
    }

    pub fn layout(&mut self, id: u8, item: LayoutItem) -> &mut Self {
        self.items.insert(id, item);
        self
    }

    pub fn link(&self, vao: &mut VertexArray, vbo: &mut VertexBuffer){
        // we need the stride
        let stride = self.items.iter()
            .map(|item| item.1)
            .map(|item| match item {
                LayoutItem::Integer(count) => *count as usize * mem::size_of::<GLfloat>(), 
                LayoutItem::Float(count) => *count as usize * mem::size_of::<GLint>()
            })
            .fold(0, |acc, item| acc + item);
        
        // assign to the pointer attribs
        let mut offset_pointer = 0;
        for item in self.items.iter() {
            // distinguish between types
            match item.1 {
                LayoutItem::Float(count) => {
                    vao.link_attribute(vbo, *item.0 as u32, *count as i32, gl::FLOAT, stride as i32, offset_pointer);
                    offset_pointer += *count as usize * mem::size_of::<GLfloat>();
                },
                LayoutItem::Integer(count) => {
                    vao.link_attribute(vbo, *item.0 as u32, *count as i32, gl::INT, stride as i32, offset_pointer);
                    offset_pointer += *count as usize * mem::size_of::<GLint>();
                },
            }
        }
    }
}

impl VertexArray {
    pub fn new() -> Self {
        let mut tmp_id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut tmp_id);
        }
        Self { 
            id: tmp_id,
            bound: false,
        }
    }

    pub fn link_attribute(
            &mut self, 
            vbo: &mut VertexBuffer, 
            layout: GLuint, 
            items: GLint, 
            ty: GLenum, 
            stride_bytes: GLsizei,  
            offset: usize
        ) {
        self.bind();
        vbo.bind();
        unsafe {
            gl::VertexAttribPointer(
                layout,
                items, 
                ty, 
                gl::FALSE,
                stride_bytes,
                offset as *const _);
            
            gl::EnableVertexAttribArray(layout);
        }
    }

    pub fn bind(&mut self) {
        if !self.bound {
            unsafe {
                gl::BindVertexArray(self.id);
            }
            self.bound = true;
        }
    }

    pub fn unbind(&mut self) {
        if self.bound {
            unsafe {
                gl::BindVertexArray(0);
            }
            self.bound = false;
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        self.unbind();
        unsafe {
            gl::DeleteVertexArrays(1,&mut self.id);
        }
    }
}

impl VertexBuffer {
    pub fn new(vao: &mut VertexArray) -> Self {
        vao.bind();
        let mut tmp_id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut tmp_id);
        }

        Self {
            id: tmp_id,
            bound: false,
        }
    }

    pub fn set_data(&mut self, data: &[GLfloat]) {
        self.bind();
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, 
            (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&data[0]), gl::STATIC_DRAW)
        }
    }

    pub fn bind(&mut self) {
        if !self.bound {
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            }
            self.bound = false;
        }
    }
    
    pub fn unbind(&mut self) {
        if self.bound {
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            }
            self.bound = false;
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        self.unbind();
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

impl ElementBuffer {
    pub fn new(vao: &mut VertexArray) -> Self {
        let mut tmp_id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut tmp_id);
        }

        let mut instance = Self { 
            id: tmp_id,
            bound: false,
        };
        instance.bind(vao);
        instance
    }

    pub fn set_data(&mut self, data: &[GLuint], vao: &mut VertexArray) {
        self.bind(vao);
        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * mem::size_of::<u32>()) as GLsizeiptr,
                mem::transmute(&data[0]),gl::STATIC_DRAW);
        }
    }

    pub fn bind(&mut self, vao: &mut VertexArray) {
        vao.bind();
        if !self.bound {
            unsafe {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
            }
            self.bound = false;
        }
    }
    
    pub fn unbind(&mut self, vao: &mut VertexArray) {
        vao.unbind();
        if self.bound {
            unsafe {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            }
            self.bound = false;
        }
    }
}

impl Drop for ElementBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
