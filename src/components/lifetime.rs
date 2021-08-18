use specs::{Component, DenseVecStorage};

use super::Health;

#[derive(Debug, Clone)]
pub struct Lifetime(pub Health);

impl Component for Lifetime {
    type Storage = DenseVecStorage<Self>;
}
