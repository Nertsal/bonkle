use super::*;

pub struct Enemy {
    pub rigidbody: RigidBody,
    movement_speed: f32,
}

impl Enemy {
    pub fn new(position: Vec2, enemy_info: EnemyInfo) -> Self {
        Self {
            rigidbody: RigidBody::new(position, enemy_info.mass, Collider::new(enemy_info.size)),
            movement_speed: enemy_info.movement_speed,
        }
    }
}

pub struct EnemyInfo {
    pub mass: f32,
    pub size: f32,
    pub movement_speed: f32,
}
