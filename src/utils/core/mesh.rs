use std::marker::PhantomData;

use glam::*;

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {{
        let base = std::ptr::null::<$ty>();
        let field = std::ptr::addr_of!((*base).$field);
        field as usize - base as usize
    }};
}

pub trait VertexFormat {
    fn setup_attribs();
}

pub struct Vertex {
    pub position: Vec2,
    pub uv: Vec2,
}

impl VertexFormat for Vertex {
    fn setup_attribs() {
        unsafe {
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Self>() as i32,
                offset_of!(Self, position) as *const _,
            );
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Self>() as i32,
                offset_of!(Self, uv) as *const _,
            );

            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
        }
    }
}

pub enum DrawMode {
    Triangles = gl::TRIANGLES as isize,
    Lines = gl::LINES as isize,
}

pub struct Mesh<T: VertexFormat> {
    vao: u32,
    vbo: u32,
    ebo: u32,
    draw_mode: u32,
    vertex_count: usize,
    _marker: PhantomData<T>,
}

impl<T: VertexFormat> Mesh<T> {
    pub fn new(vertices: &[T], indices: &[u32], draw_mode: DrawMode) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            // Vertex Buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<T>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Element Buffer
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Setup vertex attributes
            T::setup_attribs();

            // Unbind VAO
            gl::BindVertexArray(0);
        }

        Mesh::<T> {
            vao,
            vbo,
            ebo,
            draw_mode: draw_mode as u32,
            vertex_count: indices.len(),
            _marker: PhantomData,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                self.draw_mode,
                self.vertex_count as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
        }
    }
}

impl<T: VertexFormat> Drop for Mesh<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}
