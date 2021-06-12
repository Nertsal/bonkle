use super::*;

pub struct Enemy {
    pub rigidbody: RigidBody,
    pub movement_speed: f32,
    pub health: f32,
}

impl Enemy {
    pub fn new(position: Vec2, enemy_info: EnemyInfo) -> Self {
        Self {
            rigidbody: RigidBody::new(position, enemy_info.mass, Collider::new(enemy_info.size)),
            movement_speed: enemy_info.movement_speed,
            health: enemy_info.health,
        }
    }
}

pub struct EnemyInfo {
    pub health: f32,
    pub mass: f32,
    pub size: f32,
    pub movement_speed: f32,
}

impl EnemyInfo {
    pub fn new(health: f32, mass: f32, size: f32, movement_speed: f32) -> Self {
        Self {
            health,
            mass,
            size,
            movement_speed,
        }
    }
}
