use std::collections::HashMap;

use glfw::Window;

use super::*;

#[derive(Debug, Clone)]
pub struct Broadcast {
    pub message: String,
    pub id: usize,
}

pub struct Project {
    pub global_variables: HashMap<String, Value>,
    pub broadcast_history: Vec<Broadcast>,
    pub sprites: Vec<Sprite>,
    pub stage: Stage,
    pub args: Vec<String>,
    pub home_path: String,
    pub export_path: String,
}

impl Project {
    pub fn new(home_path: String, export_path: String, args: Vec<String>, window: &Window) -> Self {
        Self {
            global_variables: HashMap::new(),
            broadcast_history: Vec::new(),
            sprites: Vec::new(),
            stage: Stage::new(vec![], window),
            args,
            home_path,
            export_path,
        }
    }

    pub fn broadcast(&mut self, message: String) {
        let id = self.broadcast_history.len();
        self.broadcast_history.push(Broadcast { message, id });
    }

    pub fn get_broadcast(&self, message: &str) -> Option<&Broadcast> {
        self.broadcast_history
            .iter()
            .rev()
            .find(|b| b.message == message)
    }
}

impl std::fmt::Debug for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Project")
            .field("args", &self.args)
            .field("home_path", &self.home_path)
            .field("export_path", &self.export_path)
            .finish()
    }
}
