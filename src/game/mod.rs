use super::*;

mod model;
mod renderer;

use model::*;
use renderer::*;

pub struct Game {
    renderer: Renderer,
    model: Model,
}

impl Game {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(),
            model: Model::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.renderer.update(delta_time);
        self.model.update(delta_time);

        let mut dir_x = 0.0;
        if is_key_down(KeyCode::A) {
            dir_x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            dir_x += 1.0;
        }

        let mut dir_y = 0.0;
        if is_key_down(KeyCode::S) {
            dir_y -= 1.0;
        }
        if is_key_down(KeyCode::W) {
            dir_y += 1.0;
        }

        let direction = vec2(dir_x, dir_y);
        self.model.move_direction(direction);
    }

    pub fn fixed_update(&mut self, delta_time: f32) {
        self.model.fixed_update(delta_time);
    }

    pub fn draw(&mut self) {
        self.renderer.draw(&self.model);
    }
}
