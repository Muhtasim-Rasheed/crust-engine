use crate::utils::*;

pub fn wait(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::Number(seconds)] = args {
        sprite.time_waiting = (*seconds * 60.0) as u32;
        Ok(Value::Null)
    } else {
        Err("wait() requires a single number argument".to_string())
    }
}

pub fn stop(sprite: &mut Sprite, script_id: usize, args: &[Value]) -> Result {
    if let [Value::String(action)] = args {
        match action.as_str() {
            "all" => {
                sprite.stop_request = Some(StopRequest::All);
                Ok(Value::Null)
            }
            "this" => {
                sprite.stop_request = Some(StopRequest::This);
                Ok(Value::Null)
            }
            "script" => {
                sprite.stop_request = Some(StopRequest::Script(script_id));
                Ok(Value::Null)
            }
            "other-scripts" => {
                sprite.stop_request = Some(StopRequest::OtherScripts(script_id));
                Ok(Value::Null)
            }
            "other-sprites-and-scripts" => {
                sprite.stop_request = Some(StopRequest::OtherSpritesAndScripts(script_id));
                Ok(Value::Null)
            }
            _ => Err(format!(
                "Invalid action: '{}'. Use 'all' or 'this'.",
                action
            )),
        }
    } else {
        Err("stop() requires a single string argument".to_string())
    }
}

pub fn clone(sprite: &mut Sprite) -> Result {
    sprite.clones.push(sprite.new_clone());
    Ok(Value::Null)
}

pub fn delete_clone(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::Number(cloneid)] = args {
        if let Some(index) = sprite
            .clones
            .iter()
            .position(|c| c.name == format!("{} (clone {})", sprite.name, cloneid))
        {
            sprite.clones.remove(index);
        }
    } else {
        sprite.delete_pending = true;
    }
    Ok(Value::Null)
}

pub fn skip_further_execution_if(sprite: &mut Sprite, args: &[Value]) -> Result {
    if let [Value::Boolean(condition)] = args {
        sprite.skip_further_execution_of_frame = *condition;
        Ok(Value::Null)
    } else {
        Err("skip_further_execution_if() requires a single boolean argument".to_string())
    }
}
