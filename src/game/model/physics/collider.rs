use super::*;

pub enum Collider {
    Square { size: f32 },
    Circle { radius: f32 },
}

impl Collider {
    pub fn circle(radius: f32) -> Self {
        Self::Circle { radius }
    }
    pub fn square(size: f32) -> Self {
        Self::Square { size }
    }
}
