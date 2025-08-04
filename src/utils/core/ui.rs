use crate::utils::core::{GPUTexture, ShaderProgram};

use super::mesh::{DrawMode, Mesh, Vertex};
use glam::*;

#[derive(Debug)]
pub struct BitmapFont {
    first_char: char,
    chars_per_row: u32,
    char_width: u32,
    char_height: u32,
    atlas: GPUTexture,
}

impl BitmapFont {
    pub fn new(
        atlas: GPUTexture,
        first_char: char,
        chars_per_row: u32,
        char_width: u32,
        char_height: u32,
    ) -> Self {
        BitmapFont {
            first_char,
            chars_per_row,
            char_width,
            char_height,
            atlas,
        }
    }

    pub fn get_glyph_uv(&self, ch: char) -> Option<([f32; 2], [f32; 2])> {
        let glyph_index = ch as u32 - self.first_char as u32;

        let tex_width = self.atlas.width();
        let tex_height = self.atlas.height();

        if glyph_index >= self.chars_per_row * (tex_height / self.char_height) {
            return None; // glyph not in atlas
        }

        let col = glyph_index % self.chars_per_row;
        let row = glyph_index / self.chars_per_row;

        let u0 = (col * self.char_width) as f32 / tex_width as f32;
        let v0 = (row * self.char_height) as f32 / tex_height as f32;
        let u1 = ((col + 1) * self.char_width) as f32 / tex_width as f32;
        let v1 = ((row + 1) * self.char_height) as f32 / tex_height as f32;

        Some(([u0, v0], [u1, v1]))
    }

    pub fn size(&self, text: &str, font_size: f32) -> (f32, f32) {
        let mut number_of_lines = 1;
        for ch in text.chars() {
            if ch == '\n' {
                number_of_lines += 1;
            }
        }
        let max_line_length = text.lines().map(|line| line.len()).max().unwrap_or(0);
        let height = font_size * number_of_lines as f32;
        let width =
            (self.char_width as f32 * font_size / self.char_height as f32) * max_line_length as f32;
        (width, height)
    }

    pub fn build(
        &self,
        text: &str,
        start_x: f32,
        start_y: f32,
        font_size: f32,
        italicised: bool,
        down_positive: bool,
    ) -> Mesh<Vertex> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let italic_offset = if italicised { 0.25 * font_size } else { 0.0 } / 2.0;

        let mut x = start_x;
        let mut y = start_y;
        let mut i = 0;

        for ch in text.chars() {
            if ch == '\n' {
                x = start_x;
                y += font_size;
                continue;
            }

            if let Some((uv0, uv1)) = self.get_glyph_uv(ch) {
                let h = font_size;
                let w = self.char_width as f32 * font_size / self.char_height as f32;

                let idx = i * 4;

                if down_positive {
                    vertices.push(Vertex {
                        position: glam::vec2(x - italic_offset, y + h),
                        uv: glam::vec2(uv0[0], uv1[1]),
                    }); // Top-left
                    vertices.push(Vertex {
                        position: glam::vec2(x + w - italic_offset, y + h),
                        uv: glam::vec2(uv1[0], uv1[1]),
                    }); // Top-right
                    vertices.push(Vertex {
                        position: glam::vec2(x + w + italic_offset, y),
                        uv: glam::vec2(uv1[0], uv0[1]),
                    }); // Bottom-right
                    vertices.push(Vertex {
                        position: glam::vec2(x + italic_offset, y),
                        uv: glam::vec2(uv0[0], uv0[1]),
                    }); // Bottom-left
                } else {
                    vertices.push(Vertex {
                        position: glam::vec2(x + italic_offset, y),
                        uv: glam::vec2(uv0[0], uv0[1]),
                    }); // Bottom-left
                    vertices.push(Vertex {
                        position: glam::vec2(x + w + italic_offset, y),
                        uv: glam::vec2(uv1[0], uv0[1]),
                    }); // Bottom-right
                    vertices.push(Vertex {
                        position: glam::vec2(x + w - italic_offset, y + h),
                        uv: glam::vec2(uv1[0], uv1[1]),
                    }); // Top-right
                    vertices.push(Vertex {
                        position: glam::vec2(x - italic_offset, y + h),
                        uv: glam::vec2(uv0[0], uv1[1]),
                    }); // Top-left
                }

                indices.extend_from_slice(&[idx, idx + 1, idx + 2, idx, idx + 2, idx + 3]);

                x += w; // Advance cursor
                i += 1;
            }
        }

        Mesh::new(&vertices, &indices, DrawMode::Triangles)
    }
}

pub struct TextParams<'a> {
    pub text: &'a str,
    pub font: &'a BitmapFont,
    pub shader: &'a ShaderProgram,
    pub projection: Mat4,
    pub model: Mat4,
    pub pos: Vec2,
    pub down_positive: bool,
    pub font_size: f32,
    pub color: Vec4,
    pub italicised: bool,
}

impl TextParams<'_> {
    pub fn default_params<'a>(font: &'a BitmapFont, shader: &'a ShaderProgram) -> TextParams<'a> {
        TextParams {
            text: "",
            font,
            shader,
            projection: Mat4::IDENTITY,
            model: Mat4::IDENTITY,
            pos: Vec2::ZERO,
            down_positive: true,
            font_size: 16.0,
            color: Vec4::splat(1.0),
            italicised: false,
        }
    }
}

pub fn draw_text(params: TextParams<'_>) {
    let mesh = params.font.build(
        params.text,
        params.pos.x,
        params.pos.y,
        params.font_size,
        params.italicised,
        params.down_positive,
    );

    params.shader.use_program();
    params.shader.set_uniform("u_projection", params.projection);
    params.shader.set_uniform("u_model", params.model);
    params.shader.set_uniform("u_color", params.color);
    params.font.atlas.bind();

    mesh.draw();
}
