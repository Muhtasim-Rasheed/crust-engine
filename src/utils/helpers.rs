use glam::*;
use glfw::{Key, MouseButton};

use crate::utils::{core::*, Callable, Expression, Function, State, Value};

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
        Expression::Closure {
            args,
            body,
            returns,
        } => {
            let closure = Function {
                args: args.clone(),
                body: body.clone(),
                returns: *returns.clone(),
                captured_vars: state.local_vars.to_vec(),
            };
            Value::Closure(Box::new(Callable::Function(closure)))
        }
        Expression::MemberAccess { object, key } => {
            let key = resolve_expression(key, state);
            let object = resolve_expression(object, state);
            if let Value::List(list) = object {
                if let Value::Number(key) = key {
                    if key >= 0.0 && key < list.len() as f32 {
                        return list[key as usize].clone();
                    } else {
                        return Value::Null;
                    }
                } else {
                    return Value::Null;
                }
            } else if let Value::Object(object) = object {
                if let Value::String(ref key) = key {
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
            // evaluate the function expression
            let func_val = resolve_expression(function, state);

            // evaluate args
            let args = args
                .iter()
                .map(|arg| resolve_expression(arg, state))
                .collect::<Vec<_>>();

            match func_val {
                Value::Closure(callable) => {
                    callable.call(state, &args).unwrap_or_else(|e| {
                        println!("Error calling function: {}", e);
                        Value::Null
                    })
                }
                _ => {
                    println!("Attempted to call non-function: {:?}", func_val);
                    Value::Null
                }
            }
        }
    }
}

pub fn assign_expression(expr: &Expression, value: Value, state: &mut State, is_global: bool) -> Result<(), String> {
    match expr {
        Expression::Identifier(id) => {
            if is_global {
                state.project.global_variables.insert(id.clone(), value);
            } else {
                if state.sprite.variables.get(id).is_some() {
                    state.sprite.set_variable(id, value);
                } else {
                    state.sprite.new_variable(id, value);
                }
            }
            Ok(())
        }
        Expression::MemberAccess { object, key } => {
            let key_val = resolve_expression(key, state);
            let object = get_mut_container(object, state, is_global)?;
            match (object, key_val) {
                (Value::List(list), Value::Number(idx)) => { list[idx as usize] = value; Ok(()) }
                (Value::Object(obj), Value::String(s)) => {
                    obj.insert(s.clone(), value);
                    Ok(())
                }
                _ => Err("Invalid assignment target".into()),
            }
        }
        _ => Err("Invalid assignment target".into()),
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

        "np0" => Some(Kp0),
        "np1" => Some(Kp1),
        "np2" => Some(Kp2),
        "np3" => Some(Kp3),
        "np4" => Some(Kp4),
        "np5" => Some(Kp5),
        "np6" => Some(Kp6),
        "np7" => Some(Kp7),
        "np8" => Some(Kp8),
        "np9" => Some(Kp9),

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

        "f1" => Some(F1),
        "f2" => Some(F2),
        "f3" => Some(F3),
        "f4" => Some(F4),
        "f5" => Some(F5),
        "f6" => Some(F6),
        "f7" => Some(F7),
        "f8" => Some(F8),
        "f9" => Some(F9),
        "f10" => Some(F10),
        "f11" => Some(F11),
        "f12" => Some(F12),
        "f13" => Some(F13),
        "f14" => Some(F14),
        "f15" => Some(F15),
        "f16" => Some(F16),
        "f17" => Some(F17),
        "f18" => Some(F18),
        "f19" => Some(F19),
        "f20" => Some(F20),
        "f21" => Some(F21),
        "f22" => Some(F22),
        "f23" => Some(F23),
        "f24" => Some(F24),
        "f25" => Some(F25),

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

pub fn keycode_to_string(key: Key) -> String {
    use Key::*;
    match key {
        A => "a".to_string(),
        B => "b".to_string(),
        C => "c".to_string(),
        D => "d".to_string(),
        E => "e".to_string(),
        F => "f".to_string(),
        G => "g".to_string(),
        H => "h".to_string(),
        I => "i".to_string(),
        J => "j".to_string(),
        K => "k".to_string(),
        L => "l".to_string(),
        M => "m".to_string(),
        N => "n".to_string(),
        O => "o".to_string(),
        P => "p".to_string(),
        Q => "q".to_string(),
        R => "r".to_string(),
        S => "s".to_string(),
        T => "t".to_string(),
        U => "u".to_string(),
        V => "v".to_string(),
        W => "w".to_string(),
        X => "x".to_string(),
        Y => "y".to_string(),
        Z => "z".to_string(),

        Num0 => "0".to_string(),
        Num1 => "1".to_string(),
        Num2 => "2".to_string(),
        Num3 => "3".to_string(),
        Num4 => "4".to_string(),
        Num5 => "5".to_string(),
        Num6 => "6".to_string(),
        Num7 => "7".to_string(),
        Num8 => "8".to_string(),
        Num9 => "9".to_string(),

        Kp0 => "np0".to_string(),
        Kp1 => "np1".to_string(),
        Kp2 => "np2".to_string(),
        Kp3 => "np3".to_string(),
        Kp4 => "np4".to_string(),
        Kp5 => "np5".to_string(),
        Kp6 => "np6".to_string(),
        Kp7 => "np7".to_string(),
        Kp8 => "np8".to_string(),
        Kp9 => "np9".to_string(),

        GraveAccent => "`".to_owned(),
        Minus => "-".to_owned(),
        Equal => "=".to_owned(),
        LeftBracket => "(".to_owned(),
        RightBracket => ")".to_owned(),
        Backslash => "\\".to_owned(),
        Semicolon => ";".to_owned(),
        Apostrophe => "'".to_owned(),
        Comma => ",".to_owned(),
        Period => ".".to_owned(),
        Slash => "/".to_owned(),

        LeftControl => "lctrl".to_owned(),
        RightControl => "rctrl".to_owned(),
        LeftShift => "lshift".to_owned(),
        RightShift => "rshift".to_owned(),
        LeftAlt => "lalt".to_owned(),
        RightAlt => "ralt".to_owned(),
        LeftSuper => "lsuper".to_owned(),
        RightSuper => "rsuper".to_owned(),
        Tab => "tab".to_owned(),
        Delete => "del".to_owned(),
        Backspace => "back".to_owned(),

        F1 => "f1".to_owned(),
        F2 => "f2".to_owned(),
        F3 => "f3".to_owned(),
        F4 => "f4".to_owned(),
        F5 => "f5".to_owned(),
        F6 => "f6".to_owned(),
        F7 => "f7".to_owned(),
        F8 => "f8".to_owned(),
        F9 => "f9".to_owned(),
        F10 => "f10".to_owned(),
        F11 => "f11".to_owned(),
        F12 => "f12".to_owned(),
        F13 => "f13".to_owned(),
        F14 => "f14".to_owned(),
        F15 => "f15".to_owned(),
        F16 => "f16".to_owned(),
        F17 => "f17".to_owned(),
        F18 => "f18".to_owned(),
        F19 => "f19".to_owned(),
        F20 => "f20".to_owned(),
        F21 => "f21".to_owned(),
        F22 => "f22".to_owned(),
        F23 => "f23".to_owned(),
        F24 => "f24".to_owned(),
        F25 => "f25".to_owned(),

        Left => "left".to_owned(),
        Right => "right".to_owned(),
        Up => "up".to_owned(),
        Down => "down".to_owned(),
        Space => "space".to_owned(),
        Enter => "enter".to_owned(),
        Escape => "escape".to_owned(),
        _ => "unknown".to_owned(),
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
    let mesh = Mesh::new(&vertices, &indices, DrawMode::Triangles);
    let mut m = Mat4::from_translation(start.extend(0.0));
    m = m * Mat4::from_rotation_z(angle);
    let texture = CPUTexture::new_filled(1, 1, [255; 4]).upload_to_gpu();
    shader.use_program();
    shader.set_uniform("u_color", color);
    shader.set_uniform("u_model", m);
    shader.set_uniform("u_effects", &[] as &[i32]);
    shader.set_uniform("u_effect_values", &[] as &[f32]);
    shader.set_uniform("u_effects_count", 0);
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
    let mesh = Mesh::new(&vertices, &indices, DrawMode::Triangles);
    let texture = CPUTexture::new_filled(1, 1, [255; 4]).upload_to_gpu();
    shader.use_program();
    shader.set_uniform("u_color", color);
    shader.set_uniform("u_model", Mat4::IDENTITY);
    shader.set_uniform("u_effects", &[] as &[i32]);
    shader.set_uniform("u_effect_values", &[] as &[f32]);
    shader.set_uniform("u_effects_count", 0);
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
    let mesh = Mesh::new(&vertices, &indices, DrawMode::Triangles);
    let texture = CPUTexture::new_filled(1, 1, [255; 4]).upload_to_gpu();
    shader.use_program();
    shader.set_uniform("u_color", color);
    shader.set_uniform("u_model", Mat4::IDENTITY);
    shader.set_uniform("u_effects", &[] as &[i32]);
    shader.set_uniform("u_effect_values", &[] as &[f32]);
    shader.set_uniform("u_effects_count", 0);
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

pub fn percentage_to_decibels(percentage: f32) -> f32 {
    if percentage <= 0.0 {
        return f32::NEG_INFINITY;
    }
    20.0 * percentage.log10()
}

// Helper functions that help other helper functions!!
fn get_mut_container<'a>(
    expr: &'a Expression,
    state: &'a mut State,
    is_global: bool,
) -> Result<&'a mut Value, String> {
    match expr {
        Expression::Identifier(id) => {
            if is_global {
                state.project.global_variables.get_mut(id)
                    .ok_or_else(|| format!("Global variable '{}' not found", id))
            } else {
                state.sprite.variable_mut(id, state.project, state.local_vars)
                    .ok_or_else(|| format!("Variable '{}' not found", id))
            }
        }

        Expression::MemberAccess { object, key } => {
            let key_val = crate::utils::resolve_expression(key, state); // resolve key first
            let container = get_mut_container(object, state, is_global)?; // now borrow mutable
            match (container, key_val) {
                (Value::List(list), Value::Number(idx)) => {
                    let idx = idx as usize;
                    if idx >= list.len() {
                        list.resize(idx + 1, Value::Null);
                    }
                    Ok(&mut list[idx])
                }
                (Value::Object(obj), Value::String(s)) => {
                    Ok(obj.entry(s).or_insert(Value::Null))
                }
                _ => Err("Invalid member access target".into()),
            }
        }

        _ => Err("Expression is not a valid container".into()),
    }
}

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
