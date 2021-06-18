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

impl EntityObject for Player {
    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    fn entity(&self) -> &Entity {
        &self.entity
    }

    fn entity_type(&self) -> EntityType {
        EntityType::Player
    }

    fn decide_movement(&mut self, _: Option<Vec2>, delta_time: f32) {
        if self.entity.is_alive() {
            // Calculate head target velocity
            let direction = self.head.position - self.entity.rigidbody.position;
            let target = self.head_target - self.entity.rigidbody.position;
            let angle = direction.angle_between(target).abs();
            let speed = angle.min(0.2) / 0.2;
            let direction = vec2(direction.y, -direction.x).normalize();
            let signum = direction.dot(target).signum();
            let direction = direction * signum * speed;
            self.target_head_velocity = direction * HEAD_SPEED + self.entity.rigidbody.velocity;

            // Accelerate towards target velocity
            let target_change = self.target_body_velocity - self.entity.rigidbody.velocity;
            self.entity.rigidbody.velocity += target_change * BODY_ACCELERATION * delta_time;

            let target_change = self.target_head_velocity - self.head.velocity;
            self.head.velocity += target_change * HEAD_ACCELERATION * delta_time;
        }
    }

    fn movement(&mut self, delta_time: f32) {
        self.entity.rigidbody.movement(delta_time);
        self.head.movement(delta_time);

        if self.entity.rigidbody.velocity.length() > self.entity.movement_speed {
            self.entity.rigidbody.drag(delta_time);
        }

        // Clamp distance between body and head
        let offset = self.head.position - self.entity.rigidbody.position;
        let distance = offset.length() - self.chain_length;
        self.head.position -= offset.normalize_or_zero() * distance;
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
}

impl EntityObjectInfo for PlayerInfo {
    fn into_entity_object(self: Box<Self>, position: Vec2) -> Box<dyn EntityObject> {
        Box::new(Player::new(position, *self))
    }
}
