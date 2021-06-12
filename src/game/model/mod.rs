use super::*;
use std::collections::VecDeque;

mod enemy;
mod physics;
mod player;
mod spawner;
mod update;
mod wave;

pub use enemy::*;
pub use physics::*;
pub use player::*;
use spawner::*;
use wave::*;

const PLAYER_SPEED: f32 = 50.0;
const HEAD_SPEED: f32 = 150.0;
const BODY_HIT_SPEED: f32 = 50.0;
const DRAG: f32 = 1.0;

pub struct Model {
    pub bounds: Bounds,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub spawners: Vec<Spawner>,
    pub waves: VecDeque<Wave>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            bounds: Bounds {
                min: vec2(-100.0, -75.0),
                max: vec2(100.0, 75.0),
            },
            player: Player::new(vec2(0.0, 0.0), 10.0, 20.0, 3.0, 500.0),
            enemies: vec![],
            spawners: vec![],
            waves: {
                let mut waves = VecDeque::new();
                waves.push_back(Wave {
                    groups: vec![WaveGroup {
                        enemies: vec![
                            // EnemyInfo::new(150.0, 5.0, 2.0, 25.0, EnemyType::Melee),
                            EnemyInfo::new(
                                150.0,
                                5.0,
                                2.0,
                                25.0,
                                EnemyType::Ranged {
                                    attack_time: 1.0,
                                    attack_cooldown: 1.0,
                                    projectile: Box::new(EnemyInfo::new(
                                        1.0,
                                        1.0,
                                        1.5,
                                        30.0,
                                        EnemyType::Projectile,
                                    )),
                                },
                            ),
                        ],
                    }],
                });
                waves
            },
        }
    }

    pub fn move_direction(&mut self, direction: Vec2) {
        let direction = direction.clamp_length_max(1.0);
        self.player.body.velocity = direction * PLAYER_SPEED;
    }

    pub fn head_target(&mut self, target: Vec2) {
        self.player.head_target = target;
    }
}

pub struct Bounds {
    pub min: Vec2,
    pub max: Vec2,
}
