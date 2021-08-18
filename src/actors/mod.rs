use specs::{Entity, World};

use crate::actions::Action;

mod enemy_melee;
mod enemy_ranger;

pub use enemy_melee::EnemyMelee;
pub use enemy_ranger::EnemyRanger;

pub trait Actor: Sync + Send {
    fn update(&mut self, actor: Entity, world: &World, delta_time: f32) -> Vec<Box<dyn Action>>;
}
