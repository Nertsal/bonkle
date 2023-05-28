use super::*;

pub struct Missile {
    entity: Entity,
    entity_type: EntityType,
    target_pos: Option<vec2<f32>>,
    target_precision: f32,
    attack: bool,
    impact_attack: Attack,
}

impl Missile {
    fn new(position: vec2<f32>, missile_info: MissileInfo) -> Self {
        Self {
            entity: Entity::new(position, missile_info.entity_info),
            entity_type: missile_info.entity_type,
            target_pos: missile_info.target_pos,
            target_precision: 0.5,
            attack: false,
            impact_attack: Attack {
                attack_time: Health::new(0.0),
                attack_type: AttackType::Explode {
                    projectile_count: missile_info.bombs_count,
                    projectile: missile_info.bomb,
                },
            },
        }
    }
}

impl EntityObject for Missile {
    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    fn entity(&self) -> &Entity {
        &self.entity
    }

    fn entity_type(&self) -> EntityType {
        self.entity_type
    }

    fn attack(&mut self, _: Option<vec2<f32>>, _: f32, commands: &mut Commands) {
        if self.attack
            || self
                .target_pos
                .map(|target_pos| {
                    (self.entity.rigidbody.position - target_pos).len() <= self.target_precision
                })
                .unwrap_or(false)
        {
            self.entity.health.kill();
            self.impact_attack.perform(&mut self.entity, commands);
        }
    }

    fn on_collide_bounds(&mut self, commands: &mut Commands) {
        commands.event(Event::Sound {
            sound: EventSound::Bounce,
        });
        self.attack = true;
    }

    fn on_collide(&mut self, _: &mut Commands) {
        self.attack = true;
    }
}

#[derive(Clone)]
pub struct MissileInfo {
    entity_info: EntityInfo,
    entity_type: EntityType,
    target_pos: Option<vec2<f32>>,
    bombs_count: usize,
    bomb: Box<dyn EntityObjectInfo>,
}

impl MissileInfo {
    pub fn new(
        entity_info: EntityInfo,
        entity_type: EntityType,
        target_pos: Option<vec2<f32>>,
        bombs_count: usize,
        bomb: Box<dyn EntityObjectInfo>,
    ) -> Self {
        Self {
            entity_info,
            entity_type,
            target_pos,
            bombs_count,
            bomb,
        }
    }
}

impl EntityObjectInfo for MissileInfo {
    fn into_entity_object(self: Box<Self>, position: vec2<f32>) -> Box<dyn EntityObject> {
        Box::new(Missile::new(position, *self))
    }
}
