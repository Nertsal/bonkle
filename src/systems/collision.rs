use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

use crate::{
    components::{BonkleBody, Collidable, EnemyCollider, FriendCollider, Health, Projectile, Transform},
    physics,
    resources::Bounds,
};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, FriendCollider>,
        ReadStorage<'s, EnemyCollider>,
        ReadStorage<'s, Collidable>,
        ReadStorage<'s, Projectile>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, BonkleBody>,
        WriteStorage<'s, Health>,
        Read<'s, Bounds>,
    );

    fn run(
        &mut self,
        (
            entities,
            friends,
            enemies,
            collidables,
            projectiles,
            mut transforms,
            mut bonkle_bodies,
            mut healths,
            bounds,
        ): Self::SystemData,
    ) {
        for (entity, transform, body) in (&entities, &mut transforms, &mut bonkle_bodies).join() {
            if physics::bounce_bounds(body, transform, &bounds) && projectiles.contains(entity) {
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

        let mut collisions = Vec::new();
        for (friend, _, _, friend_transform, friend_body) in (
            &entities,
            &friends,
            &collidables,
            &transforms,
            &bonkle_bodies,
        )
            .join()
        {
            for (enemy, _, _, enemy_transform, enemy_body) in (
                &entities,
                &enemies,
                &collidables,
                &transforms,
                &bonkle_bodies,
            )
                .join()
            {
                if let Some(collision) = physics::collision_info(
                    friend_body,
                    friend_transform,
                    enemy_body,
                    enemy_transform,
                ) {
                    collisions.push((friend, enemy, collision));
                }
            }
        }

        for (entity, other, (collision, hit_info)) in collisions {
            let body = bonkle_bodies.get_mut(entity).unwrap();
            body.velocity += hit_info.hit_self * collision.normal;
            if let Some(health) = healths.get_mut(entity) {
                health.change(-hit_info.hit_self);
            }
            let other_body = bonkle_bodies.get_mut(other).unwrap();
            other_body.velocity -= hit_info.hit_other * collision.normal;
            if let Some(health) = healths.get_mut(other) {
                health.change(-hit_info.hit_other);
            }
        }
    }
}
