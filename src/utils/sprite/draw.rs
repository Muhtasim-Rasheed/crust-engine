use glam::*;

use crate::utils::{
    Sprite,
    core::{Mesh, ShaderProgram, Vertex},
};

pub fn draw_sprite(sprite: &Sprite, shader: &ShaderProgram) {
    if !sprite.visible {
        return;
    }

    for clone in &sprite.clones {
        draw_sprite(clone, shader);
    }

    let tex = &sprite.costumes[sprite.costume()];
    let direction = sprite.direction.to_radians();

    let vertices = [
        Vertex {
            position: Vec2::new(-0.5, -0.5),
            uv: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec2::new(0.5, -0.5),
            uv: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec2::new(0.5, 0.5),
            uv: Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: Vec2::new(-0.5, 0.5),
            uv: Vec2::new(0.0, 0.0),
        },
    ];
    let indices = [0, 1, 2, 0, 2, 3];

    let quad = Mesh::<Vertex>::new(&vertices, &indices, crate::utils::core::DrawMode::Triangles);

    shader.use_program();
    shader.set_uniform_vec4("u_color", &Vec4::splat(1.0));
    shader.set_uniform_mat4(
        "u_model",
        &(
            Mat4::from_translation(sprite.center.extend(0.0))
            * Mat4::from_rotation_z(direction)
            * Mat4::from_scale(Vec3::new(sprite.size.x * sprite.scale, sprite.size.y * sprite.scale, 1.0))
        ),
    );
    tex.bind();
    quad.draw();
}
