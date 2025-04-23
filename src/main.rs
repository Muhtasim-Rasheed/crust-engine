mod utils;

#[macroquad::main("Crust")]
async fn main() {
    let mut args = std::env::args();
    let project_file = args.nth(1).unwrap_or_else(|| {
        println!("Usage: cargo run <project_file>");
        std::process::exit(1);
    });
    let mut runtime = utils::Runtime::new(&project_file).await;
    println!("Loaded project: {}", project_file);
    runtime.run().await;
}
