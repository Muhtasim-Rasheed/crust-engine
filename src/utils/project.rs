use std::collections::HashMap;

use super::*;

pub struct Project {
    pub global_variables: HashMap<String, Value>,
    pub broadcasted_message: Option<String>,
    pub sprites: Vec<Sprite>,
    pub stage: Stage,
    pub export_path: String,
}

impl Project {
    pub fn new(export_path: String) -> Self {
        Self {
            global_variables: HashMap::new(),
            broadcasted_message: None,
            sprites: Vec::new(),
            stage: Stage::new(vec![]),
            export_path,
        }
    }
}
