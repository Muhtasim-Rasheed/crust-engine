use std::{cell::RefCell, rc::Rc};

use glam::*;

use crate::utils::{State, Value, function, keycode_to_string, string_to_keycode, string_to_mouse};

pub fn key_down(state: &State, args: &[&mut Value]) -> function::Result {
    if let [Value::String(key)] = args {
        let key_code = string_to_keycode(key).ok_or(format!("Invalid key code: '{}'", key))?;
        Ok(Rc::new(RefCell::new(Value::Boolean(
            state.input_manager.is_key_down(key_code),
        ))))
    } else {
        Err("key_down() requires a single string argument".to_string())
    }
}

pub fn key_pressed(state: &State, args: &[&mut Value]) -> function::Result {
    if let [Value::String(key)] = args {
        let key_code = string_to_keycode(key).ok_or(format!("Invalid key code: '{}'", key))?;
        Ok(Rc::new(RefCell::new(Value::Boolean(
            state.input_manager.is_key_pressed(key_code),
        ))))
    } else {
        Err("key_pressed() requires a single string argument".to_string())
    }
}

pub fn key_released(state: &State, args: &[&mut Value]) -> function::Result {
    if let [Value::String(key)] = args {
        let key_code = string_to_keycode(key).ok_or(format!("Invalid key code: '{}'", key))?;
        Ok(Rc::new(RefCell::new(Value::Boolean(
            state.input_manager.is_key_released(key_code),
        ))))
    } else {
        Err("key_released() requires a single string argument".to_string())
    }
}

pub fn last_key(state: &State) -> function::Result {
    let key_history = state.input_manager.key_history();
    if let Some(key) = key_history.last() {
        Ok(Rc::new(RefCell::new(Value::String(keycode_to_string(
            *key,
        )))))
    } else {
        Ok(Rc::new(RefCell::new(Value::Null)))
    }
}

pub fn combination_pressed(state: &mut State, args: &[&mut Value]) -> function::Result {
    let key_codes: Result<Vec<glfw::Key>, _> = args
        .iter()
        .map(|arg| match arg {
            Value::String(s) => string_to_keycode(s).ok_or_else(|| format!("Invalid key: {}", s)),
            _ => Err("All arguments must be strings".to_string()),
        })
        .collect();

    let key_codes = match key_codes {
        Ok(codes) if !codes.is_empty() => codes,
        _ => return Ok(Rc::new(RefCell::new(Value::Boolean(false)))),
    };

    let matches = state.input_manager.combination_pressed(&key_codes);
    Ok(Rc::new(RefCell::new(Value::Boolean(matches))))
}

pub fn mouse_button_down(state: &State, args: &[&mut Value]) -> function::Result {
    if let [Value::String(button)] = args {
        let button_code =
            string_to_mouse(button).ok_or(format!("Invalid mouse button: '{}'", button))?;
        Ok(Rc::new(RefCell::new(Value::Boolean(
            state.input_manager.is_mouse_button_down(button_code),
        ))))
    } else {
        Err("mouse_button_down() requires a single string argument".to_string())
    }
}

pub fn mouse_button_pressed(state: &State, args: &[&mut Value]) -> function::Result {
    if let [Value::String(button)] = args {
        let button_code =
            string_to_mouse(button).ok_or(format!("Invalid mouse button: '{}'", button))?;
        Ok(Rc::new(RefCell::new(Value::Boolean(
            state.input_manager.is_mouse_button_pressed(button_code),
        ))))
    } else {
        Err("mouse_button_pressed() requires a single string argument".to_string())
    }
}

pub fn mouse_button_released(state: &State, args: &[&mut Value]) -> function::Result {
    if let [Value::String(button)] = args {
        let button_code =
            string_to_mouse(button).ok_or(format!("Invalid mouse button: '{}'", button))?;
        Ok(Rc::new(RefCell::new(Value::Boolean(
            state.input_manager.is_mouse_button_released(button_code),
        ))))
    } else {
        Err("mouse_button_released() requires a single string argument".to_string())
    }
}

pub fn mouse_x(state: &State) -> function::Result {
    Ok(Rc::new(RefCell::new(Value::Number(
        state.window.get_cursor_pos().0 as f32 * 2.0 - state.window.get_size().0 as f32,
    ))))
}

pub fn mouse_y(state: &State) -> function::Result {
    Ok(Rc::new(RefCell::new(Value::Number(
        -(state.window.get_cursor_pos().1 as f32 * 2.0 - state.window.get_size().1 as f32),
    ))))
}

pub fn sprite_clicked(state: &State) -> function::Result {
    if !state
        .input_manager
        .is_mouse_button_pressed(glfw::MouseButton::Left)
    {
        return Ok(Rc::new(RefCell::new(Value::Boolean(false))));
    }
    let (x, y) = state.window.get_cursor_pos();
    let xy = Vec2::new(
        x as f32 * 2.0 - state.window.get_size().0 as f32,
        -(y as f32 * 2.0 - state.window.get_size().1 as f32),
    );
    let top_left = state.sprite.center
        - Vec2::new(
            state.sprite.size.x * state.sprite.scale,
            state.sprite.size.y * state.sprite.scale,
        );
    let bottom_right = state.sprite.center
        + Vec2::new(
            state.sprite.size.x * state.sprite.scale,
            state.sprite.size.y * state.sprite.scale,
        );
    if {
        let collide_x = xy.x >= top_left.x && xy.x <= bottom_right.x;
        let collide_y = xy.y >= top_left.y && xy.y <= bottom_right.y;
        collide_x && collide_y
    } {
        Ok(Rc::new(RefCell::new(Value::Boolean(true))))
    } else {
        Ok(Rc::new(RefCell::new(Value::Boolean(false))))
    }
}

pub fn is_backdrop(state: &State, args: &[&mut Value]) -> function::Result {
    if let [Value::Number(index)] = args {
        let backdrop = state.project.stage.backdrop();
        Ok(Rc::new(RefCell::new(Value::Boolean(
            backdrop == *index as usize,
        ))))
    } else {
        Err("is_backdrop() requires a single string argument".to_string())
    }
}

pub fn broadcast_id_of(state: &State, args: &[&mut Value]) -> function::Result {
    if let [Value::String(message)] = args {
        if let Some(broadcast) = state.project.get_broadcast(message) {
            Ok(Rc::new(RefCell::new(Value::Number(broadcast.id as f32))))
        } else {
            Err(format!("Broadcast message '{}' not found", message))
        }
    } else {
        Err("broadcast_id_of() requires a single string argument".to_string())
    }
}

pub fn broadcast(state: &mut State, args: &[&mut Value]) -> function::Result {
    if let [Value::String(message)] = args {
        state.project.broadcast(message.clone());
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("broadcast() requires a single string argument".to_string())
    }
}
