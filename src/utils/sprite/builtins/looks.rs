use crate::utils::{sprite::Dialogue, *};

pub fn hide(state: &mut State) -> IntermediateResult {
    state.sprite.visible = false;
    Ok(Value::Null)
}

pub fn show(state: &mut State) -> IntermediateResult {
    state.sprite.visible = true;
    Ok(Value::Null)
}

pub fn say(state: &mut State, args: &[Value]) -> IntermediateResult {
    match args {
        [text] => {
            state.sprite.dialogue = Some(Dialogue {
                text: text.to_string(),
                duration: f32::INFINITY,
                think: false,
            });
            Ok(Value::Null)
        }
        [text, Value::Number(duration)] => {
            state.sprite.dialogue = Some(Dialogue {
                text: text.to_string(),
                duration: *duration * 60.0,
                think: false,
            });
            Ok(Value::Null)
        }
        _ => Err("say() requires a text argument or a text and duration".to_string()),
    }
}

pub fn think(state: &mut State, args: &[Value]) -> IntermediateResult {
    match args {
        [text] => {
            state.sprite.dialogue = Some(Dialogue {
                text: text.to_string(),
                duration: f32::INFINITY,
                think: true,
            });
            Ok(Value::Null)
        }
        [text, Value::Number(duration)] => {
            state.sprite.dialogue = Some(Dialogue {
                text: text.to_string(),
                duration: *duration * 60.0,
                think: true,
            });
            Ok(Value::Null)
        }
        _ => Err("think() requires a text argument or a text and duration".to_string()),
    }
}

pub fn switch_costume(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::Number(index)] = args {
        state.sprite.set_costume(*index as usize);
        Ok(Value::Null)
    } else {
        Err("switch_costume() requires a single string argument".to_string())
    }
}

pub fn next_costume(state: &mut State) -> IntermediateResult {
    state.sprite.next_costume();
    Ok(Value::Null)
}

pub fn previous_costume(state: &mut State) -> IntermediateResult {
    state.sprite.prev_costume();
    Ok(Value::Null)
}

pub fn switch_backdrop(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::Number(index)] = args {
        state.project.stage.set_backdrop(*index as usize);
        Ok(Value::Null)
    } else {
        Err("switch_backdrop() requires a single string argument".to_string())
    }
}

pub fn next_backdrop(state: &mut State) -> IntermediateResult {
    state.project.stage.next_backdrop();
    Ok(Value::Null)
}

pub fn previous_backdrop(state: &mut State) -> IntermediateResult {
    state.project.stage.prev_backdrop();
    Ok(Value::Null)
}

pub fn set_scale(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::Number(scale)] = args {
        state.sprite.scale = *scale / 100.0;
        Ok(Value::Null)
    } else {
        Err("set_scale() requires a single numeric argument".to_string())
    }
}

pub fn change_scale(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::Number(scale)] = args {
        state.sprite.scale += *scale / 100.0;
        Ok(Value::Null)
    } else {
        Err("change_scale() requires a single numeric argument".to_string())
    }
}

pub fn set_effect(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::String(effect), Value::Number(value)] = args {
        state.sprite.effects.insert(effect.to_string(), *value);
        Ok(Value::Null)
    } else {
        Err("set_effect() requires a string and a numeric argument".to_string())
    }
}

pub fn change_effect(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::String(effect), Value::Number(value)] = args {
        state
            .sprite
            .effects
            .entry(effect.to_string())
            .and_modify(|v| *v += *value)
            .or_insert(*value);
        Ok(Value::Null)
    } else {
        Err("change_effect() requires a string and a numeric argument".to_string())
    }
}

pub fn clear_effects(state: &mut State) -> IntermediateResult {
    state.sprite.effects.clear();
    Ok(Value::Null)
}

pub fn clear_effect(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::String(effect)] = args {
        state.sprite.effects.shift_remove(effect);
        Ok(Value::Null)
    } else {
        Err("clear_effect() requires a single string argument".to_string())
    }
}

pub fn go_to_layer(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::Number(layer)] = args {
        state.sprite.layer = *layer as isize;
        Ok(Value::Null)
    } else {
        Err("go_to_layer() requires a single numeric argument".to_string())
    }
}

pub fn go_by_layers(state: &mut State, args: &[Value]) -> IntermediateResult {
    if let [Value::String(direction), Value::Number(steps)] = args {
        if direction == "forwards" {
            state.sprite.layer += *steps as isize;
            Ok(Value::Null)
        } else if direction == "backwards" {
            state.sprite.layer -= *steps as isize;
            Ok(Value::Null)
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

pub fn costume(state: &State) -> IntermediateResult {
    Ok(Value::Number(state.sprite.costume() as f32))
}

pub fn backdrop(state: &State) -> IntermediateResult {
    Ok(Value::Number(state.project.stage.backdrop() as f32))
}

pub fn size(state: &State) -> IntermediateResult {
    Ok(Value::list(vec![
        Value::Number(state.sprite.size.x),
        Value::Number(state.sprite.size.y),
    ]))
}

pub fn scale(state: &State) -> IntermediateResult {
    Ok(Value::Number(state.sprite.scale * 100.0))
}

pub fn bounds(state: &State) -> IntermediateResult {
    Ok(Value::list(vec![
        Value::Number(state.sprite.center.x - state.sprite.size.x * state.sprite.scale),
        Value::Number(state.sprite.center.y - state.sprite.size.y * state.sprite.scale),
        Value::Number(state.sprite.center.x + state.sprite.size.x * state.sprite.scale),
        Value::Number(state.sprite.center.y + state.sprite.size.y * state.sprite.scale),
    ]))
}

pub fn layer(state: &State) -> IntermediateResult {
    Ok(Value::Number(state.sprite.layer as f32))
}

pub fn effect(state: &State, args: &[Value]) -> IntermediateResult {
    if let [Value::String(effect)] = args {
        Ok(Value::Number(
            *state.sprite.effects.get(effect).unwrap_or(&0.0),
        ))
    } else {
        Err("effect() requires a single string argument".to_string())
    }
}
