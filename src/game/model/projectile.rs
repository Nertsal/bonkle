use super::*;

pub struct Projectile {
    entity: Entity,
    entity_type: EntityType,
    lifetime: Health,
}

impl Projectile {
    pub fn new(position: Vec2, projectile_info: ProjectileInfo) -> Self {
        Self {
            entity: Entity::new(position, projectile_info.entity_info),
            entity_type: projectile_info.entity_type,
            lifetime: projectile_info.lifetime,
        }
    }
}

impl EntityObject for Projectile {
    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    fn entity(&self) -> &Entity {
        &self.entity
    }

    fn entity_type(&self) -> EntityType {
        self.entity_type
    }

    fn attack(&mut self, _: Option<Vec2>, delta_time: f32, _: &mut Commands) {
        self.lifetime.change(-delta_time);
        if !self.lifetime.is_alive() {
            self.entity.health.kill();
        }
    }

    fn health_frac(&self) -> f32 {
        self.lifetime.hp_frac()
    }

    fn collide_bounds(&mut self, bounds: &Bounds, commands: &mut Commands) {
        if self.entity_mut().rigidbody.bounce_bounds(bounds) {
            self.lifetime.kill();
            commands.event(Event::Sound {
                sound: EventSound::Bounce,
            });
        }
    }
}

#[derive(Clone)]
pub struct ProjectileInfo {
    entity_info: EntityInfo,
    entity_type: EntityType,
    lifetime: Health,
}

impl ProjectileInfo {
    pub fn new(lifetime: Health, entity_type: EntityType, entity_info: EntityInfo) -> Self {
        Self {
            entity_info,
            entity_type,
            lifetime,
        }
    }
}

impl EntityObjectInfo for ProjectileInfo {
    fn into_entity_object(self: Box<Self>, position: Vec2) -> Box<dyn EntityObject> {
        Box::new(Projectile::new(position, *self))
    }
}
