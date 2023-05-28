use super::*;

pub struct AreaEffect {
    pub position: vec2<f32>,
    pub radius: f32,
    pub effect: Effect,
    pub lifetime: Health,
}

pub enum Effect {
    Heal { heal: f32 },
}
