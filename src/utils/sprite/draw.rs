use glam::*;

use crate::utils::{Sprite, core::*};

pub fn draw_sprite(sprite: &Sprite, shader: &ShaderProgram, projection: Mat4, font: &BitmapFont) {
    if !sprite.visible {
        return;
    }

    for clone in &sprite.clones {
        draw_sprite(clone, shader, projection, font);
    }

    let tex = &sprite.costumes[sprite.costume()];
    let direction = sprite.direction.to_radians();

    let vertices = [
        Vertex {
            position: Vec2::new(-0.5, -0.5),
            uv: sprite.uv[0],
        },
        Vertex {
            position: Vec2::new(0.5, -0.5),
            uv: Vec2::new(sprite.uv[1].x, sprite.uv[0].y),
        },
        Vertex {
            position: Vec2::new(0.5, 0.5),
            uv: sprite.uv[1],
        },
        Vertex {
            position: Vec2::new(-0.5, 0.5),
            uv: Vec2::new(sprite.uv[0].x, sprite.uv[1].y),
        },
    ];
    let indices = [0, 1, 2, 0, 2, 3];

    let quad = Mesh::new(&vertices, &indices, crate::utils::core::DrawMode::Triangles);

    shader.use_program();
    shader.set_uniform("u_color", Vec4::splat(1.0));
    shader.set_uniform("u_projection", projection);
    shader.set_uniform(
        "u_model",
        Mat4::from_translation(sprite.center.extend(0.0))
            * Mat4::from_rotation_z(direction)
            * Mat4::from_scale(Vec3::new(
                sprite.size.x * sprite.scale,
                sprite.size.y * sprite.scale,
                1.0,
            )),
    );
    tex.bind();
    quad.draw();

    if let Some(dialogue) = &sprite.dialogue {
        let italicised = dialogue.think;
        let color = Vec4::new(1.0, 1.0, 1.0, if dialogue.think { 0.75 } else { 1.0 });

        let text_size = Vec2::from(font.size(&dialogue.text, 36.0));
        let text_x = sprite.center.x - text_size.x / 2.0;
        let text_y = sprite.center.y + sprite.size.y / 2.0 + 10.0;

        let text_params = TextParams {
            text: &dialogue.text,
            projection,
            pos: Vec2::new(text_x, text_y),
            down_positive: false,
            font_size: 36.0,
            color,
            italicised,
            ..TextParams::default_params(font, shader)
        };

        draw_text(text_params);
    }
}
