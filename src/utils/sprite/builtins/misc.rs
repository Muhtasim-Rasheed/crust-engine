use crate::utils::*;
use glam::*;
use std::{cell::RefCell, fs::File, io::Write, path::Path, rc::Rc};

pub fn args(state: &State) -> IntermediateResult {
    Ok(Value::list(
        state
            .project
            .args
            .iter()
            .map(|s| Value::String(s.clone()))
            .collect(),
    ))
}

pub fn print(state: &State, args: &[Value], raw: bool) -> IntermediateResult {
    println!(
        "{}{}",
        if !raw {
            format!("{} => ", state.sprite.name)
        } else {
            "".to_string()
        },
        args.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    Ok(Value::Null)
}

pub fn input(state: &State, args: &[Value]) -> IntermediateResult {
    if let Some(prompt) = args.first() {
        if let Value::String(prompt) = prompt {
            let mut input = String::new();
            print!("{} => {} ", state.sprite.name, prompt);
            std::io::stdout().flush().unwrap();
            std::io::stdin()
                .read_line(&mut input)
                .map_err(|e| e.to_string())?;
            input = input.trim().to_string();
            Ok(Value::String(input))
        } else {
            Ok(Value::Null)
        }
    } else {
        Ok(Value::Null)
    }
}

pub fn time(state: &State) -> IntermediateResult {
    Ok(Value::Number(state.start.elapsed().as_secs_f32()))
}

pub fn math(args: &[Value], operation: &str) -> IntermediateResult {
    if let [Value::Number(n)] = args {
        let result = match operation {
            "abs" => n.abs(),
            "sqrt" => n.sqrt(),
            "sin" => n.sin(),
            "cos" => n.cos(),
            "tan" => n.tan(),
            "asin" => n.asin(),
            "acos" => n.acos(),
            "atan" => n.atan(),
            _ => unreachable!(),
        };
        Ok(Value::Number(result))
    } else {
        Err(format!("{}() expects one number argument", operation))
    }
}

pub fn lerp(args: &[Value]) -> IntermediateResult {
    if let [Value::Number(a), Value::Number(b), Value::Number(t)] = args {
        Ok(Value::Number(crate::utils::lerp(*a, *b, *t)))
    } else {
        Err("lerp() expects three number arguments".to_string())
    }
}

pub fn property_of(state: &mut State, args: &[Value]) -> IntermediateResult {
    if args.len() != 2 {
        return Err("property_of() expects two arguments: sprite name and property".to_string());
    }
    let name = match &args[0] {
        Value::String(name) => name.clone(),
        _ => return Err("First argument must be a string (sprite name)".to_string()),
    };
    let property = match &args[1] {
        Value::String(prop) => prop.clone(),
        _ => return Err("Second argument must be a string (property name)".to_string()),
    };

    if let Some(snapshot) = state.snapshots.iter().find(|s| s.name == name) {
        Ok(snapshot
            .get(&name)
            .ok_or(format!(
                "Property '{}' not found in sprite '{}'",
                property, name
            ))?
            .clone())
    } else {
        Err(format!("No snapshot found for sprite: {}", name))
    }
}

pub fn to_rad(args: &[Value]) -> IntermediateResult {
    if let [Value::Number(n)] = args {
        Ok(Value::Number(n.to_radians()))
    } else {
        Err("to_rad() expects one number argument".to_string())
    }
}

pub fn to_deg(args: &[Value]) -> IntermediateResult {
    if let [Value::Number(n)] = args {
        Ok(Value::Number(n.to_degrees()))
    } else {
        Err("to_deg() expects one number argument".to_string())
    }
}

pub fn set_cam(state: &mut State, args: &[Value]) -> IntermediateResult {
    match args {
        [] => {
            let (width, height) = state.window.get_size();
            let sprite_center = state.sprite.center;
            *state.projection = Mat4::orthographic_rh(
                -width as f32 + sprite_center.x,
                width as f32 + sprite_center.x,
                -height as f32 + sprite_center.y,
                height as f32 + sprite_center.y,
                -1.0,
                1.0,
            );
            Ok(Value::Null)
        }
        [Value::Number(x), Value::Number(y)] => {
            let (width, height) = state.window.get_size();
            *state.projection = Mat4::orthographic_rh(
                -width as f32 + x,
                width as f32 + x,
                -height as f32 + y,
                height as f32 + y,
                -1.0,
                1.0,
            );
            Ok(Value::Null)
        }
        [Value::Number(x), Value::Number(y), Value::Number(zoom_x), Value::Number(zoom_y)] => {
            let (width, height) = state.window.get_size();
            let zoom_x = (zoom_x / 100.0).max(0.01);
            let zoom_y = (zoom_y / 100.0).max(0.01);
            *state.projection = Mat4::orthographic_rh(
                -width as f32 * zoom_x + x,
                width as f32 * zoom_x + x,
                -height as f32 * zoom_y + y,
                height as f32 * zoom_y + y,
                -1.0,
                1.0,
            );
            Ok(Value::Null)
        }
        [Value::Number(x), Value::Number(y), Value::Number(zoom_x), Value::Number(zoom_y), Value::Number(rotation)] => {
            let (width, height) = state.window.get_size();
            let zoom_x = (zoom_x / 100.0).max(0.01);
            let zoom_y = (zoom_y / 100.0).max(0.01);
            let rotation_rad = rotation.to_radians();
            let view = Mat4::from_rotation_z(rotation_rad)
                * Mat4::from_translation(Vec3::new(*x, *y, 0.0));
            *state.projection = Mat4::orthographic_rh(
                -width as f32 * zoom_x,
                width as f32 * zoom_x,
                -height as f32 * zoom_y,
                height as f32 * zoom_y,
                -1.0,
                1.0,
            ) * view;
            Ok(Value::Null)
        }
        _ => {
            Err("set_cam() expects (), (x, y), (x, y, zoom_x, zoom_y), or (x, y, zoom_x, zoom_y, rotation)".to_string())
        }
    }
}

pub fn clamp(args: &[Value]) -> IntermediateResult {
    if let [Value::Number(value), Value::Number(min), Value::Number(max)] = args {
        let clamped_value = value.clamp(*min, *max);
        Ok(Value::Number(clamped_value))
    } else {
        Err("clamp() expects three number arguments".to_string())
    }
}

pub fn len(args: &[Value]) -> IntermediateResult {
    if args.len() != 1 {
        return Err("len() expects one argument".to_string());
    }
    let list = args[0].clone().to_list();
    Ok(Value::Number(list.len() as f32))
}

pub fn key_value(args: &[Value], which: &str) -> IntermediateResult {
    if let [Value::Object(o)] = args {
        match which {
            "keys" => {
                let keys = o.borrow().keys().cloned().collect::<Vec<_>>();
                Ok(Value::list(
                    keys.into_iter()
                        .map(|k| Value::String(k))
                        .collect(),
                ))
            }
            "values" => {
                let values = o.borrow().values().cloned().collect::<Vec<_>>();
                Ok(Value::List(Rc::new(RefCell::new(values))))
            }
            _ => unreachable!(),
        }
    } else {
        Err(format!("{}() expects one object argument", which))
    }
}

pub fn random(args: &[Value]) -> IntermediateResult {
    if let [Value::Number(min), Value::Number(max)] = args {
        if *min >= *max {
            return Err("random() expects two numbers where min < max".to_string());
        }
        let random_value = rand::random_range(*min..=*max);
        Ok(Value::Number(random_value))
    } else {
        Err("random() expects two number arguments".to_string())
    }
}

pub fn distance(state: &State, args: &[Value], to: bool) -> IntermediateResult {
    match !to {
        true => {
            if let [
                Value::Number(x1),
                Value::Number(y1),
                Value::Number(x2),
                Value::Number(y2),
            ] = args
            {
                let dist = Vec2::new(*x1, *y1).distance(Vec2::new(*x2, *y2));
                Ok(Value::Number(dist))
            } else {
                Err("distance() expects four number arguments".to_string())
            }
        }
        false => match args {
            [Value::Number(x), Value::Number(y)] => {
                let dist = state.sprite.center.distance(Vec2::new(*x, *y));
                Ok(Value::Number(dist))
            }
            [Value::String(name)] => {
                if let Some(other_sprite) = state.snapshots.iter().find(|s| s.name == *name) {
                    let dist = state.sprite.center.distance(other_sprite.center);
                    Ok(Value::Number(dist))
                } else if name == "mouse" {
                    let mouse_pos = Vec2::new(
                        state.window.get_cursor_pos().0 as f32,
                        state.window.get_cursor_pos().1 as f32,
                    ) * 2.0
                        - Vec2::new(
                            state.window.get_size().0 as f32,
                            state.window.get_size().1 as f32,
                        );
                    let dist = state.sprite.center.distance(mouse_pos);
                    Ok(Value::Number(dist))
                } else {
                    Err(format!("Sprite '{}' not found", name))
                }
            }
            _ => Err("distance_to() expects either two numbers or a sprite name".to_string()),
        },
    }
}

pub fn write(state: &State, args: &[Value]) -> IntermediateResult {
    match args {
        [Value::String(content)] => {
            let time = chrono::Local::now();
            let filename = format!(
                "{}-{}.png",
                state.sprite.name,
                time.format("%Y-%m-%d_%H-%M-%S")
            );
            let path = Path::new(&state.project.export_path).join(filename);
            let mut file = File::create(path).map_err(|e| e.to_string())?;
            file.write_all(content.as_bytes())
                .map_err(|e| e.to_string())?;
        }
        [Value::String(content), Value::String(path)] => {
            let path = Path::new(path);
            let mut file = File::create(path).map_err(|e| e.to_string())?;
            file.write_all(content.as_bytes())
                .map_err(|e| e.to_string())?;
        }
        _ => {
            return Err("write() expects one or two string arguments".to_string());
        }
    }

    Ok(Value::Null)
}

pub fn read(state: &State, args: &[Value], bin: bool) -> IntermediateResult {
    let which = if bin { "read_binary" } else { "read" };

    if args.len() != 1 {
        return Err(format!("{}() expects one string argument", which));
    }

    let file_name = match &args[0] {
        Value::String(name) => name.clone(),
        _ => return Err(format!("{}() expects a string argument", which)),
    };

    let full_path = Path::new(&state.project.home_path).join(file_name);
    if !full_path.exists() {
        return Err(format!(
            "File '{}' does not exist",
            full_path.to_string_lossy()
        ));
    }

    Ok(if bin {
        Value::list(
            std::fs::read(full_path)
                .map_err(|e| e.to_string())?
                .into_iter()
                .map(|v| Value::Number(v as f32))
                .collect(),
        )
    } else {
        Value::String(std::fs::read_to_string(full_path).map_err(|e| e.to_string())?)
    })
}

pub fn parse_image(args: &[Value]) -> IntermediateResult {
    if let [Value::List(contents)] = args {
        let image = image::load_from_memory(
            contents
                .borrow()
                .iter()
                .map(|v| v.borrow().to_number() as u8)
                .collect::<Vec<u8>>()
                .as_slice(),
        )
        .map_err(|e| e.to_string())?;
        let pixels: Vec<_> = image
            .to_rgba8()
            .into_raw()
            .iter()
            .map(|&b| Value::Number(b as f32))
            .collect();
        Ok(Value::list(vec![
            Value::Number(image.width() as f32),
            Value::Number(image.height() as f32),
            Value::list(pixels),
        ]))
    } else {
        Err("parse_image() expects one list argument".to_string())
    }
}

pub fn set_uv(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [
        Value::Number(u),
        Value::Number(v),
        Value::Number(w),
        Value::Number(x),
    ] = args
    {
        state.sprite.uv = [Vec2::new(*u, *v), Vec2::new(*w, *x)];
        Ok(Value::Null)
    } else {
        Err("set_uv() expects four number arguments".to_string())
    }
}

pub fn screenshot(state: &State, args: &[Value]) -> IntermediateResult {
    if args.len() != 1 {
        return Err("screenshot() expects one string argument".to_string());
    }

    let file_name = match &args[0] {
        Value::String(name) => name.clone(),
        _ => return Err("screenshot() expects a string argument".to_string()),
    };

    let full_path = Path::new(&state.project.export_path).join(file_name);
    let (width, height) = state.window.get_framebuffer_size();
    let mut pixels = vec![0; (width * height * 3) as usize];

    unsafe {
        gl::ReadPixels(
            0,
            0,
            width,
            height,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            pixels.as_mut_ptr() as *mut _,
        );
    }

    let row_len = (width * 3) as usize;
    for y in 0..(height / 2) {
        let top = y as usize * row_len;
        let bottom = (height as usize - 1 - y as usize) * row_len;
        for x in 0..row_len {
            pixels.swap(top + x, bottom + x);
        }
    }

    image::save_buffer(
        full_path,
        &pixels,
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
    )
    .map_err(|e| e.to_string())?;

    Ok(Value::Null)
}

pub fn r#typeof(args: &[Value]) -> IntermediateResult {
    if args.len() != 1 {
        return Err("typeof() expects one argument".to_string());
    }

    let value_type = match &args[0] {
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Boolean(_) => "boolean",
        Value::List(_) => "list",
        Value::Object(_) => "object",
        Value::Closure(_) => "closure",
        Value::Null => "null",
    };

    Ok(Value::String(value_type.to_string()))
}

pub fn push(args: &[Value]) -> IntermediateResult {
    if let [Value::List(list), value] = args {
        let mut borrowed = list.borrow_mut();
        borrowed.push(Rc::new(RefCell::new(value.clone())));
        Ok(Value::List(list.clone()))
    } else {
        Err("push() expects a list and a value".to_string())
    }
}

pub fn pop(args: &[Value]) -> IntermediateResult {
    if let [Value::List(list)] = args {
        if list.borrow().is_empty() {
            return Err("pop() called on an empty list".to_string());
        }
        let value = list
            .borrow_mut()
            .pop()
            .ok_or("pop() called on an empty list")?;
        Ok(Value::List(Rc::new(RefCell::new(vec![
            Rc::new(RefCell::new(Value::List(list.clone()))),
            value.clone(),
        ]))))
    } else {
        Err("pop() expects a list".to_string())
    }
}

pub fn insert(args: &[Value]) -> IntermediateResult {
    if let [Value::Object(obj), Value::String(key), value] = args {
        obj.borrow_mut()
            .insert(key.clone(), Rc::new(RefCell::new(value.clone())));
        Ok(Value::Object(obj.clone()))
    } else if let [Value::List(list), Value::Number(index), value] = args {
        let index = *index as usize;
        if index > list.borrow().len() {
            return Err("insert() index out of bounds".to_string());
        }
        list.borrow_mut()
            .insert(index, Rc::new(RefCell::new(value.clone())));
        Ok(Value::List(list.clone()))
    } else {
        Err("insert() expects a list or an object, a key, and a value".to_string())
    }
}

pub fn remove(args: &[Value]) -> IntermediateResult {
    if let [Value::Object(obj), Value::String(key)] = args {
        obj.borrow_mut().remove(key);
        Ok(Value::Object(obj.clone()))
    } else if let [Value::List(list), Value::Number(index)] = args {
        let index = *index as usize;
        if index >= list.borrow().len() {
            return Err("remove() index out of bounds".to_string());
        }
        list.borrow_mut().remove(index);
        Ok(Value::List(list.clone()))
    } else {
        Err("remove() expects a list or an object, and a key or index".to_string())
    }
}

pub fn extend(args: &[Value]) -> IntermediateResult {
    if let [Value::List(list1), Value::List(list2)] = args {
        list1.borrow_mut().extend(list2.borrow().iter().cloned());
        Ok(Value::List(list1.clone()))
    } else {
        Err("extend() expects two lists".to_string())
    }
}

pub fn contains(args: &[Value]) -> IntermediateResult {
    if let [Value::List(list), value] = args {
        Ok(Value::Boolean(
            list.borrow().iter().any(|v| v.borrow().eq(value)),
        ))
    } else if let [Value::Object(obj), Value::String(key)] = args {
        Ok(Value::Boolean(obj.borrow().contains_key(key)))
    } else {
        Err("contains() expects a list or an object and a value or key".to_string())
    }
}

pub fn sort(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::List(list), Value::Closure(closure)] = args {
        let function_struct = &**closure;
        let mut error: Option<String> = None;
        list.borrow_mut().sort_by(|a, b| {
            let args_ = [a.clone(), b.clone()];
            let result = function_struct
                .clone()
                .call(state, &args_)
                .unwrap_or_else(|e| {
                    error = Some(format!("Error calling closure in sort(): {}", e,));
                    Rc::new(RefCell::new(Value::Null))
                });
            if let Value::Boolean(b) = &*result.borrow() {
                if *b {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            } else {
                std::cmp::Ordering::Equal
            }
        });
        if let Some(err) = error {
            return Err(err);
        }
        Ok(Value::List(list.clone()))
    } else {
        Err("sort() expects a list and a closure".to_string())
    }
}

pub fn filter(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::List(list), Value::Closure(closure)] = args {
        let function_struct = &**closure;
        let mut error: Option<String> = None;
        let filtered: Vec<_> = list
            .borrow()
            .iter()
            .filter_map(|item| {
                let args_ = [item.clone()];
                let result = function_struct
                    .clone()
                    .call(state, &args_)
                    .unwrap_or_else(|e| {
                        error = Some(format!("Error calling closure in filter(): {}", e));
                        Rc::new(RefCell::new(Value::Null))
                    });
                if let Value::Boolean(true) = &*result.borrow() {
                    Some(item.clone())
                } else {
                    None
                }
            })
            .collect();
        if let Some(err) = error {
            return Err(err);
        }
        Ok(Value::List(Rc::new(RefCell::new(filtered))))
    } else {
        Err("filter() expects a list and a closure".to_string())
    }
}

pub fn map(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::List(list), Value::Closure(closure)] = args {
        let function_struct = &**closure;
        let mut error: Option<String> = None;
        let mapped: Vec<_> = list
            .borrow()
            .iter()
            .map(|item| {
                let args_ = [item.clone()];
                function_struct
                    .clone()
                    .call(state, &args_)
                    .unwrap_or_else(|e| {
                        error = Some(format!("Error calling closure in map(): {}", e));
                        Rc::new(RefCell::new(Value::Null))
                    })
            })
            .collect();
        if let Some(err) = error {
            return Err(err);
        }
        Ok(Value::List(Rc::new(RefCell::new(mapped))))
    } else {
        Err("map() expects a list and a closure".to_string())
    }
}

pub fn split(args: &[Value]) -> IntermediateResult {
    if let [Value::String(s), Value::String(delimiter)] = args {
        let parts: Vec<_> = s
            .split(delimiter)
            .map(|part| Value::String(part.to_string()))
            .collect();
        Ok(Value::list(parts))
    } else {
        Err("split() expects two string arguments".to_string())
    }
}

pub fn join(args: &[Value]) -> IntermediateResult {
    if let [Value::List(list), Value::String(delimiter)] = args {
        let joined = list
            .borrow()
            .iter()
            .map(|v| v.borrow().to_string())
            .collect::<Vec<_>>()
            .join(delimiter);
        Ok(Value::String(joined))
    } else {
        Err("join() expects a list and a string".to_string())
    }
}

pub fn starts_with(args: &[Value]) -> IntermediateResult {
    if let [Value::String(s), Value::String(prefix)] = args {
        Ok(Value::Boolean(s.starts_with(prefix)))
    } else {
        Err("starts_with() expects two string arguments".to_string())
    }
}

pub fn ends_with(args: &[Value]) -> IntermediateResult {
    if let [Value::String(s), Value::String(suffix)] = args {
        Ok(Value::Boolean(s.ends_with(suffix)))
    } else {
        Err("ends_with() expects two string arguments".to_string())
    }
}

pub fn trim(args: &[Value]) -> IntermediateResult {
    if let [Value::String(s)] = args {
        Ok(Value::String(s.trim().to_string()))
    } else {
        Err("trim() expects one string argument".to_string())
    }
}

pub fn range(args: &[Value]) -> IntermediateResult {
    match args {
        [Value::Number(end)] => {
            let range: Vec<_> = (0..*end as i32)
                .map(|n| Value::Number(n as f32))
                .collect();
            Ok(Value::list(range))
        }
        [Value::Number(start), Value::Number(end)] => {
            if start > end {
                return Err("range() expects start <= end".to_string());
            }
            let range: Vec<_> = (*start as i32..*end as i32)
                .map(|n| Value::Number(n as f32))
                .collect();
            Ok(Value::list(range))
        }
        [
            Value::Number(start),
            Value::Number(end),
            Value::Number(step),
        ] => {
            if start > end {
                return Err("range() expects start <= end".to_string());
            }
            if *step == 0.0 {
                return Err("range() step cannot be zero".to_string());
            }
            let range: Vec<_> = (0..((end - start) / step).ceil() as i32)
                .map(|n| Value::Number(start + n as f32 * step))
                .collect();
            Ok(Value::list(range))
        }
        _ => Err("range() expects two number arguments".to_string()),
    }
}

pub fn to(args: &[Value], to: &str) -> IntermediateResult {
    match to {
        "string" => match args {
            [value] => Ok(Value::String(value.to_string())),
            [Value::Number(n), Value::Number(base)] => {
                Ok(Value::String(format_radix(*n as u32, *base as u32)))
            }
            _ => Err("to_string() expects one or two number arguments".to_string()),
        },
        "number" => match args {
            [value] => Ok(Value::Number(value.to_number())),
            _ => Err("to_number() expects one argument".to_string()),
        },
        "boolean" => match args {
            [value] => Ok(Value::Boolean(value.to_boolean())),
            _ => Err("to_boolean() expects one argument".to_string()),
        },
        "list" => match args {
            [value] => Ok(Value::List(Rc::new(RefCell::new(value.to_list())))),
            _ => Err("to_list() expects one argument".to_string()),
        },
        "object" => match args {
            [value] => Ok(Value::Object(Rc::new(RefCell::new(value.to_object())))),
            _ => Err("to_object() expects one argument".to_string()),
        },
        _ => unreachable!(),
    }
}

pub fn whoami(state: &State) -> IntermediateResult {
    Ok(Value::String(state.sprite.name.clone()))
}

pub fn cloneid(state: &State) -> IntermediateResult {
    Ok(Value::Number(state.sprite.clone_id.unwrap_or(0) as f32))
}

pub fn frame(state: &State) -> IntermediateResult {
    Ok(Value::Number(state.start.elapsed().as_secs_f32() * 60.0))
}

pub fn delta_time(state: &State) -> IntermediateResult {
    Ok(Value::Number(state.dt))
}
