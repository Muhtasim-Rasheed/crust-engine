use macroquad::prelude::*;

pub struct Stage {
    pub backdrops: Vec<Texture2D>,
    pub stamp_layer: Camera2D,
    stamp_layer_target: RenderTarget,
    current_backdrop: usize,
    last_screen_width: f32,
    last_screen_height: f32,
}

impl Stage {
    pub fn new(backdrops: Vec<Texture2D>) -> Self {
        Self {
            backdrops,
            stamp_layer: Camera2D {
                target: vec2(0.0, 0.0),
                zoom: vec2(1.0 / screen_width(), 1.0 / screen_height()),
                render_target: None,
                ..Default::default()
            },
            stamp_layer_target: render_target(screen_width() as u32, screen_height() as u32),
            current_backdrop: 0,
            last_screen_width: 0.0,
            last_screen_height: 0.0,
        }
    }

    pub fn clear_stamps(&mut self) {
        self.stamp_layer_target = render_target(screen_width() as u32, screen_height() as u32);
        self.stamp_layer.render_target = Some(self.stamp_layer_target.clone());
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

    pub fn draw(&mut self) {
        let sw = screen_width();
        let sh = screen_height();

        if sw != self.last_screen_width || sh != self.last_screen_height {
            self.last_screen_width = sw;
            self.last_screen_height = sh;
            self.stamp_layer_target = render_target(sw as u32, sh as u32);
            self.stamp_layer.zoom = vec2(1.0 / sw, 1.0 / sh);
            self.stamp_layer.render_target = Some(self.stamp_layer_target.clone());
        }

        let texture = &self.backdrops[self.current_backdrop];
        texture.set_filter(FilterMode::Nearest);
        let tw = texture.width();
        let th = texture.height();
        let size = if tw / th > sw / sh {
            vec2(sw, th * (sw / tw)) * 2.0
        } else {
            vec2(tw * (sh / th), sh) * 2.0
        };
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
        draw_texture_ex(
            &self.stamp_layer_target.texture,
            -screen_width(),
            -screen_height(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width() * 2.0, screen_height() * 2.0)),
                ..Default::default()
            },
        );
    }
}
