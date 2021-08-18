use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::{
    components::{BonkleBody, BonkleBodyController},
    resources::Time,
};

pub struct BonkleBodyControlSystem;

impl<'s> System<'s> for BonkleBodyControlSystem {
    type SystemData = (
        ReadStorage<'s, BonkleBodyController>,
        WriteStorage<'s, BonkleBody>,
        Read<'s, Time>,
    );

    fn run(&mut self, (bonkle_body_controllers, mut bonkle_bodies, time): Self::SystemData) {
        for (controller, body) in (&bonkle_body_controllers, &mut bonkle_bodies).join() {
            body.velocity += (controller.target_velocity - body.velocity)
                * controller.acceleration
                * time.delta_time;
        }
    }
}
