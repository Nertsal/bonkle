use super::*;

#[derive(Debug, Clone)]
pub struct Health {
    pub hp: f32,
    pub hp_max: f32,
}

impl Health {
    pub fn new(hp: f32) -> Self {
        Self { hp, hp_max: hp }
    }

    pub fn change(&mut self, delta: f32) {
        self.hp = (self.hp + delta).clamp(0.0, self.hp_max);
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0.0
    }

    pub fn hp_frac(&self) -> f32 {
        self.hp / self.hp_max
    }

    pub fn kill(&mut self) {
        self.hp = 0.0;
    }
}
