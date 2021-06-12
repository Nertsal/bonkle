use super::*;

pub struct Enemy {
    pub rigidbody: RigidBody,
    pub movement_speed: f32,
    pub health: f32,
    pub enemy_type: EnemyType,
}

impl Enemy {
    pub fn new(position: Vec2, enemy_info: EnemyInfo) -> Self {
        Self {
            rigidbody: RigidBody::new(position, enemy_info.mass, Collider::new(enemy_info.size)),
            movement_speed: enemy_info.movement_speed,
            health: enemy_info.health,
            enemy_type: enemy_info.enemy_type,
        }
    }
}

#[derive(Clone)]
pub struct EnemyInfo {
    pub health: f32,
    pub mass: f32,
    pub size: f32,
    pub movement_speed: f32,
    pub enemy_type: EnemyType,
}

impl EnemyInfo {
    pub fn new(
        health: f32,
        mass: f32,
        size: f32,
        movement_speed: f32,
        attack_type: EnemyType,
    ) -> Self {
        Self {
            health,
            mass,
            size,
            movement_speed,
            enemy_type: attack_type,
        }
    }
}

#[derive(Clone)]
pub enum EnemyType {
    Melee,
    Ranged {
        projectile: Box<EnemyInfo>,
        attack_time: f32,
        attack_cooldown: f32,
    },
    Projectile,
}
