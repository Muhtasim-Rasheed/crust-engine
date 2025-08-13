use glam::*;
use glfw::{Context, Window};
use kira::sound::static_sound::StaticSoundData;
use kira::{AudioManager, AudioManagerSettings, DefaultBackend};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

use crate::utils::core::*;
use crate::utils::draw_sprite;

use super::sprite::StopRequest;
use super::{Parser, Project, Tokenizer, sprite::Sprite, sprite::SpriteSnapshot};

#[derive(Deserialize, Debug)]
struct FontConfig {
    file: String,
    first_char: char,
    char_width: u32,
    char_height: u32,
    chars_per_row: u32,
}

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
    font: Option<FontConfig>,
    stage: Option<StageConfig>,
    sprites: Vec<SpriteConfig>,
    tags: Option<Vec<TagConfig>>,
}

#[derive(Debug)]
pub struct InputManager {
    keys_down: HashSet<glfw::Key>,
    keys_pressed: HashSet<glfw::Key>,
    keys_released: HashSet<glfw::Key>,
    mouse_buttons_down: HashSet<glfw::MouseButton>,
    mouse_buttons_pressed: HashSet<glfw::MouseButton>,
    mouse_buttons_released: HashSet<glfw::MouseButton>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),
            mouse_buttons_down: HashSet::new(),
            mouse_buttons_pressed: HashSet::new(),
            mouse_buttons_released: HashSet::new(),
        }
    }

    pub fn update(
        &mut self,
        window: &mut glfw::Window,
        events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    ) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_buttons_pressed.clear();
        self.mouse_buttons_released.clear();

        for (_, event) in glfw::flush_messages(events) {
            match event {
                glfw::WindowEvent::Key(key, _, action, _) => match action {
                    glfw::Action::Press => {
                        if !self.keys_down.contains(&key) {
                            self.keys_pressed.insert(key);
                        }
                        self.keys_down.insert(key);
                    }
                    glfw::Action::Release => {
                        self.keys_released.insert(key);
                        self.keys_down.remove(&key);
                    }
                    _ => {}
                },
                glfw::WindowEvent::MouseButton(button, action, _) => match action {
                    glfw::Action::Press => {
                        if !self.mouse_buttons_down.contains(&button) {
                            self.mouse_buttons_pressed.insert(button);
                        }
                        self.mouse_buttons_down.insert(button);
                    }
                    glfw::Action::Release => {
                        self.mouse_buttons_released.insert(button);
                        self.mouse_buttons_down.remove(&button);
                    }
                    _ => {}
                },
                glfw::WindowEvent::Close => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }
    }

    pub fn is_key_down(&self, key: glfw::Key) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_key_pressed(&self, key: glfw::Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(&self, key: glfw::Key) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn is_mouse_button_down(&self, button: glfw::MouseButton) -> bool {
        self.mouse_buttons_down.contains(&button)
    }

    pub fn is_mouse_button_pressed(&self, button: glfw::MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }

    pub fn is_mouse_button_released(&self, button: glfw::MouseButton) -> bool {
        self.mouse_buttons_released.contains(&button)
    }
}

pub struct Runtime {
    pub project: Project,
    pub audio_manager: AudioManager<DefaultBackend>,
    pub font: BitmapFont,
    debug_options: Vec<String>,
}

impl Runtime {
    pub fn new(file_path: &str, args: Vec<String>, window: &Window) -> Self {
        let audio_manager = AudioManager::new(AudioManagerSettings::default())
            .expect("Failed to create audio manager");
        let dir = std::path::Path::new(file_path).parent().unwrap();
        let raw = std::fs::read_to_string(file_path).unwrap();
        let config: ProjectConfig = toml::from_str(&raw).unwrap();
        let builtins = crate::utils::sprite::builtins::builtins();
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

        let mut project = Project::new(
            dir.to_string_lossy().to_string(),
            dir.join("export").to_string_lossy().to_string(),
            args,
            window,
            builtins,
        );

        let font = if let Some(font_config) = config.font {
            let font_path = dir.join(font_config.file);
            let cpu_texture = CPUTexture::load_from_file(&font_path.to_string_lossy())
                .expect("Failed to load font texture");
            BitmapFont::new(
                cpu_texture.upload_to_gpu(),
                font_config.first_char,
                font_config.char_width,
                font_config.char_height,
                font_config.chars_per_row,
            )
        } else {
            let font_bytes = include_bytes!("../../assets/font.png");
            let font_image = image::load_from_memory(font_bytes)
                .expect("Failed to load font image")
                .to_rgba8();
            let (width, height) = font_image.dimensions();
            let pixels = font_image.into_raw();
            let cpu_texture = CPUTexture::load_from_bytes(&pixels, width, height)
                .expect("Failed to load font texture");
            BitmapFont::new(cpu_texture.upload_to_gpu(), ' ', 12, 7, 12)
        };

        for path in config
            .stage
            .unwrap_or(StageConfig { backdrops: vec![] })
            .backdrops
        {
            let path = dir.join(path);
            let tex = CPUTexture::load_from_file(&path.to_string_lossy())
                .or_else(|_| {
                    CPUTexture::load_from_bytes(
                        include_bytes!("../../assets/missing.png"),
                        100,
                        100,
                    )
                })
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to load backdrop texture: {}. Error: {}",
                        path.to_string_lossy(),
                        e
                    );
                });
            let tex = tex.upload_to_gpu();
            project.stage.backdrops.push(tex);
        }

        if project.stage.backdrops.is_empty() {
            project
                .stage
                .backdrops
                .push(CPUTexture::new(1, 1).upload_to_gpu());
        }

        for sprite in config.sprites {
            let mut textures = vec![];
            for path in sprite.costumes {
                let path = dir.join(path);
                let tex = CPUTexture::load_from_file(&path.to_string_lossy())
                    .or_else(|_| {
                        CPUTexture::load_from_bytes(
                            include_bytes!("../../assets/missing.png"),
                            100,
                            100,
                        )
                    })
                    .unwrap_or_else(|e| {
                        panic!(
                            "Failed to load sprite texture: {}. Error: {}",
                            path.to_string_lossy(),
                            e
                        );
                    });
                textures.push(tex.upload_to_gpu());
            }

            let mut sounds = vec![];
            if sprite.sounds.is_some() {
                let sounds_ = sprite.sounds.unwrap();
                for sound in sounds_ {
                    let path = dir.join(&sound.file);
                    let sound_data = StaticSoundData::from_file(&*path.to_string_lossy())
                        .unwrap_or_else(|e| {
                            panic!(
                                "Failed to load sound: {}. Error: {}",
                                path.to_string_lossy(),
                                e
                            );
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
            audio_manager,
            font,
            debug_options: config.debug_options.unwrap_or(vec![]),
        }
    }

    pub async fn run(
        &mut self,
        window: &mut Window,
        events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
        shader_program: &ShaderProgram,
        glfw: &mut glfw::Glfw,
    ) {
        let mut input_manager = InputManager::new();
        let mut projection = Mat4::orthographic_rh_gl(
            -window.get_size().0 as f32,
            window.get_size().0 as f32,
            -window.get_size().1 as f32,
            window.get_size().1 as f32,
            -1.0,
            1.0,
        );
        let top_left_projection = Mat4::orthographic_rh_gl(
            0.0,
            window.get_size().0 as f32,
            window.get_size().1 as f32,
            0.0,
            -1.0,
            1.0,
        );
        let start = std::time::Instant::now();
        let mut last_time = start;
        let mut duration = std::time::Instant::now();
        let mut fps = 0.0;
        while !window.should_close() {
            let now = std::time::Instant::now();
            let dt = now.duration_since(last_time).as_secs_f32();
            if duration.elapsed().as_secs_f32() >= 1.0 {
                fps = 1.0 / dt;
                duration = std::time::Instant::now();
            }
            last_time = now;

            glfw.poll_events();

            input_manager.update(window, events);

            self.project.stage.draw(window, shader_program, &projection);

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
                sprite.step(
                    start,
                    dt,
                    &mut self.project,
                    &snapshots,
                    window,
                    &mut input_manager,
                    glfw,
                    &mut self.audio_manager,
                    shader_program,
                    &mut projection,
                    &self.font,
                );
            }

            sprites.sort_by(|a, b| a.layer.cmp(&b.layer));

            for sprite in &mut sprites {
                draw_sprite(sprite, shader_program, projection, &self.font);
            }

            self.project.sprites = sprites;

            let mut debug_texts = Vec::new();
            if self.debug_options.contains(&"show_fps".to_string()) {
                debug_texts.push(format!("FPS: {:.2}", fps));
            }
            if self.debug_options.contains(&"show_mouse_pos".to_string()) {
                let pos = vec2(
                    window.get_cursor_pos().0 as f32,
                    window.get_cursor_pos().1 as f32,
                ) * 2.0
                    - vec2(window.get_size().0 as f32, window.get_size().1 as f32);
                let pos = pos * Vec2::new(1.0, -1.0);
                debug_texts.push(format!("Mouse: {:?}", pos));
            }

            for (i, text) in debug_texts.iter().enumerate() {
                draw_text(TextParams {
                    text,
                    projection: top_left_projection,
                    pos: Vec2::new(10.0, 10.0 + i as f32 * 30.0),
                    font_size: 24.0,
                    down_positive: true,
                    ..TextParams::default_params(&self.font, shader_program)
                });
            }

            window.swap_buffers();
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
