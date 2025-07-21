use crate::utils::{sprite::function::Result, *};

pub fn play_sound(sprite: &Sprite, args: &[Value]) -> Result {
    match args {
        [Value::String(name)] => {
            if let Some(sound) = sprite.sounds.get(name) {
                macroquad::audio::play_sound(
                    &sound,
                    macroquad::audio::PlaySoundParams {
                        looped: false,
                        volume: sprite.sound_effects.get("volume").cloned().unwrap_or(100.0)
                            / 100.0,
                    },
                );
                Ok(Value::Null)
            } else {
                Err(format!("Sound '{}' not found", name))
            }
        }
        [Value::String(name), Value::Boolean(stop_other)] => {
            if *stop_other {
                stop_all_sounds(sprite)?;
            }
            if let Some(sound) = sprite.sounds.get(name) {
                macroquad::audio::play_sound(
                    &sound,
                    macroquad::audio::PlaySoundParams {
                        looped: false,
                        volume: sprite.sound_effects.get("volume").cloned().unwrap_or(100.0)
                            / 100.0,
                    },
                );
                Ok(Value::Null)
            } else {
                Err(format!("Sound '{}' not found", name))
            }
        }
        _ => Err("play_sound() requires a single string argument".to_string()),
    }
}

pub fn stop_all_sounds(sprite: &Sprite) -> Result {
    for sound in sprite.sounds.values() {
        macroquad::audio::stop_sound(sound);
    }
    Ok(Value::Null)
}

pub fn stop_sound(sprite: &Sprite, args: &[Value]) -> Result {
    if let [Value::String(name)] = args {
        if let Some(sound) = sprite.sounds.get(name) {
            macroquad::audio::stop_sound(sound);
            Ok(Value::Null)
        } else {
            Err(format!("Sound '{}' not found", name))
        }
    } else {
        Err("stop_sound() requires a single string argument".to_string())
    }
}

pub fn change_sound_effect(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::String(effect), Value::Number(value)] = args {
        sprite
            .sound_effects
            .entry(effect.clone())
            .and_modify(|v| *v += *value)
            .or_insert(*value);
        Ok(Value::Null)
    } else {
        Err("change_sound_effect() requires a string and a numeric argument".to_string())
    }
}

pub fn set_sound_effect(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::String(effect), Value::Number(value)] = args {
        sprite.sound_effects.insert(effect.clone(), *value);
        Ok(Value::Null)
    } else {
        Err("set_sound_effect() requires a string and a numeric argument".to_string())
    }
}

pub fn sound_effect(sprite: &Sprite, args: &[Value]) -> Result {
    if let [Value::String(effect)] = args {
        if let Some(value) = sprite.sound_effects.get(effect) {
            Ok(Value::Number(*value))
        } else {
            Err(format!("Sound effect '{}' not found", effect))
        }
    } else {
        Err("sound_effect() requires a single string argument".to_string())
    }
}
