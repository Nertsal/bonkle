use super::*;

pub struct Player {
    pub body: RigidBody,
    pub head: RigidBody,
    pub chain_length: f32,
    pub head_target: Vec2,
    pub health: f32,
    pub target_body_velocity: Vec2,
    pub target_head_velocity: Vec2,
}

impl Player {
    pub fn new(
        position: Vec2,
        mass: f32,
        chain_length: f32,
        body_size: f32,
        head_size: f32,
        health: f32,
    ) -> Self {
        Self {
            body: RigidBody::new(position, mass, Collider::new(body_size)),
            head: RigidBody::new(
                position + vec2(chain_length, 0.0),
                mass,
                Collider::new(head_size),
            ),
            chain_length,
            health,
            head_target: vec2(1.0, 0.0),
            target_body_velocity: vec2(0.0, 0.0),
            target_head_velocity: vec2(0.0, 0.0),
        }
    }
}
