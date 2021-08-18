use specs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::components::{PlayerHead, Transform};

pub struct PlayerHeadSystem;

impl<'s> System<'s> for PlayerHeadSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, PlayerHead>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, player_heads, mut transforms): Self::SystemData) {
        for (head_entity, head) in (&entities, &player_heads).join() {
            let head_transform = transforms.get(head_entity).unwrap();
            match transforms.get(head.connected_to) {
                Some(body_transform) => {
                    let offset = head_transform.position - body_transform.position;
                    let distance = offset.length() - head.connection_length;
                    let head_transform = transforms.get_mut(head_entity).unwrap();
                    head_transform.position -= offset.normalize_or_zero() * distance;
                }
                None => (),
            }
        }
    }
}
