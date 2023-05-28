mod model;
mod renderer;

use crate::assets::Assets;

use self::model::*;
use self::renderer::*;

use std::{collections::HashSet, rc::Rc};

use geng::prelude::*;
use geng::Camera2d;

type Color = Rgba<f32>;

const BACKGROUND_COLOR: Color = Color::BLACK;
const BORDER_COLOR: Color = Color::GRAY;
const MELEE_COLOR: Color = Color::YELLOW;
const RANGER_COLOR: Color = Color {
    r: 1.0,
    g: 0.63,
    b: 0.0,
    a: 1.0,
};
const BOMBER_COLOR: Color = Color::WHITE;
const BOMB_COLOR: Color = Color::RED;
const PROJECTILE_COLOR: Color = Color {
    r: 1.0,
    g: 0.63,
    b: 0.0,
    a: 1.0,
};
const SPAWNER_COLOR: Color = Color::RED;
const PLAYER_COLOR: Color = Color::BLUE;
const PLAYER_BORDER_COLOR: Color = Color {
    r: 0.00,
    g: 0.32,
    b: 0.67,
    a: 1.00,
};
const PLAYER_LIFE_COLOR: Color = Color {
    r: 0.00,
    g: 0.32,
    b: 0.67,
    a: 1.00,
};

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    renderer: Renderer,
    model: Model,
    last_mouse_position: vec2<f64>,
    head_control_mode: HeadControlMode,
    state: GameState,
}

enum HeadControlMode {
    Mouse,
    Keys,
}

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    Menu,
    Pregame,
    Game,
}

pub enum GameUpdate {
    Quit,
    Start,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        let game = Self {
            geng: geng.clone(),
            assets: assets.clone(),
            renderer: Renderer::new(&assets),
            model: Model::new(),
            last_mouse_position: vec2(0.0, 0.0),
            head_control_mode: HeadControlMode::Keys,
            state: GameState::Menu,
        };
        let mut music = assets.music.play();
        music.set_volume(0.05);
        game
    }

    pub fn update(&mut self, delta_time: f32) {
        if let Some(update) = self.renderer.update(delta_time, self.state, &self.model) {
            match update {
                GameUpdate::Quit => unimplemented!(),
                GameUpdate::Start => self.state = GameState::Pregame,
            }
        }
        match self.state {
            GameState::Menu => {
                self.control_head();
            }
            GameState::Pregame => {
                self.control_head();
                // TODO
                // if get_last_key_pressed().is_some() {
                //     self.state = GameState::Game;
                // }
            }
            GameState::Game => {
                self.control_head();
                self.control_body();
                self.model.update(delta_time);
            }
        }

        self.events();
    }

    fn control_body(&mut self) {
        use geng::Key;

        let window = self.geng.window();
        let is_pressed = |keys: &[Key]| keys.iter().any(|&key| window.is_key_pressed(key));

        let mut dir_x = 0.0;
        if is_pressed(&[Key::A]) {
            dir_x -= 1.0;
        }
        if is_pressed(&[Key::D]) {
            dir_x += 1.0;
        }

        let mut dir_y = 0.0;
        if is_pressed(&[Key::S]) {
            dir_y -= 1.0;
        }
        if is_pressed(&[Key::W]) {
            dir_y += 1.0;
        }

        let direction = vec2(dir_x, dir_y);
        self.model.move_direction(direction);

        // Attack
        let mut attacks = HashSet::new();
        if is_pressed(&[Key::Space]) {
            attacks.insert(0);
        }
        self.model.player_attack(attacks)
    }

    fn control_head(&mut self) {
        let mouse_position = self.geng.window().mouse_position();
        if mouse_position != self.last_mouse_position {
            // Mouse control
            let target = self.renderer.game_camera.screen_to_world(mouse_position);
            self.model.head_target(target);
            self.head_control_mode = HeadControlMode::Mouse;
        } else {
            // Keyboard control
            let mut direction = 0.0;
            if self.geng.window().is_key_pressed(geng::Key::Left) {
                direction -= 1.0;
            }
            if self.geng.window().is_key_pressed(geng::Key::Right) {
                direction += 1.0;
            }

            if direction != 0.0 {
                let target =
                    self.model.player.head.position - self.model.player.entity.rigidbody.position;
                let target = vec2(target.y, -target.x).normalize() * direction * 5.0
                    + self.model.player.head.position;
                self.model.head_target(target);
                self.head_control_mode = HeadControlMode::Keys;
            } else {
                match self.head_control_mode {
                    HeadControlMode::Mouse => (),
                    HeadControlMode::Keys => {
                        self.model.head_target(self.model.player.head.position)
                    }
                }
            }
        }
        self.last_mouse_position = mouse_position;
    }

    fn events(&mut self) {
        let events = std::mem::take(&mut self.model.events);
        for event in events {
            match event {
                Event::Sound { sound } => {
                    let sound = match sound {
                        EventSound::HeadHit => &self.assets.head_hit,
                        EventSound::BodyHit => &self.assets.body_hit,
                        EventSound::Death => &self.assets.death,
                        EventSound::Bounce => &self.assets.bounce,
                        EventSound::Explosion => &self.assets.explosion,
                    };
                    sound.play();
                }
                Event::NextWave { stage } => self.renderer.next_wave(stage),
            }
        }
    }

    pub fn fixed_update(&mut self, delta_time: f32) {
        self.model.fixed_update(delta_time);
    }

    pub fn draw(&mut self) {
        self.renderer.draw(&self.model);
    }
}
