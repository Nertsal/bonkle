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
    }

    pub fn fixed_update(&mut self, delta_time: f32) {
        self.model.fixed_update(delta_time);
    }

    pub fn draw(&mut self) {
        self.renderer.draw(&self.model);
    }
}
