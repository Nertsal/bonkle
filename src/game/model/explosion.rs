use super::*;

pub struct Explosion {
    entity: Entity,
    entity_type: EntityType,
    radius: f32,
    speed: f32,
    hit_strength: f32,
}

impl Explosion {
    pub fn new(position: vec2<f32>, explosion_info: ExplosionInfo) -> Self {
        Self {
            entity: Entity::new(position, explosion_info.entity_info),
            entity_type: explosion_info.entity_type,
            radius: explosion_info.radius,
            speed: explosion_info.speed,
            hit_strength: explosion_info.hit_strength,
        }
    }
}

impl EntityObject for Explosion {
    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    fn entity(&self) -> &Entity {
        &self.entity
    }

    fn entity_type(&self) -> EntityType {
        self.entity_type
    }

    fn attack(&mut self, _: Option<vec2<f32>>, delta_time: f32, _: &mut Commands) {
        self.entity.rigidbody.collider.radius += delta_time * self.speed;
        if self.entity.rigidbody.collider.radius > self.radius {
            self.entity.destroy = true;
        }
    }

    fn health_frac(&self) -> f32 {
        0.0
    }

    fn hit_strength(&self) -> Option<f32> {
        Some(self.hit_strength)
    }
}

#[derive(Clone)]
pub struct ExplosionInfo {
    entity_info: EntityInfo,
    entity_type: EntityType,
    radius: f32,
    speed: f32,
    hit_strength: f32,
}

impl ExplosionInfo {
    pub fn new(entity_type: EntityType, radius: f32, speed: f32, hit_strength: f32) -> Self {
        Self {
            entity_info: EntityInfo::new(
                Health::new(10000.0),
                1.0,
                true,
                0.0,
                0.0,
                Color::RED,
                PhysicsMaterial::new(0.0, 0.0),
            ),
            entity_type,
            radius,
            speed,
            hit_strength,
        }
    }
}

impl EntityObjectInfo for ExplosionInfo {
    fn into_entity_object(self: Box<Self>, position: vec2<f32>) -> Box<dyn EntityObject> {
        Box::new(Explosion::new(position, *self))
    }
}
