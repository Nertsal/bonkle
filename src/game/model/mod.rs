use super::*;

mod physics;
mod player;
mod update;

pub use physics::*;
pub use player::*;

pub struct Model {
    pub player: Player,
}

impl Model {
    pub fn new() -> Self {
        Self {
            player: Player::new(vec2(0.0, 0.0), 10.0, 20.0),
        }
    }
}
