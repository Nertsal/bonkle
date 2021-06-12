use super::*;

pub struct RigidBody {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
    pub collider: Collider,
}

impl RigidBody {
    pub fn new(position: Vec2, mass: f32, collider: Collider) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            mass,
            collider,
        }
    }
}
