use macroquad::prelude::*;

mod utils;

#[macroquad::main("Crust")]
async fn main() {
    let mut sprite = utils::Sprite::new(
        "Bread".to_string(),
        vec![
            load_texture("test/bread.png").await.unwrap(),
        ],
        200.0,
        100.0,
        screen_width() / 2.0,
        screen_height() / 2.0,
    );
    sprite.new_variable("angular velocity", utils::Value::Number(0.0));

    loop {
        clear_background(WHITE);
        
        sprite.goto_cursor();
        if is_key_down(KeyCode::Right) {
            sprite.change_variable_by(
                "angular velocity",
                utils::Value::Number(1.0),
            );
        }
        if is_key_down(KeyCode::Left) {
            sprite.change_variable_by(
                "angular velocity",
                utils::Value::Number(-1.0),
            );
        }
        sprite.set_variable(
            "angular velocity",
            utils::Value::Number(sprite.variable("angular velocity").to_number() * 0.95),
        );
        sprite.set_center_offset(0.0, (get_time() * 10.0).sin() as f32 * 10.0);
        sprite.direction += sprite.variable("angular velocity").to_number();
        sprite.draw();

        next_frame().await;
    }
}
