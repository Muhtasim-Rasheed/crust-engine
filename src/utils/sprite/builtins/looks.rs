use crate::utils::{sprite::Dialogue, *};

pub fn hide(sprite: &mut Sprite) -> Result {
    sprite.visible = false;
    Ok(Value::Null)
}

pub fn show(sprite: &mut Sprite) -> Result {
    sprite.visible = true;
    Ok(Value::Null)
}

pub fn say(sprite: &mut Sprite, args: &[Value]) -> Result {
    match args {
        [text] => {
            sprite.dialogue = Some(Dialogue {
                text: text.to_string(),
                duration: f32::INFINITY,
                think: false,
            });
            Ok(Value::Null)
        }
        [text, Value::Number(duration)] => {
            sprite.dialogue = Some(Dialogue {
                text: text.to_string(),
                duration: *duration * 60.0,
                think: false,
            });
            Ok(Value::Null)
        }
        _ => Err("say() requires a text argument or a text and duration".to_string()),
    }
}

pub fn think(sprite: &mut Sprite, args: &[Value]) -> Result {
    match args {
        [text] => {
            sprite.dialogue = Some(Dialogue {
                text: text.to_string(),
                duration: f32::INFINITY,
                think: true,
            });
            Ok(Value::Null)
        }
        [text, Value::Number(duration)] => {
            sprite.dialogue = Some(Dialogue {
                text: text.to_string(),
                duration: *duration * 60.0,
                think: true,
            });
            Ok(Value::Null)
        }
        _ => Err("think() requires a text argument or a text and duration".to_string()),
    }
}

pub fn switch_costume(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::Number(index)] = args {
        sprite.set_costume(*index as usize);
        Ok(Value::Null)
    } else {
        Err("switch_costume() requires a single string argument".to_string())
    }
}

pub fn next_costume(sprite: &mut Sprite) -> Result {
    sprite.next_costume();
    Ok(Value::Null)
}

pub fn previous_costume(sprite: &mut Sprite) -> Result {
    sprite.prev_costume();
    Ok(Value::Null)
}

pub fn switch_backdrop(project: &mut Project, args: &[Value]) -> Result {
    if let [Value::Number(index)] = args {
        project.stage.set_backdrop(*index as usize);
        Ok(Value::Null)
    } else {
        Err("switch_backdrop() requires a single string argument".to_string())
    }
}

pub fn next_backdrop(project: &mut Project) -> Result {
    project.stage.next_backdrop();
    Ok(Value::Null)
}

pub fn previous_backdrop(project: &mut Project) -> Result {
    project.stage.prev_backdrop();
    Ok(Value::Null)
}

pub fn set_scale(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::Number(scale)] = args {
        sprite.scale = *scale / 100.0;
        Ok(Value::Null)
    } else {
        Err("set_scale() requires a single numeric argument".to_string())
    }
}

pub fn change_scale(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::Number(scale)] = args {
        sprite.scale += *scale / 100.0;
        Ok(Value::Null)
    } else {
        Err("change_scale() requires a single numeric argument".to_string())
    }
}

pub fn set_effect(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::String(effect), Value::Number(value)] = args {
        sprite.effects.insert(effect.to_string(), *value);
        Ok(Value::Null)
    } else {
        Err("set_effect() requires a string and a numeric argument".to_string())
    }
}

pub fn change_effect(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::String(effect), Value::Number(value)] = args {
        sprite
            .effects
            .entry(effect.to_string())
            .and_modify(|v| *v += *value)
            .or_insert(*value);
        Ok(Value::Null)
    } else {
        Err("change_effect() requires a string and a numeric argument".to_string())
    }
}

pub fn clear_effects(sprite: &mut Sprite) -> Result {
    sprite.effects.clear();
    Ok(Value::Null)
}

pub fn clear_effect(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::String(effect)] = args {
        sprite.effects.shift_remove(effect);
        Ok(Value::Null)
    } else {
        Err("clear_effect() requires a single string argument".to_string())
    }
}

pub fn go_to_layer(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::Number(layer)] = args {
        sprite.layer = *layer as isize;
        Ok(Value::Null)
    } else {
        Err("go_to_layer() requires a single numeric argument".to_string())
    }
}

pub fn go_by_layers(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::String(direction), Value::Number(steps)] = args {
        if direction == "forwards" {
            sprite.layer += *steps as isize;
            Ok(Value::Null)
        } else if direction == "backwards" {
            sprite.layer -= *steps as isize;
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

pub fn costume(sprite: &Sprite) -> Result {
    Ok(Value::Number(sprite.costume() as f32))
}

pub fn backdrop(project: &Project) -> Result {
    Ok(Value::Number(project.stage.backdrop() as f32))
}

pub fn size(sprite: &Sprite) -> Result {
    Ok(Value::List(vec![
        Value::Number(sprite.size.x),
        Value::Number(sprite.size.y),
    ]))
}

pub fn scale(sprite: &Sprite) -> Result {
    Ok(Value::Number(sprite.scale * 100.0))
}

pub fn bounds(sprite: &Sprite) -> Result {
    Ok(Value::List(vec![
        Value::Number(sprite.center.x - sprite.size.x * sprite.scale),
        Value::Number(sprite.center.y - sprite.size.y * sprite.scale),
        Value::Number(sprite.center.x + sprite.size.x * sprite.scale),
        Value::Number(sprite.center.y + sprite.size.y * sprite.scale),
    ]))
}

pub fn layer(sprite: &Sprite) -> Result {
    Ok(Value::Number(sprite.layer as f32))
}

pub fn effect(sprite: &Sprite, args: &[Value]) -> Result {
    if let [Value::String(effect)] = args {
        Ok(Value::Number(
            *sprite.effects.get(effect).unwrap_or(&0.0) as f32
        ))
    } else {
        Err("effect() requires a single string argument".to_string())
    }
}
