use std::collections::HashMap;

use macroquad::prelude::*;
use macroquad::audio::*;

use serde::Deserialize;

use super::StopRequest;
use super::{
    Parser, Project, Sprite, SpriteSnapshot, Tokenizer
};

#[derive(Deserialize, Debug)]
struct StageConfig {
    backdrops: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct SoundConfig {
    name: String,
    file: String,
}

#[derive(Deserialize, Debug)]
struct SpriteConfig {
    name: String,
    code: String,
    costumes: Vec<String>,
    sounds: Option<Vec<SoundConfig>>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Deserialize, Debug)]
struct ProjectConfig {
    debug_options: Option<Vec<String>>,
    stage: Option<StageConfig>,
    sprites: Vec<SpriteConfig>,
}

pub struct Runtime {
    pub project: Project,
    debug_options: Vec<String>,
}

impl Runtime {
    pub async fn new(file_path: &str) -> Self {
        let dir = std::path::Path::new(file_path).parent().unwrap();
        let raw = std::fs::read_to_string(file_path).unwrap();
        let config: ProjectConfig = toml::from_str(&raw).unwrap();

        println!("{:#?}", config);

        let mut project = Project::new(dir.to_string_lossy().to_string(), dir.join("export").to_string_lossy().to_string());

        for path in config.stage.unwrap_or(StageConfig { backdrops: vec![] }).backdrops {
            let path = dir.join(path);
            let tex = load_texture(&path.to_string_lossy()).await.unwrap();
            project.stage.backdrops.push(tex);
        }

        if project.stage.backdrops.is_empty() {
            project.stage.backdrops.push(Texture2D::empty());
        }

        for sprite in config.sprites {
            let mut textures = vec![];
            for path in sprite.costumes {
                let path = dir.join(path);
                let tex = load_texture(&path.to_string_lossy()).await.unwrap_or(Texture2D::from_file_with_format(include_bytes!("../../assets/missing.png"), None));
                textures.push(tex);
            }

            let mut sounds = vec![];
            if sprite.sounds.is_some() {
                let sounds_ = sprite.sounds.unwrap();
                for sound in sounds_ {
                    let path = dir.join(&sound.file);
                    let sound_data = load_sound(&path.to_string_lossy()).await.unwrap_or_else(|_| {
                        panic!("Failed to load sound: {}. Make sure the path is correct. Relative paths are allowed.", path.to_string_lossy())
                    });
                    sounds.push((sound.name, sound_data));
                }
            }

            let sounds = sounds.into_iter().collect::<HashMap<_, _>>();
            
            let sprite_code_file = dir.join(&sprite.code);
            let code = std::fs::read_to_string(&sprite_code_file).expect("Failed to read sprite code");

            let mut tokenizer = Tokenizer::new(code);
            let tokens = tokenizer.tokenize_full();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse();

            let s = Sprite::new(sprite.name.clone(), textures, sounds, ast, sprite.w, sprite.h, sprite.x, sprite.y, dir.to_string_lossy().to_string());

            project.sprites.push(s);
        }
        
        Self {
            project,
            debug_options: config.debug_options.unwrap_or(vec![]),
        }
    }

    pub async fn run(&mut self) {
        let camera = Camera2D {
            target: vec2(0.0, 0.0),
            zoom: vec2(1.0 / screen_width(), 1.0 / screen_height()),
            ..Default::default()
        };
        loop {
            rand::srand(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64);

            set_camera(&camera);
            clear_background(WHITE);
            self.project.stage.draw();

            let mut sprites = std::mem::take(&mut self.project.sprites);
            
            let snapshots: Vec<SpriteSnapshot> = sprites.iter().map(|s| s.into()).collect();

            let mut remove_sprites = vec![];
            let sprites_len = sprites.len();
            for sprite in &mut sprites {
                if let Some(stop_request) = &sprite.stop_request {
                    match stop_request {
                        StopRequest::All => {
                            for i in 0..sprites_len {
                                remove_sprites.push(i);
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
                            for i in 0..sprites_len {
                                if snapshots[i].name != sprite.name {
                                    remove_sprites.push(i);
                                }
                            }
                        }
                    }
                }
            }
            for remove_index in remove_sprites.iter().rev() {
                sprites[*remove_index].stop_self();
            }

            for sprite in &mut sprites {
                sprite.step(&mut self.project, &snapshots, &camera);
            }

            sprites.sort_by(|a, b| a.layer.cmp(&b.layer));

            for sprite in &mut sprites {
                sprite.draw();
            }

            self.project.sprites = sprites;
            
            set_default_camera();

            let mut debugs = HashMap::new();
            debugs.insert("show_fps", get_fps().to_string());
            debugs.insert("show_mouse_pos", format!("({}, {})", mouse_position().0 * 2.0 - screen_width(), mouse_position().1 * 2.0 - screen_height()));
            let debug_options: Vec<String> = self.debug_options.iter()
                .filter(|option| debugs.contains_key(option.as_str()))
                .map(|option| format!("{}: {}", option, debugs[option.as_str()]))
                .collect();
            for (i, debug) in debug_options.iter().enumerate() {
                draw_text(debug, 10.0, 30.0 + (i as f32 * 30.0), 24.0, BLACK);
            }

            next_frame().await;
        }
    }
}
