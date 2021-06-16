use super::*;

pub struct Enemy {
    pub destroy: bool,
    pub rigidbody: RigidBody,
    pub movement_speed: f32,
    pub health: Health,
    pub enemy_type: EnemyType,
    pub color: Color,
}

impl Enemy {
    pub fn new(position: Vec2, enemy_info: EnemyInfo) -> Self {
        Self {
            destroy: false,
            rigidbody: RigidBody::new(
                position,
                enemy_info.mass,
                Collider::new(enemy_info.size),
                PhysicsMaterial::new(DRAG, BOUNCINESS),
            ),
            movement_speed: enemy_info.movement_speed,
            health: enemy_info.health,
            enemy_type: enemy_info.enemy_type,
            color: enemy_info.color,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health.is_alive()
    }
}

#[derive(Debug, Clone)]
pub struct EnemyInfo {
    pub health: Health,
    pub mass: f32,
    pub size: f32,
    pub movement_speed: f32,
    pub enemy_type: EnemyType,
    pub color: Color,
}

impl EnemyInfo {
    pub fn new(
        health: Health,
        mass: f32,
        size: f32,
        movement_speed: f32,
        color: Color,
        enemy_type: EnemyType,
    ) -> Self {
        Self {
            health,
            mass,
            size,
            movement_speed,
            enemy_type,
            color,
        }
    }
}

#[derive(Debug, Clone)]
pub enum EnemyType {
    Corpse { lifetime: Health },
    Crawler,
    Attacker { attack: Attack },
    Projectile { lifetime: Health },
}
