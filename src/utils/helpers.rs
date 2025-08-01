use glam::*;
use glfw::{Key, MouseButton};

use crate::utils::{core::*, *};

// Helper functions!

pub fn resolve_expression(expr: &Expression, state: &mut State) -> Value {
    match expr {
        Expression::Value(v) => v.clone(),
        Expression::List(l) => {
            let mut list = vec![];
            for element in l {
                list.push(resolve_expression(element, state));
            }
            Value::List(list)
        }
        Expression::Object(o) => {
            let mut object = std::collections::HashMap::new();
            for (key, value) in o {
                let resolved_value = resolve_expression(value, state);
                object.insert(key.clone(), resolved_value);
            }
            Value::Object(object)
        }
        Expression::ListMemberAccess { list, index } => {
            let index = resolve_expression(index, state);
            let list = resolve_expression(list, state);
            if let Value::List(list) = list {
                if let Value::Number(index) = index {
                    if index >= 0.0 && index < list.len() as f32 {
                        return list[index as usize].clone();
                    } else {
                        return Value::Null;
                    }
                } else {
                    return Value::Null;
                }
            } else if let Value::Object(object) = list {
                if let Value::String(ref key) = index {
                    if let Some(value) = object.get(key) {
                        return value.clone();
                    } else {
                        return Value::Null;
                    }
                } else {
                    return Value::Null;
                }
            } else {
                return Value::Null;
            }
        }
        Expression::Identifier(id) => state
            .sprite
            .variable(id, state.project, state.local_vars)
            .clone(),
        Expression::PostIncrement(id) => {
            let value = state
                .sprite
                .variable(id, state.project, state.local_vars)
                .clone();
            if let Value::Number(num) = value {
                let new_value = Value::Number(num + 1.0);
                state.sprite.set_variable(id, new_value);
                return value;
            }
            Value::Null
        }
        Expression::PreIncrement(id) => {
            let value = state
                .sprite
                .variable(id, state.project, state.local_vars)
                .clone();
            if let Value::Number(num) = value {
                let new_value = Value::Number(num + 1.0);
                state.sprite.set_variable(id, new_value.clone());
                return new_value;
            }
            Value::Null
        }
        Expression::PostDecrement(id) => {
            let value = state
                .sprite
                .variable(id, state.project, state.local_vars)
                .clone();
            if let Value::Number(num) = value {
                let new_value = Value::Number(num - 1.0);
                state.sprite.set_variable(id, new_value);
                return value;
            }
            Value::Null
        }
        Expression::PreDecrement(id) => {
            let value = state
                .sprite
                .variable(id, state.project, state.local_vars)
                .clone();
            if let Value::Number(num) = value {
                let new_value = Value::Number(num - 1.0);
                state.sprite.set_variable(id, new_value.clone());
                return new_value;
            }
            Value::Null
        }
        Expression::Binary {
            left,
            right,
            operator,
        } => {
            let left_value = resolve_expression(left, state);
            let right_value = resolve_expression(right, state);
            match operator.as_str() {
                "+" => Value::Number(left_value.to_number() + right_value.to_number()),
                "-" => Value::Number(left_value.to_number() - right_value.to_number()),
                "*" => Value::Number(left_value.to_number() * right_value.to_number()),
                "/" => Value::Number(left_value.to_number() / right_value.to_number()),
                "%" => Value::Number(left_value.to_number() % right_value.to_number()),
                "**" => Value::Number(left_value.to_number().powf(right_value.to_number())),
                "==" => Value::Boolean(left_value == right_value),
                "!=" => Value::Boolean(left_value != right_value),
                "<" => Value::Boolean(left_value.to_number() < right_value.to_number()),
                ">" => Value::Boolean(left_value.to_number() > right_value.to_number()),
                "<=" => Value::Boolean(left_value.to_number() <= right_value.to_number()),
                ">=" => Value::Boolean(left_value.to_number() >= right_value.to_number()),
                "&&" => Value::Boolean(left_value.to_boolean() && right_value.to_boolean()),
                "||" => Value::Boolean(left_value.to_boolean() || right_value.to_boolean()),
                "in" => Value::Boolean(right_value.to_list().contains(&left_value)),
                ".." => Value::String(format!(
                    "{}{}",
                    left_value.to_string(),
                    right_value.to_string()
                )),
                "^" => Value::Number(
                    (left_value.to_number() as u32 ^ right_value.to_number() as u32) as f32,
                ),
                "&" => Value::Number(
                    (left_value.to_number() as u32 & right_value.to_number() as u32) as f32,
                ),
                "|" => Value::Number(
                    (left_value.to_number() as u32 | right_value.to_number() as u32) as f32,
                ),
                "<<" => Value::Number(
                    ((left_value.to_number() as u32) << right_value.to_number() as u32) as f32,
                ),
                ">>" => Value::Number(
                    (left_value.to_number() as u32 >> right_value.to_number() as u32) as f32,
                ),
                _ => panic!("Unknown operator: {}", operator),
            }
        }
        Expression::Unary { operator, operand } => {
            let operand_value = resolve_expression(operand, state);
            match operator.as_str() {
                "-" => Value::Number(-operand_value.to_number()),
                "!" => Value::Boolean(!operand_value.to_boolean()),
                _ => panic!("Unknown operator: {}", operator),
            }
        }
        Expression::Call { function, args } => {
            let args = args
                .iter()
                .map(|arg| resolve_expression(arg, state))
                .collect::<Vec<_>>();
            if let Some(function_struct) = state.sprite.functions.clone().get(function) {
                function_struct.call(state, &args).unwrap_or_else(|e| {
                    println!("Error calling function {}(): {}", function, e);
                    Value::Null
                })
            } else if let Some(variable) = state.sprite.variables.get(function).cloned() {
                let Value::Closure(closure) = variable else {
                    println!("Variable '{}' is not a function", function);
                    return Value::Null;
                };
                let function_struct = &*closure;
                Callable::Function(function_struct.clone())
                    .call(state, &args)
                    .unwrap_or_else(|e| {
                        println!("Error calling function '{}': {}", function, e);
                        Value::Null
                    })
            } else {
                return Value::Null;
            }
        }
    }
}

pub fn evaluate_bezier(t: f32, ctrl1_y: f32, ctrl2_y: f32) -> f32 {
    let steps = 20;
    let mut closest_y = 0.0;
    let mut min_diff = f32::MAX;

    for i in 0..=steps {
        let guess = i as f32 / steps as f32;
        let x = cubic_bezier(guess, 0.0, ctrl1_y, ctrl2_y, 1.0);
        let diff = (x - t).abs();

        if diff < min_diff {
            min_diff = diff;
            closest_y = cubic_bezier(guess, 0.0, ctrl1_y, ctrl2_y, 1.0);
        }
    }

    closest_y
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

pub fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    Vec2::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t))
}

pub fn flatten(pixels: Vec<[u8; 4]>) -> Vec<u8> {
    let mut flat = vec![0; pixels.len() * 4];
    for (i, pixel) in pixels.iter().enumerate() {
        flat[i * 4] = pixel[0];
        flat[i * 4 + 1] = pixel[1];
        flat[i * 4 + 2] = pixel[2];
        flat[i * 4 + 3] = pixel[3];
    }
    flat
}

pub fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

pub fn string_to_mouse(s: &str) -> Option<MouseButton> {
    match s.to_lowercase().as_str() {
        "left" => Some(MouseButton::Left),
        "right" => Some(MouseButton::Right),
        "middle" => Some(MouseButton::Middle),
        _ => None,
    }
}

pub fn string_to_keycode(s: &str) -> Option<Key> {
    use Key::*;
    match s.to_lowercase().as_str() {
        "a" => Some(A),
        "b" => Some(B),
        "c" => Some(C),
        "d" => Some(D),
        "e" => Some(E),
        "f" => Some(F),
        "g" => Some(G),
        "h" => Some(H),
        "i" => Some(I),
        "j" => Some(J),
        "k" => Some(K),
        "l" => Some(L),
        "m" => Some(M),
        "n" => Some(N),
        "o" => Some(O),
        "p" => Some(P),
        "q" => Some(Q),
        "r" => Some(R),
        "s" => Some(S),
        "t" => Some(T),
        "u" => Some(U),
        "v" => Some(V),
        "w" => Some(W),
        "x" => Some(X),
        "y" => Some(Y),
        "z" => Some(Z),

        "0" => Some(Num0),
        "1" => Some(Num1),
        "2" => Some(Num2),
        "3" => Some(Num3),
        "4" => Some(Num4),
        "5" => Some(Num5),
        "6" => Some(Num6),
        "7" => Some(Num7),
        "8" => Some(Num8),
        "9" => Some(Num9),

        "`" => Some(GraveAccent),
        "-" => Some(Minus),
        "=" => Some(Equal),
        "(" => Some(LeftBracket),
        ")" => Some(RightBracket),
        "\\" => Some(Backslash),
        ";" => Some(Semicolon),
        "'" => Some(Apostrophe),
        "," => Some(Comma),
        "." => Some(Period),
        "/" => Some(Slash),

        "lctrl" => Some(LeftControl),
        "rctrl" => Some(RightControl),
        "lshift" => Some(LeftShift),
        "rshift" => Some(RightShift),
        "lalt" => Some(LeftAlt),
        "ralt" => Some(RightAlt),
        "lsuper" => Some(LeftSuper),
        "rsuper" => Some(RightSuper),
        "tab" => Some(Tab),
        "del" => Some(Delete),
        "back" => Some(Backspace),

        "left" => Some(Left),
        "right" => Some(Right),
        "up" => Some(Up),
        "down" => Some(Down),
        "space" => Some(Space),
        "enter" => Some(Enter),
        "escape" => Some(Escape),

        _ => None,
    }
}

pub fn draw_line(start: Vec2, end: Vec2, thickness: f32, shader: &ShaderProgram, color: Vec4) {
    let direction = (end - start).normalize();
    let length = (end - start).length();
    let angle = direction.y.atan2(direction.x);
    let half_thickness = thickness / 2.0;
    let vertices = [
        Vertex {
            position: Vec2::new(0.0, -half_thickness),
            uv: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec2::new(length, -half_thickness),
            uv: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec2::new(length, half_thickness),
            uv: Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: Vec2::new(0.0, half_thickness),
            uv: Vec2::new(0.0, 0.0),
        },
    ];
    let indices = [0, 1, 2, 0, 2, 3];
    let mesh = Mesh::new(&vertices, &indices, core::DrawMode::Triangles);
    let mut m = Mat4::from_translation(start.extend(0.0));
    m = m * Mat4::from_rotation_z(angle);
    let texture = CPUTexture::new_filled(1, 1, [255; 4]).upload_to_gpu();
    shader.use_program();
    shader.set_uniform("u_color", color);
    shader.set_uniform("u_model", m);
    texture.bind();
    mesh.draw();
}

pub fn draw_rectangle(start: Vec2, end: Vec2, shader: &ShaderProgram, color: Vec4) {
    let vertices = [
        Vertex {
            position: Vec2::new(start.x, start.y),
            uv: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec2::new(end.x, start.y),
            uv: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec2::new(end.x, end.y),
            uv: Vec2::new(1.0, 0.0),
        },
        Vertex {
            position: Vec2::new(start.x, end.y),
            uv: Vec2::new(0.0, 0.0),
        },
    ];
    let indices = [0, 1, 2, 0, 2, 3];
    let mesh = Mesh::new(&vertices, &indices, core::DrawMode::Triangles);
    let texture = CPUTexture::new_filled(1, 1, [255; 4]).upload_to_gpu();
    shader.use_program();
    shader.set_uniform("u_color", color);
    shader.set_uniform("u_model", Mat4::IDENTITY);
    texture.bind();
    mesh.draw();
}

pub fn draw_convex_polygon(xs: &Vec<f32>, ys: &Vec<f32>, shader: &ShaderProgram, color: Vec4) {
    assert_eq!(xs.len(), ys.len());
    assert!(xs.len() >= 3, "Need at least 3 points to form a polygon!");

    let vertices: Vec<Vertex> = xs
        .iter()
        .zip(ys.iter())
        .map(|(&x, &y)| Vertex {
            position: Vec2::new(x, y),
            uv: Vec2::new(0.0, 0.0),
        })
        .collect();
    let indices = trianglulate_polygon(&vertices.iter().map(|v| v.position).collect());
    let mesh = Mesh::new(&vertices, &indices, core::DrawMode::Triangles);
    let texture = CPUTexture::new_filled(1, 1, [255; 4]).upload_to_gpu();
    shader.use_program();
    shader.set_uniform("u_color", color);
    shader.set_uniform("u_model", Mat4::IDENTITY);
    texture.bind();
    mesh.draw();
}

pub fn draw_convex_polygon_lines(
    xs: &Vec<f32>,
    ys: &Vec<f32>,
    thickness: f32,
    shader: &ShaderProgram,
    color: Vec4,
) {
    assert_eq!(xs.len(), ys.len());

    for i in 0..xs.len() {
        let next_i = (i + 1) % xs.len();
        let start = Vec2::new(xs[i], ys[i]);
        let end = Vec2::new(xs[next_i], ys[next_i]);
        draw_line(start, end, thickness, shader, color);
    }
}

// Helper functions that help other helper functions!!
fn cubic_bezier(t: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
    let u = 1.0 - t;
    u * u * u * p0 + 3.0 * u * u * t * p1 + 3.0 * u * t * t * p2 + t * t * t * p3
}

fn trianglulate_polygon(vertices: &Vec<Vec2>) -> Vec<u32> {
    let mut indices = Vec::new();
    for i in 1..vertices.len() - 1 {
        indices.push(0);
        indices.push(i as u32);
        indices.push((i + 1) as u32);
    }
    indices
}
