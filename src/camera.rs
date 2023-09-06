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
use crate::shader::Shader;
use crate::uniform::Uniform;

#[allow(unused)]
pub struct Camera {
    position: glm::Vec3,
    orientation: glm::Vec3,
    up: glm::Vec3,
    
    speed: f32,
    sensitivity: f32,
    width: f32,
    height: f32,

    projection: glm::Mat4x4,
    uniform: String,
}

pub enum Movement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

#[allow(unused)]
impl Camera {
    pub fn new(uniform_name: &str,
            width_: f32, height_: f32, 
            position_: glm::Vec3,
            fov_deg: f32,
            near: f32, far: f32) 
    -> Self {

        let proj = glm::perspective(width_ as f32 / height_ as f32, 
            deg_to_rad(fov_deg), near, far);

        Self {
            position: position_,
            orientation: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            speed: 2.0, 
            sensitivity: 100.0,
            width: width_,
            height: height_,
            projection: proj,
            uniform: String::from(uniform_name),
        }
    }

    pub fn update_matrix(&self, shader: &mut Shader) {
        let center = self.position + self.orientation;
        let view = glm::look_at(&self.position, &center, &self.up);
        let mvp = self.projection * view;
        let uniform = Uniform::new(&self.uniform, shader);
        uniform.set_m4f(&mvp, shader);
    }

    pub fn movement(&mut self, mov: Movement, dt: f32) {
        use Movement::*;
        self.position += self.speed * dt * match mov {
            Forward  =>  self.orientation,
            Backward => -self.orientation,
            Left     => -glm::normalize(&glm::cross::<f32,glm::U3>(&self.orientation, &self.up)),
            Right    =>  glm::normalize(&glm::cross::<f32,glm::U3>(&self.orientation, &self.up)),
            Up       =>  self.up,
            Down     => -self.up,
        }
    }
    
    pub fn tilt(&mut self, xrel: f32, yrel: f32) {
        //                  normal
        let xrot = (xrel / self.width  ) * self.sensitivity;
        let yrot = (yrel / self.height ) * self.sensitivity;

        let new_orientation = glm::rotate_vec3(&self.orientation,
            deg_to_rad(-yrot),
            &glm::normalize(&glm::cross::<f32,glm::U3>(&self.orientation, &self.up)));

        self.orientation = new_orientation;

        self.orientation = glm::rotate_vec3(&self.orientation,
            deg_to_rad(-xrot),
            &self.up);
    }

    pub fn position(&self) -> &glm::Vec3 {
        &self.position
    }
}


fn deg_to_rad(d: f32) -> f32 { (d * std::f32::consts::PI) / 180.0 }
