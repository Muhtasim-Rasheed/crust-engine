use std::{cell::RefCell, rc::Rc};

use crate::utils::{sprite::Dialogue, *};

pub fn hide(state: &mut State) -> Result {
    state.sprite.visible = false;
    Ok(Rc::new(RefCell::new(Value::Null)))
}

pub fn show(state: &mut State) -> Result {
    state.sprite.visible = true;
    Ok(Rc::new(RefCell::new(Value::Null)))
}

pub fn say(state: &mut State, args: &[&mut Value]) -> Result {
    match args {
        [text] => {
            state.sprite.dialogue = Some(Dialogue {
                text: text.to_string(),
                duration: f32::INFINITY,
                think: false,
            });
            Ok(Rc::new(RefCell::new(Value::Null)))
        }
        [text, Value::Number(duration)] => {
            state.sprite.dialogue = Some(Dialogue {
                text: text.to_string(),
                duration: *duration * 60.0,
                think: false,
            });
            Ok(Rc::new(RefCell::new(Value::Null)))
        }
        _ => Err("say() requires a text argument or a text and duration".to_string()),
    }
}

pub fn think(state: &mut State, args: &[&mut Value]) -> Result {
    match args {
        [text] => {
            state.sprite.dialogue = Some(Dialogue {
                text: text.to_string(),
                duration: f32::INFINITY,
                think: true,
            });
            Ok(Rc::new(RefCell::new(Value::Null)))
        }
        [text, Value::Number(duration)] => {
            state.sprite.dialogue = Some(Dialogue {
                text: text.to_string(),
                duration: *duration * 60.0,
                think: true,
            });
            Ok(Rc::new(RefCell::new(Value::Null)))
        }
        _ => Err("think() requires a text argument or a text and duration".to_string()),
    }
}

pub fn switch_costume(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(index)] = args {
        state.sprite.set_costume(*index as usize);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("switch_costume() requires a single string argument".to_string())
    }
}

pub fn next_costume(state: &mut State) -> Result {
    state.sprite.next_costume();
    Ok(Rc::new(RefCell::new(Value::Null)))
}

pub fn previous_costume(state: &mut State) -> Result {
    state.sprite.prev_costume();
    Ok(Rc::new(RefCell::new(Value::Null)))
}

pub fn switch_backdrop(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(index)] = args {
        state.project.stage.set_backdrop(*index as usize);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("switch_backdrop() requires a single string argument".to_string())
    }
}

pub fn next_backdrop(state: &mut State) -> Result {
    state.project.stage.next_backdrop();
    Ok(Rc::new(RefCell::new(Value::Null)))
}

pub fn previous_backdrop(state: &mut State) -> Result {
    state.project.stage.prev_backdrop();
    Ok(Rc::new(RefCell::new(Value::Null)))
}

pub fn set_scale(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(scale)] = args {
        state.sprite.scale = *scale / 100.0;
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("set_scale() requires a single numeric argument".to_string())
    }
}

pub fn change_scale(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(scale)] = args {
        state.sprite.scale += *scale / 100.0;
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("change_scale() requires a single numeric argument".to_string())
    }
}

pub fn set_effect(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::String(effect), Value::Number(value)] = args {
        state.sprite.effects.insert(effect.to_string(), *value);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("set_effect() requires a string and a numeric argument".to_string())
    }
}

pub fn change_effect(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::String(effect), Value::Number(value)] = args {
        state
            .sprite
            .effects
            .entry(effect.to_string())
            .and_modify(|v| *v += *value)
            .or_insert(*value);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("change_effect() requires a string and a numeric argument".to_string())
    }
}

pub fn clear_effects(state: &mut State) -> Result {
    state.sprite.effects.clear();
    Ok(Rc::new(RefCell::new(Value::Null)))
}

pub fn clear_effect(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::String(effect)] = args {
        state.sprite.effects.shift_remove(effect);
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("clear_effect() requires a single string argument".to_string())
    }
}

pub fn go_to_layer(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::Number(layer)] = args {
        state.sprite.layer = *layer as isize;
        Ok(Rc::new(RefCell::new(Value::Null)))
    } else {
        Err("go_to_layer() requires a single numeric argument".to_string())
    }
}

pub fn go_by_layers(state: &mut State, args: &[&mut Value]) -> Result {
    if let [Value::String(direction), Value::Number(steps)] = args {
        if direction == "forwards" {
            state.sprite.layer += *steps as isize;
            Ok(Rc::new(RefCell::new(Value::Null)))
        } else if direction == "backwards" {
            state.sprite.layer -= *steps as isize;
            Ok(Rc::new(RefCell::new(Value::Null)))
        } else {
            Err(
                "go_by_layers() requires 'forwards' or 'backwards' as the first argument"
                    .to_string(),
            )
        }
    } else {
        Err("go_by_layers() requires a string and a numeric argument".to_string())
    }
}

pub fn costume(state: &State) -> Result {
    Ok(Rc::new(RefCell::new(Value::Number(
        state.sprite.costume() as f32
    ))))
}

pub fn backdrop(state: &State) -> Result {
    Ok(Rc::new(RefCell::new(Value::Number(
        state.project.stage.backdrop() as f32,
    ))))
}

pub fn size(state: &State) -> Result {
    Ok(Rc::new(RefCell::new(Value::List(vec![
        Rc::new(RefCell::new(Value::Number(state.sprite.size.x))),
        Rc::new(RefCell::new(Value::Number(state.sprite.size.y))),
    ]))))
}

pub fn scale(state: &State) -> Result {
    Ok(Rc::new(RefCell::new(Value::Number(
        state.sprite.scale * 100.0,
    ))))
}

pub fn bounds(state: &State) -> Result {
    Ok(Rc::new(RefCell::new(Value::List(vec![
        Rc::new(RefCell::new(Value::Number(
            state.sprite.center.x - state.sprite.size.x * state.sprite.scale,
        ))),
        Rc::new(RefCell::new(Value::Number(
            state.sprite.center.y - state.sprite.size.y * state.sprite.scale,
        ))),
        Rc::new(RefCell::new(Value::Number(
            state.sprite.center.x + state.sprite.size.x * state.sprite.scale,
        ))),
        Rc::new(RefCell::new(Value::Number(
            state.sprite.center.y + state.sprite.size.y * state.sprite.scale,
        ))),
    ]))))
}

pub fn layer(state: &State) -> Result {
    Ok(Rc::new(RefCell::new(Value::Number(
        state.sprite.layer as f32,
    ))))
}

pub fn effect(state: &State, args: &[&mut Value]) -> Result {
    if let [Value::String(effect)] = args {
        Ok(Rc::new(RefCell::new(Value::Number(
            *state.sprite.effects.get(effect).unwrap_or(&0.0) as f32,
        ))))
    } else {
        Err("effect() requires a single string argument".to_string())
    }
}
