use macroquad::window::Conf;

mod utils;

fn window_config() -> Conf {
    Conf {
        window_title: "Crust".to_owned(),
        window_width: 1024,
        window_height: 576,
        window_resizable: false,
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
