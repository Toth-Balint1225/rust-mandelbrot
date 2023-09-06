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

extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;

use sdl2::mouse::MouseUtil;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use sdl2::{event::Event, video::GLProfile};
use gl::types::{GLfloat, GLuint, GLsizei};
use std::ptr;

mod shader;
use shader::Shader;

mod buffer;
use buffer::{VertexArray, VertexBuffer, ElementBuffer, VertexAttribDescriptor, LayoutItem};

mod uniform;
use uniform::Uniform;

mod texture;

mod camera;
use camera::{Camera, Movement};

// mod mesh;
// use mesh::Mesh;

const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = 1080.0;

fn viewport(position: &glm::Vec2, mag: f32) -> glm::Mat3 {
    let mut transform = glm::mat3(
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    );
    transform = glm::translate2d(&transform, position);
    // let aspect = WIDTH / HEIGHT;
    // let zoom = glm::mat3(
    //     1.0 / mag, 0.0,                  0.0,
    //     0.0,                 1.0 / mag, 0.0,
    //     0.0,                 0.0,                  1.0,
    // );
    // transform * zoom 
    transform = glm::scale2d(&transform,&glm::vec2(1.0 / mag, 1.0 / mag));
    transform
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    
    // attributes for the GL
    let gl_attr = video_subsys.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsys.window("Mandelbrot fractal", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()   // this one is actually very important
        .build().map_err(|e| e.to_string())?;
    
    // let mouse = &sdl_context.mouse();

    // graphics context
    let _ctx = window.gl_create_context()?;
    // some hella unsafe raw pointery stuff
    gl::load_with(|name| video_subsys.gl_get_proc_address(name) as *const _);

    // --------------------------------------------------------------
    let vertices : [GLfloat;16] = [
        -1.0, -1.0,   -1.0, -1.0,
         1.0, -1.0,    1.0, -1.0,  
         1.0,  1.0,    1.0,  1.0,   
        -1.0,  1.0,   -1.0,  1.0, 
    ];

    let indices: [GLuint;6] = [
        0, 1, 2,
        0, 2, 3,
    ];
    let mut shader = Shader::new("resources/shader/mandelbrot_vert.glsl","resources/shader/mandelbrot_frag.glsl")?;
    shader.bind();
    let mut vao = VertexArray::new();
    let mut vbo = VertexBuffer::new(&mut vao);
    vbo.set_data(&vertices);
    let mut ebo = ElementBuffer::new(&mut vao);
    ebo.set_data(&indices, &mut vao);
    VertexAttribDescriptor::new()
        .layout(0, LayoutItem::Float(2)) // vertex
        .layout(1, LayoutItem::Float(2))  // texture
        .link(&mut vao, &mut vbo);
    vao.unbind();
    vbo.unbind();
    ebo.unbind(&mut vao);
    shader.unbind();
    

    let _unit = glm::mat3(
        1.0, 0.0, 0.0, 
        0.0, 1.0, 0.0, 
        0.0, 0.0, 1.0,  
    );

    let model = _unit;
    let mut pos = glm::vec2(-0.5, 0.0);
    let mut mag: f32 = 1.0;

    let mut view = viewport(&pos, mag);
    let projection = glm::mat3(
        WIDTH / HEIGHT, 0.0, 0.0,  
        0.0, 1.0, 0.0,
        0.0, 0.0, 0.0);
    let mut mvp = projection * view * model;
    let mvp_uniform = Uniform::new("mvp",&mut shader);
    mvp_uniform.set_m3f(&mvp, &mut shader);

    let mut iter = 2;
    let iter_uniform = Uniform::new("max_iter",&mut shader);
    iter_uniform.seti(iter,&mut shader);

    let mut evt_pump = sdl_context.event_pump()?;
    let mut t1 = std::time::Instant::now();
    let mut t2: std::time::Instant;
    let mut evt_manager = EvtManager::new();
    'active: loop {
        t2 = std::time::Instant::now();
        let dt = t2 - t1;
        t1 = t2;
        // positioning
        view = viewport(&pos, mag);
        mvp = projection * view * model;
        // mvp = view * model;
        mvp_uniform.set_m3f(&mvp, &mut shader);
        iter_uniform.seti(iter, &mut shader);

        // drawing

        unsafe {
            gl::ClearColor(0.0,0.0,0.0,1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        shader.bind();
        vao.bind();
        unsafe {
            gl::DrawElements(gl::TRIANGLES, indices.len() as GLsizei, gl::UNSIGNED_INT, ptr::null() as *const _);
        }
        window.gl_swap_window();

        for evt in evt_pump.poll_iter() {
            match evt {
                Event::Quit {..} => break 'active,
                Event::KeyDown {
                    timestamp: _,
                    keycode,
                    keymod: _,
                    repeat: _,
                    window_id: _,
                    scancode: _,
                } => evt_manager.key_down(keycode),
                Event::KeyUp {
                    timestamp: _,
                    keycode,
                    keymod: _,
                    repeat: _,
                    window_id: _,
                    scancode: _,
                } => evt_manager.key_up(keycode),
                _ => {},
            }
        }
        evt_manager.update_pos(&mut pos, &mut mag, &mut iter, dt.as_secs_f32());
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

#[allow(unused)]
struct EvtManager {
    forward:  bool,
    backward: bool,
    left:     bool,
    right:    bool,
    up:       bool,
    down:     bool,
    inc:      bool,
    dec:      bool,

    mouse: bool,
    mouse_x: f32,
    mouse_y: f32,
}
        
#[allow(unused)]
impl EvtManager {
    fn new() -> Self {
        Self {
            forward:  false,
            backward: false,
            left:     false,
            right:    false,
            up:       false,
            down:     false,

            inc:      false,
            dec:      false,

            mouse:    false,
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }

    fn key_state_set(&mut self, key: Option<Keycode>, state: bool) {
        use sdl2::keyboard::Keycode::*;
        match key {
            Some(code) => {
                match code {
                    W      => self.forward  = state,
                    S      => self.backward = state,
                    A      => self.left     = state,
                    D      => self.right    = state,
                    Q      => self.inc      = state,
                    E      => self.dec      = state,
                    Space  => self.up       = state,
                    LShift => self.down     = state,
                    _ => {},
                };
            },
            None => {}
        }
    }

    fn key_up(&mut self, key: Option<Keycode>) {
        self.key_state_set(key, false)
    }

    fn key_down(&mut self, key: Option<Keycode>) {
        self.key_state_set(key, true)
    }

    fn mouse_up(&mut self, mouse: &MouseUtil) {
        self.mouse = false;
        mouse.show_cursor(true);
    }

    fn mouse_down(&mut self, mouse: &MouseUtil) {
        self.mouse = true;
        mouse.show_cursor(false);
    }

    fn mouse_movement(&mut self, xrel: f32, yrel: f32, window: &Window, mouse: &MouseUtil) {
        if self.mouse {
            self.mouse_x = xrel;
            self.mouse_y = yrel;
            mouse.warp_mouse_in_window(window, (WIDTH / 2.0) as i32, (HEIGHT / 2.0) as i32);
        }
    }

    fn update_camera(&self, camera: &mut Camera, dt: f32) {
        if self.forward {
            camera.movement(Movement::Forward, dt);
        }
        if self.backward {
            camera.movement(Movement::Backward, dt);
        }
        if self.left {
            camera.movement(Movement::Left, dt);
        }
        if self.right {
            camera.movement(Movement::Right, dt);
        }
        if self.up {
            camera.movement(Movement::Up, dt);
        }
        if self.down {
            camera.movement(Movement::Down, dt);
        }

        if self.mouse {
            camera.tilt(self.mouse_x, self.mouse_y);
        }
    }

    fn update_pos(&mut self, pos: &mut glm::Vec2, mag: &mut f32, iter: &mut i32, dt: f32) {
        // let inc = 1.0 / (*mag * 10.0);
        // let mag_inc = *mag / 10.0;
        let inc = dt / *mag;
        let mag_inc = *mag * dt;
        if self.forward {
            pos.y += inc;
        }
        if self.backward {
            pos.y -= inc;
        }
        if self.left {
            pos.x -= inc;
        }
        if self.right {
            pos.x += inc;
        }
        if self.up {
            *mag += mag_inc;
        }
        if self.down {
            *mag -= mag_inc;
        }
        if self.inc && *iter < 1024 {
            *iter *= 2; 
            self.inc = false;
        }
        if self.dec && *iter > 2 {
            *iter /= 2;
            self.dec = false;
        }
        if *mag < 0.001 {
            *mag = 0.001
        }
    }
}
