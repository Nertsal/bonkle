use super::*;

pub struct Minion {
    pub entity: Entity,
}

impl Minion {
    pub fn new(position: vec2<f32>, minion_info: MinionInfo) -> Self {
        Self {
            entity: Entity::new(position, minion_info.entity_info),
        }
    }
}

impl EntityObject for Minion {
    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    fn entity(&self) -> &Entity {
        &self.entity
    }

    fn entity_type(&self) -> EntityType {
        EntityType::Minion
    }

    fn attack_targets(&self) -> Vec<EntityType> {
        vec![EntityType::Enemy]
    }

    fn movement_targets(&self) -> Vec<EntityType> {
        vec![EntityType::Enemy]
    }
}

#[derive(Clone)]
pub struct MinionInfo {
    pub entity_info: EntityInfo,
}

impl MinionInfo {
    pub fn new(entity_info: EntityInfo) -> Self {
        Self { entity_info }
    }
}

impl EntityObjectInfo for MinionInfo {
    fn into_entity_object(self: Box<Self>, position: vec2<f32>) -> Box<dyn EntityObject> {
        Box::new(Minion::new(position, *self))
    }
}
