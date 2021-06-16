use super::*;

pub struct Minion {
    pub entity: Entity,
}

impl Minion {
    pub fn new(position: Vec2, minion_info: MinionInfo) -> Self {
        Self {
            entity: Entity::new(position, minion_info.entity_info),
        }
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

    pub fn into_entity_object(self, position: Vec2) -> EntityObject {
        EntityObject::Minion(Minion::new(position, self))
    }
}
