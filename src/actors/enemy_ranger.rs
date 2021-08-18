use specs::{Entity, World, WorldExt};

use crate::{
    actions::{Action, FollowAction, ShootAction},
    components::{ColorComponent, FriendTarget, Health, Lifetime, Transform},
    constants::{BOUNCINESS, PROJECTILE_COLOR},
    physics::PhysicsMaterial,
};

use super::Actor;

pub struct EnemyRanger {
    pub movement_speed: f32,
    pub attack_cooldown: Health,
}

impl Actor for EnemyRanger {
    fn update(&mut self, actor: Entity, world: &World, delta_time: f32) -> Vec<Box<dyn Action>> {
        let mut actions: Vec<Box<dyn Action>> = vec![Box::new(FollowAction::<FriendTarget>::new(
            self.movement_speed,
        ))];
        self.attack_cooldown.change(-delta_time);
        if !self.attack_cooldown.is_alive() {
            let transforms = &world.read_component::<Transform>();
            let position = transforms.get(actor).unwrap().position;
            if let Some(target) = crate::find_closest_entity_world::<FriendTarget>(position, world)
                .map(|target| transforms.get(target))
                .flatten()
                .map(|transform| transform.position)
            {
                actions.push(Box::new(ShootAction {
                    lifetime: Lifetime(Health::new(5.0)),
                    color: ColorComponent(PROJECTILE_COLOR),
                    mass: 5.0,
                    radius: 1.5,
                    speed: 30.0,
                    position,
                    target,
                    bullets: 1,
                    spread_radians: 0.0,
                    physics_material: PhysicsMaterial::new(0.0, BOUNCINESS),
                }));
            }
            self.attack_cooldown.reset();
        }
        actions
    }
}
