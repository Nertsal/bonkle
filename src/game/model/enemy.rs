use super::*;

pub struct Enemy {
    pub entity: Entity,
    pub enemy_type: EnemyType,
}

#[derive(Clone)]
pub enum EnemyType {
    Corpse { lifetime: Health },
    Crawler,
    Attacker { attack: Attack },
    Projectile { lifetime: Health },
}

impl Enemy {
    pub fn new(position: Vec2, enemy_info: EnemyInfo) -> Self {
        Self {
            entity: Entity::new(position, enemy_info.entity_info),
            enemy_type: enemy_info.enemy_type,
        }
    }
}

#[derive(Clone)]
pub struct EnemyInfo {
    pub enemy_type: EnemyType,
    pub entity_info: EntityInfo,
}

impl EnemyInfo {
    pub fn new(enemy_type: EnemyType, entity_info: EntityInfo) -> Self {
        Self {
            enemy_type,
            entity_info,
        }
    }

    pub fn into_entity_object(self, position: Vec2) -> EntityObject {
        EntityObject::Enemy(Enemy::new(position, self))
    }
}
