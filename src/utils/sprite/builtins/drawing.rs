use crate::utils::{core::*, *};
use glam::*;

pub fn set_color(state: &mut State, args: &[Value]) -> Result {
    if let [
        Value::Number(r),
        Value::Number(g),
        Value::Number(b),
        Value::Number(a),
    ] = args
    {
        let color = Vec4::new(*r / 255.0, *g / 255.0, *b / 255.0, *a / 255.0);
        state.sprite.draw_color = color;
        Ok(Value::Null)
    } else {
        Err("set_color() requires four number arguments: r, g, b, a".to_string())
    }
}

pub fn change_r(state: &mut State, args: &[Value]) -> Result {
    if let [Value::Number(amount)] = args {
        state.sprite.draw_color.x = (state.sprite.draw_color.x + *amount / 255.0).clamp(0.0, 1.0);
        Ok(Value::Null)
    } else {
        Err("change_r() requires a single number argument".to_string())
    }
}

pub fn change_g(state: &mut State, args: &[Value]) -> Result {
    if let [Value::Number(amount)] = args {
        state.sprite.draw_color.y = (state.sprite.draw_color.y + *amount / 255.0).clamp(0.0, 1.0);
        Ok(Value::Null)
    } else {
        Err("change_g() requires a single number argument".to_string())
    }
}

pub fn change_b(state: &mut State, args: &[Value]) -> Result {
    if let [Value::Number(amount)] = args {
        state.sprite.draw_color.z = (state.sprite.draw_color.z + *amount / 255.0).clamp(0.0, 1.0);
        Ok(Value::Null)
    } else {
        Err("change_b() requires a single number argument".to_string())
    }
}

pub fn change_a(state: &mut State, args: &[Value]) -> Result {
    if let [Value::Number(amount)] = args {
        state.sprite.draw_color.w = (state.sprite.draw_color.w + *amount / 255.0).clamp(0.0, 1.0);
        Ok(Value::Null)
    } else {
        Err("change_a() requires a single number argument".to_string())
    }
}

pub fn line(state: &State, args: &[Value]) -> Result {
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
        // draw_line(
        //     start.x,
        //     start.y,
        //     end.x,
        //     end.y,
        //     *thickness,
        //     state.sprite.draw_color,
        // );
        Ok(Value::Null)
    } else {
        Err("line() requires five number arguments: x1, y1, x2, y2, thickness".to_string())
    }
}

pub fn rect(state: &State, args: &[Value]) -> Result {
    if let [
        Value::Number(x),
        Value::Number(y),
        Value::Number(width),
        Value::Number(height),
    ] = args
    {
        // draw_rectangle(*x, *y, *width, *height, state.sprite.draw_color);
        Ok(Value::Null)
    } else {
        Err("rect() requires four number arguments: x, y, width, height".to_string())
    }
}

pub fn hrect(state: &State, args: &[Value]) -> Result {
    if let [
        Value::Number(x),
        Value::Number(y),
        Value::Number(width),
        Value::Number(height),
        Value::Number(thickness),
    ] = args
    {
        // draw_rectangle_lines(*x, *y, *width, *height, *thickness, state.sprite.draw_color);
        Ok(Value::Null)
    } else {
        Err("hrect() requires four number arguments: x, y, width, height".to_string())
    }
}

pub fn circle(state: &State, args: &[Value]) -> Result {
    if let [Value::Number(x), Value::Number(y), Value::Number(radius)] = args {
        // draw_circle(*x, *y, *radius, state.sprite.draw_color);
        Ok(Value::Null)
    } else {
        Err("circle() requires three number arguments: x, y, radius".to_string())
    }
}

pub fn hcircle(state: &State, args: &[Value]) -> Result {
    if let [
        Value::Number(x),
        Value::Number(y),
        Value::Number(radius),
        Value::Number(thickness),
    ] = args
    {
        // draw_circle_lines(*x, *y, *radius, *thickness, state.sprite.draw_color);
        Ok(Value::Null)
    } else {
        Err("hcircle() requires four number arguments: x, y, radius, thickness".to_string())
    }
}

pub fn ellipse(state: &State, args: &[Value]) -> Result {
    match args {
        [
            Value::Number(x),
            Value::Number(y),
            Value::Number(rx),
            Value::Number(ry),
        ] => {
            // draw_ellipse(*x, -*y, *rx, *ry, 0.0, state.sprite.draw_color);
            Ok(Value::Null)
        }
        [
            Value::Number(x),
            Value::Number(y),
            Value::Number(rx),
            Value::Number(ry),
            Value::Number(rotation),
        ] => {
            // draw_ellipse(*x, -*y, *rx, *ry, *rotation, state.sprite.draw_color);
            Ok(Value::Null)
        }
        _ => Err("ellipse() requires four number arguments: x, y, rx, ry, [rotation]".to_string()),
    }
}

pub fn hellipse(state: &State, args: &[Value]) -> Result {
    match args {
        [
            Value::Number(x),
            Value::Number(y),
            Value::Number(rx),
            Value::Number(ry),
            Value::Number(thickness),
        ] => {
            // draw_ellipse_lines(*x, -*y, *rx, *ry, 0.0, *thickness, state.sprite.draw_color);
            Ok(Value::Null)
        }
        [
            Value::Number(x),
            Value::Number(y),
            Value::Number(rx),
            Value::Number(ry),
            Value::Number(rotation),
            Value::Number(thickness),
        ] => {
            // draw_ellipse_lines(*x, -*y, *rx, *ry, *rotation, *thickness, state.sprite.draw_color);
            Ok(Value::Null)
        }
        _ => Err(
            "hellipse() requires five number arguments: x, y, rx, ry, [rotation], thickness"
                .to_string(),
        ),
    }
}

pub fn polygon(state: &State, args: &[Value]) -> Result {
    if let [Value::List(xs), Value::List(ys)] = args {
        if xs.len() != ys.len() {
            return Err(
                "polygon() requires two lists of equal length: x and y coordinates".to_string(),
            );
        }
        let xs = xs.iter().map(|v| v.to_number()).collect::<Vec<f32>>();
        let ys = ys.iter().map(|v| v.to_number()).collect::<Vec<f32>>();
        draw_convex_polygon(&xs, &ys, state.sprite.draw_color);
        Ok(Value::Null)
    } else {
        Err("polygon() requires two lists of numbers: x and y coordinates".to_string())
    }
}

pub fn hpolygon(state: &State, args: &[Value]) -> Result {
    if let [Value::Number(thickness), Value::List(xs), Value::List(ys)] = args {
        if xs.len() != ys.len() {
            return Err(
                "hpolygon() requires two lists of equal length: x and y coordinates".to_string(),
            );
        }
        let xs = xs.iter().map(|v| v.to_number()).collect::<Vec<f32>>();
        let ys = ys.iter().map(|v| v.to_number()).collect::<Vec<f32>>();
        draw_convex_polygon_lines(&xs, &ys, *thickness, state.sprite.draw_color);
        Ok(Value::Null)
    } else {
        Err(
            "hpolygon() requires two lists of numbers and a thickness: x and y coordinates"
                .to_string(),
        )
    }
}

pub fn textured_quad(args: &[Value]) -> Result {
    if let [
        Value::List(parse_image_result),
        Value::Number(x1),
        Value::Number(y1),
        Value::Number(x2),
        Value::Number(y2),
        Value::Number(x3),
        Value::Number(y3),
        Value::Number(x4),
        Value::Number(y4),
    ] = args
    {
        if let [
            Value::Number(width),
            Value::Number(height),
            Value::List(pixels),
        ] = parse_image_result.as_slice()
        {
            // let mut image = Image::gen_image_color(
            //     *width as u16,
            //     *height as u16,
            //     Color::new(0.0, 0.0, 0.0, 0.0),
            // );
            // for i in 0..*width as usize {
            //     for j in 0..*height as usize {
            //         let index = (i + j * (*width as usize)) * 4;
            //         let r = pixels[index].to_number() / 255.0;
            //         let g = pixels[index + 1].to_number() / 255.0;
            //         let b = pixels[index + 2].to_number() / 255.0;
            //         let a = pixels[index + 3].to_number() / 255.0;
            //         image.set_pixel(i as u32, j as u32, Color::new(r, g, b, a));
            //     }
            // }
            // let p1 = vec2(*x1, *y1);
            // let p2 = vec2(*x2, *y2);
            // let p3 = vec2(*x3, *y3);
            // let p4 = vec2(*x4, *y4);
            // let resolution = 128;
            // for i in 0..=resolution {
            //     let t = i as f32 / resolution as f32;

            //     let left = crate::utils::lerp_vec2(p1, p4, t);
            //     let right = crate::utils::lerp_vec2(p2, p3, t);
            //     let uv_left = vec2(0.0, t);
            //     let uv_right = vec2(1.0, t);

            //     for j in 0..=resolution {
            //         let s = j as f32 / resolution as f32;

            //         let pos = crate::utils::lerp_vec2(left, right, s);
            //         let uv = crate::utils::lerp_vec2(uv_left, uv_right, s);

            //         let color = crate::utils::sample_texture(&image, uv);
            //         draw_rectangle(pos.x - 4.0, pos.y - 4.0, 8.0, 8.0, color);
            //     }
            // }
            let mut cpu_texture = CPUTexture::new(*width as u32, *height as u32);
            for i in 0..*width as usize {
                for j in 0..*height as usize {
                    let index = (i + j * (*width as usize)) * 4;
                    let r = pixels[index].to_number() as u8;
                    let g = pixels[index + 1].to_number() as u8;
                    let b = pixels[index + 2].to_number() as u8;
                    let a = pixels[index + 3].to_number() as u8;
                    cpu_texture.set(i as u32, j as u32, U8Vec4::new(r, g, b, a));
                }
            }
            let gpu_texture = cpu_texture.upload_to_gpu();
            let quad = [
                Vertex {
                    position: vec2(*x1, *y1),
                    uv: vec2(0.0, 1.0),
                },
                Vertex {
                    position: vec2(*x2, *y2),
                    uv: vec2(1.0, 1.0),
                },
                Vertex {
                    position: vec2(*x3, *y3),
                    uv: vec2(1.0, 0.0),
                },
                Vertex {
                    position: vec2(*x4, *y4),
                    uv: vec2(0.0, 0.0),
                },
            ];
            let indices = [0, 1, 2, 0, 2, 3];
            let mesh = Mesh::new(&quad, &indices, core::DrawMode::Triangles);
            gpu_texture.bind();
            mesh.draw();
            Ok(Value::Null)
        } else {
            Err("textured_quad() requires an image with width, height, and pixel data".to_string())
        }
    } else {
        Err("textured_quad() requires an image and four pairs of coordinates".to_string())
    }
}

pub fn stamp(state: &mut State) -> Result {
    state.project.stage.stamp_buffer.bind();
    draw_sprite(state.sprite, state.shader_program);
    Framebuffer::unbind();
    Ok(Value::Null)
}

pub fn clear_all_stamps(state: &mut State) -> Result {
    state.project.stage.clear_stamps();
    Ok(Value::Null)
}

pub fn r(state: &State) -> Result {
    Ok(Value::Number(state.sprite.draw_color.x * 255.0))
}

pub fn g(state: &State) -> Result {
    Ok(Value::Number(state.sprite.draw_color.y * 255.0))
}

pub fn b(state: &State) -> Result {
    Ok(Value::Number(state.sprite.draw_color.z * 255.0))
}

pub fn a(state: &State) -> Result {
    Ok(Value::Number(state.sprite.draw_color.w * 255.0))
}
