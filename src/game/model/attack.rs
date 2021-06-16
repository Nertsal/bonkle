use super::*;

#[derive(Debug, Clone)]
pub struct Attack {
    pub attack_time: Health,
    pub attack_type: AttackType,
}

#[derive(Debug, Clone)]
pub enum AttackType {
    Shoot {
        projectile: Box<EnemyInfo>,
        target_pos: Vec2,
    },
    Bomb {
        projectile: Box<EnemyInfo>,
        projectile_count: usize,
    },
}

impl Attack {
    pub fn perform(&mut self, entity: &mut Entity, commands: &mut Commands) {
        match &self.attack_type {
            AttackType::Shoot {
                projectile,
                target_pos,
            } if !self.attack_time.is_alive() => {
                let mut projectile = Enemy::new(entity.rigidbody.position, (**projectile).clone());
                projectile.entity.rigidbody.velocity =
                    (*target_pos - projectile.entity.rigidbody.position).normalize()
                        * projectile.entity.movement_speed;
                commands.spawn_entity(projectile);
            }
            AttackType::Bomb {
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
                        let mut projectile =
                            Enemy::new(entity.rigidbody.position, (**projectile).clone());
                        let angle = (i as f32) * std::f32::consts::PI * 2.0
                            / (*projectile_count as f32)
                            + random_offset;
                        let (sin, cos) = angle.sin_cos();
                        projectile.entity.rigidbody.velocity =
                            vec2(cos, sin) * projectile.entity.movement_speed;
                        commands.spawn_entity(projectile);
                    }
                    entity.destroy = true;
                    commands.spawn_particles(entity.rigidbody.position, 500.0, BOMB_COLOR);
                    commands.event(Event::Sound {
                        sound: EventSound::Explosion,
                    });
                }
            }
            _ => (),
        }
        if !self.attack_time.is_alive() {
            self.attack_time.hp = self.attack_time.hp_max;
        }
    }
}
