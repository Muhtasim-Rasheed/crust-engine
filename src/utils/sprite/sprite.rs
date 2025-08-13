// Hello fellow contributor, welcome to Crust's `sprite.rs` file!
//
// This file is still *pretty big* (~1100 lines) and contains the core logic for handling
// sprites, their behaviors, and interactions in the Crust game engine.
//
// Don't worry: You don't need to understand everything at once. Take your time to read through the code.
//
// We're actively refactoring this file to keep things modular and readable,
// and we're proud it's no longer a 2300-line monster.
//
// Happy coding!

use glam::*;
use indexmap::IndexMap;
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle};
use kira::{AudioManager, DefaultBackend};
use std::collections::HashMap;
use std::f32::consts::*;
use std::path::PathBuf;

use crate::utils::core::*;
use crate::utils::*;

pub struct State<'a> {
    pub start: std::time::Instant,
    pub dt: f32,
    pub sprite: &'a mut Sprite,
    pub project: &'a mut Project,
    pub snapshots: &'a [SpriteSnapshot],
    pub window: &'a mut glfw::Window,
    pub input_manager: &'a mut InputManager,
    pub glfw: &'a mut glfw::Glfw,
    pub audio_manager: &'a mut AudioManager<DefaultBackend>,
    pub shader_program: &'a ShaderProgram,
    pub projection: &'a mut Mat4,
    pub font: &'a BitmapFont,
    pub local_vars: &'a [(String, Value)],
    pub script_id: usize,
}

#[derive(Clone, Debug)]
pub struct SpriteSnapshot {
    pub name: String,
    pub center: Vec2,
    pub size: Vec2,
    pub scale: f32,
    pub direction: f32,
    pub completed_broadcasts: Vec<usize>,
    pub tags: Vec<String>,
}

impl SpriteSnapshot {
    pub fn get(&self, name: &str) -> Option<Value> {
        match name {
            "name" => Some(Value::String(self.name.clone())),
            "x" => Some(Value::Number(self.center.x)),
            "y" => Some(Value::Number(self.center.y)),
            "size" => Some(Value::List(vec![
                Value::Number(self.size.x),
                Value::Number(self.size.y),
            ])),
            "scale" => Some(Value::Number(self.scale)),
            "direction" => Some(Value::Number(self.direction)),
            "completed_broadcasts" => Some(Value::List(
                self.completed_broadcasts
                    .iter()
                    .map(|id| Value::Number(*id as f32))
                    .collect(),
            )),
            "tags" => Some(Value::List(
                self.tags.iter().map(|t| Value::String(t.clone())).collect(),
            )),
            _ => None,
        }
    }
}

impl From<&Sprite> for SpriteSnapshot {
    fn from(sprite: &Sprite) -> Self {
        SpriteSnapshot {
            name: sprite.name.clone(),
            center: sprite.center,
            size: sprite.size,
            scale: sprite.scale,
            direction: sprite.direction,
            completed_broadcasts: sprite.completed_broadcasts.clone(),
            tags: sprite.tags.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RotationStyle {
    AllAround,
    LeftRight,
    DontRotate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StopRequest {
    All,                           // Stop all sprites and scripts
    This,                          // Stop this sprite and all its scripts
    Script(usize),                 // Stop a specific script by ID
    OtherScripts(usize), // Stop all scripts of this sprite except the one with the given ID
    OtherSpritesAndScripts(usize), // Stop all other sprites and all scripts except the one with the given ID
}

#[derive(Debug)]
pub(super) struct Glide {
    pub(super) start_x: f32,
    pub(super) start_y: f32,
    pub(super) end_x: f32,
    pub(super) end_y: f32,
    pub(super) duration: usize,
    pub(super) remaining: usize,
    pub(super) ctrl1: Vec2,
    pub(super) ctrl2: Vec2,
}

#[derive(Debug)]
pub(super) struct Dialogue {
    pub(super) text: String,
    pub(super) duration: f32,
    pub(super) think: bool,
}

#[derive(Debug)]
pub struct Sprite {
    pub name: String,
    pub costumes: Vec<GPUTexture>,
    pub sounds: HashMap<String, StaticSoundData>,
    pub center: Vec2,
    pub size: Vec2,
    pub direction: f32,
    pub rotation_style: RotationStyle,
    pub scale: f32,
    pub layer: isize,
    pub variables: HashMap<String, Value>,
    pub effects: IndexMap<String, f32>,
    pub sound_filters: IndexMap<String, f32>,
    pub draw_color: Vec4,
    pub functions: HashMap<String, Callable>,
    pub clone_id: Option<usize>,
    pub stop_request: Option<StopRequest>,
    pub tags: Vec<String>,
    pub(super) visible: bool,
    pub(super) clones: Vec<Sprite>,
    pub(super) dialogue: Option<Dialogue>,
    pub(super) edge_bounce: bool,
    pub(super) current_costume: usize,
    pub(super) time_waiting: u32,
    pub(super) glide: Option<Glide>,
    pub(super) delete_pending: bool,
    pub(super) sound_handles: HashMap<String, StaticSoundHandle>,
    pub(super) skip_further_execution_of_frame: bool,
    pub(super) uv: [Vec2; 2],
    clone_setup: Vec<Statement>,
    clone_update: Vec<Vec<Statement>>,
    setup_ast: Vec<Statement>,
    update_ast: Vec<Vec<Statement>>,
    setup_finished: bool,
    broadcast_recievers: HashMap<String, Vec<Statement>>,
    boolean_recievers: Vec<(Expression, Vec<Statement>, bool)>,
    completed_broadcasts: Vec<usize>,
}

impl Sprite {
    pub fn new(
        name: String,
        costumes: Vec<GPUTexture>,
        sounds: HashMap<String, StaticSoundData>,
        ast: Vec<Statement>,
        tags: Vec<String>,
        w: f32,
        h: f32,
        x: f32,
        y: f32,
        visibility: bool,
        layer: isize,
        direction: f32,
        base_dir: String,
    ) -> Self {
        let mut setup_ast = vec![];
        let mut update_ast = vec![];
        let mut broadcast_recievers = HashMap::new();
        let mut boolean_recievers = vec![];
        let mut functions = HashMap::new();
        let mut clone_setup = vec![];
        let mut clone_update = vec![];
        for statement in ast {
            match statement {
                Statement::Setup { body } => {
                    setup_ast.extend(body);
                }
                Statement::Update { body } => {
                    update_ast.push(body);
                }
                Statement::CloneSetup { body } => {
                    clone_setup = body;
                }
                Statement::CloneUpdate { body } => {
                    clone_update.push(body);
                }
                Statement::FunctionDefinition {
                    name,
                    args,
                    body,
                    returns,
                } => {
                    functions.insert(
                        name.clone(),
                        Callable::Function(Function {
                            args: args.clone(),
                            body: body.clone(),
                            returns: returns.clone(),
                            captured_vars: vec![],
                        }),
                    );
                }
                Statement::WhenBroadcasted { broadcast, body } => {
                    broadcast_recievers.insert(broadcast.clone(), body);
                }
                Statement::WhenBoolean { condition, body } => {
                    boolean_recievers.push((condition.clone(), body, false));
                }
                Statement::Import { path } => {
                    fn import_module(
                        path: PathBuf,
                        visited: &mut Vec<String>,
                        setup_ast: &mut Vec<Statement>,
                    ) -> HashMap<String, Function> {
                        let md = path.metadata().unwrap();
                        if md.is_dir() {
                            println!("Importing directory as module: {}", &path.display());
                            let children = std::fs::read_dir(&path)
                                .unwrap_or_else(|_| {
                                    panic!("Failed to read directory: {}", &path.display());
                                })
                                .map(|entry| entry.unwrap().path())
                                .collect::<Vec<_>>();
                            let mut functions = HashMap::new();
                            for child in children {
                                let imported_functions = import_module(child, visited, setup_ast);
                                functions.extend(imported_functions);
                            }
                            return functions;
                        }
                        let code = std::fs::read_to_string(&path).unwrap_or_else(|_| {
                            println!("Failed to load module: {}", &path.display());
                            String::new()
                        });
                        let mut tokenizer = Tokenizer::new(code);
                        let tokens = tokenizer.tokenize_full();
                        let mut parser = Parser::new(tokens);
                        let ast = parser.parse();
                        let mut functions = HashMap::new();
                        for statement in ast {
                            match statement {
                                Statement::FunctionDefinition {
                                    name,
                                    args,
                                    body,
                                    returns,
                                } => {
                                    functions.insert(
                                        name.clone(),
                                        Function {
                                            args: args.clone(),
                                            body: body.clone(),
                                            returns: returns.clone(),
                                            captured_vars: vec![],
                                        },
                                    );
                                }
                                Statement::Setup { body } => {
                                    for statement in body {
                                        match statement {
                                            Statement::Assignment {
                                                is_global,
                                                identifier,
                                                value,
                                            } => {
                                                (|| {
                                                    setup_ast.insert(
                                                        0,
                                                        Statement::Assignment {
                                                            is_global,
                                                            identifier,
                                                            value: value.clone(),
                                                        },
                                                    );
                                                })(
                                                );
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                Statement::Import { path } => {
                                    if visited.contains(&path) {
                                        println!("Circular import detected: {}, skipping", path);
                                        return functions;
                                    }
                                    visited.push(path.clone());
                                    let imported_functions =
                                        import_module(PathBuf::from(&path), visited, setup_ast);
                                    functions.extend(imported_functions);
                                }
                                _ => {}
                            }
                        }
                        functions
                    }
                    let mut visited: Vec<String> = vec![];
                    let imported_functions = import_module(
                        PathBuf::from(&base_dir).join(&path),
                        &mut visited,
                        &mut setup_ast,
                    );
                    for (name, function) in imported_functions {
                        functions.insert(name, Callable::Function(function));
                    }
                }
                _ => {}
            }
        }
        let mut costumes = costumes;
        if costumes.is_empty() {
            costumes.push(CPUTexture::new(100, 100).upload_to_gpu());
        }
        Self {
            name,
            setup_ast,
            update_ast,
            functions,
            setup_finished: false,
            costumes,
            center: vec2(x, y),
            size: vec2(w, h),
            current_costume: 0,
            sounds,
            scale: 1.0,
            layer: layer,
            visible: visibility,
            direction: direction,
            rotation_style: RotationStyle::AllAround,
            variables: HashMap::new(),
            effects: IndexMap::new(),
            sound_filters: IndexMap::new(),
            time_waiting: 0,
            dialogue: None,
            glide: None,
            draw_color: Vec3::splat(0.0).extend(1.0),
            edge_bounce: false,
            clones: vec![],
            clone_setup,
            clone_update,
            clone_id: None,
            delete_pending: false,
            stop_request: None,
            tags,
            broadcast_recievers,
            boolean_recievers,
            sound_handles: HashMap::new(),
            skip_further_execution_of_frame: false,
            uv: [vec2(0.0, 1.0), vec2(1.0, 0.0)],
            completed_broadcasts: vec![],
        }
    }

    pub fn new_clone(&self) -> Self {
        let name = format!("{} (clone {})", self.name, self.clones.len() + 1);
        let setup_ast = self.clone_setup.clone();
        let update_ast = self.clone_update.clone();
        let functions = self.functions.clone();
        let costumes = self.costumes.clone();
        let sounds = self.sounds.clone();
        let center = self.center;
        let size = self.size;
        Self {
            name,
            setup_ast,
            update_ast,
            functions,
            setup_finished: false,
            costumes,
            center,
            size,
            current_costume: self.current_costume,
            sounds,
            scale: self.scale,
            layer: self.layer,
            visible: true,
            direction: self.direction,
            rotation_style: self.rotation_style.clone(),
            variables: self.variables.clone(),
            effects: self.effects.clone(),
            sound_filters: self.sound_filters.clone(),
            time_waiting: 0,
            dialogue: None,
            glide: None,
            draw_color: self.draw_color,
            edge_bounce: self.edge_bounce,
            clones: vec![],
            clone_setup: self.clone_setup.clone(),
            clone_update: self.clone_update.clone(),
            clone_id: Some(self.clones.len() + 1),
            delete_pending: false,
            stop_request: None,
            tags: self.tags.clone(),
            broadcast_recievers: self.broadcast_recievers.clone(),
            boolean_recievers: self.boolean_recievers.clone(),
            sound_handles: HashMap::new(),
            skip_further_execution_of_frame: false,
            uv: self.uv,
            completed_broadcasts: vec![],
        }
    }

    pub fn goto(&mut self, x: f32, y: f32) {
        self.center = vec2(x, y);
    }

    pub fn point(&mut self, x: f32, y: f32) {
        let dx = y - self.center.y;
        let dy = x - self.center.x;
        self.direction = dx.atan2(dy).to_degrees();
    }

    pub fn goto_cursor(&mut self, window: &glfw::Window) {
        let (x, y) = window.get_cursor_pos();
        self.goto(
            x as f32 * 2. - window.get_size().0 as f32,
            y as f32 * 2. - window.get_size().1 as f32,
        );
    }

    pub fn point_cursor(&mut self, window: &glfw::Window) {
        let (x, y) = window.get_cursor_pos();
        let (x, y) = (
            x as f32 * 2. - window.get_size().0 as f32,
            y as f32 * 2. - window.get_size().1 as f32,
        );
        let dx = y - self.center.y;
        let dy = x - self.center.x;
        self.direction = dx.atan2(dy).to_degrees();
    }

    pub fn move_by(&mut self, step: f32, window: &glfw::Window) {
        self.center.x += -step * self.direction.to_radians().cos();
        self.center.y += -step * self.direction.to_radians().sin();
        self.handle_edge_bounce(window);
    }

    pub fn handle_edge_bounce(&mut self, window: &glfw::Window) {
        if self.edge_bounce {
            let screen_width = window.get_size().0 as f32;
            let screen_height = window.get_size().1 as f32;
            if self.center.x < -screen_width / 2. || self.center.x > screen_width / 2. {
                self.direction = (180.0 - self.direction) % 360.0;
            }
            if self.center.y < -screen_height / 2. || self.center.y > screen_height / 2. {
                self.direction = (-self.direction) % 360.0;
            }
        }
    }

    pub fn set_costume(&mut self, index: usize) {
        if index < self.costumes.len() {
            self.current_costume = index;
        } else {
            println!("Costume index out of bounds");
        }
    }

    pub fn next_costume(&mut self) {
        self.current_costume = (self.current_costume + 1) % self.costumes.len();
    }

    pub fn prev_costume(&mut self) {
        if self.current_costume == 0 {
            self.current_costume = self.costumes.len() - 1;
        } else {
            self.current_costume -= 1;
        }
    }

    pub fn costume(&self) -> usize {
        self.current_costume
    }

    pub fn new_variable(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn set_variable(&mut self, name: &str, value: Value) {
        if let Some(var) = self.variables.get_mut(name) {
            *var = value;
        } else {
            println!("Variable '{}' not found", name);
        }
    }

    pub fn variable(&self, name: &str, project: &Project, local_vars: &[(String, Value)]) -> Value {
        if let Some(var) = local_vars.iter().find(|(n, _)| n == name) {
            return var.1.clone();
        } else if let Some(var) = self.variables.get(name) {
            var.clone()
        } else if let Some(var) = project.global_variables.get(name) {
            var.clone()
        } else if let Some(function) = self.functions.get(name) {
            Value::Closure(Box::new(function.clone()))
        } else if let Some(function) = project.builtins.get(name) {
            Value::Closure(Box::new(function.clone()))
        } else {
            match name {
                "PI" => Value::Number(PI),
                "E" => Value::Number(E),
                _ => {
                    println!("Variable '{}' not found", name);
                    Value::Null
                }
            }
        }
    }

    pub fn execute_statement(statement: &Statement, state: &mut State<'_>) {
        match statement {
            Statement::Assignment {
                is_global,
                identifier,
                value,
            } => {
                let value = crate::utils::resolve_expression(value, state);
                if *is_global {
                    state
                        .project
                        .global_variables
                        .insert(identifier.clone(), value);
                } else {
                    if state.sprite.variables.get(identifier).is_none() {
                        state.sprite.new_variable(identifier, value.clone());
                    } else {
                        state.sprite.set_variable(identifier, value);
                    }
                }
            }
            Statement::ListMemberAssignment {
                is_global,
                identifier,
                index,
                value,
            } => {
                let value = crate::utils::resolve_expression(value, state);
                let index = crate::utils::resolve_expression(index, state);
                if let Value::Number(index) = index {
                    if *is_global {
                        if let Some(global_list) = state
                            .project
                            .global_variables
                            .get_mut(identifier.to_string().as_str())
                        {
                            if let Value::List(global_list) = global_list {
                                global_list[index as usize] = value;
                            }
                        }
                    } else {
                        if let Some(local_list) = state
                            .sprite
                            .variables
                            .get_mut(identifier.to_string().as_str())
                        {
                            if let Value::List(local_list) = local_list {
                                local_list[index as usize] = value;
                            }
                        }
                    }
                } else if let Value::String(key) = index {
                    if *is_global {
                        if let Some(global_list) = state
                            .project
                            .global_variables
                            .get_mut(identifier.to_string().as_str())
                        {
                            if let Value::Object(global_list) = global_list {
                                global_list.insert(key.clone(), value);
                            }
                        }
                    } else {
                        if let Some(local_list) = state
                            .sprite
                            .variables
                            .get_mut(identifier.to_string().as_str())
                        {
                            if let Value::Object(local_list) = local_list {
                                local_list.insert(key.clone(), value);
                            }
                        }
                    }
                } else {
                    println!("Invalid index type for list assignment (expected number or string)");
                }
            }
            Statement::Nop => {}
            Statement::Assert { condition } => {
                if !crate::utils::resolve_expression(condition, state).to_boolean() {
                    println!("assert {:?}: Failed", condition);
                } else {
                    println!("assert {:?}: Passed", condition);
                }
            }
            Statement::Match {
                value,
                cases,
                default,
            } => {
                let resolved_value = crate::utils::resolve_expression(value, state);
                for (case_value, body) in cases {
                    if resolved_value == crate::utils::resolve_expression(case_value, state) {
                        for statement in body {
                            Sprite::execute_statement(statement, state);
                        }
                        return;
                    }
                }
                if let Some(default_body) = default {
                    for statement in default_body {
                        Sprite::execute_statement(statement, state);
                    }
                }
            }
            Statement::If {
                condition,
                body,
                else_if_bodies,
                else_body,
            } => {
                if crate::utils::resolve_expression(condition, state).to_boolean() {
                    for statement in body {
                        Sprite::execute_statement(statement, state);
                    }
                } else {
                    for (else_if_condition, else_if_body) in else_if_bodies {
                        if crate::utils::resolve_expression(else_if_condition, state).to_boolean() {
                            for statement in else_if_body {
                                Sprite::execute_statement(statement, state);
                            }
                            return;
                        }
                    }
                    if let Some(else_body) = else_body {
                        for statement in else_body {
                            Sprite::execute_statement(statement, state);
                        }
                    }
                }
            }
            Statement::While { condition, body } => {
                while crate::utils::resolve_expression(condition, state).to_boolean() {
                    for statement in body {
                        Sprite::execute_statement(statement, state);
                    }
                }
            }
            Statement::For {
                identifier,
                iterable,
                body,
            } => {
                for value in crate::utils::resolve_expression(iterable, state).to_list() {
                    let mut new_local_vars = state.local_vars.to_vec();
                    new_local_vars.push((identifier.clone(), value));
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
                        local_vars: &new_local_vars,
                        script_id: state.script_id,
                    };
                    for statement in body {
                        Sprite::execute_statement(statement, &mut new_state);
                    }
                }
            }
            Statement::Call(c) => {
                if let Expression::Call { function, args } = c {
                    let args = args
                        .iter()
                        .map(|arg| crate::utils::resolve_expression(arg, state))
                        .collect::<Vec<_>>();
                    if let Some(callable) = state.sprite.functions.clone().get(function) {
                        callable.call(state, &args).unwrap_or_else(|e| {
                            println!("Error calling {}(): {}", function, e);
                            Value::Null
                        });
                    } else if let Some(callable) = state.project.builtins.get(function).cloned() {
                        callable.call(state, &args).unwrap_or_else(|e| {
                            println!("Error calling builtin function '{}': {}", function, e);
                            Value::Null
                        });
                    } else if let Some(variable) = state.sprite.variables.get(function).cloned() {
                        let Value::Closure(closure) = variable else {
                            println!("Variable '{}' is not a function", function);
                            return;
                        };
                        let function_struct = *closure;
                        function_struct.call(state, &args).unwrap_or_else(|e| {
                            println!("Error calling closure '{}': {}", function, e);
                            Value::Null
                        });
                    } else {
                        println!("Unknown function or variable '{}'", function);
                    }
                }
            }
            _ => {}
        }
    }

    fn get_script_id(&self, ast: Vec<Statement>) -> usize {
        if self.setup_ast == ast {
            return 0;
        }

        if let Some(index) = self.update_ast.iter().position(|x| x == &ast) {
            return index + 1;
        }

        if let Some(index) = self
            .broadcast_recievers
            .iter()
            .position(|(_, body)| body == &ast)
        {
            return index + self.update_ast.len() + 1;
        }

        if let Some(index) = self
            .boolean_recievers
            .iter()
            .position(|(_, body, _)| body == &ast)
        {
            return index + self.update_ast.len() + self.broadcast_recievers.len() + 1;
        }

        0
    }

    pub fn stop_script(&mut self, script_id: usize) {
        if script_id == 0 {
            self.setup_ast.clear();
            return;
        }

        if let Some(ast) = self.update_ast.get_mut(script_id - 1) {
            ast.clear();
            return;
        }

        if script_id <= self.update_ast.len() + self.broadcast_recievers.len() + 1 {
            let index = script_id - self.update_ast.len() - 1;
            if index < self.broadcast_recievers.len() {
                let name = self.broadcast_recievers.keys().nth(index).cloned();
                if let Some(name) = name {
                    self.broadcast_recievers
                        .retain(|broadcast, _| *broadcast != name.clone());
                    return;
                }
            }
        }

        if script_id
            <= self.update_ast.len()
                + self.broadcast_recievers.len()
                + self.boolean_recievers.len()
                + 1
        {
            let index = script_id - self.update_ast.len() - self.broadcast_recievers.len() - 1;
            if index < self.boolean_recievers.len() {
                let (_, _, called) = &mut self.boolean_recievers[index];
                *called = true; // mark as called so it won't be executed
            }
        }
    }

    pub fn stop_other_scripts(&mut self, script_id: usize) {
        if script_id == 0 {
            self.update_ast.clear();
            return;
        }

        if script_id <= self.update_ast.len() {
            self.setup_ast.clear();
            let mut update_ast = std::mem::take(&mut self.update_ast);
            for ast in &mut update_ast {
                if self.get_script_id(ast.clone()) != script_id {
                    ast.clear();
                }
            }
            self.update_ast = update_ast;
            return;
        }

        if script_id <= self.update_ast.len() + self.broadcast_recievers.len() + 1 {
            let index = script_id - self.update_ast.len() - 1;
            if index < self.broadcast_recievers.len() {
                let name = self.broadcast_recievers.keys().nth(index).cloned();
                if let Some(name) = name {
                    self.broadcast_recievers
                        .retain(|broadcast, _| *broadcast != name.clone());
                    return;
                }
            }
        }

        if script_id
            <= self.update_ast.len()
                + self.broadcast_recievers.len()
                + self.boolean_recievers.len()
                + 1
        {
            let index = script_id - self.update_ast.len() - self.broadcast_recievers.len() - 1;
            if index < self.boolean_recievers.len() {
                let (_, _, called) = &mut self.boolean_recievers[index];
                *called = true; // mark as called so it won't be executed
            }
        }
    }

    pub fn stop_self(&mut self) {
        self.setup_ast.clear();
        self.update_ast.clear();
        self.broadcast_recievers.clear();
        self.boolean_recievers.clear();
        self.clones.iter_mut().for_each(|clone| {
            clone.stop_self();
        });
    }

    pub fn step(
        &mut self,
        start: std::time::Instant,
        dt: f32,
        project: &mut Project,
        snapshots: &[SpriteSnapshot],
        window: &mut glfw::Window,
        input_manager: &mut InputManager,
        glfw: &mut glfw::Glfw,
        audio_manager: &mut AudioManager<DefaultBackend>,
        shader_program: &ShaderProgram,
        projection: &mut Mat4,
        font: &BitmapFont,
    ) {
        if let Some(glide) = &mut self.glide {
            let t = 1.0 - (glide.remaining as f32 / glide.duration as f32);
            if glide.remaining > 0 {
                let eased = crate::utils::evaluate_bezier(t, glide.ctrl1.y, glide.ctrl2.y);
                self.center.x = glide.start_x + (glide.end_x - glide.start_x) * eased;
                self.center.y = glide.start_y + (glide.end_y - glide.start_y) * eased;
                glide.remaining -= 1;
            } else {
                self.glide = None;
            }

            return;
        }

        if self.effects.len() > 32 {
            self.effects.shift_remove_index(0);
        }

        if !self.setup_finished {
            for statement in self.setup_ast.clone() {
                if self.time_waiting > 0 {
                    self.time_waiting -= 1;
                    break;
                }
                if let Some(dialogue) = &mut self.dialogue {
                    if dialogue.duration > 0.0 {
                        dialogue.duration -= 1.0;
                    } else {
                        self.dialogue = None;
                    }
                    break;
                }
                Sprite::execute_statement(
                    &statement,
                    &mut State {
                        start,
                        dt,
                        sprite: self,
                        project,
                        snapshots,
                        window,
                        input_manager,
                        glfw,
                        audio_manager,
                        shader_program,
                        projection,
                        font,
                        local_vars: &vec![],
                        script_id: 0,
                    },
                );
                if self.skip_further_execution_of_frame {
                    self.skip_further_execution_of_frame = false;
                    break;
                }
            }
            self.setup_finished = true;
        } else {
            for ast in self.update_ast.clone() {
                for (i, statement) in ast.iter().enumerate() {
                    if self.time_waiting > 0 {
                        self.time_waiting -= 1;
                        break;
                    }
                    if let Some(dialogue) = &mut self.dialogue {
                        if dialogue.duration > 0.0 {
                            dialogue.duration -= 1.0;
                        } else {
                            self.dialogue = None;
                        }
                        break;
                    }
                    Sprite::execute_statement(
                        &statement,
                        &mut State {
                            start,
                            dt,
                            sprite: self,
                            project,
                            snapshots,
                            window,
                            input_manager,
                            glfw,
                            audio_manager,
                            shader_program,
                            projection,
                            font,
                            local_vars: &vec![],
                            script_id: i + 1,
                        },
                    );
                    if self.skip_further_execution_of_frame {
                        self.skip_further_execution_of_frame = false;
                        break;
                    }
                }
            }
        }

        for (i, (broadcast, body)) in self.broadcast_recievers.clone().iter().enumerate() {
            if let Some(broadcasted) = project.get_broadcast(broadcast).cloned() {
                if self.completed_broadcasts.contains(&broadcasted.id) {
                    continue;
                }
                for statement in body {
                    if self.time_waiting > 0 {
                        self.time_waiting -= 1;
                        break;
                    }
                    if let Some(dialogue) = &mut self.dialogue {
                        if dialogue.duration > 0.0 {
                            dialogue.duration -= 1.0;
                        } else {
                            self.dialogue = None;
                        }
                        break;
                    }
                    let update_ast_len = self.update_ast.len();
                    Sprite::execute_statement(
                        &statement,
                        &mut State {
                            start,
                            dt,
                            sprite: self,
                            project,
                            snapshots,
                            window,
                            input_manager,
                            glfw,
                            audio_manager,
                            shader_program,
                            projection,
                            font,
                            local_vars: &vec![],
                            script_id: i + update_ast_len + 1,
                        },
                    );
                    if self.skip_further_execution_of_frame {
                        self.skip_further_execution_of_frame = false;
                        break;
                    }
                }
                self.completed_broadcasts.push(broadcasted.id);
            }
        }

        self.boolean_recievers.retain(|(_, _, called)| !called);

        let mut called_s = vec![];
        for (i, (expr, body, _)) in self.boolean_recievers.clone().iter().enumerate() {
            let update_broadcast_len = self.update_ast.len() + self.broadcast_recievers.len();
            let value = crate::utils::resolve_expression(
                &expr,
                &mut State {
                    start,
                    dt,
                    sprite: self,
                    project,
                    snapshots,
                    window,
                    input_manager,
                    glfw,
                    audio_manager,
                    shader_program,
                    projection,
                    font,
                    local_vars: &vec![],
                    script_id: i + update_broadcast_len + 1,
                },
            );
            if value.to_boolean() {
                for statement in body {
                    if self.time_waiting > 0 {
                        self.time_waiting -= 1;
                        break;
                    }
                    if let Some(dialogue) = &mut self.dialogue {
                        if dialogue.duration > 0.0 {
                            dialogue.duration -= 1.0;
                        } else {
                            self.dialogue = None;
                        }
                        break;
                    }
                    let update_broadcast_len =
                        self.update_ast.len() + self.broadcast_recievers.len();
                    Sprite::execute_statement(
                        &statement,
                        &mut State {
                            start,
                            dt,
                            sprite: self,
                            project,
                            snapshots,
                            window,
                            input_manager,
                            glfw,
                            audio_manager,
                            shader_program,
                            projection,
                            font,
                            local_vars: &vec![],
                            script_id: i + update_broadcast_len + 1,
                        },
                    );
                    if self.skip_further_execution_of_frame {
                        self.skip_further_execution_of_frame = false;
                        break;
                    }
                }
                called_s.push(i);
            }
        }

        // mark all called boolean recievers as called
        for i in called_s {
            if let Some((_, _, called)) = self.boolean_recievers.get_mut(i) {
                *called = true;
            }
        }

        // filter out clones that are marked for deletion
        self.clones.retain(|clone| !clone.delete_pending);

        let mut remove_clones = vec![];
        let clones_len = self.clones.len();
        for sprite in &mut self.clones {
            if let Some(stop_request) = &sprite.stop_request {
                match stop_request {
                    StopRequest::All => {
                        for i in 0..clones_len {
                            remove_clones.push(i);
                        }
                    }
                    StopRequest::This => {
                        sprite.stop_self();
                    }
                    StopRequest::Script(script_id) => {
                        sprite.stop_script(*script_id);
                    }
                    StopRequest::OtherScripts(script_id) => {
                        sprite.stop_other_scripts(*script_id);
                    }
                    StopRequest::OtherSpritesAndScripts(script_id) => {
                        sprite.stop_other_scripts(*script_id);
                        for i in 0..clones_len {
                            if snapshots[i].name != sprite.name {
                                remove_clones.push(i);
                            }
                        }
                    }
                }
            }
        }
        for remove_index in remove_clones.iter().rev() {
            self.clones[*remove_index].stop_self();
        }

        // idk run step for all the clones too
        for clone in &mut self.clones {
            clone.step(
                start,
                dt,
                project,
                snapshots,
                window,
                input_manager,
                glfw,
                audio_manager,
                shader_program,
                projection,
                font,
            );
        }
    }
}
