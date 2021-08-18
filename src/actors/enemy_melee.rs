use specs::{Entity, World};

use crate::{
    actions::{Action, FollowAction},
    components::FriendTarget,
};

use super::Actor;

pub struct EnemyMelee {
    pub movement_speed: f32,
}

impl Actor for EnemyMelee {
    fn update(&mut self, _actor: Entity, _world: &World, _delta_time: f32) -> Vec<Box<dyn Action>> {
        vec![Box::new(FollowAction::<FriendTarget>::new(
            self.movement_speed,
        ))]
    }
}
