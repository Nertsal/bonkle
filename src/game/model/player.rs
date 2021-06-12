use super::*;

pub struct Player {
    pub body: RigidBody,
    pub head: RigidBody,
    pub chain_length: f32,
    pub head_target: Vec2,
}

impl Player {
    pub fn new(position: Vec2, mass: f32, chain_length: f32) -> Self {
        Self {
            body: RigidBody::new(position, mass, Collider::circle(3.0)),
            head: RigidBody::new(
                position + vec2(chain_length, 0.0),
                mass,
                Collider::circle(3.0),
            ),
            chain_length,
            head_target: vec2(0.0, 0.0),
        }
    }
}
