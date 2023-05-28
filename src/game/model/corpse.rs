use super::*;

#[derive(Debug, Clone, Copy)]
pub enum DeadState {
    Destroy,
    Corpse,
    Idle,
}

pub struct Corpse {
    entity: Entity,
    entity_type: EntityType,
    lifetime: Health,
}

impl Corpse {
    pub fn new(position: vec2<f32>, corpse_info: CorpseInfo) -> Self {
        Self {
            entity: {
                let mut entity = Entity::new(position, corpse_info.entity_info);
                entity.rigidbody.velocity = corpse_info.velocity;
                entity
            },
            entity_type: corpse_info.entity_type,
            lifetime: corpse_info.lifetime,
        }
    }
}

impl EntityObject for Corpse {
    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    fn entity(&self) -> &Entity {
        &self.entity
    }

    fn entity_type(&self) -> EntityType {
        self.entity_type
    }

    fn dead(&mut self, delta_time: f32) -> DeadState {
        let mut destroy = DeadState::Idle;
        self.lifetime.change(-delta_time);
        if !self.lifetime.is_alive() {
            destroy = DeadState::Destroy;
        }
        self.entity.color.a = self.lifetime.hp_frac() * 0.5;
        destroy
    }
}

#[derive(Clone)]
pub struct CorpseInfo {
    entity_info: EntityInfo,
    entity_type: EntityType,
    lifetime: Health,
    velocity: vec2<f32>,
}

impl CorpseInfo {
    pub fn new(
        entity_type: EntityType,
        lifetime: Health,
        velocity: vec2<f32>,
        entity_info: EntityInfo,
    ) -> Self {
        Self {
            entity_info,
            entity_type,
            lifetime,
            velocity,
        }
    }
}

impl EntityObjectInfo for CorpseInfo {
    fn into_entity_object(self: Box<Self>, position: vec2<f32>) -> Box<dyn EntityObject> {
        Box::new(Corpse::new(position, *self))
    }
}
