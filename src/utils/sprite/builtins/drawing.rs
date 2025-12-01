use std::{cell::RefCell, rc::Rc};

use crate::utils::{core::*, *};
use glam::*;

pub fn set_color(state: &mut State, args: &[&mut Value]) -> Result {
    if let [
        Value::Number(r),
        Value::Number(g),
        Value::Number(b),
        Value::Number(a),
    ] = args
    {
        let color = Vec4::new(*r / 255.0, *g / 255.0, *b / 255.0, *a / 255.0);
        state.sprite.draw_color = color;
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("set_color() requires four number arguments: r, g, b, a".to_string())
    }
}

pub fn change_r(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(amount)] = args {
        state.sprite.draw_color.x = (state.sprite.draw_color.x + *amount / 255.0).clamp(0.0, 1.0);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("change_r() requires a single number argument".to_string())
    }
}

pub fn change_g(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(amount)] = args {
        state.sprite.draw_color.y = (state.sprite.draw_color.y + *amount / 255.0).clamp(0.0, 1.0);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("change_g() requires a single number argument".to_string())
    }
}

pub fn change_b(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(amount)] = args {
        state.sprite.draw_color.z = (state.sprite.draw_color.z + *amount / 255.0).clamp(0.0, 1.0);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("change_b() requires a single number argument".to_string())
    }
}

pub fn change_a(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(amount)] = args {
        state.sprite.draw_color.w = (state.sprite.draw_color.w + *amount / 255.0).clamp(0.0, 1.0);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("change_a() requires a single number argument".to_string())
    }
}

pub fn line(state: &State, args: &[&mut Value]) -> Result {
    if let [
        Value::Number(x1),
        Value::Number(y1),
        Value::Number(x2),
        Value::Number(y2),
        Value::Number(thickness),
    ] = args
    {
        let start = Vec2::new(*x1, *y1);
        let end = Vec2::new(*x2, *y2);
        let thickness = *thickness;
        let color = state.sprite.draw_color;
        draw_line(start, end, thickness, state.shader_program, color);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("line() requires five number arguments: x1, y1, x2, y2, thickness".to_string())
    }
}

pub fn rect(state: &State, args: &[&mut Value]) -> Result {
    if let [
        Value::Number(x),
        Value::Number(y),
        Value::Number(width),
        Value::Number(height),
    ] = args
    {
        let start = Vec2::new(*x, *y);
        let end = Vec2::new(*width, *height) + start;
        draw_rectangle(start, end, state.shader_program, state.sprite.draw_color);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("rect() requires four number arguments: x, y, width, height".to_string())
    }
}

pub fn hrect(state: &State, args: &[&mut Value]) -> Result {
    if let [
        Value::Number(x),
        Value::Number(y),
        Value::Number(width),
        Value::Number(height),
        Value::Number(thickness),
    ] = args
    {
        let start = Vec2::new(*x, *y);
        let end = Vec2::new(*width, *height) + start;
        draw_line(
            start,
            Vec2::new(end.x, start.y),
            *thickness,
            state.shader_program,
            state.sprite.draw_color,
        );
        draw_line(
            Vec2::new(end.x, start.y),
            end,
            *thickness,
            state.shader_program,
            state.sprite.draw_color,
        );
        draw_line(
            end,
            Vec2::new(start.x, end.y),
            *thickness,
            state.shader_program,
            state.sprite.draw_color,
        );
        draw_line(
            Vec2::new(start.x, end.y),
            start,
            *thickness,
            state.shader_program,
            state.sprite.draw_color,
        );
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("hrect() requires four number arguments: x, y, width, height".to_string())
    }
}

pub fn circle(state: &State, args: &[&mut Value]) -> Result {
    if let [Value::Number(x), Value::Number(y), Value::Number(radius)] = args {
        const NUM_SEGMENTS: usize = 64;
        let mut xs = Vec::with_capacity(NUM_SEGMENTS);
        let mut ys = Vec::with_capacity(NUM_SEGMENTS);
        for i in 0..NUM_SEGMENTS {
            let angle = (i as f32 / NUM_SEGMENTS as f32) * std::f32::consts::PI * 2.0;
            xs.push(*x + radius * angle.cos());
            ys.push(*y + radius * angle.sin());
        }
        draw_convex_polygon(&xs, &ys, state.shader_program, state.sprite.draw_color);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("circle() requires three number arguments: x, y, radius".to_string())
    }
}

pub fn hcircle(state: &State, args: &[&mut Value]) -> Result {
    if let [
        Value::Number(x),
        Value::Number(y),
        Value::Number(radius),
        Value::Number(thickness),
    ] = args
    {
        const NUM_SEGMENTS: usize = 64;
        let mut xs = Vec::with_capacity(NUM_SEGMENTS);
        let mut ys = Vec::with_capacity(NUM_SEGMENTS);
        for i in 0..NUM_SEGMENTS {
            let angle = (i as f32 / NUM_SEGMENTS as f32) * std::f32::consts::PI * 2.0;
            xs.push(*x + radius * angle.cos());
            ys.push(*y + radius * angle.sin());
        }
        draw_convex_polygon_lines(
            &xs,
            &ys,
            *thickness,
            state.shader_program,
            state.sprite.draw_color,
        );
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("hcircle() requires four number arguments: x, y, radius, thickness".to_string())
    }
}

pub fn ellipse(state: &State, args: &[&mut Value]) -> Result {
    const NUM_SEGMENTS: usize = 64;
    match args {
        [
            Value::Number(x),
            Value::Number(y),
            Value::Number(rx),
            Value::Number(ry),
        ] => {
            let x = *x;
            let y = *y;
            let rx = *rx;
            let ry = *ry;

            let mut xs = Vec::with_capacity(NUM_SEGMENTS);
            let mut ys = Vec::with_capacity(NUM_SEGMENTS);

            for i in 0..NUM_SEGMENTS {
                let angle = (i as f32 / NUM_SEGMENTS as f32) * std::f32::consts::PI * 2.0;
                xs.push(x + rx * angle.cos());
                ys.push(y + ry * angle.sin());
            }

            draw_convex_polygon(&xs, &ys, state.shader_program, state.sprite.draw_color);

            Ok(Rc::new(RefCell::new(Value::Null)))
        }
        [
            Value::Number(x),
            Value::Number(y),
            Value::Number(rx),
            Value::Number(ry),
            Value::Number(rotation),
        ] => {
            let x = *x;
            let y = *y;
            let rx = *rx;
            let ry = *ry;

            let mut xs = Vec::with_capacity(NUM_SEGMENTS);
            let mut ys = Vec::with_capacity(NUM_SEGMENTS);

            for i in 0..NUM_SEGMENTS {
                let angle =
                    (i as f32 / NUM_SEGMENTS as f32) * std::f32::consts::PI * 2.0 + rotation;
                xs.push(x + rx * angle.cos());
                ys.push(y + ry * angle.sin());
            }

            draw_convex_polygon(&xs, &ys, state.shader_program, state.sprite.draw_color);

            Ok(Rc::new(RefCell::new(Value::Null)))
        }
        _ => Err("ellipse() requires four number arguments: x, y, rx, ry, [rotation]".to_string()),
    }
}

pub fn hellipse(state: &State, args: &[&mut Value]) -> Result {
    const NUM_SEGMENTS: usize = 64;
    match args {
        [
            Value::Number(x),
            Value::Number(y),
            Value::Number(rx),
            Value::Number(ry),
            Value::Number(thickness),
        ] => {
            let x = *x;
            let y = *y;
            let rx = *rx;
            let ry = *ry;

            let mut xs = Vec::with_capacity(NUM_SEGMENTS);
            let mut ys = Vec::with_capacity(NUM_SEGMENTS);

            for i in 0..NUM_SEGMENTS {
                let angle = (i as f32 / NUM_SEGMENTS as f32) * std::f32::consts::PI * 2.0;
                xs.push(x + rx * angle.cos());
                ys.push(y + ry * angle.sin());
            }

            draw_convex_polygon_lines(
                &xs,
                &ys,
                *thickness,
                state.shader_program,
                state.sprite.draw_color,
            );

            Ok(Rc::new(RefCell::new(Value::Null)))
        }
        [
            Value::Number(x),
            Value::Number(y),
            Value::Number(rx),
            Value::Number(ry),
            Value::Number(rotation),
            Value::Number(thickness),
        ] => {
            let x = *x;
            let y = *y;
            let rx = *rx;
            let ry = *ry;

            let mut xs = Vec::with_capacity(NUM_SEGMENTS);
            let mut ys = Vec::with_capacity(NUM_SEGMENTS);

            for i in 0..NUM_SEGMENTS {
                let angle =
                    (i as f32 / NUM_SEGMENTS as f32) * std::f32::consts::PI * 2.0 + rotation;
                xs.push(x + rx * angle.cos());
                ys.push(y + ry * angle.sin());
            }

            draw_convex_polygon_lines(
                &xs,
                &ys,
                *thickness,
                state.shader_program,
                state.sprite.draw_color,
            );

            Ok(Rc::new(RefCell::new(Value::Null)))
        }
        _ => Err(
            "hellipse() requires four number arguments: x, y, rx, ry, thickness, [rotation]"
                .to_string(),
        ),
    }
}

pub fn polygon(state: &State, args: &[&mut Value]) -> Result {
    if let [Value::List(xs), Value::List(ys)] = args {
        if xs.len() != ys.len() {
            return Err(
                "polygon() requires two lists of equal length: x and y coordinates".to_string(),
            );
        }
        let xs = xs
            .iter()
            .map(|v| v.borrow().to_number())
            .collect::<Vec<f32>>();
        let ys = ys
            .iter()
            .map(|v| v.borrow().to_number())
            .collect::<Vec<f32>>();
        draw_convex_polygon(&xs, &ys, state.shader_program, state.sprite.draw_color);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("polygon() requires two lists of numbers: x and y coordinates".to_string())
    }
}

pub fn hpolygon(state: &State, args: &[&mut Value]) -> Result {
    if let [Value::Number(thickness), Value::List(xs), Value::List(ys)] = args {
        if xs.len() != ys.len() {
            return Err(
                "hpolygon() requires two lists of equal length: x and y coordinates".to_string(),
            );
        }
        let xs = xs
            .iter()
            .map(|v| v.borrow().to_number())
            .collect::<Vec<f32>>();
        let ys = ys
            .iter()
            .map(|v| v.borrow().to_number())
            .collect::<Vec<f32>>();
        draw_convex_polygon_lines(
            &xs,
            &ys,
            *thickness,
            state.shader_program,
            state.sprite.draw_color,
        );
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err(
            "hpolygon() requires two lists of numbers and a thickness: x and y coordinates"
                .to_string(),
        )
    }
}

pub fn text(state: &State, args: &[&mut Value]) -> Result {
    if let [
        Value::String(text),
        Value::Number(x),
        Value::Number(y),
        Value::Number(font_size),
    ] = args
    {
        let pos = Vec2::new(*x, *y);
        draw_text(TextParams {
            text,
            projection: *state.projection,
            model: Mat4::IDENTITY,
            pos,
            down_positive: false,
            font_size: *font_size * 2.0,
            color: state.sprite.draw_color,
            italicised: false,
            ..TextParams::default_params(state.font, state.shader_program)
        });
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("text() requires a string and two numbers: text, x, y".to_string())
    }
}

pub fn textured_tri(state: &State, args: &[&mut Value]) -> Result {
    if let [
        Value::List(parse_image_result),
        Value::List(xs),
        Value::List(ys),
        Value::List(us),
        Value::List(vs),
    ] = args
    {
        let guards = parse_image_result
            .iter()
            .map(|v| v.borrow())
            .collect::<Vec<_>>();
        let parse_image_result = guards.iter().map(|v| &**v).collect::<Vec<_>>();
        if let [
            Value::Number(width),
            Value::Number(height),
            Value::List(pixels),
        ] = parse_image_result.as_slice()
        {
            if xs.len() != ys.len() || xs.len() != us.len() || xs.len() != vs.len() || xs.len() != 3
            {
                return Err(
                    "textured_tri() requires three lists of equal length: x, y, u, v coordinates"
                        .to_string(),
                );
            }
            let mut cpu_texture = CPUTexture::new(*width as u32, *height as u32);
            cpu_texture.data = pixels
                .chunks(4)
                .map(|c| {
                    if c.len() == 4 {
                        U8Vec4::new(
                            c[0].borrow().to_number() as u8,
                            c[1].borrow().to_number() as u8,
                            c[2].borrow().to_number() as u8,
                            c[3].borrow().to_number() as u8,
                        )
                    } else {
                        U8Vec4::new(0, 0, 0, 0)
                    }
                })
                .collect();
            let gpu_texture = cpu_texture.upload_to_gpu();
            let vertices: Vec<Vertex> = xs
                .iter()
                .zip(ys.iter())
                .zip(us.iter())
                .zip(vs.iter())
                .map(|(((x, y), u), v)| Vertex {
                    position: vec2(x.borrow().to_number(), y.borrow().to_number()),
                    uv: vec2(u.borrow().to_number(), v.borrow().to_number()),
                })
                .collect();
            let indices = [0, 1, 2];
            let mesh = Mesh::new(&vertices, &indices, core::DrawMode::Triangles);
            state.shader_program.use_program();
            state
                .shader_program
                .set_uniform("u_color", Vec4::splat(1.0));
            state
                .shader_program
                .set_uniform("u_projection", *state.projection);
            state.shader_program.set_uniform("u_model", Mat4::IDENTITY);
            state.shader_program.set_uniform("u_effects", &[] as &[i32]);
            state
                .shader_program
                .set_uniform("u_effect_values", &[] as &[f32]);
            state.shader_program.set_uniform("u_effects_count", 0);
            gpu_texture.bind();
            mesh.draw();
            Ok(Rc::new(RefCell::new(Value::Null)))
        } else {
            return Err(
                "textured_tri() requires an image with width, height, and pixel data".to_string(),
            );
        }
    } else {
        return Err(
            "textured_tri() requires an image and lists of x, y, u, v coordinates".to_string(),
        );
    }
}

pub fn stamp(state: &State) -> Result {
    state.project.stage.stamp_buffer.bind();
    draw_sprite(
        state.sprite,
        state.shader_program,
        Mat4::orthographic_rh_gl(
            -state.window.get_size().0 as f32,
            state.window.get_size().0 as f32,
            -state.window.get_size().1 as f32,
            state.window.get_size().1 as f32,
            -1.0,
            1.0,
        ),
        state.font,
    );
    Framebuffer::unbind();
    Ok(Rc::new(RefCell::new(Value::Null)))
}

pub fn clear_all_stamps(state: &State) -> Result {
    state.project.stage.clear_stamps();
    Ok(Rc::new(RefCell::new(Value::Null)))
}

pub fn r(state: &State) -> Result {
    Ok(Rc::new(RefCell::new(Value::Number(
        state.sprite.draw_color.x * 255.0,
    ))))
}

pub fn g(state: &State) -> Result {
    Ok(Rc::new(RefCell::new(Value::Number(
        state.sprite.draw_color.y * 255.0,
    ))))
}

pub fn b(state: &State) -> Result {
    Ok(Rc::new(RefCell::new(Value::Number(
        state.sprite.draw_color.z * 255.0,
    ))))
}

pub fn a(state: &State) -> Result {
    Ok(Rc::new(RefCell::new(Value::Number(
        state.sprite.draw_color.w * 255.0,
    ))))
}
