use std::collections::HashSet;

use super::*;

mod area_effect;
mod attack;
mod commands;
mod corpse;
mod enemy;
mod entity;
mod event;
mod health;
mod minion;
mod particle;
mod physics;
mod player;
mod spawner;
mod update;
mod wave;

pub use area_effect::*;
pub use attack::*;
use commands::*;
use corpse::*;
pub use enemy::*;
pub use entity::*;
pub use event::*;
pub use health::*;
pub use minion::*;
pub use particle::*;
pub use physics::*;
pub use player::*;
use spawner::*;
use wave::*;

const PLAYER_SPEED: f32 = 50.0;
const HEAD_SPEED: f32 = 150.0;
const BODY_HIT_SPEED: f32 = 50.0;
const DRAG: f32 = 1.0;
const BOUNCINESS: f32 = 0.2;
const CORPSE_LIFETIME: f32 = 2.5;
const PARTICLE_LIFETIME: f32 = 1.0;
const BODY_ACCELERATION: f32 = 3.0;
const HEAD_ACCELERATION: f32 = 10.0;
const BODY_IMPACT: f32 = 50.0;

pub struct Model {
    pub bounds: Bounds,
    pub spawn_bounds: Bounds,
    pub player: Player,
    pub entities: Vec<Box<dyn EntityObject>>,
    pub particles: Vec<Particle>,
    pub area_effects: Vec<AreaEffect>,
    pub spawners: Vec<Spawner>,
    pub current_stage: usize,
    pub events: Vec<Event>,
}

impl Model {
    pub fn new() -> Self {
        let bounds = Bounds {
            min: vec2(-160.0, -90.0),
            max: vec2(160.0, 90.0),
        };
        Self {
            bounds,
            spawn_bounds: Bounds::inside(bounds, 20.0),
            player: Player::new(
                vec2(0.0, 0.0),
                PlayerInfo::new(
                    3.0,
                    20.0,
                    EntityInfo::new(
                        Health::new(250.0),
                        10.0,
                        2.0,
                        PLAYER_SPEED,
                        BLUE,
                        PhysicsMaterial::new(0.0, 1.0),
                    ),
                ),
            ),
            entities: vec![],
            area_effects: vec![],
            spawners: vec![],
            particles: vec![],
            current_stage: 0,
            events: vec![],
        }
    }

    pub fn move_direction(&mut self, direction: Vec2) {
        let direction = direction.clamp_length_max(1.0);
        self.player.target_body_velocity = direction * PLAYER_SPEED;
    }

    pub fn head_target(&mut self, target: Vec2) {
        self.player.head_target = target;
    }

    pub fn player_attack(&mut self, attacks: HashSet<usize>) {
        self.player.perform_attacks.extend(attacks);
    }
}

#[derive(Clone, Copy)]
pub struct Bounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl Bounds {
    pub fn inside(bounds: Self, offset: f32) -> Self {
        Self {
            min: bounds.min + vec2(offset, offset),
            max: bounds.max - vec2(offset, offset),
        }
    }
}
