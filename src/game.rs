use crate::{
    assets::*,
    config::Config,
    model::*,
    render::GameRender,
    util::{RealConversions, Vec2RealConversions},
};

use geng::prelude::*;

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    render: GameRender,
    model: Model,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, config: Config) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            render: GameRender::new(geng, assets),
            model: Model::new(config),
        }
    }

    fn get_input(&mut self) -> PlayerInput {
        use geng::Key;
        let is_key_down = |keys: &[Key]| {
            keys.iter()
                .any(|&key| self.geng.window().is_key_pressed(key))
        };

        // Movement
        let mut target_move_dir: vec2<f32> = vec2::ZERO;
        if is_key_down(&[Key::A]) {
            target_move_dir.x -= 1.0;
        }
        if is_key_down(&[Key::D]) {
            target_move_dir.x += 1.0;
        }
        if is_key_down(&[Key::S]) {
            target_move_dir.y -= 1.0;
        }
        if is_key_down(&[Key::W]) {
            target_move_dir.y += 1.0;
        }

        // TODO: Head

        PlayerInput {
            target_move_dir: target_move_dir.as_r32(),
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.render.draw(&self.model, framebuffer);
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time.as_r32();

        let input = self.get_input();
        self.model.update(input, delta_time);
    }
}
