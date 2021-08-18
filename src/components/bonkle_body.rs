use macroquad::prelude::Vec2;
use specs::{Component, VecStorage};

use crate::physics::PhysicsMaterial;

pub struct BonkleBody {
    pub mass: f32,
    pub radius: f32,
    pub velocity: Vec2,
    pub physics_material: PhysicsMaterial,
}

impl Component for BonkleBody {
    type Storage = VecStorage<Self>;
}
