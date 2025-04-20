use std::collections::HashMap;

use macroquad::prelude::*;

use super::Value;

#[derive(Clone, Copy, PartialEq)]
pub enum RotationStyle {
    AllAround,
    LeftRight,
    DontRotate,
}

pub struct Sprite {
    pub name: String,
    pub costumes: Vec<Texture2D>,
    pub center: Vec2,
    pub size: Vec2,
    pub direction: f32,
    pub rotation_style: RotationStyle,
    pub scale: f32,
    pub variables: HashMap<String, Value>,
    center_offset: Vec2,
    current_costume: usize,
}

impl Sprite {
    pub fn new(name: String, costumes: Vec<Texture2D>, w: f32, h: f32, x: f32, y: f32) -> Self {
        let costumes = costumes
            .into_iter()
            .map(|texture| {
                texture.set_filter(FilterMode::Nearest);
                texture
            })
            .collect();
        Self {
            name,
            costumes,
            center: vec2(x, y),
            center_offset: vec2(0.0, 0.0),
            size: vec2(w, h),
            current_costume: 0,
            scale: 1.0,
            direction: 0.0,
            rotation_style: RotationStyle::AllAround,
            variables: HashMap::new(),
        }
    }

    pub fn set_center_offset(&mut self, x: f32, y: f32) {
        self.center_offset = vec2(x, y);
    }

    pub fn goto(&mut self, x: f32, y: f32) {
        self.center = vec2(x, y);
    }

    pub fn goto_cursor(&mut self) {
        let (x, y) = mouse_position();
        self.goto(x, y);
    }

    pub fn goto_other(&mut self, sprites: &[Sprite], name: &str) {
        if let Some(sprite) = sprites.iter().find(|s| s.name == name) {
            self.goto(sprite.center.x, sprite.center.y);
        } else {
            println!("Sprite with name '{}' not found", name);
        }
    }

    pub fn move_by(&mut self, step: f32) {
        self.center.x += step * self.direction.cos();
        self.center.y += step * self.direction.sin();
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

    pub fn change_variable_by(&mut self, name: &str, value: Value) {
        if let Some(var) = self.variables.get_mut(name) {
            var.change_by(value);
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
    
    pub fn draw(&self) {
        let scaled_size = self.size * self.scale;
        let top_left = self.center + self.center_offset - scaled_size / 2.0;

        draw_texture_ex(
            &self.costumes[self.current_costume],
            top_left.x,
            top_left.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(scaled_size),
                rotation: if self.rotation_style == RotationStyle::AllAround {
                    self.direction.to_radians()
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
