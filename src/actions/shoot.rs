use macroquad::prelude::{vec2, Vec2};
use specs::{Builder, Entity, WorldExt};

use crate::{
    components::{
        BonkleBody, Collidable, ColorComponent, EnemyCollider, Health, Lifetime, Projectile,
        Transform,
    },
    physics::PhysicsMaterial,
};

use super::Action;

pub struct ShootAction {
    pub lifetime: Lifetime,
    pub color: ColorComponent,
    pub mass: f32,
    pub radius: f32,
    pub speed: f32,
    pub position: Vec2,
    pub target: Vec2,
    pub bullets: usize,
    pub spread_radians: f32,
    pub physics_material: PhysicsMaterial,
}

impl Action for ShootAction {
    fn perform(self: Box<Self>, world: &mut specs::World, _actor: Entity) {
        let angle_offset = vec2(1.0, 0.0).angle_between(self.target - self.position);
        for i in 0..self.bullets {
            let angle = (i as f32) * self.spread_radians / (self.bullets as f32) + angle_offset;
            let (sin, cos) = angle.sin_cos();
            let velocity = vec2(cos, sin) * self.speed;
            world
                .create_entity()
                .with(EnemyCollider)
                .with(Collidable)
                .with(Projectile)
                .with(Health::new(1.0))
                .with(self.lifetime.clone())
                .with(self.color.clone())
                .with(Transform {
                    position: self.position,
                })
                .with(BonkleBody {
                    mass: self.mass,
                    radius: self.radius,
                    velocity,
                    physics_material: self.physics_material,
                })
                .build();
        }
    }
}
