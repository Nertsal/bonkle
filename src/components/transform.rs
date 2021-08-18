use macroquad::prelude::Vec2;
use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vec2,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
