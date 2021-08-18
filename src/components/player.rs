use macroquad::prelude::Vec2;
use specs::{Component, DenseVecStorage, Entity, NullStorage};

#[derive(Default)]
pub struct FriendCollider;

impl Component for FriendCollider {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct FriendTarget;

impl Component for FriendTarget {
    type Storage = NullStorage<Self>;
}

pub struct PlayerBody {
    pub movement_speed: f32,
}

impl Component for PlayerBody {
    type Storage = DenseVecStorage<Self>;
}

pub struct PlayerHead {
    pub movement_speed: f32,
    pub target_pos: Vec2,
    pub connected_to: Entity,
    pub connection_length: f32,
}

impl Component for PlayerHead {
    type Storage = DenseVecStorage<Self>;
}
