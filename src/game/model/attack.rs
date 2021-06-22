use super::*;

#[derive(Clone)]
pub struct Attack {
    pub attack_time: Health,
    pub attack_type: AttackType,
}

#[derive(Clone)]
pub enum AttackType {
    Shoot {
        projectile: Box<dyn EntityObjectInfo>,
        target_pos: Vec2,
    },
    Explode {
        projectile: Box<dyn EntityObjectInfo>,
        projectile_count: usize,
    },
    Drop {
        drop: Box<dyn EntityObjectInfo>,
    },
}

impl Attack {
    pub fn perform(&mut self, entity: &mut Entity, commands: &mut Commands) {
        match &self.attack_type {
            AttackType::Shoot {
                projectile,
                target_pos,
            } => {
                if !self.attack_time.is_alive() {
                    let mut projectile = projectile
                        .clone()
                        .into_entity_object(entity.rigidbody.position);
                    projectile.entity_mut().rigidbody.velocity =
                        (*target_pos - projectile.entity().rigidbody.position).normalize()
                            * projectile.entity().movement_speed;
                    commands.spawn_entity(projectile);
                }
            }
            AttackType::Explode {
                projectile,
                projectile_count,
            } => {
                if self.attack_time.is_alive() {
                    let time_frac = self.attack_time.hp_frac();
                    entity.color = Color::new(
                        (BOMB_COLOR.r - BOMBER_COLOR.r) * (1.0 - time_frac) + BOMBER_COLOR.r,
                        (BOMB_COLOR.g - BOMBER_COLOR.g) * (1.0 - time_frac) + BOMBER_COLOR.g,
                        (BOMB_COLOR.b - BOMBER_COLOR.b) * (1.0 - time_frac) + BOMBER_COLOR.b,
                        1.0,
                    );
                } else {
                    let random_offset = macroquad::rand::gen_range(0.0, std::f32::consts::PI);
                    for i in 0..*projectile_count {
                        let mut projectile = projectile
                            .clone()
                            .into_entity_object(entity.rigidbody.position);
                        let angle = (i as f32) * std::f32::consts::PI * 2.0
                            / (*projectile_count as f32)
                            + random_offset;
                        let (sin, cos) = angle.sin_cos();
                        projectile.entity_mut().rigidbody.velocity =
                            vec2(cos, sin) * projectile.entity().movement_speed;
                        commands.spawn_entity(projectile);
                    }
                    entity.destroy = true;
                    commands.spawn_particles(entity.rigidbody.position, 500.0, BOMB_COLOR);
                    commands.event(Event::Sound {
                        sound: EventSound::Explosion,
                    });
                }
            }

            AttackType::Drop { drop } => {
                if !self.attack_time.is_alive() {
                    let drop = drop.clone().into_entity_object(entity.rigidbody.position);
                    commands.spawn_entity(drop);
                    commands.event(Event::Sound {
                        sound: EventSound::Explosion,
                    });
                }
            }
        }
        if !self.attack_time.is_alive() {
            self.attack_time.hp = self.attack_time.hp_max;
        }
    }
}
