use macroquad::{miniquad::conf::Icon, prelude::ImageFormat, texture::Image, window::Conf};

mod utils;

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
    let mut args = std::env::args();
    let project_file = args.nth(1).unwrap_or_else(|| {
        println!("Usage: cargo run <project_file>");
        std::process::exit(1);
    });
    let project_file = project_file.trim_matches('"');
    let mut runtime = utils::Runtime::new(&project_file).await;
    println!("Loaded project: {}", project_file);
    runtime.run().await;
}
