use specs::{Entities, Join, Read, System, WriteStorage};

use crate::{
    components::{Health, Lifetime},
    resources::Time,
};

pub struct LifetimeSystem;

impl<'s> System<'s> for LifetimeSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Lifetime>,
        WriteStorage<'s, Health>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut lifetimes, mut healths, time): Self::SystemData) {
        for (entity, lifetime) in (&entities, &mut lifetimes).join() {
            lifetime.0.change(-time.delta_time);
            if !lifetime.0.is_alive() {
                match healths.get_mut(entity) {
                    Some(health) => {
                        health.kill();
                    }
                    None => {
                        entities.delete(entity).unwrap();
                    }
                }
            }
        }
    }
}
