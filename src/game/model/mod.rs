use super::*;

mod enemy;
mod event;
mod particle;
mod physics;
mod player;
mod spawner;
mod update;
mod wave;

pub use enemy::*;
pub use event::*;
pub use particle::*;
pub use physics::*;
pub use player::*;
use spawner::*;
use wave::*;

const PLAYER_SPEED: f32 = 50.0;
const HEAD_SPEED: f32 = 150.0;
const BODY_HIT_SPEED: f32 = 50.0;
const DRAG: f32 = 1.0;
const CORPSE_LIFETIME: f32 = 2.5;
const PARTICLE_LIFETIME: f32 = 1.0;
const BODY_ACCELERATION: f32 = 3.0;
const HEAD_ACCELERATION: f32 = 10.0;
const BODY_IMPACT: f32 = 50.0;

pub struct Model {
    pub bounds: Bounds,
    pub spawn_bounds: Bounds,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub particles: Vec<Particle>,
    pub spawners: Vec<Spawner>,
    pub current_stage: usize,
    pub events: Vec<Event>,
    pub game_start_timer: f32,
}

impl Model {
    pub fn new() -> Self {
        Self {
            bounds: Bounds {
                min: vec2(-95.0, -70.0),
                max: vec2(95.0, 70.0),
            },
            spawn_bounds: Bounds {
                min: vec2(-75.0, -50.0),
                max: vec2(75.0, 50.0),
            },
            player: Player::new(vec2(0.0, 0.0), 10.0, 20.0, 2.0, 3.0, 250.0),
            enemies: vec![],
            spawners: vec![],
            particles: vec![],
            current_stage: 0,
            events: vec![],
            game_start_timer: 5.0,
        }
    }

    pub fn move_direction(&mut self, direction: Vec2) {
        let direction = direction.clamp_length_max(1.0);
        self.player.target_body_velocity = direction * PLAYER_SPEED;
    }

    pub fn head_target(&mut self, target: Vec2) {
        self.player.head_target = target;
    }
}

pub struct Bounds {
    pub min: Vec2,
    pub max: Vec2,
}
