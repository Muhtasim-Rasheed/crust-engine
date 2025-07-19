use std::collections::HashMap;

use macroquad::audio::*;
use macroquad::prelude::*;

use serde::Deserialize;

use crate::utils::draw_sprite;

use super::sprite::StopRequest;
use super::{Parser, Project, Tokenizer, sprite::Sprite, sprite::SpriteSnapshot};

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
    visible: Option<bool>,
    layer: Option<isize>,
    direction: Option<f32>,
}

#[derive(Deserialize, Clone, Debug)]
struct TagConfig {
    name: String,
    code: Option<String>,
    sprites: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct ProjectConfig {
    debug_options: Option<Vec<String>>,
    stage: Option<StageConfig>,
    sprites: Vec<SpriteConfig>,
    tags: Option<Vec<TagConfig>>,
}

pub struct Runtime {
    pub project: Project,
    debug_options: Vec<String>,
}

impl Runtime {
    pub async fn new(file_path: &str, args: Vec<String>) -> Self {
        let dir = std::path::Path::new(file_path).parent().unwrap();
        let raw = std::fs::read_to_string(file_path).unwrap();
        let config: ProjectConfig = toml::from_str(&raw).unwrap();
        let tags = config
            .tags
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|tag| {
                let code = tag.code;
                (tag.name, (tag.sprites, code))
            })
            .collect::<HashMap<_, _>>();

        println!("{:#?}", config);

        let mut project = Project::new(
            dir.to_string_lossy().to_string(),
            dir.join("export").to_string_lossy().to_string(),
            args,
        );

        for path in config
            .stage
            .unwrap_or(StageConfig { backdrops: vec![] })
            .backdrops
        {
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
                let tex = load_texture(&path.to_string_lossy()).await.unwrap_or(
                    Texture2D::from_file_with_format(
                        include_bytes!("../../assets/missing.png"),
                        None,
                    ),
                );
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
            let code =
                std::fs::read_to_string(&sprite_code_file).expect("Failed to read sprite code");

            let mut tokenizer = Tokenizer::new(code);
            let tokens = tokenizer.tokenize_full();
            let mut parser = Parser::new(tokens);
            let mut ast = parser.parse();
            let mut sprite_tags = vec![];

            for (tag_name, (sprites, code)) in tags.iter() {
                if sprites.contains(&sprite.name) || tag_name == "*" {
                    if let Some(code) = code {
                        let code = std::fs::read_to_string(dir.join(code))
                            .expect("Failed to read tag code");
                        let mut tokenizer = Tokenizer::new(code.clone());
                        let tokens = tokenizer.tokenize_full();
                        let mut parser = Parser::new(tokens);
                        let tag_ast = parser.parse();
                        ast.extend(tag_ast);
                    }
                    sprite_tags.push(tag_name.clone());
                }
            }

            let s = Sprite::new(
                sprite.name.clone(),
                textures,
                sounds,
                ast,
                sprite_tags,
                sprite.w,
                sprite.h,
                sprite.x,
                sprite.y,
                sprite.visible.unwrap_or(true),
                sprite.layer.unwrap_or(0),
                sprite.direction.unwrap_or(0.0),
                dir.to_string_lossy().to_string(),
            );

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
            rand::srand(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            );

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
                draw_sprite(sprite);
            }

            self.project.sprites = sprites;

            set_default_camera();

            let mut debugs = HashMap::new();
            debugs.insert("show_fps", get_fps().to_string());
            debugs.insert(
                "show_mouse_pos",
                format!(
                    "({}, {})",
                    mouse_position().0 * 2.0 - screen_width(),
                    mouse_position().1 * 2.0 - screen_height()
                ),
            );
            let debug_options: Vec<String> = self
                .debug_options
                .iter()
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

pub fn create_new_project(name: &str) -> std::path::PathBuf {
    let dir = std::path::Path::new(name);
    if dir.exists() {
        panic!("Project directory already exists: {}", name);
    }
    std::fs::create_dir_all(dir).expect("Failed to create project directory");

    let toml_file_content = r#"debug_options = ["show_fps", "show_mouse_pos"]

[stage]
backdrops = []

[[sprites]]
name = "default-sprite"
code = "sprites/default.crst"
costumes = ["sprites/crust.png"]
x = 0
y = 0
w = 200
h = 200"#;

    let default_sprite_code = r#"// Default Sprite Code

setup {
    print("Hello, World!");
}

update {}

// For more information on how to write Crust code, please visit the documentation:
// https://muhtasim-rasheed.github.io/crust-engine/"#;

    let toml_file_path = dir.join("project.toml");
    std::fs::write(toml_file_path.clone(), toml_file_content)
        .expect("Failed to write project.toml");

    std::fs::create_dir_all(dir.join("sprites")).expect("Failed to create sprites directory");

    let default_sprite_code_path = dir.join("sprites").join("default.crst");
    std::fs::write(default_sprite_code_path, default_sprite_code)
        .expect("Failed to write default sprite code");

    let default_sprite_costume_path = dir.join("sprites").join("crust.png");
    std::fs::write(
        default_sprite_costume_path,
        include_bytes!("../../assets/logo_background.png"),
    )
    .expect("Failed to write default sprite costume");

    toml_file_path
}
