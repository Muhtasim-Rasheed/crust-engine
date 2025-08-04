use crate::utils::*;

pub fn wait(state: &mut State, args: &[Value]) -> Result {
    if let [Value::Number(seconds)] = args {
        state.sprite.time_waiting = (*seconds * 60.0) as u32;
        Ok(Value::Null)
    } else {
        Err("wait() requires a single number argument".to_string())
    }
}

pub fn stop(state: &mut State, args: &[Value]) -> Result {
    if let [Value::String(action)] = args {
        match action.as_str() {
            "all" => {
                state.sprite.stop_request = Some(StopRequest::All);
                Ok(Value::Null)
            }
            "this" => {
                state.sprite.stop_request = Some(StopRequest::This);
                Ok(Value::Null)
            }
            "script" => {
                state.sprite.stop_request = Some(StopRequest::Script(state.script_id));
                Ok(Value::Null)
            }
            "other-scripts" => {
                state.sprite.stop_request = Some(StopRequest::OtherScripts(state.script_id));
                Ok(Value::Null)
            }
            "other-sprites-and-scripts" => {
                state.sprite.stop_request =
                    Some(StopRequest::OtherSpritesAndScripts(state.script_id));
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

pub fn clone(state: &mut State) -> Result {
    state.sprite.clones.push(state.sprite.new_clone());
    Ok(Value::Null)
}

pub fn delete_clone(state: &mut State, args: &[Value]) -> Result {
    if let [Value::Number(cloneid)] = args {
        if let Some(index) = state
            .sprite
            .clones
            .iter()
            .position(|c| c.name == format!("{} (clone {})", state.sprite.name, cloneid))
        {
            state.sprite.clones.remove(index);
        }
    } else {
        state.sprite.delete_pending = true;
    }
    Ok(Value::Null)
}

pub fn skip_further_execution_if(state: &mut State, args: &[Value]) -> Result {
    if let [Value::Boolean(condition)] = args {
        state.sprite.skip_further_execution_of_frame = *condition;
        Ok(Value::Null)
    } else {
        Err("skip_further_execution_if() requires a single boolean argument".to_string())
    }
}
