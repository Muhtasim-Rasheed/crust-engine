use indexmap::IndexMap;
use kira::{Tween, sound::static_sound::StaticSoundHandle};

use crate::utils::{sprite::function::Result, *};

fn update_sound_handle(sound_filters: &IndexMap<String, f32>, handle: &mut StaticSoundHandle) {
    for (effect, value) in sound_filters {
        match effect.as_str() {
            "volume" => handle.set_volume(percentage_to_decibels(*value / 100.0), Tween::default()),
            "pitch" => handle.set_playback_rate(*value as f64 / 100.0, Tween::default()),
            "pan" => handle.set_panning((*value / 100.0) * 2.0 - 1.0, Tween::default()),
            _ => continue,
        }
    }
}

pub fn play_sound(state: &mut State, args: &[Value]) -> Result {
    fn play_sound_inner(state: &mut State, name: &str) -> Result {
        if let Some(sound) = state.sprite.sounds.get(name) {
            let mut handle = state
                .audio_manager
                .play(sound.clone())
                .map_err(|e| e.to_string())?;
            update_sound_handle(&state.sprite.sound_filters, &mut handle);
            state.sprite.sound_handles.insert(name.to_string(), handle);
            Ok(Value::Null)
        } else {
            Err(format!("Sound '{}' not found", name))
        }
    }
    match args {
        [Value::String(name)] => play_sound_inner(state, name),
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

pub fn change_sound_filter(state: &mut State, args: &[Value]) -> Result {
    if let [Value::String(effect), Value::Number(value)] = args {
        state
            .sprite
            .sound_filters
            .entry(effect.clone())
            .and_modify(|v| *v += *value)
            .or_insert(*value);
        for sound_handle in state.sprite.sound_handles.values_mut() {
            update_sound_handle(&state.sprite.sound_filters, sound_handle);
        }
        Ok(Value::Null)
    } else {
        Err("change_sound_filter() requires a string and a numeric argument".to_string())
    }
}

pub fn set_sound_filter(state: &mut State, args: &[Value]) -> Result {
    if let [Value::String(effect), Value::Number(value)] = args {
        state.sprite.sound_filters.insert(effect.clone(), *value);
        for sound_handle in state.sprite.sound_handles.values_mut() {
            update_sound_handle(&state.sprite.sound_filters, sound_handle);
        }
        Ok(Value::Null)
    } else {
        Err("set_sound_filter() requires a string and a numeric argument".to_string())
    }
}

pub fn sound_filter(state: &State, args: &[Value]) -> Result {
    if let [Value::String(effect)] = args {
        if let Some(value) = state.sprite.sound_filters.get(effect) {
            Ok(Value::Number(*value))
        } else {
            Err(format!("Sound effect '{}' not found", effect))
        }
    } else {
        Err("sound_filter() requires a single string argument".to_string())
    }
}
