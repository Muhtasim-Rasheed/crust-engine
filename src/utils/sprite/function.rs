use std::{cell::RefCell, rc::Rc};

use crate::utils::*;

pub type Result = std::result::Result<Rc<RefCell<Value>>, String>;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub args: Vec<String>,
    pub body: Vec<Statement>,
    pub returns: Expression,
    pub captured_vars: Vec<(String, Rc<RefCell<Value>>)>,
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

        let mut new_local_vars = state.local_vars.to_vec();
        for (i, arg) in self.args.iter().enumerate() {
            new_local_vars.push((arg.clone(), args[i].clone()));
        }
        for (name, value) in &self.captured_vars {
            new_local_vars.push((name.clone(), value.clone()));
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
            local_vars: new_local_vars.as_mut_slice(),
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

#[derive(Debug, Clone)]
pub struct BuiltinFunction {
    pub name: String,
    pub inner: fn(&mut State, &[Rc<RefCell<Value>>]) -> Result,
}

impl PartialEq for BuiltinFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
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
            Callable::Builtin(builtin) => (builtin.inner)(state, args),
        }
    }
}
