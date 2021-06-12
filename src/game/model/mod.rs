use super::*;

mod enemy;
mod physics;
mod player;
mod update;

pub use enemy::*;
pub use physics::*;
pub use player::*;

const PLAYER_SPEED: f32 = 40.0;
const HEAD_SPEED: f32 = 70.0;

pub struct Model {
    pub player: Player,
    pub enemies: Vec<Enemy>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            player: Player::new(vec2(0.0, 0.0), 10.0, 20.0, 3.0),
            enemies: vec![Enemy::new(vec2(50.0, 10.0), 5.0, 2.0)],
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
