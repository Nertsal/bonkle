use super::*;

#[derive(Debug, Clone)]
pub struct Entity {
    pub destroy: bool,
    pub rigidbody: RigidBody,
    pub movement_speed: f32,
    pub health: Health,
    pub color: Color,
}

impl Entity {
    pub fn new(position: Vec2, entity_info: EntityInfo) -> Self {
        Self {
            destroy: false,
            rigidbody: RigidBody::new(
                position,
                entity_info.mass,
                Collider::new(entity_info.size),
                entity_info.physics_material,
            ),
            movement_speed: entity_info.movement_speed,
            health: entity_info.health,
            color: entity_info.color,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health.is_alive()
    }
}

#[derive(Debug, Clone)]
pub struct EntityInfo {
    pub health: Health,
    pub mass: f32,
    pub size: f32,
    pub movement_speed: f32,
    pub color: Color,
    pub physics_material: PhysicsMaterial,
}

impl EntityInfo {
    pub fn new(
        health: Health,
        mass: f32,
        size: f32,
        movement_speed: f32,
        color: Color,
        physics_material: PhysicsMaterial,
    ) -> Self {
        Self {
            health,
            mass,
            size,
            movement_speed,
            color,
            physics_material,
        }
    }
}

pub enum EntityObject {
    Player(Player),
    Minion(Minion),
    Enemy(Enemy),
}

impl EntityObject {
    pub fn entity_mut(&mut self) -> &mut Entity {
        match self {
            Self::Player(player) => &mut player.entity,
            Self::Minion(minion) => &mut minion.entity,
            Self::Enemy(enemy) => &mut enemy.entity,
        }
    }

    pub fn entity(&self) -> &Entity {
        match &self {
            Self::Player(player) => &player.entity,
            Self::Minion(minion) => &minion.entity,
            Self::Enemy(enemy) => &enemy.entity,
        }
    }
}

#[derive(Clone)]
pub enum EntityObjectInfo {
    Player(PlayerInfo),
    Minion(MinionInfo),
    Enemy(EnemyInfo),
}

impl EntityObjectInfo {
    pub fn into_entity_object(self, position: Vec2) -> EntityObject {
        match self {
            Self::Player(player) => player.into_entity_object(position),
            Self::Minion(minion) => minion.into_entity_object(position),
            Self::Enemy(enemy) => enemy.into_entity_object(position),
        }
    }
}
