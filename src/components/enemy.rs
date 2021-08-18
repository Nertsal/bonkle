use specs::{Component, NullStorage};

#[derive(Default)]
pub struct EnemyCollider;

impl Component for EnemyCollider {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct EnemyTarget;

impl Component for EnemyTarget {
    type Storage = NullStorage<Self>;
}
