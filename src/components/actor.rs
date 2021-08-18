use specs::{Component, VecStorage};

use crate::actors::Actor;

pub struct ActorComponent(pub Box<dyn Actor>);

impl Component for ActorComponent {
    type Storage = VecStorage<Self>;
}
