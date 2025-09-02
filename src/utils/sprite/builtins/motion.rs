use crate::utils::{sprite::Glide, *};
use glam::*;

pub fn r#move(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::Number(step)] = args {
        state.sprite.move_by(*step, state.window);
        Ok(Value::Null)
    } else {
        Err("move() requires a single numeric argument".to_string())
    }
}

pub fn turn_cw(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::Number(angle)] = args {
        state.sprite.direction += *angle;
        Ok(Value::Null)
    } else {
        Err("turn_cw() requires a single numeric argument".to_string())
    }
}

pub fn turn_ccw(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::Number(angle)] = args {
        state.sprite.direction -= *angle;
        Ok(Value::Null)
    } else {
        Err("turn_ccw() requires a single numeric argument".to_string())
    }
}

pub fn goto(state: &mut State, args: &[Value]) -> IntermediateResult {
    match args {
        [Value::Number(x), Value::Number(y)] => {
            state.sprite.goto(*x, *y);
            Ok(Value::Null)
        }
        [Value::String(name)] => {
            if name == "mouse" {
                state.sprite.goto_cursor(state.window);
                Ok(Value::Null)
            } else if name == "random" {
                state.sprite.goto(
                    rand::random_range(
                        -(state.window.get_size().0 as f32)..=(state.window.get_size().0 as f32),
                    ),
                    rand::random_range(
                        -(state.window.get_size().1 as f32)..=(state.window.get_size().1 as f32),
                    ),
                );
                Ok(Value::Null)
            } else if let Some(target) = state.snapshots.iter().find(|s| s.name == *name) {
                state.sprite.goto(target.center.x, target.center.y);
                Ok(Value::Null)
            } else {
                Err(format!("goto() target '{}' not found", name))
            }
        }
        _ => Err("goto() requires two position arguments or a target name".to_string()),
    }
}

pub fn glide(state: &mut State, args: &[Value]) -> IntermediateResult {
    match args {
        [Value::Number(x), Value::Number(y), Value::Number(duration)] => {
            let duration = *duration * 60.0;
            state.sprite.glide = Some(Glide {
                start_x: state.sprite.center.x,
                start_y: state.sprite.center.y,
                end_x: *x,
                end_y: *y,
                duration: duration as usize,
                remaining: duration as usize,
                ctrl1: vec2(0.0, 0.0), // No easing
                ctrl2: vec2(1.0, 1.0),
            });
            Ok(Value::Null)
        }
        [
            Value::Number(x),
            Value::Number(y),
            Value::Number(duration),
            Value::String(easing),
        ] => {
            let duration = *duration * 60.0;
            let easing = easing.to_lowercase();
            let (ctrl1, ctrl2) = match easing.as_str() {
                "linear" => (vec2(0.0, 0.0), vec2(1.0, 1.0)),
                "ease" => (vec2(0.25, 0.01), vec2(0.25, 1.0)),
                "ease-in" => (vec2(0.42, 0.0), vec2(1.0, 1.0)),
                "ease-out" => (vec2(0.0, 0.0), vec2(0.58, 1.0)),
                "ease-in-out" => (vec2(0.42, 0.0), vec2(0.58, 1.0)),
                _ => (vec2(0.0, 0.0), vec2(1.0, 1.0)), // Default to linear
            };
            state.sprite.glide = Some(Glide {
                start_x: state.sprite.center.x,
                start_y: state.sprite.center.y,
                end_x: *x,
                end_y: *y,
                duration: duration as usize,
                remaining: duration as usize,
                ctrl1,
                ctrl2,
            });
            Ok(Value::Null)
        }
        _ => Err(
            "glide() requires two position arguments and a duration, optionally with easing"
                .to_string(),
        ),
    }
}

pub fn point(state: &mut State, args: &[Value]) -> IntermediateResult {
    match args {
        [Value::Number(angle)] => {
            state.sprite.direction = *angle;
            Ok(Value::Null)
        }
        [Value::Number(x), Value::Number(y)] => {
            state.sprite.point(*x, *y);
            Ok(Value::Null)
        }
        [Value::String(name)] => {
            if name == "cursor" {
                state.sprite.point_cursor(state.window);
                Ok(Value::Null)
            } else if name == "random" {
                state.sprite.direction = rand::random_range(0.0..=360.0);
                Ok(Value::Null)
            } else if let Some(target) = state.snapshots.iter().find(|s| s.name == *name) {
                state.sprite.point(target.center.x, target.center.y);
                Ok(Value::Null)
            } else {
                Err(format!("point() target '{}' not found", name))
            }
        }
        _ => Err("point() requires two position arguments or a target name".to_string()),
    }
}

pub fn set_pos(state: &mut State, args: &[Value], which: &str) -> IntermediateResult {
    if let [Value::Number(value)] = args {
        match which {
            "x" => {
                state.sprite.center.x = *value;
                Ok(Value::Null)
            }
            "y" => {
                state.sprite.center.y = *value;
                Ok(Value::Null)
            }
            _ => unreachable!(),
        }
    } else {
        Err("set_pos() requires a single numeric argument".to_string())
    }
}

pub fn change_pos(state: &mut State, args: &[Value], which: &str) -> IntermediateResult {
    if let [Value::Number(value)] = args {
        match which {
            "x" => {
                state.sprite.center.x += *value;
                Ok(Value::Null)
            }
            "y" => {
                state.sprite.center.y += *value;
                Ok(Value::Null)
            }
            _ => unreachable!(),
        }
    } else {
        Err("change_pos() requires a single numeric argument".to_string())
    }
}

pub fn edge_bounce(state: &mut State, args: &[Value]) -> IntermediateResult {
    match args {
        [Value::Boolean(enabled)] => {
            state.sprite.edge_bounce = *enabled;
            Ok(Value::Null)
        }
        [] => {
            state.sprite.edge_bounce = !state.sprite.edge_bounce;
            Ok(Value::Null)
        }
        _ => Err("edge_bounce() requires a single boolean argument or no arguments".to_string()),
    }
}

pub fn rotation_style(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::String(style)] = args {
        match style.to_lowercase().as_str() {
            "all-around" => state.sprite.rotation_style = RotationStyle::AllAround,
            "left-right" => state.sprite.rotation_style = RotationStyle::LeftRight,
            "dont-rotate" => state.sprite.rotation_style = RotationStyle::DontRotate,
            _ => return Err(format!("Invalid rotation style: '{}'", style)),
        }
        Ok(Value::Null)
    } else {
        Err("rotation_style() requires a single string argument".to_string())
    }
}

pub fn direction(state: &State) -> IntermediateResult {
    Ok(Value::Number(state.sprite.direction))
}

pub fn position(state: &State, which: &str) -> IntermediateResult {
    match which {
        "x" => Ok(Value::Number(state.sprite.center.x)),
        "y" => Ok(Value::Number(state.sprite.center.y)),
        _ => Err(format!("Invalid position argument: '{}'", which)),
    }
}
