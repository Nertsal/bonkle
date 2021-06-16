use super::*;

pub struct Enemy {
    pub entity: Entity,
    pub enemy_type: EnemyType,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct EnemyInfo {
    pub entity_info: EntityInfo,
    pub enemy_type: EnemyType,
}
