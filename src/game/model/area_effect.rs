use super::*;

pub struct AreaEffect {
    pub position: Vec2,
    pub radius: f32,
    pub effect: Effect,
    pub lifetime: Health,
}

pub enum Effect {
    Heal { heal: f32 },
}
