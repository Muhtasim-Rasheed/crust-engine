// use macroquad::{
//     input::{set_cursor_grab, show_mouse},
//     window::*,
// };

// use crate::utils::*;

// pub fn set_window_width(args: &[Value]) -> Result {
//     if let [Value::Number(width)] = args {
//         request_new_screen_size(*width, screen_height());
//         Ok(Value::Null)
//     } else {
//         Err("set_window_width() requires a single numeric argument".to_string())
//     }
// }

// pub fn set_window_height(args: &[Value]) -> Result {
//     if let [Value::Number(height)] = args {
//         request_new_screen_size(screen_width(), *height);
//         Ok(Value::Null)
//     } else {
//         Err("set_window_height() requires a single numeric argument".to_string())
//     }
// }

// pub fn set_window_size(args: &[Value]) -> Result {
//     if let [Value::Number(width), Value::Number(height)] = args {
//         request_new_screen_size(*width, *height);
//         Ok(Value::Null)
//     } else {
//         Err("set_window_size() requires two numeric arguments".to_string())
//     }
// }

// pub fn set_window_state(args: &[Value]) -> Result {
//     if let [Value::String(state)] = args {
//         match state.as_str() {
//             "normal" => set_fullscreen(false),
//             "fullscreen" => set_fullscreen(true),
//             _ => return Err(format!("Invalid window state: '{}'", state)),
//         }
//         Ok(Value::Null)
//     } else {
//         Err("set_window_state() requires a single string argument".to_string())
//     }
// }

// pub fn set_window_x(args: &[Value]) -> Result {
//     if let [Value::Number(x)] = args {
//         macroquad::miniquad::window::set_window_position(
//             *x as u32,
//             macroquad::miniquad::window::get_window_position().1,
//         );
//         Ok(Value::Null)
//     } else {
//         Err("set_window_x() requires a single numeric argument".to_string())
//     }
// }

// pub fn set_window_y(args: &[Value]) -> Result {
//     if let [Value::Number(y)] = args {
//         macroquad::miniquad::window::set_window_position(
//             macroquad::miniquad::window::get_window_position().0,
//             *y as u32,
//         );
//         Ok(Value::Null)
//     } else {
//         Err("set_window_y() requires a single numeric argument".to_string())
//     }
// }

// pub fn set_window_position(args: &[Value]) -> Result {
//     if let [Value::Number(x), Value::Number(y)] = args {
//         macroquad::miniquad::window::set_window_position(*x as u32, *y as u32);
//         Ok(Value::Null)
//     } else {
//         Err("set_window_position() requires two numeric arguments".to_string())
//     }
// }

// pub fn pointer_grab(args: &[Value]) -> Result {
//     if let [Value::Boolean(grab)] = args {
//         set_cursor_grab(*grab);
//         show_mouse(!grab);
//         Ok(Value::Null)
//     } else {
//         Err("pointer_grab() requires a single boolean argument".to_string())
//     }
// }

// pub fn window_width() -> Result {
//     Ok(Value::Number(screen_width()))
// }

// pub fn window_height() -> Result {
//     Ok(Value::Number(screen_height()))
// }
