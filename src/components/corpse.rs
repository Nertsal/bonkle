use specs::{Component, VecStorage};

use super::Health;

pub struct Corpse {
    pub lifetime: Health,
}

impl Component for Corpse {
    type Storage = VecStorage<Self>;
}

impl Corpse {
    pub fn new(lifetime: f32) -> Self {
        Self {
            lifetime: Health::new(lifetime),
        }
    }
}
