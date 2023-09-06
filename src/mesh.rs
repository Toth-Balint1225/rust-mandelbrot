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

use crate::buffer::*;
use crate::shader::Shader;
use crate::texture::Texture;

// pub struct Vertex {
//     items: Vec<f32>
// }

pub struct Mesh {
    vao: VertexArray,
    vbo: VertexBuffer,
    ebo: ElementBuffer,
    shader: Shader,
    texture: Texture,
}

pub struct MeshBuilder {
    shader: Result<Shader, String>,
    vertices: Option<Vec<f32>>,
    indices: Option<Vec<u32>>,
    attrib_desc: Option<VertexAttribDescriptor>,
    texture: Option<Texture>,
}

impl Mesh {
    fn builder() -> MeshBuilder {
        MeshBuilder {
            shader: Err("No Shader provided.".to_owned()),
            vertices: None,
            indices: None, 
            attrib_desc: None,
            texture: None,
        }
    }
    fn draw(&mut self) {}
}

impl MeshBuilder {
    fn shader(&mut self, vertex: &str, fragment: &str) -> &mut Self {
        self.shader = Shader::new(vertex, fragment);
        self
    }

    fn vertices(&mut self, vs: Vec<f32>) -> &mut Self {
        self.vertices = Some(vs);
        self
    }

    fn indices(&mut self, is: Vec<u32>) -> &mut Self {
        self.indices = Some(is);
        self
    }

    fn vertex_layout(&mut self, descriptor: VertexAttribDescriptor) -> &mut Self {
        self.attrib_desc = Some(descriptor);
        self
    }

    fn texture(&mut self, texture: Texture) -> &mut Self {
        self.texture = Some(texture);
        self
    }

    fn build(&mut self) -> Result<Mesh,String> {
        let mut vao_ = VertexArray::new();
        let mut vbo_ = VertexBuffer::new(&mut vao_);
        match &self.vertices {
            Some(vtc) => vbo_.set_data(vtc),
            None => return Err("No vertex data provided".to_owned()),
        }
        let mut ebo_ = ElementBuffer::new(&mut vao_);
        match &self.indices {
            Some(idc) => ebo_.set_data(idc, &mut vao_),
            None => return Err("No index data provided".to_owned()),
        }
        let mut texture_;
        match &self.texture {
            Some(tex) => texture_ = tex,
            None => return Err("No texture provided".to_owned())
        }
        vbo_.unbind();
        vao_.unbind();
        ebo_.unbind(&mut vao_);
        let shdr = self.shader.as_mut().unwrap();
        Ok(Mesh {
            shader: std::mem::take(shdr),
            ebo: ebo_,
            vao: vao_,
            vbo: vbo_,
            texture: *texture_,
        })
    }
}

// vec![
//     vert!{coord_3d![1.0, 1.0, 1.0], col_rgb![1.0, 0.0, 0.0], texture![0.0, 0.0], normal![0.0, 1.0, 0.0]},
// ]
