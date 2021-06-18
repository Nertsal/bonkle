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

impl EntityObject for Enemy {
    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    fn entity(&self) -> &Entity {
        &self.entity
    }

    fn entity_type(&self) -> EntityType {
        EntityType::Enemy
    }

    fn attack_targets(&self) -> Vec<EntityType> {
        vec![EntityType::Player, EntityType::Minion]
    }

    fn attack(&mut self, target_pos: Option<Vec2>, delta_time: f32, commands: &mut Commands) {
        match &mut self.enemy_type {
            EnemyType::Attacker { attack } => {
                attack.attack_time.change(-delta_time);
                if let Some(target) = target_pos {
                    match &mut attack.attack_type {
                        AttackType::Shoot { target_pos, .. } => {
                            *target_pos = target;
                        }
                        _ => (),
                    }

                    attack.perform(&mut self.entity, commands);
                }
            }
            EnemyType::Projectile { lifetime, .. } => {
                lifetime.change(-delta_time);
                if !lifetime.is_alive() {
                    self.entity.health.kill();
                }
            }
            _ => (),
        }
    }

    fn movement_targets(&self) -> Vec<EntityType> {
        vec![EntityType::Player, EntityType::Minion]
    }

    fn decide_movement(&mut self, target_pos: Option<Vec2>, delta_time: f32) {
        if let Some(target_pos) = target_pos {
            match &self.enemy_type {
                EnemyType::Crawler | EnemyType::Attacker { .. } => {
                    let target_direction = target_pos - self.entity.rigidbody.position;
                    let target_velocity = target_direction.normalize() * self.entity.movement_speed;
                    self.entity.rigidbody.velocity +=
                        (target_velocity - self.entity.rigidbody.velocity) * delta_time;
                }
                _ => (),
            }
        }
    }

    fn dead(&mut self, delta_time: f32) -> bool {
        let mut destroy = false;
        match &mut self.enemy_type {
            EnemyType::Corpse { lifetime } => {
                lifetime.change(-delta_time);
                if !lifetime.is_alive() {
                    destroy = true;
                }
                self.entity.color.a = lifetime.hp_frac() * 0.5;
            }
            EnemyType::Attacker { attack } if !attack.attack_time.is_alive() => {
                match attack.attack_type {
                    AttackType::Bomb { .. } => {
                        destroy = true;
                    }
                    _ => (),
                }
            }
            _ => {
                self.enemy_type = EnemyType::Corpse {
                    lifetime: Health::new(CORPSE_LIFETIME),
                }
            }
        }
        destroy
    }

    fn health_frac(&self) -> f32 {
        match &self.enemy_type {
            EnemyType::Projectile { lifetime } => lifetime.hp_frac(),
            _ => self.entity.health.hp_frac(),
        }
    }

    fn collide_bounds(&mut self, bounds: &Bounds, commands: &mut Commands) {
        if self.entity_mut().rigidbody.bounce_bounds(bounds) {
            if let EnemyType::Projectile { lifetime } = &mut self.enemy_type {
                lifetime.kill();
            }

            commands.event(Event::Sound {
                sound: EventSound::Bounce,
            });
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
}

impl EntityObjectInfo for EnemyInfo {
    fn into_entity_object(self: Box<Self>, position: Vec2) -> Box<dyn EntityObject> {
        Box::new(Enemy::new(position, *self))
    }
}
