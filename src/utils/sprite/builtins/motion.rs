use crate::utils::{sprite::Glide, *};
use macroquad::prelude::*;

pub fn r#move(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::Number(step)] = args {
        sprite.move_by(*step);
        Ok(Value::Null)
    } else {
        Err("move() requires a single numeric argument".to_string())
    }
}

pub fn turn_cw(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::Number(angle)] = args {
        sprite.direction += *angle;
        Ok(Value::Null)
    } else {
        Err("turn_cw() requires a single numeric argument".to_string())
    }
}

pub fn turn_ccw(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::Number(angle)] = args {
        sprite.direction -= *angle;
        Ok(Value::Null)
    } else {
        Err("turn_ccw() requires a single numeric argument".to_string())
    }
}

pub fn goto(
    sprite: &mut Sprite,
    snapshots: &[SpriteSnapshot],
    args: &[Value],
) -> Result {
    match args {
        [Value::Number(x), Value::Number(y)] => {
            sprite.goto(*x, *y);
            Ok(Value::Null)
        }
        [Value::String(name)] => {
            if name == "mouse" {
                sprite.goto_cursor();
                Ok(Value::Null)
            } else if name == "random" {
                sprite.goto(
                    rand::gen_range(-screen_width(), screen_width()),
                    rand::gen_range(-screen_height(), screen_height()),
                );
                Ok(Value::Null)
            } else if let Some(target) = snapshots.iter().find(|s| s.name == *name) {
                sprite.goto(target.center.x, target.center.y);
                Ok(Value::Null)
            } else {
                Err(format!("goto() target '{}' not found", name))
            }
        }
        _ => Err("goto() requires two position arguments or a target name".to_string()),
    }
}

pub fn glide(sprite: &mut Sprite, args: &[Value]) -> Result {
    match args {
        [Value::Number(x), Value::Number(y), Value::Number(duration)] => {
            let duration = *duration * 60.0;
            sprite.glide = Some(Glide {
                start_x: sprite.center.x,
                start_y: sprite.center.y,
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
            sprite.glide = Some(Glide {
                start_x: sprite.center.x,
                start_y: sprite.center.y,
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

pub fn point(
    sprite: &mut Sprite,
    snapshots: &[SpriteSnapshot],
    args: &[Value],
) -> Result {
    match args {
        [Value::Number(angle)] => {
            sprite.direction = *angle;
            Ok(Value::Null)
        }
        [Value::Number(x), Value::Number(y)] => {
            sprite.point(*x, *y);
            Ok(Value::Null)
        }
        [Value::String(name)] => {
            if name == "cursor" {
                sprite.point_cursor();
                Ok(Value::Null)
            } else if name == "random" {
                sprite.direction = rand::gen_range(0.0, 360.0);
                Ok(Value::Null)
            } else if let Some(target) = snapshots.iter().find(|s| s.name == *name) {
                sprite.point(target.center.x, target.center.y);
                Ok(Value::Null)
            } else {
                Err(format!("point() target '{}' not found", name))
            }
        }
        _ => Err("point() requires two position arguments or a target name".to_string()),
    }
}

pub fn set_pos(sprite: &mut Sprite, args: &[Value], which: &str) -> Result {
    if let [Value::Number(value)] = args {
        match which {
            "x" => {
                sprite.center.x = *value;
                Ok(Value::Null)
            }
            "y" => {
                sprite.center.y = *value;
                Ok(Value::Null)
            }
            _ => unreachable!(),
        }
    } else {
        Err("set_pos() requires a single numeric argument".to_string())
    }
}

pub fn change_pos(
    sprite: &mut Sprite,
    args: &[Value],
    which: &str,
) -> Result {
    if let [Value::Number(value)] = args {
        match which {
            "x" => {
                sprite.center.x += *value;
                Ok(Value::Null)
            }
            "y" => {
                sprite.center.y += *value;
                Ok(Value::Null)
            }
            _ => unreachable!(),
        }
    } else {
        Err("change_pos() requires a single numeric argument".to_string())
    }
}

pub fn edge_bounce(sprite: &mut Sprite, args: &[Value]) -> Result {
    match args {
        [Value::Boolean(enabled)] => {
            sprite.edge_bounce = *enabled;
            Ok(Value::Null)
        }
        [] => {
            sprite.edge_bounce = !sprite.edge_bounce;
            Ok(Value::Null)
        }
        _ => Err("edge_bounce() requires a single boolean argument or no arguments".to_string()),
    }
}

pub fn rotation_style(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::String(style)] = args {
        match style.to_lowercase().as_str() {
            "all-around" => sprite.rotation_style = RotationStyle::AllAround,
            "left-right" => sprite.rotation_style = RotationStyle::LeftRight,
            "dont-rotate" => sprite.rotation_style = RotationStyle::DontRotate,
            _ => return Err(format!("Invalid rotation style: '{}'", style)),
        }
        Ok(Value::Null)
    } else {
        Err("rotation_style() requires a single string argument".to_string())
    }
}

pub fn direction(sprite: &Sprite) -> Result {
    Ok(Value::Number(sprite.direction))
}

pub fn position(sprite: &Sprite, which: &str) -> Result {
    match which {
        "x" => Ok(Value::Number(sprite.center.x)),
        "y" => Ok(Value::Number(sprite.center.y)),
        _ => Err(format!("Invalid position argument: '{}'", which)),
    }
}
