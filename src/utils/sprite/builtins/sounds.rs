use kira::Tween;

use crate::utils::{sprite::function::Result, *};

pub fn play_sound(state: &mut State, args: &[Value]) -> Result {
    fn play_sound_inner(
        state: &mut State,
        name: &str,
    ) -> Result {
        if let Some(sound) = state.sprite.sounds.get(name) {
            let handle = state.audio_manager.play(sound.clone())
                .map_err(|e| e.to_string())?;
            state.sprite.sound_handles.insert(name.to_string(), handle);
            Ok(Value::Null)
        } else {
            Err(format!("Sound '{}' not found", name))
        }
    }
    match args {
        [Value::String(name)] => {
            play_sound_inner(state, name)
        }
        [Value::String(name), Value::Boolean(stop_other)] => {
            if *stop_other {
                stop_all_sounds(state)?;
            }
            play_sound_inner(state, name)
        }
        _ => Err("play_sound() requires a single string argument".to_string()),
    }
}

pub fn stop_all_sounds(state: &mut State) -> Result {
    for sound_handle in state.sprite.sound_handles.values_mut() {
        sound_handle.stop(Tween::default());
    }
    Ok(Value::Null)
}

pub fn stop_sound(state: &mut State, args: &[Value]) -> Result {
    if let [Value::String(name)] = args {
        if let Some(sound_handle) = state.sprite.sound_handles.get_mut(name) {
            sound_handle.stop(Tween::default());
            Ok(Value::Null)
        } else {
            Err(format!("Sound '{}' not found", name))
        }
    } else {
        Err("stop_sound() requires a single string argument".to_string())
    }
}

pub fn change_sound_effect(state: &mut State, args: &[Value]) -> Result {
    if let [Value::String(effect), Value::Number(value)] = args {
        state.sprite
            .sound_effects
            .entry(effect.clone())
            .and_modify(|v| *v += *value)
            .or_insert(*value);
        Ok(Value::Null)
    } else {
        Err("change_sound_effect() requires a string and a numeric argument".to_string())
    }
}

pub fn set_sound_effect(state: &mut State, args: &[Value]) -> Result {
    if let [Value::String(effect), Value::Number(value)] = args {
        state.sprite.sound_effects.insert(effect.clone(), *value);
        Ok(Value::Null)
    } else {
        Err("set_sound_effect() requires a string and a numeric argument".to_string())
    }
}

pub fn sound_effect(state: &State, args: &[Value]) -> Result {
    if let [Value::String(effect)] = args {
        if let Some(value) = state.sprite.sound_effects.get(effect) {
            Ok(Value::Number(*value))
        } else {
            Err(format!("Sound effect '{}' not found", effect))
        }
    } else {
        Err("sound_effect() requires a single string argument".to_string())
    }
}
