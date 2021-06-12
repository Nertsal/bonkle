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

    pub fn collide(&self, other: &Self) -> Option<Collision> {
        let offset = self.position - other.position;
        let penetration = self.collider.radius + other.collider.radius - offset.length();
        if penetration >= 0.0 {
            Some(Collision {
                normal: offset.normalize(),
                penetration,
            })
        } else {
            None
        }
    }
}
