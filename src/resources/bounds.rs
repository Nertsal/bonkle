use macroquad::prelude::{vec2, Vec2};

#[derive(Debug, Clone, Default)]
pub struct Bounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl Bounds {
    pub fn inside(bounds: Self, offset: f32) -> Self {
        Self {
            min: bounds.min + vec2(offset, offset),
            max: bounds.max - vec2(offset, offset),
        }
    }
}
