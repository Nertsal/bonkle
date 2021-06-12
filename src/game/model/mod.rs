use super::*;
use std::collections::VecDeque;

mod enemy;
mod particle;
mod physics;
mod player;
mod spawner;
mod update;
mod wave;

pub use enemy::*;
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

pub struct Model {
    pub bounds: Bounds,
    pub spawn_bounds: Bounds,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub particles: Vec<Particle>,
    pub spawners: Vec<Spawner>,
    pub waves: VecDeque<Wave>,
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
            player: Player::new(vec2(0.0, 0.0), 10.0, 20.0, 3.0, 250.0),
            enemies: vec![],
            spawners: vec![],
            particles: vec![],
            waves: {
                let melee = EnemyInfo::new(150.0, 5.0, 2.0, 25.0, MELEE_COLOR, EnemyType::Melee);
                let ranger = EnemyInfo::new(
                    150.0,
                    5.0,
                    2.0,
                    25.0,
                    RANGER_COLOR,
                    EnemyType::Ranger {
                        attack_time: 1.0,
                        attack_cooldown: 1.0,
                        projectile: Box::new(EnemyInfo::new(
                            1.0,
                            5.0,
                            1.5,
                            30.0,
                            PROJECTILE_COLOR,
                            EnemyType::Projectile,
                        )),
                    },
                );
                let mut waves = VecDeque::new();
                waves.push_back(Wave {
                    groups: vec![WaveGroup {
                        enemies: vec![melee.clone(), melee.clone()],
                        radius: 10.0,
                    }],
                });
                waves.push_back(Wave {
                    groups: vec![WaveGroup {
                        enemies: vec![melee.clone(), ranger.clone()],
                        radius: 10.0,
                    }],
                });
                waves.push_back(Wave {
                    groups: vec![
                        WaveGroup {
                            enemies: vec![melee.clone(), ranger.clone()],
                            radius: 10.0,
                        },
                        WaveGroup {
                            enemies: vec![melee.clone(), ranger.clone()],
                            radius: 10.0,
                        },
                    ],
                });
                waves
            },
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
