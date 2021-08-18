use macroquad::prelude::Vec2;
use specs::{Component, VecStorage};

pub struct BonkleBodyController {
    pub target_velocity: Vec2,
    pub acceleration: f32,
}

impl Component for BonkleBodyController {
    type Storage = VecStorage<Self>;
}
