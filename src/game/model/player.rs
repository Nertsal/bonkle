use super::*;

pub struct Player {
    pub entity: Entity,
    pub head: RigidBody,
    pub chain_length: f32,
    pub head_target: Vec2,
    pub target_body_velocity: Vec2,
    pub target_head_velocity: Vec2,
    pub perform_attacks: HashSet<usize>,
    pub attacks: Vec<Attack>,
}

impl Player {
    pub fn new(position: Vec2, player_info: PlayerInfo) -> Self {
        Self {
            head: RigidBody::new(
                position + vec2(player_info.chain_length, 0.0),
                player_info.entity_info.mass,
                Collider::new(player_info.head_size),
                PhysicsMaterial::new(0.0, 0.0),
            ),
            entity: Entity::new(position, player_info.entity_info),
            chain_length: player_info.chain_length,
            head_target: vec2(1.0, 0.0),
            target_body_velocity: vec2(0.0, 0.0),
            target_head_velocity: vec2(0.0, 0.0),
            perform_attacks: HashSet::new(),
            attacks: vec![],
        }
    }
}

#[derive(Clone)]
pub struct PlayerInfo {
    pub entity_info: EntityInfo,
    pub head_size: f32,
    pub chain_length: f32,
}

impl PlayerInfo {
    pub fn new(head_size: f32, chain_length: f32, entity_info: EntityInfo) -> Self {
        Self {
            entity_info,
            head_size,
            chain_length,
        }
    }

    pub fn into_entity_object(self, position: Vec2) -> EntityObject {
        EntityObject::Player(Player::new(position, self))
    }
}
