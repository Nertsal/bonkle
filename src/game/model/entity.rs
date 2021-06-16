use super::*;

#[derive(Debug, Clone)]
pub struct Entity {
    pub destroy: bool,
    pub rigidbody: RigidBody,
    pub movement_speed: f32,
    pub health: Health,
    pub color: Color,
    pub entity_type: EntityType,
}

#[derive(Debug, Clone)]
pub enum EntityType {
    Player {
        head: RigidBody,
        target_body_velocity: Vec2,
        target_head_velocity: Vec2,
        chain_length: f32,
        head_target: Vec2,
    },
    Enemy {
        enemy_type: EnemyType,
    },
}

impl Entity {
    pub fn new(position: Vec2, entity_info: EntityInfo) -> Self {
        Self {
            destroy: false,
            rigidbody: RigidBody::new(
                position,
                entity_info.mass,
                Collider::new(entity_info.size),
                PhysicsMaterial::new(DRAG, BOUNCINESS),
            ),
            movement_speed: entity_info.movement_speed,
            health: entity_info.health,
            entity_type: entity_info.entity_type,
            color: entity_info.color,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health.is_alive()
    }

    pub fn attack(&mut self, commands: &mut Commands) {
        match &self.entity_type {
            EntityType::Player { .. } => (),
            EntityType::Enemy { enemy_type } => match enemy_type {
                EnemyType::Attacker { attack } => {
                    let (color_change, destroy) = attack.perform(&self, commands);
                    self.destroy = destroy;
                    if let Some(color_change) = color_change {
                        self.color = color_change;
                    }
                }
                _ => (),
            },
        }
    }

    pub fn reset_attacks(&mut self) {
        match &mut self.entity_type {
            EntityType::Player { .. } => {}
            EntityType::Enemy { enemy_type } => match enemy_type {
                EnemyType::Attacker { attack } if !attack.attack_time.is_alive() => {
                    attack.attack_time.hp = attack.attack_time.hp_max;
                }
                _ => (),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityInfo {
    pub health: Health,
    pub mass: f32,
    pub size: f32,
    pub movement_speed: f32,
    pub entity_type: EntityType,
    pub color: Color,
}

impl EntityInfo {
    pub fn new(
        health: Health,
        mass: f32,
        size: f32,
        movement_speed: f32,
        color: Color,
        entity_type: EntityType,
    ) -> Self {
        Self {
            health,
            mass,
            size,
            movement_speed,
            entity_type,
            color,
        }
    }
}
