use std::{
    fs::File,
    path::Path,
    io::{
        Read,
        Write,
    },
};

use macroquad::prelude::*;

use super::{Expression, Function, Project, Sprite, SpriteSnapshot, Value};

// Helper functions!

pub fn resolve_expression(expr: &Expression, project: &mut Project, sprite: &mut Sprite, local_vars: &[(String, Value)], snapshots: &[SpriteSnapshot], camera: &Camera2D) -> Value {
    match expr {
        Expression::Value(v) => v.clone(),
        Expression::List(l) => {
            let mut list = vec![];
            for element in l {
                list.push(resolve_expression(element, project, sprite, local_vars, snapshots, camera));
            }
            Value::List(list)
        }
        Expression::Object(o) => {
            let mut object = std::collections::HashMap::new();
            for (key, value) in o {
                let resolved_value = resolve_expression(value, project, sprite, local_vars, snapshots, camera);
                object.insert(key.clone(), resolved_value);
            }
            Value::Object(object)
        }
        Expression::ListMemberAccess { list, index } => {
            let index = resolve_expression(index, project, sprite, local_vars, snapshots, camera);
            let list = resolve_expression(list, project, sprite, local_vars, snapshots, camera);
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
        Expression::Binary { left, right, operator } => {
            let left_value = resolve_expression(left, project, sprite, local_vars, snapshots, camera);
            let right_value = resolve_expression(right, project, sprite, local_vars, snapshots, camera);
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
            let operand_value = resolve_expression(operand, project, sprite, local_vars, snapshots, camera);
            match operator.as_str() {
                "-" => Value::Number(-operand_value.to_number()),
                "!" => Value::Boolean(!operand_value.to_boolean()),
                _ => panic!("Unknown operator: {}", operator),
            }
        }
        Expression::Call { function, args } => {
            let args = args.iter()
                .map(|arg| resolve_expression(arg, project, sprite, local_vars, snapshots, camera))
                .collect::<Vec<_>>();
            match function.as_str() {
                "args" => {
                    let args = std::env::args().collect::<Vec<_>>();
                    Value::List(args.iter().map(|arg| Value::String(arg.clone())).collect())
                }
                "input" => {
                    if let Some(prompt) = args.get(0) {
                        if let Value::String(prompt) = prompt {
                            let mut input = String::new();
                            print!("{} => {} ", sprite.name, prompt);
                            std::io::stdout().flush().unwrap();
                            std::io::stdin().read_line(&mut input).unwrap();
                            input = input.trim().to_string();
                            Value::String(input)
                        } else {
                            Value::Null
                        }
                    } else {
                        Value::Null
                    }
                }
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
                "property_of" => {
                    if let [Value::String(sprite), Value::String(property)] = args.as_slice() {
                        // All sprites are in the snapshots
                        for snapshot in snapshots {
                            if snapshot.name == *sprite {
                                if let Some(value) = snapshot.get(property) {
                                    return value.clone();
                                } else {
                                    println!("Property '{}' not found in sprite '{}'", property, sprite);
                                    return Value::Null;
                                }
                            }
                        }
                        println!("Sprite '{}' not found in snapshots", sprite);
                        return Value::Null;
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
                "len" => {
                    if let [Value::String(s)] = args.as_slice() {
                        Value::Number(s.len() as f32)
                    } else if let [Value::List(l)] = args.as_slice() {
                        Value::Number(l.len() as f32)
                    } else if let [Value::Object(o)] = args.as_slice() {
                        Value::Number(o.len() as f32)
                    } else {
                        Value::Null
                    }
                }
                "keys" => {
                    if let [Value::Object(o)] = args.as_slice() {
                        Value::List(o.keys().map(|k| Value::String(k.clone())).collect())
                    } else {
                        Value::Null
                    }
                }
                "values" => {
                    if let [Value::Object(o)] = args.as_slice() {
                        Value::List(o.values().cloned().collect())
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
                        let path = Path::new(&project.home_path).join(path);
                        let file = File::open(path).expect("Failed to open file");
                        let mut reader = std::io::BufReader::new(file);
                        let mut contents = String::new();
                        reader.read_to_string(&mut contents).expect("Failed to read file");
                        Value::String(contents)
                    } else {
                        Value::Null
                    }
                }
                "import_binary" => {
                    if let [Value::String(path)] = args.as_slice() {
                        let path = Path::new(&project.home_path).join(path);
                        let file = File::open(path).expect("Failed to open file");
                        let mut reader = std::io::BufReader::new(file);
                        let mut contents = Vec::new();
                        reader.read_to_end(&mut contents).expect("Failed to read file");
                        Value::List(contents.iter().map(|&b| Value::Number(b as f32)).collect())
                    } else {
                        Value::Null
                    }
                }
                "parse_image" => {
                    if let [Value::List(contents)] = args.as_slice() {
                        let image = image::load_from_memory(contents.iter().map(|v| v.to_number() as u8).collect::<Vec<u8>>().as_slice()).expect("Failed to load image");
                        let pixels: Vec<Value> = image.to_rgba8().into_raw().iter().map(|&b| Value::Number(b as f32)).collect();
                        Value::List(vec![Value::Number(image.width() as f32), Value::Number(image.height() as f32), Value::List(pixels)])
                    } else {
                        Value::Null
                    }
                }
                "typeof" => {
                    match args.as_slice() {
                        [Value::String(_)] => Value::String("string".to_string()),
                        [Value::Number(_)] => Value::String("number".to_string()),
                        [Value::Boolean(_)] => Value::String("boolean".to_string()),
                        [Value::List(_)] => Value::String("list".to_string()),
                        _ => Value::Null,
                    }
                }
                "push" => {
                    if let [Value::List(list), value] = args.as_slice() {
                        let mut new_list = list.clone();
                        new_list.push(value.clone());
                        Value::List(new_list)
                    } else {
                        Value::Null
                    }
                }
                "pop" => {
                    if let [Value::List(list)] = args.as_slice() {
                        if list.len() > 0 {
                            let mut new_list = list.clone();
                            let popped_value = new_list.pop().unwrap();
                            Value::List(vec![Value::List(new_list), popped_value])
                        } else {
                            Value::Null
                        }
                    } else {
                        Value::Null
                    }
                }
                "insert" => {
                    if let [Value::List(list), Value::Number(index), value] = args.as_slice() {
                        if *index >= 0.0 && *index <= list.len() as f32 {
                            let mut new_list = list.clone();
                            new_list.insert(*index as usize, value.clone());
                            Value::List(new_list)
                        } else {
                            Value::Null
                        }
                    } else {
                        Value::Null
                    }
                }
                "remove" => {
                    if let [Value::List(list), Value::Number(index)] = args.as_slice() {
                        if *index >= 0.0 && *index < list.len() as f32 {
                            let mut new_list = list.clone();
                            let removed_value = new_list.remove(*index as usize);
                            Value::List(vec![Value::List(new_list), removed_value])
                        } else {
                            Value::Null
                        }
                    } else {
                        Value::Null
                    }
                }
                "extend" => {
                    if let [Value::List(list1), Value::List(list2)] = args.as_slice() {
                        let mut new_list = list1.clone();
                        new_list.extend(list2.clone());
                        Value::List(new_list)
                    } else {
                        Value::Null
                    }
                },
                "whoami" => Value::String(sprite.name.clone()),
                // "cloneid" => {
                //     if let Some(clone_id) = sprite.name.split(" (clone ").nth(1) {
                //         if let Some(clone_id) = clone_id.strip_suffix(')') {
                //             Value::Number(clone_id.parse::<f32>().unwrap_or(0.0))
                //         } else {
                //             Value::Null
                //         }
                //     } else {
                //         Value::Null
                //     }
                // }
                "cloneid" => Value::Number(sprite.clone_id.unwrap_or(0) as f32),
                "frame" => Value::Number(get_time() as f32 * 60.0),
                "delta_time" => Value::Number(get_frame_time() as f32),
                "direction" => Value::Number(sprite.direction),
                "x" => Value::Number(sprite.center.x),
                "y" => Value::Number(sprite.center.y),
                "costume" => Value::Number(sprite.costume() as f32),
                "backdrop" => Value::Number(project.stage.backdrop() as f32),
                "size" => Value::List(vec![Value::Number(sprite.size.x), Value::Number(sprite.size.y)]),
                "scale" => Value::Number(sprite.scale * 100.0),
                "bounds" => Value::List(vec![
                    Value::Number(sprite.center.x - sprite.size.x * sprite.scale),
                    Value::Number(sprite.center.y - sprite.size.y * sprite.scale),
                    Value::Number(sprite.center.x + sprite.size.x * sprite.scale),
                    Value::Number(sprite.center.y + sprite.size.y * sprite.scale),
                ]),
                "layer" => Value::Number(sprite.layer as f32),
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
                _ => {
                    if let Some(function_struct) = sprite.functions.clone().get(function) {
                        let Function { args: args_, body, returns } = function_struct;
                        if args_.len() == args.len() {
                            let mut local_vars_: Vec<(String, Value)> = vec![];
                            for (i, arg) in args_.iter().enumerate() {
                                if let Some(arg_value) = args.get(i) {
                                    local_vars_.push((arg.clone(), arg_value.clone()));
                                } else {
                                    println!("Missing argument for function '{}'", function);
                                }
                            }
                            local_vars_.append(&mut local_vars.to_vec());
                            for statement in body {
                                sprite.execute_statement(statement, project, snapshots, camera, &local_vars_);
                            }
                            let returns = resolve_expression(returns, project, sprite, &local_vars_, snapshots, camera);
                            return returns;
                        } else {
                            return Value::Null;
                        }
                    } else {
                        return Value::Null;
                    }
                }
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
    Vec2::new(
        lerp(a.x, b.x, t),
        lerp(a.y, b.y, t),
    )
}

pub fn sample_texture(texture: &Image, uv: Vec2) -> Color {
    let tex_width = texture.width() as usize;
    let tex_height = texture.height() as usize;

    let x = (uv.x * tex_width as f32).clamp(0.0, tex_width as f32 - 1.0) as u32;
    let y = (uv.y * tex_height as f32).clamp(0.0, tex_height as f32 - 1.0) as u32;
    
    texture.get_pixel(x, y)
}

// pub fn compute_resolution(p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2, texture: &Image) -> usize {
//     let width = texture.width();
//     let height = texture.height();

//     let screen_diag = [
//         p1.distance(p2),
//         p2.distance(p3),
//         p3.distance(p4),
//         p4.distance(p1),
//     ]
//     .into_iter()
//     .fold(0.0, f32::max);

//     // One step per screen-space pixel of the longest side (scaled by tex size)
//     let longest_tex_side = width.max(height);
//     (screen_diag * longest_tex_side as f32 / width.max(1) as f32)
//         .clamp(4.0, 512.0) as usize
// }

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

