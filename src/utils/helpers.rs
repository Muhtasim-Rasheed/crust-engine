use macroquad::prelude::*;

use super::{Expression, Project, Sprite, Value};

// Helper functions!

pub fn resolve_expression(expr: &Expression, project: &Project, sprite: &Sprite) -> Value {
    match expr {
        Expression::Value(v) => v.clone(),
        Expression::Identifier(id) => sprite.variable(id).clone(),
        Expression::Binary { left, right, operator } => {
            let left_value = resolve_expression(left, project, sprite);
            let right_value = resolve_expression(right, project, sprite);
            match operator.as_str() {
                "+" => Value::Number(left_value.to_number() + right_value.to_number()),
                "-" => Value::Number(left_value.to_number() - right_value.to_number()),
                "*" => Value::Number(left_value.to_number() * right_value.to_number()),
                "/" => Value::Number(left_value.to_number() / right_value.to_number()),
                "==" => Value::Boolean(left_value == right_value),
                "!=" => Value::Boolean(left_value != right_value),
                "<" => Value::Boolean(left_value.to_number() < right_value.to_number()),
                ">" => Value::Boolean(left_value.to_number() > right_value.to_number()),
                "<=" => Value::Boolean(left_value.to_number() <= right_value.to_number()),
                ">=" => Value::Boolean(left_value.to_number() >= right_value.to_number()),
                _ => panic!("Unknown operator: {}", operator),
            }
        }
        Expression::Call { function, args } => {
            // let sprite = project.get_sprite(&sprite).unwrap().clone();
            let args = args.iter()
                .map(|arg| resolve_expression(arg, project, sprite))
                .collect::<Vec<_>>();
            match function.as_str() {
                "direction" => Value::Number(sprite.direction),
                "x" => Value::Number(sprite.center.x),
                "y" => Value::Number(sprite.center.y),
                "mouse_x" => Value::Number(mouse_position().0),
                "mouse_y" => Value::Number(mouse_position().1),
                "costume" => Value::Number(sprite.costume() as f32),
                "backdrop" => Value::Number(project.stage.backdrop() as f32),
                "size" => Value::Number(sprite.scale * 100.0),
                "key_down" => {
                    if let [Value::String(key)] = args.as_slice() {
                        Value::Boolean(is_key_down(string_to_keycode(key).unwrap_or(KeyCode::Unknown)))
                    } else {
                        Value::Null
                    }
                }
                "concat" => {
                    let mut result = String::new();
                    for arg in args {
                        result.push_str(&arg.to_string());
                    }
                    Value::String(result)
                }
                _ => Value::Null
            }
        }
    }
}

pub fn evaluate_bezier(t: f32, ctrl1_y: f32, ctrl2_y: f32) -> f32 {
    let steps = 20;
    let mut closest_y = 0.0;
    let mut min_diff = f32::MAX;

    for i in 0..=steps {
        let guess = i as f32 / steps as f32;
        let x = cubic_bezier(guess, 0.0, 0.42, 0.58, 1.0); // or ctrl1_x, ctrl2_x
        let diff = (x - t).abs();

        if diff < min_diff {
            min_diff = diff;
            closest_y = cubic_bezier(guess, 0.0, ctrl1_y, ctrl2_y, 1.0);
        }
    }

    closest_y
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
