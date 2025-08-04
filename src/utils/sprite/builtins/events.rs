use crate::utils::*;
use glam::*;

pub fn key_down(state: &State, args: &[Value]) -> Result {
    if let [Value::String(key)] = args {
        let key_code = string_to_keycode(key).ok_or(format!("Invalid key code: '{}'", key))?;
        Ok(Value::Boolean(state.input_manager.is_key_down(key_code)))
    } else {
        Err("key_down() requires a single string argument".to_string())
    }
}

pub fn key_pressed(state: &State, args: &[Value]) -> Result {
    if let [Value::String(key)] = args {
        let key_code = string_to_keycode(key).ok_or(format!("Invalid key code: '{}'", key))?;
        Ok(Value::Boolean(state.input_manager.is_key_pressed(key_code)))
    } else {
        Err("key_pressed() requires a single string argument".to_string())
    }
}

pub fn key_released(state: &State, args: &[Value]) -> Result {
    if let [Value::String(key)] = args {
        let key_code = string_to_keycode(key).ok_or(format!("Invalid key code: '{}'", key))?;
        Ok(Value::Boolean(
            state.input_manager.is_key_released(key_code),
        ))
    } else {
        Err("key_released() requires a single string argument".to_string())
    }
}

pub fn mouse_button_down(state: &State, args: &[Value]) -> Result {
    if let [Value::String(button)] = args {
        let button_code =
            string_to_mouse(button).ok_or(format!("Invalid mouse button: '{}'", button))?;
        Ok(Value::Boolean(
            state.input_manager.is_mouse_button_down(button_code),
        ))
    } else {
        Err("mouse_button_down() requires a single string argument".to_string())
    }
}

pub fn mouse_button_pressed(state: &State, args: &[Value]) -> Result {
    if let [Value::String(button)] = args {
        let button_code =
            string_to_mouse(button).ok_or(format!("Invalid mouse button: '{}'", button))?;
        Ok(Value::Boolean(
            state.input_manager.is_mouse_button_pressed(button_code),
        ))
    } else {
        Err("mouse_button_pressed() requires a single string argument".to_string())
    }
}

pub fn mouse_button_released(state: &State, args: &[Value]) -> Result {
    if let [Value::String(button)] = args {
        let button_code =
            string_to_mouse(button).ok_or(format!("Invalid mouse button: '{}'", button))?;
        Ok(Value::Boolean(
            state.input_manager.is_mouse_button_released(button_code),
        ))
    } else {
        Err("mouse_button_released() requires a single string argument".to_string())
    }
}

pub fn mouse_x(state: &State) -> Result {
    Ok(Value::Number(
        state.window.get_cursor_pos().0 as f32 * 2.0 - state.window.get_size().0 as f32,
    ))
}

pub fn mouse_y(state: &State) -> Result {
    Ok(Value::Number(
        -(state.window.get_cursor_pos().1 as f32 * 2.0 - state.window.get_size().1 as f32),
    ))
}

pub fn sprite_clicked(state: &State) -> Result {
    if !state
        .input_manager
        .is_mouse_button_pressed(glfw::MouseButton::Left)
    {
        return Ok(Value::Boolean(false));
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
        Ok(Value::Boolean(true))
    } else {
        Ok(Value::Boolean(false))
    }
}

pub fn is_backdrop(state: &State, args: &[Value]) -> Result {
    if let [Value::Number(index)] = args {
        let backdrop = state.project.stage.backdrop();
        if backdrop == *index as usize {
            Ok(Value::Boolean(true))
        } else {
            Ok(Value::Boolean(false))
        }
    } else {
        Err("is_backdrop() requires a single string argument".to_string())
    }
}

pub fn broadcast_id_of(state: &State, args: &[Value]) -> Result {
    if let [Value::String(message)] = args {
        if let Some(broadcast) = state.project.get_broadcast(message) {
            Ok(Value::Number(broadcast.id as f32))
        } else {
            Err(format!("Broadcast message '{}' not found", message))
        }
    } else {
        Err("broadcast_id_of() requires a single string argument".to_string())
    }
}

pub fn broadcast(state: &mut State, args: &[Value]) -> Result {
    if let [Value::String(message)] = args {
        state.project.broadcast(message.clone());
        Ok(Value::Null)
    } else {
        Err("broadcast() requires a single string argument".to_string())
    }
}
