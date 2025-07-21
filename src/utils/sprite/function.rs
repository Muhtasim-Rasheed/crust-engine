use macroquad::prelude::Camera2D;

use crate::utils::*;

pub type Result = std::result::Result<Value, String>;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub args: Vec<String>,
    pub body: Vec<Statement>,
    pub returns: Expression,
}

impl Function {
    fn call(
        &self,
        sprite: &mut Sprite,
        project: &mut Project,
        snapshots: &[SpriteSnapshot],
        camera: &Camera2D,
        local_vars: &[(String, Value)],
        script_id: usize,
        args: &[Value],
    ) -> Result {
        if args.len() != self.args.len() {
            return Err(format!(
                "Called with incorrect number of arguments: expected {}, got {}",
                self.args.len(),
                args.len()
            ));
        }

        let mut new_local_vars = local_vars.to_vec();
        for (i, arg) in self.args.iter().enumerate() {
            new_local_vars.push((arg.clone(), args[i].clone()));
        }

        for statement in &self.body {
            sprite.execute_statement(
                statement,
                project,
                snapshots,
                camera,
                &new_local_vars,
                script_id,
            );
        }

        Ok(crate::utils::resolve_expression(
            &self.returns,
            project,
            sprite,
            &new_local_vars,
            snapshots,
            camera,
            script_id,
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuiltinFunction {
    pub inner: fn(
        &mut Sprite,
        &mut Project,
        &[SpriteSnapshot],
        &Camera2D,
        &[(String, Value)],
        usize,
        &[Value],
    ) -> Result,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Function(Function),
    Builtin(BuiltinFunction),
}

impl Callable {
    pub fn call(
        &self,
        sprite: &mut Sprite,
        project: &mut Project,
        snapshots: &[SpriteSnapshot],
        camera: &Camera2D,
        local_vars: &[(String, Value)],
        script_id: usize,
        args: &[Value],
    ) -> Result {
        match self {
            Callable::Function(func) => func.call(
                sprite, project, snapshots, camera, local_vars, script_id, args,
            ),
            Callable::Builtin(builtin) => (builtin.inner)(
                sprite, project, snapshots, camera, local_vars, script_id, args,
            ),
        }
    }
}
