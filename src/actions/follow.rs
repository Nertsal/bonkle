use std::marker::PhantomData;

use specs::{Component, Entity, WorldExt};

use crate::components::{BonkleBodyController, Transform};

use super::Action;

pub struct FollowAction<T: Component + Send + Sync> {
    phantom: PhantomData<T>,
    movement_speed: f32,
}

impl<T: Component + Send + Sync> FollowAction<T> {
    pub fn new(movement_speed: f32) -> Self {
        Self {
            movement_speed,
            phantom: PhantomData,
        }
    }
}

impl<T: Component + Send + Sync> Action for FollowAction<T> {
    fn perform(self: Box<Self>, world: &mut specs::World, actor: Entity) {
        let transforms = world.read_component::<Transform>();
        let position = transforms.get(actor).unwrap().position;

        let target = crate::find_closest_entity_world::<T>(position, world);
        if let Some(target) = target
            .map(|target| transforms.get(target))
            .flatten()
            .map(|transform| transform.position)
        {
            world
                .write_component::<BonkleBodyController>()
                .get_mut(actor)
                .unwrap()
                .target_velocity = (target - position).normalize_or_zero() * self.movement_speed;
        }
    }
}
