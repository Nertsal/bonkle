use super::*;
use macroquad::audio::{PlaySoundParams, Sound};
use std::{collections::HashSet, rc::Rc};

mod model;
mod renderer;

use model::*;
use renderer::*;

const BACKGROUND_COLOR: Color = BLACK;
const BORDER_COLOR: Color = GRAY;
const MELEE_COLOR: Color = YELLOW;
const RANGER_COLOR: Color = ORANGE;
const BOMBER_COLOR: Color = WHITE;
const BOMB_COLOR: Color = RED;
const PROJECTILE_COLOR: Color = ORANGE;
const SPAWNER_COLOR: Color = RED;
const PLAYER_COLOR: Color = BLUE;
const PLAYER_BORDER_COLOR: Color = DARKBLUE;
const PLAYER_LIFE_COLOR: Color = DARKBLUE;

pub struct Assets {
    body_hit: Sound,
    head_hit: Sound,
    death: Sound,
    bounce: Sound,
    explosion: Sound,
    music: Sound,
    tutorial: Texture2D,
}

pub struct Game {
    renderer: Renderer,
    model: Model,
    assets: Rc<Assets>,
    last_mouse_position: Vec2,
    head_control_mode: HeadControlMode,
    paused: bool,
}

impl Game {
    pub async fn new() -> Self {
        let assets = Rc::new(Assets {
            body_hit: macroquad::audio::load_sound("body_hit.wav").await.unwrap(),
            head_hit: macroquad::audio::load_sound("head_hit.wav").await.unwrap(),
            death: macroquad::audio::load_sound("death.wav").await.unwrap(),
            bounce: macroquad::audio::load_sound("bounce.wav").await.unwrap(),
            explosion: macroquad::audio::load_sound("explosion.wav").await.unwrap(),
            music: macroquad::audio::load_sound("music.wav").await.unwrap(),
            tutorial: macroquad::texture::load_texture("tutorial.png")
                .await
                .unwrap(),
        });
        assets.tutorial.set_filter(FilterMode::Nearest);
        let game = Self {
            renderer: Renderer::new(&assets),
            model: Model::new(),
            assets,
            last_mouse_position: vec2(0.0, 0.0),
            head_control_mode: HeadControlMode::Keys,
            paused: true,
        };
        macroquad::audio::play_sound(
            game.assets.music.clone(),
            PlaySoundParams {
                looped: true,
                volume: 0.05,
            },
        );
        game
    }

    pub fn update(&mut self, delta_time: f32) {
        self.renderer.update(delta_time, self.paused, &self.model);
        if !self.paused {
            self.model.update(delta_time);
        } else if get_last_key_pressed().is_some() {
            self.paused = false;
        }

        self.control_player();

        self.events();

        if is_key_pressed(KeyCode::R) {
            self.model = Model::new();
            self.renderer = Renderer::new(&self.assets);
            self.paused = true;
        }
    }

    fn control_player(&mut self) {
        // Move body
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

        // Move head
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_position = vec2(mouse_x, mouse_y);
        if mouse_position != self.last_mouse_position {
            let target = self.renderer.game_camera.screen_to_world(mouse_position);
            self.model.head_target(target);
            self.head_control_mode = HeadControlMode::Mouse;
        } else {
            let mut direction = 0.0;
            if is_key_down(KeyCode::Left) {
                direction -= 1.0;
            }
            if is_key_down(KeyCode::Right) {
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

        // Attack
        let mut attacks = HashSet::new();
        if is_key_pressed(KeyCode::Space) {
            attacks.insert(0);
        }
        self.model.player_attack(attacks)
    }

    fn events(&mut self) {
        let events = std::mem::take(&mut self.model.events);
        for event in events {
            match event {
                Event::Sound { sound } => {
                    let sound = match sound {
                        EventSound::HeadHit => self.assets.head_hit.clone(),
                        EventSound::BodyHit => self.assets.body_hit.clone(),
                        EventSound::Death => self.assets.death.clone(),
                        EventSound::Bounce => self.assets.bounce.clone(),
                        EventSound::Explosion => self.assets.explosion.clone(),
                    };
                    macroquad::audio::play_sound_once(sound);
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

enum HeadControlMode {
    Mouse,
    Keys,
}
