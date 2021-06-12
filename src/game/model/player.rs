use super::*;

pub struct Player {
    pub body: RigidBody,
    pub head: RigidBody,
    pub chain_length: f32,
    pub head_target: Vec2,
}

impl Player {
    pub fn new(position: Vec2, mass: f32, chain_length: f32, size: f32) -> Self {
        Self {
            body: RigidBody::new(position, mass, Collider::new(size)),
            head: RigidBody::new(
                position + vec2(chain_length, 0.0),
                mass,
                Collider::new(size),
            ),
            chain_length,
            head_target: vec2(1.0, 0.0),
        }
    }
}
