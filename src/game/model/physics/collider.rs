use super::*;

#[derive(Debug, Clone)]
pub struct Collider {
    pub radius: f32,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

#[derive(Debug, Clone)]
pub struct Collision {
    pub normal: Vec2,
    pub penetration: f32,
}

#[derive(Debug, Clone)]
pub struct HitInfo {
    pub contact: Vec2,
    pub hit_strength: f32,
}
