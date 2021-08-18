use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

use crate::{
    components::{BonkleBody, ColorComponent, Corpse, EnemyCollider, FriendCollider, Health, Transform},
    constants::{CORPSE_ALPHA, CORPSE_LIFETIME},
    resources::Time,
};

pub struct CheckDeadSystem;

impl<'s> System<'s> for CheckDeadSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Health>,
        WriteStorage<'s, Corpse>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, BonkleBody>,
        WriteStorage<'s, ColorComponent>,
        WriteStorage<'s, EnemyCollider>,
        WriteStorage<'s, FriendCollider>,
    );

    fn run(
        &mut self,
        (
            entities,
            healths,
            mut corpses,
            mut transforms,
            mut bonkle_bodies,
            mut colors,
            mut enemies,
            mut friends,
        ): Self::SystemData,
    ) {
        let mut dead_entities = Vec::new();
        for (entity, health) in (&entities, &healths).join() {
            if !health.is_alive() {
                dead_entities.push((
                    transforms.remove(entity).unwrap(),
                    bonkle_bodies.remove(entity).unwrap(),
                    colors.remove(entity).unwrap(),
                    enemies.remove(entity),
                    friends.remove(entity),
                ));
                entities.delete(entity).unwrap();
            }
        }

        for (transform, body, color, enemy, friend) in dead_entities {
            let mut builder = entities
                .build_entity()
                .with(Corpse::new(CORPSE_LIFETIME), &mut corpses)
                .with(transform, &mut transforms)
                .with(body, &mut bonkle_bodies)
                .with(color, &mut colors);
            if let Some(enemy) = enemy {
                builder = builder.with(enemy, &mut enemies);
            }
            if let Some(friend) = friend {
                builder = builder.with(friend, &mut friends);
            }
            builder.build();
        }
    }
}

pub struct UpdateCorpsesSystem;

impl<'s> System<'s> for UpdateCorpsesSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Corpse>,
        WriteStorage<'s, ColorComponent>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut corpses, mut colors, time): Self::SystemData) {
        for (entity, corpse, color) in (&entities, &mut corpses, &mut colors).join() {
            corpse.lifetime.change(-time.delta_time);
            color.0.a = corpse.lifetime.hp_frac() * CORPSE_ALPHA;
            if !corpse.lifetime.is_alive() {
                entities.delete(entity).unwrap();
            }
        }
    }
}
