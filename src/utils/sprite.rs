use std::collections::HashMap;
use std::f32::consts::PI;

use macroquad::prelude::*;
use macroquad::audio::*;

use super::{Expression, Project, Statement, Value};

#[derive(Clone, Debug)]
pub struct SpriteSnapshot {
    pub name: String,
    pub center: Vec2,
}

impl From<&Sprite> for SpriteSnapshot {
    fn from(sprite: &Sprite) -> Self {
        SpriteSnapshot {
            name: sprite.name.clone(),
            center: sprite.center,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RotationStyle {
    AllAround,
    LeftRight,
    DontRotate,
}

#[derive(Debug)]
pub struct Glide {
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    duration: usize,
    remaining: usize,

    pub ctrl1: Vec2,
    pub ctrl2: Vec2,
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
    current_costume: usize,
    crawler: usize,
    setup_ast: Vec<Statement>,
    update_ast: Vec<Statement>,
    setup_finished: bool,
    time_waiting: u32,
    glide: Option<Glide>,
}

impl Sprite {
    pub fn new(name: String, costumes: Vec<Texture2D>, sounds: HashMap<String, Sound>, ast: Vec<Statement>, w: f32, h: f32, x: f32, y: f32) -> Self {
        let costumes = costumes
            .into_iter()
            .map(|texture| {
                texture.set_filter(FilterMode::Nearest);
                texture
            })
            .collect();
        let mut setup_ast = vec![];
        let mut update_ast = vec![];
        for statement in ast {
            match statement {
                Statement::Setup { body } => {
                    setup_ast = body;
                }
                Statement::Update { body } => {
                    update_ast = body;
                }
                _ => {}
            }
        }
        Self {
            name,
            crawler: 0,
            setup_ast,
            update_ast,
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
            glide: None,
        }
    }

    pub fn goto(&mut self, x: f32, y: f32) {
        self.center = vec2(x, y);
    }

    pub fn goto_cursor(&mut self) {
        let (x, y) = mouse_position();
        self.goto(x, y);
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

    pub fn variable(&self, name: &str) -> Value {
        if let Some(var) = self.variables.get(name) {
            var.clone()
        } else {
            println!("Variable '{}' not found", name);
            Value::Null
        }
    }

    fn execute_statement(&mut self, statement: &Statement, project: &mut Project, snapshots: &[SpriteSnapshot]) {
        match statement {
            Statement::Assignment { is_global, identifier, value } => {
                let value = super::resolve_expression(value, project, self);
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
            Statement::If { condition, body, else_body } => {
                let condition_value = super::resolve_expression(condition, project, self);
                if condition_value.to_boolean() {
                    for statement in body {
                        self.execute_statement(statement, project, snapshots);
                    }
                } else if let Some(else_body) = else_body {
                    for statement in else_body {
                        self.execute_statement(statement, project, snapshots);
                    }
                }
            }
            Statement::While { condition, body } => {
                while super::resolve_expression(condition, project, self).to_boolean() {
                    for statement in body {
                        self.execute_statement(statement, project, snapshots);
                    }
                }
            }
            Statement::Repeat { times, body } => {
                let times_value = super::resolve_expression(times, project, self);
                if let Value::Number(times) = times_value {
                    for _ in 0..times as usize {
                        for statement in body {
                            self.execute_statement(statement, project, snapshots);
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
                        .map(|arg| super::resolve_expression(arg, project, self))
                        .collect::<Vec<_>>();
                    match function.as_str() {
                        // ============= MISC ============= \\
                        "print" => {
                            println!("{} => {}", self.name, args[0].to_string());
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
                        "set_size" => {
                            if let [Value::Number(size)] = args.as_slice() {
                                self.scale = *size;
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
                                            volume: self.sound_effects.get("volume").cloned().unwrap_or(1.0) / 100.0,
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
                                            volume: self.sound_effects.get("volume").cloned().unwrap_or(1.0) / 100.0,
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
                        // ============= CONTROL ============= \\
                        "wait" => {
                            if let [Value::Number(seconds)] = args.as_slice() {
                                self.time_waiting = (*seconds * 60.0) as u32;
                            } else {
                                println!("Invalid arguments for wait");
                            }
                        }
                        _ => {
                            println!("Unknown function: {}", function);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn step(&mut self, project: &mut Project, snapshots: &[SpriteSnapshot]) {
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

        if self.time_waiting > 0 {
            self.time_waiting -= 1;
            return;
        }
        
        if !self.setup_finished {
            while self.crawler < self.setup_ast.len() {
                self.execute_statement(&self.setup_ast[self.crawler].clone(), project, snapshots);
                self.crawler += 1;
            }
            self.setup_finished = true;
            self.crawler = 0;
        } else {
            while self.crawler < self.update_ast.len() {
                self.execute_statement(&self.update_ast[self.crawler].clone(), project, snapshots);
                self.crawler += 1;
            }
            self.crawler = 0;
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
                }
                _ => {} // Do absolutely nothing
            }
        }
        let processed_texture = Texture2D::from_image(&effect_image);

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
    }
}
