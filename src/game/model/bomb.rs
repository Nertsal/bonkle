use super::*;

pub struct Bomb {
    entity: Entity,
    entity_type: EntityType,
    lifetime: Health,
    attack: bool,
    impact_attack: Attack,
}

impl Bomb {
    fn new(position: vec2<f32>, bomb_info: BombInfo) -> Self {
        Self {
            entity: Entity::new(position, bomb_info.entity_info),
            entity_type: bomb_info.entity_type,
            lifetime: bomb_info.lifetime,
            attack: false,
            impact_attack: Attack {
                attack_time: Health::new(0.0),
                attack_type: AttackType::Drop {
                    drop: bomb_info.drop,
                },
            },
        }
    }
}

impl EntityObject for Bomb {
    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    fn entity(&self) -> &Entity {
        &self.entity
    }

    fn entity_type(&self) -> EntityType {
        self.entity_type
    }

    fn attack(&mut self, _: Option<vec2<f32>>, delta_time: f32, commands: &mut Commands) {
        self.lifetime.change(-delta_time);
        if self.attack || !self.lifetime.is_alive() {
            self.entity.destroy = true;
            self.impact_attack.perform(&mut self.entity, commands);
        }
    }

    fn on_collide(&mut self, _: &mut Commands) {
        self.attack = true;
    }
}

#[derive(Clone)]
pub struct BombInfo {
    entity_info: EntityInfo,
    entity_type: EntityType,
    lifetime: Health,
    drop: Box<dyn EntityObjectInfo>,
}

impl BombInfo {
    pub fn new(
        entity_info: EntityInfo,
        entity_type: EntityType,
        lifetime: Health,
        drop: Box<dyn EntityObjectInfo>,
    ) -> Self {
        Self {
            entity_info,
            entity_type,
            lifetime,
            drop,
        }
    }
}

impl EntityObjectInfo for BombInfo {
    fn into_entity_object(self: Box<Self>, position: vec2<f32>) -> Box<dyn EntityObject> {
        Box::new(Bomb::new(position, *self))
    }
}
