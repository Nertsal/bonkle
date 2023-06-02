use crate::{
    assets::*,
    model::*,
    render::GameRender,
    util::{RealConversions, Vec2RealConversions},
};

use geng::prelude::*;

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    render: GameRender,
    framebuffer_size: vec2<usize>,
    model: Model,
    head_control_mode: HeadControlMode,
}

#[derive(Debug)]
enum HeadControlMode {
    Delta,
    LookAt,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, config: Config, entities: EntitiesAssets) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            render: GameRender::new(geng, assets),
            framebuffer_size: vec2(1, 1),
            model: Model::new(config, entities),
            head_control_mode: HeadControlMode::Delta,
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
        let head_target = match self.head_control_mode {
            HeadControlMode::Delta => {
                let mut dir = 0.0;
                if is_key_down(&[Key::Left]) {
                    dir += 1.0; // Counter clock-wise
                }
                if is_key_down(&[Key::Right]) {
                    dir -= 1.0; // Clock-wise
                }
                RotationTarget::Relative {
                    delta: Angle::from_radians(dir.as_r32()),
                }
            }
            HeadControlMode::LookAt => {
                let mouse_pos = self.geng.window().cursor_position().as_f32();
                let world_pos = self
                    .model
                    .camera
                    .screen_to_world(self.framebuffer_size.as_f32(), mouse_pos);
                RotationTarget::LookAt {
                    position: world_pos.as_r32(),
                }
            }
        };

        PlayerInput {
            target_move_dir: target_move_dir.as_r32(),
            head_target,
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.render.draw(&self.model, framebuffer);
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time.as_r32();

        let input = self.get_input();
        self.model.update(input, delta_time);
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { .. } => {
                self.head_control_mode = HeadControlMode::Delta;
            }
            geng::Event::MouseDown { .. } => {
                self.head_control_mode = HeadControlMode::LookAt;
            }
            geng::Event::MouseMove { .. } => {
                self.head_control_mode = HeadControlMode::LookAt;
            }
            geng::Event::MouseUp { .. } => {
                self.head_control_mode = HeadControlMode::LookAt;
            }
            _ => {}
        }
    }
}
