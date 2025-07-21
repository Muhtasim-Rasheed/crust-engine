use macroquad::prelude::*;

use crate::utils::*;

// Helper functions!

pub fn resolve_expression(
    expr: &Expression,
    project: &mut Project,
    sprite: &mut Sprite,
    local_vars: &[(String, Value)],
    snapshots: &[SpriteSnapshot],
    camera: &Camera2D,
    script_id: usize,
) -> Value {
    match expr {
        Expression::Value(v) => v.clone(),
        Expression::List(l) => {
            let mut list = vec![];
            for element in l {
                list.push(resolve_expression(
                    element, project, sprite, local_vars, snapshots, camera, script_id,
                ));
            }
            Value::List(list)
        }
        Expression::Object(o) => {
            let mut object = std::collections::HashMap::new();
            for (key, value) in o {
                let resolved_value = resolve_expression(
                    value, project, sprite, local_vars, snapshots, camera, script_id,
                );
                object.insert(key.clone(), resolved_value);
            }
            Value::Object(object)
        }
        Expression::ListMemberAccess { list, index } => {
            let index = resolve_expression(
                index, project, sprite, local_vars, snapshots, camera, script_id,
            );
            let list = resolve_expression(
                list, project, sprite, local_vars, snapshots, camera, script_id,
            );
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
        Expression::Identifier(id) => sprite.variable(id, project, local_vars).clone(),
        Expression::PostIncrement(id) => {
            let value = sprite.variable(id, project, local_vars).clone();
            if let Value::Number(num) = value {
                let new_value = Value::Number(num + 1.0);
                sprite.set_variable(id, new_value);
                return value;
            }
            Value::Null
        }
        Expression::PreIncrement(id) => {
            let value = sprite.variable(id, project, local_vars).clone();
            if let Value::Number(num) = value {
                let new_value = Value::Number(num + 1.0);
                sprite.set_variable(id, new_value.clone());
                return new_value;
            }
            Value::Null
        }
        Expression::PostDecrement(id) => {
            let value = sprite.variable(id, project, local_vars).clone();
            if let Value::Number(num) = value {
                let new_value = Value::Number(num - 1.0);
                sprite.set_variable(id, new_value);
                return value;
            }
            Value::Null
        }
        Expression::PreDecrement(id) => {
            let value = sprite.variable(id, project, local_vars).clone();
            if let Value::Number(num) = value {
                let new_value = Value::Number(num - 1.0);
                sprite.set_variable(id, new_value.clone());
                return new_value;
            }
            Value::Null
        }
        Expression::Binary {
            left,
            right,
            operator,
        } => {
            let left_value = resolve_expression(
                left, project, sprite, local_vars, snapshots, camera, script_id,
            );
            let right_value = resolve_expression(
                right, project, sprite, local_vars, snapshots, camera, script_id,
            );
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
            let operand_value = resolve_expression(
                operand, project, sprite, local_vars, snapshots, camera, script_id,
            );
            match operator.as_str() {
                "-" => Value::Number(-operand_value.to_number()),
                "!" => Value::Boolean(!operand_value.to_boolean()),
                _ => panic!("Unknown operator: {}", operator),
            }
        }
        Expression::Call { function, args } => {
            let args = args
                .iter()
                .map(|arg| {
                    resolve_expression(
                        arg, project, sprite, local_vars, snapshots, camera, script_id,
                    )
                })
                .collect::<Vec<_>>();
            if let Some(function_struct) = sprite.functions.clone().get(function) {
                function_struct
                    .call(
                        sprite, project, snapshots, camera, local_vars, script_id, &args,
                    )
                    .unwrap_or_else(|e| {
                        println!("Error calling function {}(): {}", function, e);
                        Value::Null
                    })
            } else if let Some(variable) = sprite.variables.get(function).cloned() {
                let Value::Closure(closure) = variable else {
                    println!("Variable '{}' is not a function", function);
                    return Value::Null;
                };
                let function_struct = &*closure;
                Callable::Function(function_struct.clone())
                    .call(
                        sprite, project, snapshots, camera, local_vars, script_id, &args,
                    )
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

pub fn draw_convex_polygon(xs: &Vec<f32>, ys: &Vec<f32>, color: Color) {
    assert_eq!(xs.len(), ys.len());
    assert!(xs.len() >= 3, "Need at least 3 points to form a polygon!");

    let center_x = xs.iter().sum::<f32>() / xs.len() as f32;
    let center_y = ys.iter().sum::<f32>() / ys.len() as f32;

    for i in 0..xs.len() {
        let next_i = (i + 1) % xs.len();
        draw_triangle(
            Vec2::new(center_x, center_y),
            Vec2::new(xs[i], ys[i]),
            Vec2::new(xs[next_i], ys[next_i]),
            color,
        );
    }
}

pub fn draw_convex_polygon_lines(xs: &Vec<f32>, ys: &Vec<f32>, thickness: f32, color: Color) {
    assert_eq!(xs.len(), ys.len());

    for i in 0..xs.len() {
        let next_i = (i + 1) % xs.len();
        draw_line(xs[i], ys[i], xs[next_i], ys[next_i], thickness, color);
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

pub fn sample_texture(texture: &Image, uv: Vec2) -> Color {
    let tex_width = texture.width() as usize;
    let tex_height = texture.height() as usize;

    let x = (uv.x * tex_width as f32).clamp(0.0, tex_width as f32 - 1.0) as u32;
    let y = (uv.y * tex_height as f32).clamp(0.0, tex_height as f32 - 1.0) as u32;

    texture.get_pixel(x, y)
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

pub fn string_to_keycode(s: &str) -> Option<KeyCode> {
    use KeyCode::*;
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

        "0" => Some(Key0),
        "1" => Some(Key1),
        "2" => Some(Key2),
        "3" => Some(Key3),
        "4" => Some(Key4),
        "5" => Some(Key5),
        "6" => Some(Key6),
        "7" => Some(Key7),
        "8" => Some(Key8),
        "9" => Some(Key9),

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

// Helper functions that help other helper functions!!
fn cubic_bezier(t: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
    let u = 1.0 - t;
    u * u * u * p0 + 3.0 * u * u * t * p1 + 3.0 * u * t * t * p2 + t * t * t * p3
}
