use specs::{Component, VecStorage};

pub struct Collidable;

impl Component for Collidable {
    type Storage = VecStorage<Self>;
}
