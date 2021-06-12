use super::*;

mod physics;
mod player;
mod update;

pub use physics::*;
pub use player::*;

const PLAYER_SPEED: f32 = 40.0;

pub struct Model {
    pub player: Player,
}

impl Model {
    pub fn new() -> Self {
        Self {
            player: Player::new(vec2(0.0, 0.0), 10.0, 20.0),
        }
    }

    pub fn move_direction(&mut self, direction: Vec2) {
        let direction = direction.normalize_or_zero();
        self.player.body.velocity = direction * PLAYER_SPEED;
    }
}
