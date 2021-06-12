use super::*;

pub struct Collider {
    pub radius: f32,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

pub struct Collision {
    pub normal: Vec2,
    pub penetration: f32,
}
