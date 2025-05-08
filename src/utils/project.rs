use std::collections::HashMap;

use super::*;

pub struct Project {
    pub global_variables: HashMap<String, Value>,
    pub broadcasted_message: Option<String>,
    pub sprites: Vec<Sprite>,
    pub stage: Stage,
    pub home_path: String,
    pub export_path: String,
}

impl Project {
    pub fn new(home_path: String, export_path: String) -> Self {
        Self {
            global_variables: HashMap::new(),
            broadcasted_message: None,
            sprites: Vec::new(),
            stage: Stage::new(vec![]),
            home_path,
            export_path,
        }
    }
}
