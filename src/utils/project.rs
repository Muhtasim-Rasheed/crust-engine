use std::collections::HashMap;

use macroquad::prelude::*;

use super::*;

pub struct Project {
    pub global_variables: HashMap<String, Value>,
    pub sprites: Vec<Sprite>,
    pub stage: Stage,
}

impl Project {
    pub fn new() -> Self {
        Self {
            global_variables: HashMap::new(),
            sprites: Vec::new(),
            stage: Stage::new(vec![]),
        }
    }

    pub fn draw(&mut self) {
        clear_background(WHITE);
        self.stage.draw();
        for sprite in &mut self.sprites {
            sprite.draw();
        }
    }
}
