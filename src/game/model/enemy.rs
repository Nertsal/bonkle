use super::*;

pub struct Enemy {
    pub rigidbody: RigidBody,
}

impl Enemy {
    pub fn new(position: Vec2, mass: f32, size: f32) -> Self {
        Self {
            rigidbody: RigidBody::new(position, mass, Collider::new(size)),
        }
    }
}
