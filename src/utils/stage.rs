use macroquad::prelude::*;

pub struct Stage {
    pub backdrops: Vec<Texture2D>,
    current_backdrop: usize,
}

impl Stage {
    pub fn new(backdrops: Vec<Texture2D>) -> Self {
        Self {
            backdrops,
            current_backdrop: 0,
        }
    }

    pub fn set_backdrop(&mut self, index: usize) {
        if index < self.backdrops.len() {
            self.current_backdrop = index;
        }
    }

    pub fn next_backdrop(&mut self) {
        self.current_backdrop = (self.current_backdrop + 1) % self.backdrops.len();
    }

    pub fn prev_backdrop(&mut self) {
        if self.current_backdrop == 0 {
            self.current_backdrop = self.backdrops.len() - 1;
        } else {
            self.current_backdrop -= 1;
        }
    }

    pub fn backdrop(&self) -> usize {
        self.current_backdrop
    }

    pub fn draw(&self) {
        let texture = &self.backdrops[self.current_backdrop];
        texture.set_filter(FilterMode::Nearest);
        let sw = screen_width();
        let sh = screen_height();
        let tw = texture.width();
        let th = texture.height();
        let size = if tw / th > sw / sh {
            vec2(sw, th * (sw / tw)) * 2.0
        } else {
            vec2(tw * (sh / th), sh) * 2.0
        };
        // let x = (sw - size.x) / 2.0;
        // let y = (sh - size.y) / 2.0;
        let x = -size.x / 2.0;
        let y = -size.y / 2.0;
        draw_texture_ex(
            texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
    }
}
