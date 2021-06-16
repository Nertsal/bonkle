use super::*;

pub struct Enemy {
    pub destroy: bool,
    pub rigidbody: RigidBody,
    pub movement_speed: f32,
    pub health: Health,
    pub enemy_type: EnemyType,
    pub color: Color,
}

impl Enemy {
    pub fn new(position: Vec2, enemy_info: EnemyInfo) -> Self {
        Self {
            destroy: false,
            rigidbody: RigidBody::new(
                position,
                enemy_info.mass,
                Collider::new(enemy_info.size),
                PhysicsMaterial::new(DRAG, BOUNCINESS),
            ),
            movement_speed: enemy_info.movement_speed,
            health: enemy_info.health,
            enemy_type: enemy_info.enemy_type,
            color: enemy_info.color,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health.is_alive()
    }

    pub fn attack(&mut self, commands: &mut Commands) {
        match &mut self.enemy_type {
            EnemyType::Attacker { attack } => {
                match &attack.attack_type {
                    AttackType::Shoot {
                        projectile,
                        target_pos,
                    } if !attack.attack_time.is_alive() => {
                        let mut projectile =
                            Enemy::new(self.rigidbody.position, (**projectile).clone());
                        projectile.rigidbody.velocity =
                            (*target_pos - projectile.rigidbody.position).normalize()
                                * projectile.movement_speed;
                        commands.spawn_enemy(projectile);
                    }
                    AttackType::Bomb {
                        projectile,
                        projectile_count,
                    } => {
                        if attack.attack_time.is_alive() {
                            let time_frac = attack.attack_time.hp_frac();
                            self.color = Color::new(
                                (BOMB_COLOR.r - BOMBER_COLOR.r) * (1.0 - time_frac)
                                    + BOMBER_COLOR.r,
                                (BOMB_COLOR.g - BOMBER_COLOR.g) * (1.0 - time_frac)
                                    + BOMBER_COLOR.g,
                                (BOMB_COLOR.b - BOMBER_COLOR.b) * (1.0 - time_frac)
                                    + BOMBER_COLOR.b,
                                1.0,
                            );
                        } else {
                            let random_offset =
                                macroquad::rand::gen_range(0.0, std::f32::consts::PI);
                            for i in 0..*projectile_count {
                                let mut projectile =
                                    Enemy::new(self.rigidbody.position, (**projectile).clone());
                                let angle = (i as f32) * std::f32::consts::PI * 2.0
                                    / (*projectile_count as f32)
                                    + random_offset;
                                let (sin, cos) = angle.sin_cos();
                                projectile.rigidbody.velocity =
                                    vec2(cos, sin) * projectile.movement_speed;
                                commands.spawn_enemy(projectile);
                            }
                            self.destroy = true;
                            commands.spawn_particles(self.rigidbody.position, 500.0, BOMB_COLOR);
                            commands.event(Event::Sound {
                                sound: EventSound::Explosion,
                            });
                        }
                    }
                    _ => (),
                }
                if !attack.attack_time.is_alive() {
                    attack.attack_time.hp = attack.attack_time.hp_max;
                }
            }
            _ => (),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnemyInfo {
    pub health: Health,
    pub mass: f32,
    pub size: f32,
    pub movement_speed: f32,
    pub enemy_type: EnemyType,
    pub color: Color,
}

impl EnemyInfo {
    pub fn new(
        health: Health,
        mass: f32,
        size: f32,
        movement_speed: f32,
        color: Color,
        enemy_type: EnemyType,
    ) -> Self {
        Self {
            health,
            mass,
            size,
            movement_speed,
            enemy_type,
            color,
        }
    }
}

#[derive(Debug, Clone)]
pub enum EnemyType {
    Corpse { lifetime: Health },
    Crawler,
    Attacker { attack: Attack },
    Projectile { lifetime: Health },
}
