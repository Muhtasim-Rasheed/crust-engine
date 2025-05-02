use std::{fs::File, path::Path, io::Read};

use macroquad::prelude::*;

use super::{Expression, Project, Sprite, Value};

// Helper functions!

pub fn resolve_expression(expr: &Expression, project: &Project, sprite: &Sprite, local_vars: &[(String, Value)]) -> Value {
    match expr {
        Expression::Value(v) => v.clone(),
        Expression::Identifier(id) => sprite.variable(id, project, local_vars).clone(),
        Expression::Binary { left, right, operator } => {
            let left_value = resolve_expression(left, project, sprite, local_vars);
            let right_value = resolve_expression(right, project, sprite, local_vars);
            match operator.as_str() {
                "+" => Value::Number(left_value.to_number() + right_value.to_number()),
                "-" => Value::Number(left_value.to_number() - right_value.to_number()),
                "*" => Value::Number(left_value.to_number() * right_value.to_number()),
                "/" => Value::Number(left_value.to_number() / right_value.to_number()),
                "%" => Value::Number(left_value.to_number() % right_value.to_number()),
                "^" => Value::Number(left_value.to_number().powf(right_value.to_number())),
                "==" => Value::Boolean(left_value == right_value),
                "!=" => Value::Boolean(left_value != right_value),
                "<" => Value::Boolean(left_value.to_number() < right_value.to_number()),
                ">" => Value::Boolean(left_value.to_number() > right_value.to_number()),
                "<=" => Value::Boolean(left_value.to_number() <= right_value.to_number()),
                ">=" => Value::Boolean(left_value.to_number() >= right_value.to_number()),
                "&&" => Value::Boolean(left_value.to_boolean() && right_value.to_boolean()),
                "||" => Value::Boolean(left_value.to_boolean() || right_value.to_boolean()),
                _ => panic!("Unknown operator: {}", operator),
            }
        }
        Expression::Unary { operator, operand } => {
            let operand_value = resolve_expression(operand, project, sprite, local_vars);
            match operator.as_str() {
                "-" => Value::Number(-operand_value.to_number()),
                "!" => Value::Boolean(!operand_value.to_boolean()),
                _ => panic!("Unknown operator: {}", operator),
            }
        }
        Expression::Call { function, args } => {
            let args = args.iter()
                .map(|arg| resolve_expression(arg, project, sprite, local_vars))
                .collect::<Vec<_>>();
            match function.as_str() {
                "time" => Value::Number(get_time() as f32),
                "concat" => {
                    let mut result = String::new();
                    for arg in args {
                        result.push_str(&arg.to_string());
                    }
                    Value::String(result)
                }
                "abs" => Value::Number(args[0].to_number().abs()),
                "sqrt" => Value::Number(args[0].to_number().sqrt()),
                "sin" => Value::Number(args[0].to_number().sin()),
                "cos" => Value::Number(args[0].to_number().cos()),
                "tan" => Value::Number(args[0].to_number().tan()),
                "asin" => Value::Number(args[0].to_number().asin()),
                "acos" => Value::Number(args[0].to_number().acos()),
                "atan" => Value::Number(args[0].to_number().atan()),
                "lerp" => {
                    if let [Value::Number(a), Value::Number(b), Value::Number(t)] = args.as_slice() {
                        Value::Number(lerp(*a, *b, *t))
                    } else {
                        Value::Null
                    }
                }
                "to_rad" => {
                    if let [Value::Number(deg)] = args.as_slice() {
                        Value::Number(deg.to_radians())
                    } else {
                        Value::Null
                    }
                }
                "to_deg" => {
                    if let [Value::Number(rad)] = args.as_slice() {
                        Value::Number(rad.to_degrees())
                    } else {
                        Value::Null
                    }
                }
                "clamp" => {
                    if let [Value::Number(value), Value::Number(min), Value::Number(max)] = args.as_slice() {
                        Value::Number(clamp(*value, *min, *max))
                    } else {
                        Value::Null
                    }
                }
                "of" => {
                    if let [Value::Number(index), Value::List(list)] = args.as_slice() {
                        if let Some(value) = list.get(*index as usize) {
                            value.clone()
                        } else {
                            Value::Null
                        }
                    } else {
                        Value::Null
                    }
                }
                "random" => {
                    if let [Value::Number(min), Value::Number(max)] = args.as_slice() {
                        Value::Number(rand::gen_range(*min, *max))
                    } else {
                        Value::Null
                    }
                }
                "distance" => {
                    if let [Value::Number(x1), Value::Number(y1), Value::Number(x2), Value::Number(y2)] = args.as_slice() {
                        Value::Number(((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt())
                    } else {
                        Value::Null
                    }
                }
                "distance_to" => {
                    if let [Value::Number(x), Value::Number(y)] = args.as_slice() {
                        Value::Number(((sprite.center.x - x).powi(2) + (sprite.center.y - y).powi(2)).sqrt())
                    } else {
                        Value::Null
                    }
                }
                "import" => {
                    if let [Value::String(path)] = args.as_slice() {
                        let path = Path::new(&project.export_path).join(path);
                        let file = File::open(path).expect("Failed to open file");
                        let mut reader = std::io::BufReader::new(file);
                        let mut contents = String::new();
                        reader.read_to_string(&mut contents).expect("Failed to read file");
                        Value::String(contents)
                    } else {
                        Value::Null
                    }
                }
                "direction" => Value::Number(sprite.direction),
                "x" => Value::Number(sprite.center.x),
                "y" => Value::Number(sprite.center.y),
                "costume" => Value::Number(sprite.costume() as f32),
                "backdrop" => Value::Number(project.stage.backdrop() as f32),
                "size" => Value::Number(sprite.scale * 100.0),
                "effect" => {
                    if let [Value::String(effect)] = args.as_slice() {
                        Value::Number(sprite.effects.get(effect).cloned().unwrap_or(0.0))
                    } else {
                        Value::Number(0.0)
                    }
                }
                "sound_effect" => {
                    if let [Value::String(effect)] = args.as_slice() {
                        Value::Number(sprite.sound_effects.get(effect).cloned().unwrap_or(0.0))
                    } else {
                        Value::Number(0.0)
                    }
                }
                "key_down" => {
                    if let [Value::String(key)] = args.as_slice() {
                        Value::Boolean(is_key_down(string_to_keycode(key).unwrap_or(KeyCode::Unknown)))
                    } else {
                        Value::Null
                    }
                }
                "key_pressed" => {
                    if let [Value::String(key)] = args.as_slice() {
                        Value::Boolean(is_key_pressed(string_to_keycode(key).unwrap_or(KeyCode::Unknown)))
                    } else {
                        Value::Null
                    }
                }
                "key_released" => {
                    if let [Value::String(key)] = args.as_slice() {
                        Value::Boolean(is_key_released(string_to_keycode(key).unwrap_or(KeyCode::Unknown)))
                    } else {
                        Value::Null
                    }
                }
                "did_get_clicked" => {
                    let xy = mouse_position();
                    let top_left = sprite.center - vec2(sprite.size.x * sprite.scale, sprite.size.y * sprite.scale);
                    let bottom_right = sprite.center + vec2(sprite.size.x * sprite.scale, sprite.size.y * sprite.scale);
                    let rect = Rect::new(top_left.x, top_left.y, top_left.x - bottom_right.x, top_left.y - bottom_right.y);
                    if rect.contains(xy.into()) {
                        Value::Boolean(true)
                    } else {
                        Value::Boolean(false)
                    }
                }
                "mouse_x" => Value::Number(mouse_position().0 * 2.0 - screen_width()),
                "mouse_y" => Value::Number(mouse_position().1 * 2.0 - screen_height()),
                "is_broadcasted" => {
                    if let [Value::String(broadcast)] = args.as_slice() {
                        Value::Boolean(project.broadcasted_message.is_some() && project.broadcasted_message.as_ref().unwrap() == broadcast)
                    } else {
                        Value::Null
                    }
                }
                "is_backdrop" => {
                    if let [Value::Number(backdrop)] = args.as_slice() {
                        Value::Boolean(project.stage.backdrop() == *backdrop as usize)
                    } else {
                        Value::Null
                    }
                }
                "r" => Value::Number(sprite.draw_color.r),
                "g" => Value::Number(sprite.draw_color.g),
                "b" => Value::Number(sprite.draw_color.b),
                "window_width" => Value::Number(screen_width()),
                "window_height" => Value::Number(screen_height()),
                _ => Value::Null
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

// Helper functions that help other helper functions!!
fn cubic_bezier(t: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
    let u = 1.0 - t;
    u*u*u*p0 + 3.0*u*u*t*p1 + 3.0*u*t*t*p2 + t*t*t*p3
}

fn string_to_keycode(s: &str) -> Option<KeyCode> {
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

