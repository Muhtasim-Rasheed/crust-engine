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

use macroquad::{miniquad::conf::Icon, prelude::ImageFormat, texture::Image, window::Conf};
use clap::Parser;

mod utils;

#[derive(Parser)]
#[command(name = "Crust", version = env!("CARGO_PKG_VERSION"), about = "A Scratch-like game development tool with a custom scripting language.")]
struct Args {
    #[arg(short, long)]
    project: Option<String>,
    #[arg(last = true)]
    additional_args: Vec<String>,
}

fn window_config() -> Conf {
    let small = utils::flatten(Image::from_file_with_format(include_bytes!("../icons/icon16.png"), Some(ImageFormat::Png)).unwrap().get_image_data().to_vec());
    let medium = utils::flatten(Image::from_file_with_format(include_bytes!("../icons/icon32.png"), Some(ImageFormat::Png)).unwrap().get_image_data().to_vec());
    let big = utils::flatten(Image::from_file_with_format(include_bytes!("../icons/icon64.png"), Some(ImageFormat::Png)).unwrap().get_image_data().to_vec());
    Conf {
        window_title: "Crust".to_owned(),
        window_width: 1024,
        window_height: 576,
        window_resizable: false,
        icon: Some(Icon {
            small: small.try_into().unwrap(),
            medium: medium.try_into().unwrap(),
            big: big.try_into().unwrap(),
        }),
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    // let mut args = std::env::args();
    let args = Args::parse();
    let project_file = args.project.unwrap_or_else(|| {
        rfd::FileDialog::new()
            .set_title("Select Desired Crust Project")
            .add_filter("Crust Project", &["toml"])
            .pick_file()
            .map(|file| file.as_path().to_string_lossy().to_string())
            .unwrap_or_else(|| panic!("No project file selected"))
    });
    let project_file = project_file.trim_matches('"');
    let mut runtime = utils::Runtime::new(&project_file, args.additional_args).await;
    println!("Loaded project: {}", project_file);
    runtime.run().await;
}
