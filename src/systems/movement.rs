use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::{
    components::{BonkleBody, Transform},
    resources::Time,
};

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, BonkleBody>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, bonkle_bodies, time): Self::SystemData) {
        for (transform, bonkle_body) in (&mut transforms, &bonkle_bodies).join() {
            transform.position.x += bonkle_body.velocity.x * time.delta_time;
            transform.position.y += bonkle_body.velocity.y * time.delta_time;
        }
    }
}

pub struct DragSystem;

impl<'s> System<'s> for DragSystem {
    type SystemData = (WriteStorage<'s, BonkleBody>, Read<'s, Time>);

    fn run(&mut self, (mut bonkle_bodies, time): Self::SystemData) {
        for body in (&mut bonkle_bodies).join() {
            body.velocity *= 1.0 - body.physics_material.drag * time.delta_time;
        }
    }
}
