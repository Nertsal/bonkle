use std::ops::{Deref, DerefMut};

use super::*;

#[derive(Debug, Clone)]
pub struct Entity {
    pub destroy: bool,
    pub rigidbody: RigidBody,
    pub movement_speed: f32,
    pub health: Health,
    pub color: Color,
}

impl Entity {
    pub fn new(position: Vec2, entity_info: EntityInfo) -> Self {
        Self {
            destroy: false,
            rigidbody: RigidBody::new(
                position,
                entity_info.mass,
                entity_info.is_kinematic,
                Collider::new(entity_info.size),
                entity_info.physics_material,
            ),
            movement_speed: entity_info.movement_speed,
            health: entity_info.health,
            color: entity_info.color,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health.is_alive()
    }

    pub fn entity_info(&self) -> EntityInfo {
        EntityInfo::new(
            self.health.clone(),
            self.rigidbody.mass,
            self.rigidbody.is_kinematic,
            self.rigidbody.collider.radius,
            self.movement_speed,
            self.color,
            self.rigidbody.physics_material.clone(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct EntityInfo {
    pub health: Health,
    pub mass: f32,
    pub is_kinematic: bool,
    pub size: f32,
    pub movement_speed: f32,
    pub color: Color,
    pub physics_material: PhysicsMaterial,
}

impl EntityInfo {
    pub fn new(
        health: Health,
        mass: f32,
        is_kinematic: bool,
        size: f32,
        movement_speed: f32,
        color: Color,
        physics_material: PhysicsMaterial,
    ) -> Self {
        Self {
            health,
            mass,
            is_kinematic,
            size,
            movement_speed,
            color,
            physics_material,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Player,
    Minion,
    Enemy,
}

pub trait EntityObject {
    fn entity_mut(&mut self) -> &mut Entity;

    fn entity(&self) -> &Entity;

    fn entity_type(&self) -> EntityType;

    fn attack_targets(&self) -> Vec<EntityType> {
        vec![]
    }

    fn attack(&mut self, _target_pos: Option<Vec2>, _delta_time: f32, _commands: &mut Commands) {}

    fn movement_targets(&self) -> Vec<EntityType> {
        vec![]
    }

    fn decide_movement(&mut self, _target_pos: Option<Vec2>, _delta_time: f32) {}

    fn movement(&mut self, delta_time: f32) {
        self.entity_mut().rigidbody.movement(delta_time);

        if self.entity().rigidbody.velocity.length() > self.entity().movement_speed {
            self.entity_mut().rigidbody.drag(delta_time);
        }
    }

    fn dead(&mut self, _delta_time: f32) -> DeadState {
        DeadState::Corpse
    }

    fn health_frac(&self) -> f32 {
        self.entity().health.hp_frac()
    }

    fn collide_bounds(&mut self, bounds: &Bounds) -> bool {
        self.entity_mut().rigidbody.bounce_bounds(bounds)
    }

    fn on_collide_bounds(&mut self, commands: &mut Commands) {
        commands.event(Event::Sound {
            sound: EventSound::Bounce,
        });
    }

    fn hit_strength(&self) -> Option<f32> {
        None
    }

    fn collide(&mut self, other: &mut Box<dyn EntityObject>) -> Option<HitInfo> {
        let hit_self = self.hit_strength();
        let hit_other = other.hit_strength();
        self.entity_mut()
            .rigidbody
            .collide(&mut other.rigidbody, hit_self, hit_other)
    }

    fn on_collide(&mut self, _commands: &mut Commands) {}
}

impl Deref for dyn EntityObject {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        self.entity()
    }
}

impl DerefMut for dyn EntityObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.entity_mut()
    }
}

pub trait EntityObjectInfo: EntityObjectInfoClone {
    fn into_entity_object(self: Box<Self>, position: Vec2) -> Box<dyn EntityObject>;
}

pub trait EntityObjectInfoClone {
    fn clone_box(&self) -> Box<dyn EntityObjectInfo>;
}

impl<T> EntityObjectInfoClone for T
where
    T: 'static + EntityObjectInfo + Clone,
{
    fn clone_box(&self) -> Box<dyn EntityObjectInfo> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn EntityObjectInfo> {
    fn clone(&self) -> Box<dyn EntityObjectInfo> {
        self.clone_box()
    }
}
