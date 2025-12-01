use std::{cell::RefCell, rc::Rc};

use glfw::WindowMode;

use crate::utils::*;

pub fn set_window_width(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(width)] = args {
        state
            .window
            .set_size(*width as i32, state.window.get_size().1);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("set_window_width() requires a single numeric argument".to_string())
    }
}

pub fn set_window_height(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(height)] = args {
        state
            .window
            .set_size(state.window.get_size().0, *height as i32);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("set_window_height() requires a single numeric argument".to_string())
    }
}

pub fn set_window_size(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(width), Value::Number(height)] = args {
        state.window.set_size(*width as i32, *height as i32);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("set_window_size() requires two numeric arguments".to_string())
    }
}

pub fn set_window_state(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::String(mode)] = args {
        let (xpos, ypos) = state.window.get_pos();
        let (width, height) = state.window.get_size();
        match mode.as_str() {
            "normal" => state.window.set_monitor(
                WindowMode::Windowed,
                xpos as i32,
                ypos as i32,
                width as u32,
                height as u32,
                None,
            ),
            "fullscreen" => state.glfw.with_primary_monitor(|_, m| {
                state.window.set_monitor(
                    m.map_or(WindowMode::Windowed, |m| WindowMode::FullScreen(m)),
                    xpos as i32,
                    ypos as i32,
                    width as u32,
                    height as u32,
                    None,
                )
            }),
            _ => return Err(format!("Invalid window state: '{}'", mode)),
        }
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("set_window_state() requires a single string argument".to_string())
    }
}

pub fn set_window_x(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(x)] = args {
        state.window.set_pos(*x as i32, state.window.get_pos().1);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("set_window_x() requires a single numeric argument".to_string())
    }
}

pub fn set_window_y(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(y)] = args {
        state.window.set_pos(state.window.get_pos().0, *y as i32);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("set_window_y() requires a single numeric argument".to_string())
    }
}

pub fn set_window_position(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(x), Value::Number(y)] = args {
        state.window.set_pos(*x as i32, *y as i32);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("set_window_position() requires two numeric arguments".to_string())
    }
}

pub fn pointer_grab(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Boolean(grab)] = args {
        if *grab {
            state.window.set_cursor_mode(glfw::CursorMode::Disabled);
        } else {
            state.window.set_cursor_mode(glfw::CursorMode::Normal);
        }
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("pointer_grab() requires a single boolean argument".to_string())
    }
}

pub fn window_width(state: &mut State) -> Result {
    Ok(Rc::new(RefCell::new(Value::Number(
        state.window.get_size().0 as f32,
    ))))
}

pub fn window_height(state: &mut State) -> Result {
    Ok(Rc::new(RefCell::new(Value::Number(
        state.window.get_size().1 as f32,
    ))))
}
