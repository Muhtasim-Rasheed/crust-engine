use std::f32::consts::PI;

use crate::utils::{RotationStyle, Sprite};

trait Effect {
    fn apply(&self, pixel: Color) -> Color;
}

enum GrayscaleType {
    Averaged(f32),
    Weighted(f32),
}

enum MultiplyType {
    All(f32),
    R(f32),
    G(f32),
    B(f32),
}

enum AddType {
    All(f32),
    R(f32),
    G(f32),
    B(f32),
}

struct Brightness(f32);
impl Effect for Brightness {
    fn apply(&self, pixel: Color) -> Color {
        let brightness = (self.0 / 100.0).clamp(-1.0, 1.0);
        Color::new(
            pixel.r + brightness,
            pixel.g + brightness,
            pixel.b + brightness,
            pixel.a,
        )
    }
}

struct Ghost(f32);
impl Effect for Ghost {
    fn apply(&self, pixel: Color) -> Color {
        let alpha = (self.0 / 100.0).clamp(0.0, 1.0);
        Color::new(pixel.r, pixel.g, pixel.b, pixel.a * alpha)
    }
}

struct Hue(f32);
impl Effect for Hue {
    fn apply(&self, pixel: Color) -> Color {
        let hue = self.0;
        let cos_a = (hue * PI / 180.).cos();
        let sin_a = (hue * PI / 180.).sin();
        let onethird: f32 = 1. / 3.;
        Color::new(
            pixel.r * (cos_a + (1. - cos_a) / 3.)
                + pixel.g * (onethird * (1. - cos_a) - onethird.sqrt() * sin_a)
                + pixel.b * (onethird * (1. - cos_a) + onethird.sqrt() * sin_a),
            pixel.r * (onethird * (1. - cos_a) + onethird.sqrt() * sin_a)
                + pixel.g * (cos_a + onethird * (1. - cos_a))
                + pixel.b * (onethird * (1. - cos_a) - onethird.sqrt() * sin_a),
            pixel.r * (onethird * (1. - cos_a) - onethird.sqrt() * sin_a)
                + pixel.g * (onethird * (1. - cos_a) + onethird.sqrt() * sin_a)
                + pixel.b * (cos_a + onethird * (1. - cos_a)),
            pixel.a,
        )
    }
}

struct Saturation(f32);
impl Effect for Saturation {
    fn apply(&self, pixel: Color) -> Color {
        let saturation = (self.0 / 100.0).clamp(0.0, 100.0);
        Color::new(
            crate::utils::lerp(
                pixel.r * 0.299 + pixel.g * 0.587 + pixel.b * 0.114,
                pixel.r,
                saturation,
            ),
            crate::utils::lerp(
                pixel.r * 0.299 + pixel.g * 0.587 + pixel.b * 0.114,
                pixel.g,
                saturation,
            ),
            crate::utils::lerp(
                pixel.r * 0.299 + pixel.g * 0.587 + pixel.b * 0.114,
                pixel.b,
                saturation,
            ),
            pixel.a,
        )
    }
}

struct Sepia(f32);
impl Effect for Sepia {
    fn apply(&self, pixel: Color) -> Color {
        let sepia = (self.0 / 100.0).clamp(0.0, 1.0);
        Color::new(
            crate::utils::lerp(
                pixel.r,
                pixel.r * 0.393 + pixel.g * 0.769 + pixel.b * 0.189,
                sepia,
            ),
            crate::utils::lerp(
                pixel.g,
                pixel.g * 0.349 + pixel.b * 0.686 + pixel.r * 0.168,
                sepia,
            ),
            crate::utils::lerp(
                pixel.b,
                pixel.b * 0.272 + pixel.r * 0.534 + pixel.g * 0.131,
                sepia,
            ),
            pixel.a,
        )
    }
}

struct Grayscale(GrayscaleType);
impl Effect for Grayscale {
    fn apply(&self, pixel: Color) -> Color {
        match self.0 {
            GrayscaleType::Averaged(value) => {
                let grayscale = (value / 100.0).clamp(0.0, 1.0);
                Color::new(
                    crate::utils::lerp(
                        pixel.r,
                        pixel.r / 3. + pixel.g / 3. + pixel.b / 3.,
                        grayscale,
                    ),
                    crate::utils::lerp(
                        pixel.g,
                        pixel.r / 3. + pixel.g / 3. + pixel.b / 3.,
                        grayscale,
                    ),
                    crate::utils::lerp(
                        pixel.b,
                        pixel.r / 3. + pixel.g / 3. + pixel.b / 3.,
                        grayscale,
                    ),
                    pixel.a,
                )
            }
            GrayscaleType::Weighted(value) => {
                let grayscale = (value / 100.0).clamp(0.0, 1.0);
                Color::new(
                    crate::utils::lerp(
                        pixel.r,
                        pixel.r * 0.299 + pixel.g * 0.587 + pixel.b * 0.114,
                        grayscale,
                    ),
                    crate::utils::lerp(
                        pixel.g,
                        pixel.r * 0.299 + pixel.g * 0.587 + pixel.b * 0.114,
                        grayscale,
                    ),
                    crate::utils::lerp(
                        pixel.b,
                        pixel.r * 0.299 + pixel.g * 0.587 + pixel.b * 0.114,
                        grayscale,
                    ),
                    pixel.a,
                )
            }
        }
    }
}

struct Invert(f32);
impl Effect for Invert {
    fn apply(&self, pixel: Color) -> Color {
        let invert = (self.0 / 100.0).clamp(0.0, 1.0);
        Color::new(
            crate::utils::lerp(pixel.r, 1.0 - pixel.r, invert),
            crate::utils::lerp(pixel.g, 1.0 - pixel.g, invert),
            crate::utils::lerp(pixel.b, 1.0 - pixel.b, invert),
            pixel.a,
        )
    }
}

struct Multiply(MultiplyType);
impl Effect for Multiply {
    fn apply(&self, pixel: Color) -> Color {
        match self.0 {
            MultiplyType::All(value) => {
                Color::new(pixel.r * value, pixel.g * value, pixel.b * value, pixel.a)
            }
            MultiplyType::R(value) => Color::new(pixel.r * value, pixel.g, pixel.b, pixel.a),
            MultiplyType::G(value) => Color::new(pixel.r, pixel.g * value, pixel.b, pixel.a),
            MultiplyType::B(value) => Color::new(pixel.r, pixel.g, pixel.b * value, pixel.a),
        }
    }
}

struct Add(AddType);
impl Effect for Add {
    fn apply(&self, pixel: Color) -> Color {
        match self.0 {
            AddType::All(value) => {
                Color::new(pixel.r + value, pixel.g + value, pixel.b + value, pixel.a)
            }
            AddType::R(value) => Color::new(pixel.r + value, pixel.g, pixel.b, pixel.a),
            AddType::G(value) => Color::new(pixel.r, pixel.g + value, pixel.b, pixel.a),
            AddType::B(value) => Color::new(pixel.r, pixel.g, pixel.b + value, pixel.a),
        }
    }
}

pub fn draw_sprite(sprite: &Sprite) {
    for clone in &sprite.clones {
        draw_sprite(clone);
    }

    if !sprite.visible {
        return;
    }

    let scaled_size = sprite.size * sprite.scale;
    let top_left = sprite.center - scaled_size / 2.0;

    let mut effect_image = sprite.costumes[sprite.costume()].get_texture_data();
    for (effect, value) in &sprite.effects {
        let effect = match effect.as_str() {
            "brightness" => Box::new(Brightness(*value)) as Box<dyn Effect>,
            "ghost" => Box::new(Ghost(*value)) as Box<dyn Effect>,
            "hue" => Box::new(Hue(*value)) as Box<dyn Effect>,
            "saturation" => Box::new(Saturation(*value)) as Box<dyn Effect>,
            "sepia" => Box::new(Sepia(*value)) as Box<dyn Effect>,
            "grayscale-averaged" => {
                Box::new(Grayscale(GrayscaleType::Averaged(*value))) as Box<dyn Effect>
            }
            "grayscale-weighted" => {
                Box::new(Grayscale(GrayscaleType::Weighted(*value))) as Box<dyn Effect>
            }
            "invert" => Box::new(Invert(*value)) as Box<dyn Effect>,
            "multiply" => Box::new(Multiply(MultiplyType::All(*value))) as Box<dyn Effect>,
            "multiply-r" => Box::new(Multiply(MultiplyType::R(*value))) as Box<dyn Effect>,
            "multiply-g" => Box::new(Multiply(MultiplyType::G(*value))) as Box<dyn Effect>,
            "multiply-b" => Box::new(Multiply(MultiplyType::B(*value))) as Box<dyn Effect>,
            "add" => Box::new(Add(AddType::All(*value))) as Box<dyn Effect>,
            "add-r" => Box::new(Add(AddType::R(*value))) as Box<dyn Effect>,
            "add-g" => Box::new(Add(AddType::G(*value))) as Box<dyn Effect>,
            "add-b" => Box::new(Add(AddType::B(*value))) as Box<dyn Effect>,
            _ => continue, // Do absolutely nothing
        };
        for i in 0..effect_image.width() {
            for j in 0..effect_image.height() {
                let pixel = effect_image.get_pixel(i as u32, j as u32);
                let new_pixel = effect.apply(pixel);
                effect_image.set_pixel(i as u32, j as u32, new_pixel);
            }
        }
    }
    let processed_texture = Texture2D::from_image(&effect_image);
    processed_texture.set_filter(FilterMode::Nearest);

    draw_texture_ex(
        &processed_texture,
        top_left.x,
        top_left.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(scaled_size),
            rotation: if sprite.rotation_style == RotationStyle::AllAround {
                sprite.direction.to_radians()
            } else if sprite.rotation_style == RotationStyle::LeftRight
                || sprite.rotation_style == RotationStyle::DontRotate
            {
                0.0
            } else {
                0.0
            },
            flip_x: if sprite.rotation_style == RotationStyle::LeftRight {
                sprite.direction > 90.0 && sprite.direction < 270.0
            } else {
                false
            },
            ..Default::default()
        },
    );

    if let Some(dialogue) = &sprite.dialogue {
        let dialogue_size = measure_text(&dialogue.text, None, 72, 1.0);
        draw_text_ex(
            &dialogue.text,
            sprite.center.x - dialogue_size.width / 2.0,
            sprite.center.y - scaled_size.y / 2.0 - dialogue_size.height,
            TextParams {
                font_size: 72,
                color: if dialogue.think {
                    Color::new(1.0, 1.0, 1.0, 0.7)
                } else {
                    Color::new(1.0, 1.0, 1.0, 1.0)
                },
                ..Default::default()
            },
        );
    }
}
