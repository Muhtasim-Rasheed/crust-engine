use crate::utils::*;
use macroquad::{
    input::*,
    math::{Rect, Vec2},
    window::{screen_height, screen_width},
};

pub fn key_down(args: &[Value]) -> Result {
    if let [Value::String(key)] = args {
        let key_code = string_to_keycode(key).ok_or(format!("Invalid key code: '{}'", key))?;
        Ok(Value::Boolean(is_key_down(key_code)))
    } else {
        Err("key_down() requires a single string argument".to_string())
    }
}

pub fn key_pressed(args: &[Value]) -> Result {
    if let [Value::String(key)] = args {
        let key_code = string_to_keycode(key).ok_or(format!("Invalid key code: '{}'", key))?;
        Ok(Value::Boolean(macroquad::input::is_key_pressed(key_code)))
    } else {
        Err("key_pressed() requires a single string argument".to_string())
    }
}

pub fn key_released(args: &[Value]) -> Result {
    if let [Value::String(key)] = args {
        let key_code = string_to_keycode(key).ok_or(format!("Invalid key code: '{}'", key))?;
        Ok(Value::Boolean(macroquad::input::is_key_released(key_code)))
    } else {
        Err("key_released() requires a single string argument".to_string())
    }
}

pub fn mouse_button_down(args: &[Value]) -> Result {
    if let [Value::String(button)] = args {
        let button_code =
            string_to_mouse(button).ok_or(format!("Invalid mouse button: '{}'", button))?;
        Ok(Value::Boolean(macroquad::input::is_mouse_button_down(
            button_code,
        )))
    } else {
        Err("mouse_button_down() requires a single string argument".to_string())
    }
}

pub fn mouse_button_pressed(args: &[Value]) -> Result {
    if let [Value::String(button)] = args {
        let button_code =
            string_to_mouse(button).ok_or(format!("Invalid mouse button: '{}'", button))?;
        Ok(Value::Boolean(macroquad::input::is_mouse_button_pressed(
            button_code,
        )))
    } else {
        Err("mouse_button_pressed() requires a single string argument".to_string())
    }
}

pub fn mouse_button_released(args: &[Value]) -> Result {
    if let [Value::String(button)] = args {
        let button_code =
            string_to_mouse(button).ok_or(format!("Invalid mouse button: '{}'", button))?;
        Ok(Value::Boolean(macroquad::input::is_mouse_button_released(
            button_code,
        )))
    } else {
        Err("mouse_button_released() requires a single string argument".to_string())
    }
}

pub fn mouse_x() -> Result {
    Ok(Value::Number(
        mouse_position().0 as f32 * 2.0 - screen_width() as f32,
    ))
}

pub fn mouse_y() -> Result {
    Ok(Value::Number(
        mouse_position().1 as f32 * 2.0 - screen_height() as f32,
    ))
}

pub fn sprite_clicked(sprite: &Sprite) -> Result {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return Ok(Value::Boolean(false));
    }
    let xy = mouse_position();
    let top_left =
        sprite.center - Vec2::new(sprite.size.x * sprite.scale, sprite.size.y * sprite.scale);
    let bottom_right =
        sprite.center + Vec2::new(sprite.size.x * sprite.scale, sprite.size.y * sprite.scale);
    let rect = Rect::new(
        top_left.x,
        top_left.y,
        top_left.x - bottom_right.x,
        top_left.y - bottom_right.y,
    );
    if rect.contains(xy.into()) {
        Ok(Value::Boolean(true))
    } else {
        Ok(Value::Boolean(false))
    }
}

pub fn is_backdrop(project: &Project, args: &[Value]) -> Result {
    if let [Value::Number(index)] = args {
        let backdrop = project.stage.backdrop();
        if backdrop == *index as usize {
            Ok(Value::Boolean(true))
        } else {
            Ok(Value::Boolean(false))
        }
    } else {
        Err("is_backdrop() requires a single string argument".to_string())
    }
}

pub fn broadcast_id_of(project: &Project, args: &[Value]) -> Result {
    if let [Value::String(message)] = args {
        if let Some(broadcast) = project.get_broadcast(message) {
            Ok(Value::Number(broadcast.id as f32))
        } else {
            Err(format!("Broadcast message '{}' not found", message))
        }
    } else {
        Err("broadcast_id_of() requires a single string argument".to_string())
    }
}

pub fn broadcast(project: &mut Project, args: &[Value]) -> Result {
    if let [Value::String(message)] = args {
        project.broadcast(message.clone());
        Ok(Value::Null)
    } else {
        Err("broadcast() requires a single string argument".to_string())
    }
}
