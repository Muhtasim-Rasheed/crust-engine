use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::utils::*;

pub type IntermediateResult = std::result::Result<Value, String>;
type Result = std::result::Result<Rc<RefCell<Value>>, String>;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub args: Vec<String>,
    pub body: Vec<Statement>,
    pub returns: Expression,
    pub captured_vars: HashMap<String, Rc<RefCell<Value>>>,
}

impl Function {
    fn call(&self, state: &mut State, args: &[Rc<RefCell<Value>>]) -> Result {
        if args.len() != self.args.len() {
            return Err(format!(
                "Called with incorrect number of arguments: expected {}, got {}",
                self.args.len(),
                args.len()
            ));
        }

        let mut new_local_vars = state.local_vars.clone();
        for (i, arg) in self.args.iter().enumerate() {
            new_local_vars.insert(arg.clone(), args[i].clone());
        }
        for (name, value) in &self.captured_vars {
            new_local_vars.insert(name.clone(), value.clone());
        }
        let mut new_state = State {
            start: state.start,
            dt: state.dt,
            sprite: state.sprite,
            project: state.project,
            snapshots: state.snapshots,
            window: state.window,
            input_manager: state.input_manager,
            glfw: state.glfw,
            audio_manager: state.audio_manager,
            shader_program: state.shader_program,
            projection: state.projection,
            font: state.font,
            local_vars: &mut new_local_vars,
            script_id: state.script_id,
        };

        for statement in &self.body {
            Sprite::execute_statement(statement, &mut new_state);
        }

        Ok(crate::utils::resolve_expression(
            &self.returns,
            &mut new_state,
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuiltinFunction {
    pub inner: fn(&mut State, &[Rc<RefCell<Value>>]) -> IntermediateResult,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Function(Function),
    Builtin(BuiltinFunction),
}

impl Callable {
    pub fn call(&self, state: &mut State, args: &[Rc<RefCell<Value>>]) -> Result {
        match self {
            Callable::Function(func) => func.call(state, args),
            Callable::Builtin(builtin) => {
                (builtin.inner)(state, args).map(RefCell::new).map(Rc::new)
            }
        }
    }
}
