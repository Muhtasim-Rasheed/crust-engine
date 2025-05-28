use std::collections::HashMap;
use std::f32::consts::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use macroquad::audio::*;
use macroquad::prelude::*;

use super::Parser;
use super::Tokenizer;
use super::{Expression, Project, Statement, Value};

#[derive(Clone, Debug)]
pub struct SpriteSnapshot {
    pub name: String,
    pub center: Vec2,
    pub size: Vec2,
    pub scale: f32,
    pub direction: f32,
}

impl SpriteSnapshot {
    pub fn get(&self, name: &str) -> Option<Value> {
        match name {
            "name" => Some(Value::String(self.name.clone())),
            "x" => Some(Value::Number(self.center.x)),
            "y" => Some(Value::Number(self.center.y)),
            "size" => Some(Value::List(vec![Value::Number(self.size.x), Value::Number(self.size.y)])),
            "scale" => Some(Value::Number(self.scale)),
            "direction" => Some(Value::Number(self.direction)),
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
    All, // Stop all sprites and scripts
    This, // Stop this sprite and all its scripts
    Script(usize), // Stop a specific script by ID
    OtherScripts(usize), // Stop all scripts of this sprite except the one with the given ID
    OtherSpritesAndScripts(usize), // Stop all other sprites and all scripts except the one with the given ID
}

#[derive(Debug)]
struct Glide {
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    duration: usize,
    remaining: usize,
    ctrl1: Vec2,
    ctrl2: Vec2,
}

#[derive(Debug)]
struct Dialogue {
    text: String,
    duration: f32,
    think: bool,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub args: Vec<String>,
    pub body: Vec<Statement>,
    pub returns: Expression,
}

#[derive(Debug)]
pub struct Sprite {
    pub name: String,
    pub costumes: Vec<Texture2D>,
    pub sounds: HashMap<String, Sound>,
    pub center: Vec2,
    pub size: Vec2,
    pub direction: f32,
    pub rotation_style: RotationStyle,
    pub scale: f32,
    pub layer: usize,
    pub variables: HashMap<String, Value>,
    pub effects: HashMap<String, f32>,
    pub sound_effects: HashMap<String, f32>,
    pub draw_color: Color,
    pub functions: HashMap<String, Function>,
    pub clone_id: Option<usize>,
    pub stop_request: Option<StopRequest>,
    clones: Vec<Sprite>,
    clone_setup: Vec<Statement>,
    clone_update: Vec<Vec<Statement>>,
    dialogue: Option<Dialogue>,
    edge_bounce: bool,
    current_costume: usize,
    setup_ast: Vec<Statement>,
    update_ast: Vec<Vec<Statement>>,
    setup_finished: bool,
    time_waiting: u32,
    glide: Option<Glide>,
    delete_pending: bool,
}

impl Sprite {
    pub fn new(name: String, costumes: Vec<Texture2D>, sounds: HashMap<String, Sound>, ast: Vec<Statement>, w: f32, h: f32, x: f32, y: f32, base_dir: String) -> Self {
        let mut setup_ast = vec![];
        let mut update_ast = vec![];
        let mut functions = HashMap::new();
        let mut clone_setup = vec![];
        let mut clone_update = vec![];
        for statement in ast {
            match statement {
                Statement::Setup { body } => {
                    setup_ast = body;
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
                Statement::FunctionDefinition { name, args, body, returns } => {
                    functions.insert(name.clone(), Function {
                        args: args.clone(),
                        body: body.clone(),
                        returns: returns.clone(),
                    });
                }
                Statement::Import { path } => {
                    fn import_module(path: &str, base_dir: &str, visited: &mut Vec<String>, setup_ast: &mut Vec<Statement>) -> HashMap<String, Function> {
                        let path = Path::new(base_dir).join(path);
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
                                Statement::FunctionDefinition { name, args, body, returns } => {
                                    functions.insert(name.clone(), Function {
                                        args: args.clone(),
                                        body: body.clone(),
                                        returns: returns.clone(),
                                    });
                                }
                                Statement::Setup { body } => {
                                    for statement in body {
                                        match statement {
                                            Statement::Assignment { is_global, identifier, value } => {
                                                (|| {
                                                    setup_ast.insert(0, Statement::Assignment {
                                                        is_global,
                                                        identifier,
                                                        value: value.clone(),
                                                    });
                                                })();
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
                                    let imported_functions = import_module(&path, &base_dir, visited, setup_ast);
                                    visited.push(path);
                                    functions.extend(imported_functions);
                                }
                                _ => {}
                            }
                        }
                        functions
                    }
                    let mut visited: Vec<String> = vec![];
                    let imported_functions = import_module(&path, &base_dir, &mut visited, &mut setup_ast);
                    for (name, function) in imported_functions {
                        functions.insert(name, function);
                    }
                }
                _ => {}
            }
        }
        let mut costumes = costumes;
        if costumes.is_empty() {
            costumes.push(Texture2D::from_image(&Image::gen_image_color(1, 1, Color::new(0.0, 0.0, 0.0, 0.0))));
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
            layer: 0,
            direction: 0.0,
            rotation_style: RotationStyle::AllAround,
            variables: HashMap::new(),
            effects: HashMap::new(),
            sound_effects: HashMap::new(),
            time_waiting: 0,
            dialogue: None,
            glide: None,
            draw_color: BLACK,
            edge_bounce: false,
            clones: vec![],
            clone_setup,
            clone_update,
            clone_id: None,
            delete_pending: false,
            stop_request: None,
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
            direction: self.direction,
            rotation_style: self.rotation_style.clone(),
            variables: self.variables.clone(),
            effects: self.effects.clone(),
            sound_effects: self.sound_effects.clone(),
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
        }
    }

    pub fn goto(&mut self, x: f32, y: f32) {
        self.center = vec2(x, y);
    }

    pub fn goto_cursor(&mut self) {
        let (x, y) = mouse_position();
        self.goto(x*2.-screen_width(), y*2.-screen_height());
    }

    pub fn goto_other(&mut self, snapshots: &[SpriteSnapshot], name: &str) {
        if let Some(snapshot) = snapshots.iter().find(|s| s.name == name) {
            self.goto(snapshot.center.x, snapshot.center.y);
        } else {
            println!("Sprite with name '{}' not found", name);
        }
    }

    pub fn move_by(&mut self, step: f32) {
        self.center.x += step * self.direction.to_radians().cos();
        self.center.y += step * self.direction.to_radians().sin();
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
        if let Some(var) = self.variables.get(name) {
            var.clone()
        } else if let Some(var) = project.global_variables.get(name) {
            var.clone()
        } else if let Some(var) = local_vars.iter().find(|(n, _)| n == name) {
            var.1.clone()
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

    fn clone(&mut self) {
        self.clones.push(self.new_clone());
    }

    pub fn execute_statement(&mut self, statement: &Statement, project: &mut Project, snapshots: &[SpriteSnapshot], camera: &Camera2D, local_vars: &[(String, Value)], script_id: usize) {
        match statement {
            Statement::Assignment { is_global, identifier, value } => {
                let value = super::resolve_expression(value, project, self, local_vars, snapshots, camera, script_id);
                if *is_global {
                    project.global_variables.insert(identifier.clone(), value);
                } else {
                    if self.variables.get(identifier).is_none() {
                        self.new_variable(identifier, value.clone());
                    } else {
                        self.set_variable(identifier, value);
                    }
                }
            }
            Statement::ListMemberAssignment { is_global, identifier, index, value } => {
                let value = super::resolve_expression(value, project, self, local_vars, snapshots, camera, script_id);
                let index = super::resolve_expression(index, project, self, local_vars, snapshots, camera, script_id);
                if let Value::Number(index) = index {
                    if *is_global {
                        if let Some(global_list) = project.global_variables.get_mut(identifier.to_string().as_str()) {
                            if let Value::List(global_list) = global_list {
                                global_list[index as usize] = value;
                            }
                        }
                    } else {
                        if let Some(local_list) = self.variables.get_mut(identifier.to_string().as_str()) {
                            if let Value::List(local_list) = local_list {
                                local_list[index as usize] = value;
                            }
                        }
                    }
                } else if let Value::String(key) = index {
                    if *is_global {
                        if let Some(global_list) = project.global_variables.get_mut(identifier.to_string().as_str()) {
                            if let Value::Object(global_list) = global_list {
                                global_list.insert(key.clone(), value);
                            }
                        }
                    } else {
                        if let Some(local_list) = self.variables.get_mut(identifier.to_string().as_str()) {
                            if let Value::Object(local_list) = local_list {
                                local_list.insert(key.clone(), value);
                            }
                        }
                    }
                } else {
                    println!("Invalid index type for list assignment (expected number or string)");
                }
            }
            Statement::If { condition, body, else_body } => {
                let condition_value = super::resolve_expression(condition, project, self, local_vars, snapshots, camera, script_id);
                if condition_value.to_boolean() {
                    for statement in body {
                        self.execute_statement(statement, project, snapshots, camera, local_vars, script_id);
                    }
                } else if let Some(else_body) = else_body {
                    for statement in else_body {
                        self.execute_statement(statement, project, snapshots, camera, local_vars, script_id);
                    }
                }
            }
            Statement::While { condition, body } => {
                while super::resolve_expression(condition, project, self, local_vars, snapshots, camera, script_id).to_boolean() { 
                    for statement in body {
                        self.execute_statement(statement, project, snapshots, camera, local_vars, script_id);
                    }
                }
            }
            Statement::Repeat { times, body } => {
                let times_value = super::resolve_expression(times, project, self, local_vars, snapshots, camera, script_id);
                if let Value::Number(times) = times_value {
                    for _ in 0..times as usize {
                        for statement in body {
                            self.execute_statement(statement, project, snapshots, camera, local_vars, script_id);
                        }
                    }
                } else {
                    println!("Invalid argument for repeat");
                }
            }
            Statement::Call(c) => {
                if let Expression::Call { function, args } = c {
                    let args = args
                        .iter()
                        .map(|arg| super::resolve_expression(arg, project, self, local_vars, snapshots, camera, script_id))
                        .collect::<Vec<_>>();
                    match function.as_str() {
                        // ============= MISC ============= \\
                        "print" => {
                            println!("{} => {:?}", self.name, args);
                        }
                        "export" => {
                            match args.as_slice() {
                                [Value::String(content)] => {
                                    let time = chrono::Local::now();
                                    let filename = format!("{}-{}.png", self.name, time.format("%Y-%m-%d_%H-%M-%S"));
                                    let path = Path::new(&project.export_path).join(filename);
                                    let mut file = File::create(path).unwrap();
                                    file.write_all(content.as_bytes()).unwrap();
                                }
                                [Value::String(content), Value::String(path)] => {
                                    let path = Path::new(path);
                                    let mut file = File::create(path).unwrap();
                                    file.write_all(content.as_bytes()).unwrap();
                                }
                                _ => {
                                    println!("Invalid arguments for export_to");
                                }
                            }
                        }
                        "screenshot" => {
                            match args.as_slice() {
                                [Value::String(path)] => {
                                    let screenshot = get_screen_data();
                                    screenshot.export_png(&path);
                                }
                                _ => {
                                    let time = chrono::Local::now();
                                    let path = format!("{}-{}.png", self.name, time.format("%Y-%m-%d_%H-%M-%S"));
                                    let screenshot = get_screen_data();
                                    screenshot.export_png(&path);
                                }
                            }
                        }
                        // ============= MOTION ============= \\
                        "move" => {
                            if let [Value::Number(step)] = args.as_slice() {
                                self.move_by(*step);
                            } else {
                                println!("Invalid arguments for move");
                            }
                        }
                        "turn_cw" => {
                            if let [Value::Number(angle)] = args.as_slice() {
                                self.direction += *angle;
                            } else {
                                println!("Invalid arguments for turn_cw");
                            }
                        }
                        "turn_ccw" => {
                            if let [Value::Number(angle)] = args.as_slice() {
                                self.direction -= *angle;
                            } else {
                                println!("Invalid arguments for turn_ccw");
                            }
                        }
                        "goto" => {
                            match args.as_slice() {
                                [Value::Number(x), Value::Number(y)] => self.goto(*x, *y),
                                [Value::String(name)] => {
                                    if name == "mouse" {
                                        self.goto_cursor();
                                    } else if name == "random" {
                                        let x = rand::gen_range(-1024.0, 1024.0);
                                        let y = rand::gen_range(-576.0, 576.0);
                                        self.goto(x, y);
                                    } else {
                                        self.goto_other(&snapshots, name);
                                    }
                                }
                                _ => println!("Invalid arguments for goto"),
                            }
                        }
                        "glide" => {
                            match args.as_slice() {
                                [Value::Number(x), Value::Number(y), Value::Number(duration)] => {
                                    let duration = *duration * 60.0;
                                    self.glide = Some(Glide {
                                        start_x: self.center.x,
                                        start_y: self.center.y,
                                        end_x: *x,
                                        end_y: *y,
                                        duration: duration as usize,
                                        remaining: duration as usize,
                                        ctrl1: vec2(0.0, 0.0), // No easing
                                        ctrl2: vec2(1.0, 1.0),
                                    });
                                }
                                [Value::Number(x), Value::Number(y), Value::Number(duration), Value::String(easing)] => {
                                    let duration = *duration * 60.0;
                                    let easing = easing.to_lowercase();
                                    let (ctrl1, ctrl2) = match easing.as_str() {
                                        "linear" => (vec2(0.0, 0.0), vec2(1.0, 1.0)),
                                        "ease" => (vec2(0.25, 0.01), vec2(0.25, 1.0)),
                                        "ease-in" => (vec2(0.42, 0.0), vec2(1.0, 1.0)),
                                        "ease-out" => (vec2(0.0, 0.0), vec2(0.58, 1.0)),
                                        "ease-in-out" => (vec2(0.42, 0.0), vec2(0.58, 1.0)),
                                        _ => (vec2(0.0, 0.0), vec2(1.0, 1.0)), // Default to linear
                                    };
                                    self.glide = Some(Glide {
                                        start_x: self.center.x,
                                        start_y: self.center.y,
                                        end_x: *x,
                                        end_y: *y,
                                        duration: duration as usize,
                                        remaining: duration as usize,
                                        ctrl1,
                                        ctrl2,
                                    });
                                }
                                _ => println!("Invalid arguments for glide"),
                            }
                        }
                        "point" => {
                            match args.as_slice() {
                                [Value::Number(angle)] => {
                                    self.direction = *angle;
                                }
                                [Value::String(name)] => {
                                    if name == "mouse" {
                                        let (x, y) = mouse_position();
                                        self.direction = (y - self.center.y).atan2(x - self.center.x).to_degrees();
                                    } else {
                                        if let Some(snapshot) = snapshots.iter().find(|s| s.name == *name) {
                                            let dx = snapshot.center.x - self.center.x;
                                            let dy = snapshot.center.y - self.center.y;
                                            self.direction = dy.atan2(dx).to_degrees();
                                        } else {
                                            println!("Sprite with name '{}' not found", name);
                                        }
                                    }
                                }
                                [Value::Number(x), Value::Number(y)] => {
                                    let dx = y - self.center.y;
                                    let dy = x - self.center.x;
                                    self.direction = dx.atan2(dy).to_degrees();
                                }
                                _ => println!("Invalid arguments for point"),
                            }
                        }
                        "change_x" => {
                            if let [Value::Number(step)] = args.as_slice() {
                                self.center.x += *step;
                                if self.edge_bounce {
                                    if self.center.x < -screen_width() / 2.0 {
                                        self.center.x = -screen_width() / 2.0;
                                        self.direction = 180.0 - self.direction;
                                    } else if self.center.x > screen_width() / 2.0 {
                                        self.center.x = screen_width() / 2.0;
                                        self.direction = 180.0 - self.direction;
                                    }
                                }
                            } else {
                                println!("Invalid arguments for change_x");
                            }
                        }
                        "set_x" => {
                            if let [Value::Number(x)] = args.as_slice() {
                                self.center.x = *x;
                            } else {
                                println!("Invalid arguments for set_x");
                            }
                        }
                        "change_y" => {
                            if let [Value::Number(step)] = args.as_slice() {
                                self.center.y += *step;
                                if self.edge_bounce {
                                    if self.center.y < -screen_height() / 2.0 {
                                        self.center.y = -screen_height() / 2.0;
                                        self.direction = -self.direction;
                                    } else if self.center.y > screen_height() / 2.0 {
                                        self.center.y = screen_height() / 2.0;
                                        self.direction = -self.direction;
                                    }
                                }
                            } else {
                                println!("Invalid arguments for change_y");
                            }
                        }
                        "set_y" => {
                            if let [Value::Number(y)] = args.as_slice() {
                                self.center.y = *y;
                            } else {
                                println!("Invalid arguments for set_y");
                            }
                        }
                        "edge_bounce" => {
                            if let [Value::Boolean(bounce)] = args.as_slice() {
                                self.edge_bounce = *bounce;
                            } else {
                                println!("Invalid arguments for edge_bounce");
                            }
                        }
                        "rotation_style" => {
                            if let [Value::String(style)] = args.as_slice() {
                                self.rotation_style = match style.as_str() {
                                    "all-around" => RotationStyle::AllAround,
                                    "left-right" => RotationStyle::LeftRight,
                                    "dont-rotate" => RotationStyle::DontRotate,
                                    _ => RotationStyle::AllAround,
                                };
                            } else {
                                println!("Invalid arguments for rotation_style");
                            }
                        }
                        // ============= LOOKS ============= \\
                        "say" => {
                            match args.as_slice() {
                                [text] => {
                                    self.dialogue = Some(Dialogue {
                                        text: text.to_string(),
                                        duration: f32::INFINITY,
                                        think: false,
                                    });
                                }
                                [text, Value::Number(duration)] => {
                                    self.dialogue = Some(Dialogue {
                                        text: text.to_string(),
                                        duration: *duration * 60.0,
                                        think: false,
                                    });
                                }
                                _ => println!("Invalid arguments for say"),
                            }
                        }
                        "think" => {
                            match args.as_slice() {
                                [text] => {
                                    self.dialogue = Some(Dialogue {
                                        text: text.to_string(),
                                        duration: f32::INFINITY,
                                        think: true,
                                    });
                                }
                                [text, Value::Number(duration)] => {
                                    self.dialogue = Some(Dialogue {
                                        text: text.to_string(),
                                        duration: *duration * 60.0,
                                        think: true,
                                    });
                                }
                                _ => println!("Invalid arguments for think"),
                            }
                        }
                        "switch_costume" => {
                            if let [Value::Number(index)] = args.as_slice() {
                                self.set_costume(*index as usize);
                            } else {
                                println!("Invalid arguments for switch_costume");
                            }
                        }
                        "next_costume" => {
                            self.next_costume();
                        }
                        "previous_costume" => {
                            self.prev_costume();
                        }
                        "switch_backdrop" => {
                            if let [Value::Number(index)] = args.as_slice() {
                                project.stage.set_backdrop(*index as usize);
                            } else {
                                println!("Invalid arguments for switch_backdrop");
                            }
                        }
                        "next_backdrop" => {
                            project.stage.next_backdrop();
                        }
                        "previous_backdrop" => {
                            project.stage.prev_backdrop();
                        }
                        "change_size" => {
                            if let [Value::Number(increment)] = args.as_slice() {
                                self.scale += *increment;
                            } else {
                                println!("Invalid arguments for change_size");
                            }
                        }
                        "set_scale" => {
                            if let [Value::Number(size)] = args.as_slice() {
                                self.scale = *size / 100.0;
                            } else {
                                println!("Invalid arguments for set_size");
                            }
                        }
                        "change_effect" => {
                            if let [Value::String(effect), Value::Number(value)] = args.as_slice() {
                                self.effects
                                    .entry(effect.clone())
                                    .and_modify(|v| *v += *value)
                                    .or_insert(*value);
                            } else {
                                println!("Invalid arguments for change_effect");
                            }
                        }
                        "set_effect" => {
                            if let [Value::String(effect), Value::Number(value)] = args.as_slice() {
                                self.effects.insert(effect.clone(), *value);
                            } else {
                                println!("Invalid arguments for set_effect");
                            }
                        }
                        "clear_effects" => {
                            self.effects.clear();
                        }
                        "clear_effect" => {
                            if let [Value::String(effect)] = args.as_slice() {
                                self.effects.remove(effect);
                            } else {
                                println!("Invalid arguments for clear_effect");
                            }
                        }
                        "go_to_layer" => {
                            if let [Value::Number(layer)] = args.as_slice() {
                                self.layer = *layer as usize;
                            } else {
                                println!("Invalid arguments for go_to_layer");
                            }
                        }
                        "go_by_layers" => {
                            if let [Value::String(direction), Value::Number(steps)] = args.as_slice() {
                                if direction == "forwards" {
                                    self.layer += *steps as usize;
                                } else if direction == "backwards" {
                                    self.layer -= *steps as usize;
                                }
                            } else {
                                println!("Invalid arguments for go_by_layers");
                            }
                        }
                        // ============= SOUND ============= \\
                        "play_sound" => {
                            match args.as_slice() {
                                [Value::String(name)] => {
                                    if let Some(sound) = self.sounds.get(name) {
                                        play_sound(&sound, PlaySoundParams {
                                            looped: false,
                                            volume: self.sound_effects.get("volume").cloned().unwrap_or(100.0) / 100.0,
                                        });
                                    } else {
                                        println!("Sound '{}' not found", name);
                                    }
                                }
                                [Value::String(name), Value::Boolean(stop_other)] => {
                                    if *stop_other {
                                        for sound in self.sounds.values() {
                                            stop_sound(sound);
                                        }
                                    }
                                    if let Some(sound) = self.sounds.get(name) {
                                        play_sound(&sound, PlaySoundParams {
                                            looped: false,
                                            volume: self.sound_effects.get("volume").cloned().unwrap_or(100.0) / 100.0,
                                        });
                                    } else {
                                        println!("Sound '{}' not found", name);
                                    }
                                }
                                _ => println!("Invalid arguments for play_sound"),
                            }
                        }
                        "stop_all_sounds" => {
                            for sound in self.sounds.values() {
                                stop_sound(sound);
                            }
                        }
                        "stop_sound" => {
                            if let [Value::String(name)] = args.as_slice() {
                                if let Some(sound) = self.sounds.get(name) {
                                    stop_sound(sound);
                                } else {
                                    println!("Sound '{}' not found", name);
                                }
                            } else {
                                println!("Invalid arguments for stop_sound");
                            }
                        }
                        "change_sound_effect" => {
                            if let [Value::String(effect), Value::Number(value)] = args.as_slice() {
                                self.sound_effects
                                    .entry(effect.clone())
                                    .and_modify(|v| *v += *value)
                                    .or_insert(*value);
                            } else {
                                println!("Invalid arguments for change_sound_effect");
                            }
                        }
                        "set_sound_effect" => {
                            if let [Value::String(effect), Value::Number(value)] = args.as_slice() {
                                self.sound_effects.insert(effect.clone(), *value);
                            } else {
                                println!("Invalid arguments for set_sound_effect");
                            }
                        }
                        // ============= EVENTS ============= \\
                        "broadcast" => {
                            if let [Value::String(message)] = args.as_slice() {
                                project.broadcasted_message = Some(message.clone());
                            } else {
                                println!("Invalid arguments for broadcast");
                            }
                        }
                        // ============= CONTROL ============= \\
                        "wait" => {
                            if let [Value::Number(seconds)] = args.as_slice() {
                                self.time_waiting = (*seconds * 60.0) as u32;
                            } else {
                                println!("Invalid arguments for wait");
                            }
                        }
                        "stop" => {
                            if let [Value::String(action)] = args.as_slice() {
                                match action.as_str() {
                                    "all" => {
                                        self.stop_request = Some(StopRequest::All);
                                    }
                                    "this" => {
                                        self.stop_request = Some(StopRequest::This);
                                    }
                                    "script" => {
                                        self.stop_request = Some(StopRequest::Script(script_id));
                                    }
                                    "other-scripts" => {
                                        self.stop_request = Some(StopRequest::OtherScripts(script_id));
                                    }
                                    "other-sprites-and-scripts" => {
                                        self.stop_request = Some(StopRequest::OtherSpritesAndScripts(script_id));
                                    }
                                    _ => println!("Invalid action for stop"),
                                }
                            } else {
                                println!("Invalid arguments for stop");
                            }
                        }
                        "clone" => self.clone(),
                        "delete_clone" => {
                            if let [Value::Number(cloneid)] = args.as_slice() {
                                if let Some(index) = self.clones.iter().position(|c| c.name == format!("{} (clone {})", self.name, cloneid)) {
                                    self.clones.remove(index);
                                }
                            } else {
                                self.delete_pending = true;
                            }
                        }
                        // ============= DRAWING ============= \\
                        "set_color" => {
                            if let [Value::Number(r), Value::Number(g), Value::Number(b)] = args.as_slice() {
                                self.draw_color = Color::new(r / 255.0, g / 255.0, b / 255.0, 1.0);
                            } else {
                                println!("Invalid arguments for set_color");
                            }
                        }
                        "change_r" => {
                            if let [Value::Number(r)] = args.as_slice() {
                                self.draw_color.r += r.clamp(-1.0, 1.0)
                            } else {
                                println!("Invalid arguments for change_r");
                            }
                        }
                        "change_g" => {
                            if let [Value::Number(g)] = args.as_slice() {
                                self.draw_color.g += g.clamp(-1.0, 1.0)
                            } else {
                                println!("Invalid arguments for change_g");
                            }
                        }
                        "change_b" => {
                            if let [Value::Number(b)] = args.as_slice() {
                                self.draw_color.b += b.clamp(-1.0, 1.0)
                            } else {
                                println!("Invalid arguments for change_b");
                            }
                        }
                        "line" => {
                            if let [Value::Number(x1), Value::Number(y1), Value::Number(x2), Value::Number(y2), Value::Number(thickness)] = args.as_slice() {
                                draw_line(*x1, *y1, *x2, *y2, *thickness, self.draw_color);
                            } else {
                                println!("Invalid arguments for line");
                            }
                        }
                        "rect" => {
                            if let [Value::Number(x1), Value::Number(y1), Value::Number(x2), Value::Number(y2)] = args.as_slice() {
                                let w = x2 - x1;
                                let h = y2 - y1;
                                draw_rectangle(*x1, *y1, w, h, self.draw_color);
                            } else {
                                println!("Invalid arguments for rect");
                            }
                        }
                        "hrect" => {
                            if let [Value::Number(x1), Value::Number(y1), Value::Number(x2), Value::Number(y2), Value::Number(thickness)] = args.as_slice() {
                                let w = x2 - x1;
                                let h = y2 - y1;
                                draw_rectangle_lines(*x1, *y1, w, h, *thickness, self.draw_color);

                            } else {
                                println!("Invalid arguments for hrect");
                            }
                        }
                        "circle" => {
                            if let [Value::Number(x1), Value::Number(y1), Value::Number(radius)] = args.as_slice() {
                                draw_circle(*x1, *y1, *radius, self.draw_color);
                            } else {
                                println!("Invalid arguments for circle");
                            }
                        }
                        "hcircle" => {
                            if let [Value::Number(x1), Value::Number(y1), Value::Number(radius), Value::Number(thickness)] = args.as_slice() {
                                draw_circle_lines(*x1, *y1, *radius, *thickness, self.draw_color);
                            } else {
                                println!("Invalid arguments for hcircle");
                            }
                        }
                        "ellipse" => {
                            match args.as_slice() {
                                [Value::Number(x1), Value::Number(y1), Value::Number(w), Value::Number(h)] => {
                                    draw_ellipse(*x1, -*y1, *w, *h, 0.0, self.draw_color);
                                }
                                [Value::Number(x1), Value::Number(y1), Value::Number(w), Value::Number(h), Value::Number(rotation)] => {
                                    draw_ellipse(*x1, -*y1, *w, *h, *rotation, self.draw_color);
                                }
                                _ => {
                                    println!("Invalid arguments for ellipse");
                                }
                            }
                        }
                        "hellipse" => {
                            match args.as_slice() {
                                [Value::Number(x1), Value::Number(y1), Value::Number(w), Value::Number(h), Value::Number(thickness)] => {
                                    draw_ellipse_lines(*x1, -*y1, *w, *h, 0.0, *thickness, self.draw_color);
                                }
                                [Value::Number(x1), Value::Number(y1), Value::Number(w), Value::Number(h), Value::Number(rotation), Value::Number(thickness)] => {
                                    draw_ellipse_lines(*x1, -*y1, *w, *h, *rotation, *thickness, self.draw_color);
                                }
                                _ => {
                                    println!("Invalid arguments for hellipse");
                                }
                            }
                        }
                        "polygon" => {
                            let mut xs: Vec<f32> = vec![];
                            let mut ys: Vec<f32> = vec![];
                            let mut even = false;
                            for arg in args {
                                if !even {
                                    xs.push(arg.to_number());
                                } else {
                                    ys.push(arg.to_number());
                                }
                                even = !even;
                            }
                            if xs.len() != ys.len() {
                                println!("Inequal number of x's and y's")
                            } else {
                                super::draw_convex_polygon(&xs, &ys, self.draw_color);
                            }
                        }
                        "hpolygon" => {
                            let mut xs: Vec<f32> = vec![];
                            let mut ys: Vec<f32> = vec![];
                            let mut even = false;
                            for i in 1..args.len() {
                                let arg = &args[i];
                                if !even {
                                    xs.push(arg.to_number());
                                } else {
                                    ys.push(arg.to_number());
                                }
                                even = !even;
                            }
                            if xs.len() != ys.len() {
                                println!("Inequal number of x's and y's")
                            } else {
                                let thickness = args[0].to_number();
                                super::draw_convex_polygon_lines(&xs, &ys, thickness, self.draw_color);
                            }
                        }
                        "textured_quad" => {
                            if let [Value::List(parse_image_result),
                                    Value::Number(x1), Value::Number(y1), Value::Number(x2), Value::Number(y2),
                                    Value::Number(x3), Value::Number(y3), Value::Number(x4), Value::Number(y4)] = args.as_slice() {
                                if let [Value::Number(width), Value::Number(height), Value::List(pixels)] = parse_image_result.as_slice() {
                                    let mut image = Image::gen_image_color(*width as u16, *height as u16, Color::new(0.0, 0.0, 0.0, 0.0));
                                    for i in 0..*width as usize {
                                        for j in 0..*height as usize {
                                            let index = (i + j * (*width as usize)) * 4;
                                            let r = pixels[index].to_number() / 255.0;
                                            let g = pixels[index + 1].to_number() / 255.0;
                                            let b = pixels[index + 2].to_number() / 255.0;
                                            let a = pixels[index + 3].to_number() / 255.0;
                                            image.set_pixel(i as u32, j as u32, Color::new(r, g, b, a));
                                        }
                                    }
                                    let p1 = vec2(*x1, *y1);
                                    let p2 = vec2(*x2, *y2);
                                    let p3 = vec2(*x3, *y3);
                                    let p4 = vec2(*x4, *y4);
                                    // let resolution = super::compute_resolution(p1, p2, p3, p4, &image); // throw ts away we dont need 8 fps
                                    let resolution = 128;
                                    for i in 0..=resolution {
                                        let t = i as f32 / resolution as f32;

                                        let left = super::lerp_vec2(p1, p4, t);
                                        let right = super::lerp_vec2(p2, p3, t);
                                        let uv_left = vec2(0.0, t);
                                        let uv_right = vec2(1.0, t);

                                        for j in 0..=resolution {
                                            let s = j as f32 / resolution as f32;

                                            let pos = super::lerp_vec2(left, right, s);
                                            let uv = super::lerp_vec2(uv_left, uv_right, s);

                                            let color = super::sample_texture(&image, uv);
                                            // let color = Color::new(uv.x, uv.y, 1.0, 1.0);
                                            draw_rectangle(pos.x - 4.0, pos.y - 4.0, 8.0, 8.0, color);
                                        }
                                    }
                                } else {
                                    println!("Invalid arguments for textured_quad");
                                }
                            } else {
                                println!("Invalid arguments for textured_quad");
                            }
                        }
                        "stamp" => {
                            set_camera(&project.stage.stamp_layer);
                            self.draw();
                            set_camera(camera);
                        }
                        "clear_all_stamps" => {
                            project.stage.clear_stamps();
                        }
                        // ============= WINDOW ============= \\
                        "set_window_width" => {
                            if let [Value::Number(width)] = args.as_slice() {
                                request_new_screen_size(*width, screen_height());
                            } else {
                                println!("Invalid arguments for set_window_width");
                            }
                        }
                        "set_window_height" => {
                            if let [Value::Number(height)] = args.as_slice() {
                                request_new_screen_size(screen_width(), *height);
                            } else {
                                println!("Invalid arguments for set_window_height");
                            }
                        }
                        "set_window_size" => {
                            if let [Value::Number(width), Value::Number(height)] = args.as_slice() {
                                request_new_screen_size(*width, *height);
                            } else {
                                println!("Invalid arguments for set_window_size");
                            }
                        }
                        "set_window_state" => {
                            if let [Value::String(state)] = args.as_slice() {
                                match state.as_str() {
                                    "normal" => set_fullscreen(false),
                                    "fullscreen" => set_fullscreen(true),
                                    _ => println!("Invalid arguments for set_window_state")
                                }
                            } else {
                                println!("Invalid arguments for set_window_state");
                            }
                        }
                        "set_window_x" => {
                            if let [Value::Number(x)] = args.as_slice() {
                                miniquad::window::set_window_position(*x as u32, miniquad::window::get_window_position().1);
                            } else {
                                println!("Invalid arguments for set_window_x");
                            }
                        }
                        "set_window_y" => {
                            if let [Value::Number(y)] = args.as_slice() {
                                miniquad::window::set_window_position(miniquad::window::get_window_position().0, *y as u32);
                            } else {
                                println!("Invalid arguments for set_window_y");
                            }
                        }
                        "set_window_position" => {
                            if let [Value::Number(x), Value::Number(y)] = args.as_slice() {
                                miniquad::window::set_window_position(*x as u32, *y as u32);
                            } else {
                                println!("Invalid arguments for set_window_position");
                            }
                        }
                        "pointer_grab" => {
                            if let [Value::Boolean(grab)] = args.as_slice() {
                                set_cursor_grab(*grab);
                                show_mouse(!grab);
                            } else {
                                println!("Invalid arguments for pointer_grab");
                            }
                        }
                        _ => {
                            if let Some(function_struct) = self.functions.clone().get(function) {
                                let Function { args: args_, body, .. } = function_struct;
                                if args_.len() == args.len() {
                                    let mut local_vars_: Vec<(String, Value)> = vec![];
                                    for (i, arg) in args_.iter().enumerate() {
                                        if let Some(arg_value) = args.get(i) {
                                            local_vars_.push((arg.clone(), arg_value.clone()));
                                        } else {
                                            println!("Missing argument for function '{}'", function);
                                        }
                                    }
                                    local_vars_.append(&mut local_vars.to_vec());
                                    for statement in body {
                                        self.execute_statement(statement, project, snapshots, camera, &local_vars_, script_id);
                                    }
                                } else {
                                    println!("Invalid number of arguments for function '{}'", function);
                                }
                            } else {
                                println!("Unknown function: {}", function);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn get_script_id(&self, ast: Vec<Statement>) -> usize {
        if self.setup_ast == ast {
            0
        } else {
            self.update_ast.iter().position(|x| x == &ast).unwrap_or(0) + 1
        }
    }

    pub fn stop_script(&mut self, script_id: usize) {
        if script_id == 0 {
            self.setup_ast.clear();
        } else if let Some(ast) = self.update_ast.get_mut(script_id - 1) {
            ast.clear();
        }
    }

    pub fn stop_other_scripts(&mut self, script_id: usize) {
        if script_id == 0 {
            self.update_ast.clear();
        } else {
            self.setup_ast.clear();
            let mut update_ast = std::mem::take(&mut self.update_ast);
            for ast in &mut update_ast {
                if self.get_script_id(ast.clone()) != script_id {
                    ast.clear();
                }
            }
            self.update_ast = update_ast;
        }
    }

    pub fn stop_self(&mut self) {
        self.setup_ast.clear();
        self.update_ast.clear();
        self.clones.clear();
    }

    pub fn step(&mut self, project: &mut Project, snapshots: &[SpriteSnapshot], camera: &Camera2D) {
        // TODO: Correctly implement dialogue waiting
        if let Some(dialogue) = &mut self.dialogue {
            if dialogue.duration > 0.0 {
                dialogue.duration -= 1.0;
            } else {
                self.dialogue = None;
            }
        }

        if let Some(glide) = &mut self.glide {
            let t = 1.0 - (glide.remaining as f32 / glide.duration as f32);
            if glide.remaining > 0 {
                let eased = super::evaluate_bezier(t, glide.ctrl1.y, glide.ctrl2.y);
                self.center.x = glide.start_x + (glide.end_x - glide.start_x) * eased;
                self.center.y = glide.start_y + (glide.end_y - glide.start_y) * eased;
                glide.remaining -= 1;
            } else {
                self.glide = None;
            }

            return;
        }
        
        // TODO: Correctly implement the wait function
        if self.time_waiting > 0 {
            self.time_waiting -= 1;
            return;
        }

        if !self.setup_finished {
            for statement in self.setup_ast.clone() {
                self.execute_statement(&statement, project, snapshots, camera, &vec![], 0);
            }
            self.setup_finished = true;
        } else {
            for ast in self.update_ast.clone() {
                // for statement in ast {
                for (i, statement) in ast.iter().enumerate() {
                    self.execute_statement(&statement, project, snapshots, camera, &vec![], i + 1);
                }
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
            clone.step(project, snapshots, camera);
        }
    }

    pub fn draw(&self) {
        let scaled_size = self.size * self.scale;
        let top_left = self.center - scaled_size / 2.0;
        
        // Apply effects on a new texture
        let mut effect_image = self.costumes[self.current_costume].get_texture_data();
        for (effect, value) in &self.effects {
            match effect.as_str() {
                "brightness" => {
                    let brightness = (value / 100.0).clamp(-1.0, 1.0);
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(pixel.r + brightness, pixel.g + brightness, pixel.b + brightness, pixel.a)
                            );
                        }
                    }
                }
                "ghost" => {
                    let alpha = (value / 100.0).clamp(0.0, 1.0);
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(pixel.r, pixel.g, pixel.b, pixel.a * alpha)
                            );
                        }
                    }
                },
                "hue" => {
                    let hue = value;
                    let cos_a = (hue*PI/180.).cos();
                    let sin_a = (hue*PI/180.).sin();
                    let onethird: f32 = 1./3.;
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(
                                    pixel.r*(cos_a+(1. - cos_a)/3.) + 
                                    pixel.g*(onethird * (1. - cos_a) - onethird.sqrt() * sin_a) + 
                                    pixel.b*(onethird * (1. - cos_a) + onethird.sqrt() * sin_a),

                                    pixel.r*(onethird * (1. - cos_a) + onethird.sqrt() * sin_a) +
                                    pixel.g*(cos_a + onethird*(1. - cos_a)) +
                                    pixel.b*(onethird * (1. - cos_a) - onethird.sqrt() * sin_a),

                                    pixel.r*(onethird * (1. - cos_a) - onethird.sqrt() * sin_a) +
                                    pixel.g*(onethird * (1. - cos_a) + onethird.sqrt() * sin_a) +
                                    pixel.b*(cos_a + onethird * (1. - cos_a)),

                                    pixel.a
                                )
                            );
                        }
                    }
                },
                "saturation" => {
                    let saturation = (value / 100.0).clamp(0.0, 100.0);
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(
                                    super::lerp(pixel.r*0.299 + pixel.g*0.587 + pixel.b*0.114, pixel.r, saturation), 
                                    super::lerp(pixel.r*0.299 + pixel.g*0.587 + pixel.b*0.114, pixel.g, saturation),  
                                    super::lerp(pixel.r*0.299 + pixel.g*0.587 + pixel.b*0.114, pixel.b, saturation),  
                                    pixel.a)
                            );
                        }
                    }
                },
                "sepia" => {
                    let sepia = (value / 100.0).clamp(0.0, 1.0);
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(
                                    super::lerp(pixel.r, pixel.r*0.393 + pixel.g*0.769 + pixel.b*0.189, sepia), 
                                    super::lerp(pixel.g, pixel.g*0.349 + pixel.b*0.686 + pixel.r*0.168, sepia), 
                                    super::lerp(pixel.b, pixel.b*0.272 + pixel.r*0.534 + pixel.g*0.131, sepia), 
                                    pixel.a)
                            );
                        }
                    }
                },
                "grayscale-averaged" => {
                    let grayscale = (value / 100.0).clamp(0.0, 1.0);
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(
                                    super::lerp(pixel.r, pixel.r/3. + pixel.g/3. + pixel.b/3., grayscale), 
                                    super::lerp(pixel.g, pixel.r/3. + pixel.g/3. + pixel.b/3., grayscale), 
                                    super::lerp(pixel.b, pixel.r/3. + pixel.g/3. + pixel.b/3., grayscale), 
                                    pixel.a)
                            );
                        }
                    }
                },
                "grayscale-weighted" => {
                    let grayscale = (value / 100.0).clamp(0.0, 1.0);
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(
                                    super::lerp(pixel.r, pixel.r*0.299 + pixel.g*0.587 + pixel.b*0.114, grayscale), 
                                    super::lerp(pixel.g, pixel.r*0.299 + pixel.g*0.587 + pixel.b*0.114, grayscale),  
                                    super::lerp(pixel.b, pixel.r*0.299 + pixel.g*0.587 + pixel.b*0.114, grayscale),  
                                    pixel.a)
                            );
                        }
                    }
                },
                "invert" => {
                    let invert = (value / 100.0).clamp(0.0, 1.0);
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(
                                    super::lerp(pixel.r, 1.0 - pixel.r, invert), 
                                    super::lerp(pixel.g, 1.0 - pixel.g, invert), 
                                    super::lerp(pixel.b, 1.0 - pixel.b, invert), 
                                    pixel.a)
                            );
                        }
                    }
                },
                "multiply" => {
                    let multiply = value / 1.0;
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(
                                    pixel.r*multiply,
                                    pixel.g*multiply, 
                                    pixel.b*multiply,
                                    pixel.a)
                            );
                        }
                    }
                },
                "multiply-r" => {
                    let multiply = value / 1.0;
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(
                                    pixel.r*multiply,
                                    pixel.g, 
                                    pixel.b,
                                    pixel.a)
                            );
                        }
                    }
                },
                "multiply-g" => {
                    let multiply = value / 1.0;
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(
                                    pixel.r,
                                    pixel.g*multiply, 
                                    pixel.b,
                                    pixel.a)
                            );
                        }
                    }
                },
                "multiply-b" => {
                    let multiply = value / 1.0;
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(
                                    pixel.r,
                                    pixel.g, 
                                    pixel.b*multiply,
                                    pixel.a)
                            );
                        }
                    }
                },
                "add" => {
                    let add = value / 1.0;
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(
                                    pixel.r+add,
                                    pixel.g+add, 
                                    pixel.b+add,
                                    pixel.a)
                            );
                        }
                    }
                },
                "add-r" => {
                    let add = value / 1.0;
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(
                                    pixel.r+add,
                                    pixel.g, 
                                    pixel.b,
                                    pixel.a)
                            );
                        }
                    }
                },
                "add-g" => {
                    let add = value / 1.0;
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(
                                    pixel.r,
                                    pixel.g+add, 
                                    pixel.b,
                                    pixel.a)
                            );
                        }
                    }
                },
                "add-b" => {
                    let add = value / 1.0;
                    for i in 0..effect_image.width() {
                        for j in 0..effect_image.height() {
                            let pixel = effect_image.get_pixel(i as u32, j as u32);
                            effect_image.set_pixel(
                                i as u32, j as u32,
                                Color::new(
                                    pixel.r,
                                    pixel.g, 
                                    pixel.b+add,
                                    pixel.a)
                            );
                        }
                    }
                },
                _ => {} // Do absolutely nothing
            }
        }
        let processed_texture = Texture2D::from_image(&effect_image);
        processed_texture.set_filter(FilterMode::Nearest);

        // draw all the clones first so they are behind
        for clone in &self.clones {
            clone.draw();
        }

        draw_texture_ex(
            &processed_texture,
            top_left.x,
            top_left.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(scaled_size),
                rotation: if self.rotation_style == RotationStyle::AllAround {
                    self.direction.to_radians()
                } else if self.rotation_style == RotationStyle::LeftRight || self.rotation_style == RotationStyle::DontRotate {
                    0.0
                } else {
                    0.0
                },
                flip_x: if self.rotation_style == RotationStyle::LeftRight {
                    self.direction > 90.0 && self.direction < 270.0
                } else {
                    false
                },
                ..Default::default()
            },
        );

        if let Some(dialogue) = &self.dialogue {
            let dialogue_size = measure_text(&dialogue.text, None, 72, 1.0);
            draw_text_ex(
                &dialogue.text,
                self.center.x - dialogue_size.width / 2.0,
                self.center.y - scaled_size.y / 2.0 - dialogue_size.height,
                TextParams {
                    font_size: 72,
                    color: if dialogue.think {
                        Color::new(1.0, 1.0, 1.0, 0.7)
                    } else {
                        Color::new(1.0, 1.0, 1.0, 1.0)
                    },
                    ..Default::default()
                },
            );
        }
    }
}
