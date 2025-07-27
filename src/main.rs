// Crust is a Scratch-like game development tool with a custom scripting language.
// Copyright (C) 2025  Muhtasim Noor Al Rasheed & P4ncake
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

const VERT_SHADER: &str = include_str!("../assets/shaders/vertex.glsl");
const FRAG_SHADER: &str = include_str!("../assets/shaders/fragment.glsl");

use clap::Parser;
use glfw::WindowHint;

use crate::utils::core::ShaderProgram;

mod utils;

#[derive(Parser)]
#[command(name = "Crust", version, about)]
struct Args {
    /// Create a new Crust project with the given name.
    #[arg(short, long)]
    new: Option<String>,
    /// The path to the Crust project file. If not provided, a file dialog will open to select one.
    #[arg(short, long)]
    project: Option<String>,
    /// Additional arguments to pass to the Crust runtime.
    #[arg(last = true)]
    additional_args: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Some(new_project_name) = args.new {
        let toml_path = utils::create_new_project(&new_project_name);
        println!("Created new project: {}", new_project_name);
        println!(
            "run `crust --project {}` to run the project.",
            toml_path.display()
        );
        return;
    }

    let project_file = args.project.unwrap_or_else(|| {
        rfd::FileDialog::new()
            .set_title("Select Desired Crust Project")
            .add_filter("Crust Project", &["toml"])
            .pick_file()
            .map(|file| file.as_path().to_string_lossy().to_string())
            .unwrap_or_else(|| panic!("No project file selected"))
    });
    let project_file = project_file.trim_matches('"');

    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).expect("Failed to initialize GLFW");
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(1024, 576, "Crust", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);

    {
        let icon = image::load_from_memory(include_bytes!("../assets/logo_background.png"))
            .expect("Failed to load icon image")
            .to_rgba8();
        let width = icon.width();
        let height = icon.height();
        let pixels = icon.into_raw();
        window.set_icon_from_pixels(vec![glfw::PixelImage {
            width,
            height,
            pixels: pixels
                .chunks_exact(4)
                .map(|p| {
                    let r = p[0] as u32;
                    let g = p[1] as u32;
                    let b = p[2] as u32;
                    let a = p[3] as u32;
                    (a << 24) | (r << 16) | (g << 8) | b
                })
                .collect(),
        }]);
    }

    let mut runtime = utils::Runtime::new(&project_file, args.additional_args, &window);
    println!("Loaded project: {}", project_file);
    let shader_program = ShaderProgram::new(VERT_SHADER, FRAG_SHADER);
    runtime.run(&window, &shader_program).await;
}
