use crate::utils::core::*;
use glam::*;
use glfw::Window;

pub struct Stage {
    pub backdrops: Vec<GPUTexture>,
    pub stamp_buffer: Framebuffer,
    current_backdrop: usize,
    last_screen_width: i32,
    last_screen_height: i32,
}

impl Stage {
    pub fn new(backdrops: Vec<GPUTexture>, window: &Window) -> Self {
        Self {
            backdrops,
            stamp_buffer: Framebuffer::new(
                window.get_size().0 as u32,
                window.get_size().1 as u32,
                false,
            ),
            current_backdrop: 0,
            last_screen_width: 0,
            last_screen_height: 0,
        }
    }

    pub fn clear_stamps(&mut self) {
        self.stamp_buffer.bind();
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        Framebuffer::unbind();
    }

    pub fn set_backdrop(&mut self, index: usize) {
        if index < self.backdrops.len() {
            self.current_backdrop = index;
        }
    }

    pub fn next_backdrop(&mut self) {
        self.current_backdrop = (self.current_backdrop + 1) % self.backdrops.len();
    }

    pub fn prev_backdrop(&mut self) {
        if self.current_backdrop == 0 {
            self.current_backdrop = self.backdrops.len() - 1;
        } else {
            self.current_backdrop -= 1;
        }
    }

    pub fn backdrop(&self) -> usize {
        self.current_backdrop
    }

    pub fn draw(&mut self, window: &Window, shader_program: &ShaderProgram, projection: &Mat4) {
        let sw = window.get_size().0;
        let sh = window.get_size().1;

        if sw != self.last_screen_width || sh != self.last_screen_height {
            unsafe {
                gl::Viewport(0, 0, sw, sh);
            }
            self.last_screen_width = sw;
            self.last_screen_height = sh;
            self.stamp_buffer = Framebuffer::new(sw as u32, sh as u32, false);
        }

        let texture = &self.backdrops[self.current_backdrop];
        let tw = texture.width() as f32;
        let th = texture.height() as f32;
        let sw = sw as f32;
        let sh = sh as f32;
        let size = if tw / th > sw / sh {
            vec2(sw, th * (sw / tw)) * 2.0
        } else {
            vec2(tw * (sh / th), sh) * 2.0
        };
        let x = -size.x / 2.0;
        let y = -size.y / 2.0;
        let backdrop_quad = [
            Vertex {
                position: vec2(x, y + size.y),
                uv: vec2(0.0, 1.0),
            }, // Top-left
            Vertex {
                position: vec2(x + size.x, y + size.y),
                uv: vec2(1.0, 1.0),
            }, // Top-right
            Vertex {
                position: vec2(x + size.x, y),
                uv: vec2(1.0, 0.0),
            }, // Bottom-right
            Vertex {
                position: vec2(x, y),
                uv: vec2(0.0, 0.0),
            }, // Bottom-left
        ];
        let stamp_quad = [
            Vertex {
                position: vec2(-sw, -sh),
                uv: vec2(0.0, 0.0),
            }, // Bottom-left
            Vertex {
                position: vec2(sw, -sh),
                uv: vec2(1.0, 0.0),
            }, // Bottom-right
            Vertex {
                position: vec2(sw, sh),
                uv: vec2(1.0, 1.0),
            }, // Top-right
            Vertex {
                position: vec2(-sw, sh),
                uv: vec2(0.0, 1.0),
            }, // Top-left
        ];
        let indices = [0, 1, 2, 0, 2, 3];
        let backdrop_mesh = Mesh::new(&backdrop_quad, &indices, DrawMode::Triangles);
        let stamp_mesh = Mesh::new(&stamp_quad, &indices, DrawMode::Triangles);

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.use_program();
        shader_program.set_uniform_vec4("u_color", &vec4(1.0, 1.0, 1.0, 1.0));
        shader_program.set_uniform_mat4("u_projection", projection);
        texture.bind();
        backdrop_mesh.draw();
        shader_program.set_uniform_vec4("u_color", &vec4(1.0, 1.0, 1.0, 1.0));
        shader_program.set_uniform_mat4("u_projection", projection);
        self.stamp_buffer.texture().bind();
        stamp_mesh.draw();
    }
}
