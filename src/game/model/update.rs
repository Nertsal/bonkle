use super::*;

impl Model {
    pub fn update(&mut self, delta_time: f32) {
        if self.player.health.is_alive() && self.enemies.len() == 0 && self.spawners.len() == 0 {
            self.next_wave();
        }

        self.update_spawners(delta_time);
        self.particles(delta_time);
    }

    fn update_spawners(&mut self, delta_time: f32) {
        let mut remove_spawners = Vec::new();
        for (index, spawner) in self.spawners.iter_mut().enumerate() {
            spawner.time_left -= delta_time;
            if spawner.time_left <= 0.0 {
                remove_spawners.push(index);
            }
        }
        remove_spawners.reverse();
        for spawner_index in remove_spawners {
            let spawner = self.spawners.remove(spawner_index);
            self.spawn_group(spawner.position, spawner.spawn_group);
        }
    }

    fn particles(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.rigidbody.movement(delta_time);
            particle.rigidbody.bounce_bounds(&self.bounds);
            particle.rigidbody.drag(delta_time);
            particle.lifetime.change(-delta_time);
            particle.color.a = particle.lifetime.hp_frac() * 0.5;
        }
        self.particles
            .retain(|particle| particle.lifetime.is_alive());
    }

    pub fn fixed_update(&mut self, delta_time: f32) {
        self.attack(delta_time);
        self.area_effects(delta_time);
        self.move_player(delta_time);
        self.move_enemies(delta_time);
        self.collide();
        self.check_dead(delta_time);
    }

    fn attack(&mut self, delta_time: f32) {
        let mut spawn_enemies = Vec::new();
        for enemy in &mut self.enemies {
            match &mut enemy.enemy_type {
                EnemyType::Attacker { attack } => {
                    attack.attack_time.change(-delta_time);
                    match &attack.attack_type {
                        AttackType::Shoot { projectile } if !attack.attack_time.is_alive() => {
                            let mut projectile =
                                Enemy::new(enemy.rigidbody.position, (**projectile).clone());
                            projectile.rigidbody.velocity = (self.player.body.position
                                - projectile.rigidbody.position)
                                .normalize()
                                * projectile.movement_speed;
                            spawn_enemies.push(projectile);
                        }
                        AttackType::Bomb {
                            projectile,
                            projectile_count,
                        } => {
                            if attack.attack_time.is_alive() {
                                let time_frac = attack.attack_time.hp_frac();
                                enemy.color = Color::new(
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
                                    let mut projectile = Enemy::new(
                                        enemy.rigidbody.position,
                                        (**projectile).clone(),
                                    );
                                    let angle = (i as f32) * std::f32::consts::PI * 2.0
                                        / (*projectile_count as f32)
                                        + random_offset;
                                    let (sin, cos) = angle.sin_cos();
                                    projectile.rigidbody.velocity =
                                        vec2(cos, sin) * projectile.movement_speed;
                                    spawn_enemies.push(projectile);
                                }
                                enemy.health.kill();
                                self.events.push(Event::Sound {
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

                EnemyType::Projectile { lifetime, .. } => {
                    lifetime.change(-delta_time);
                    if !lifetime.is_alive() {
                        enemy.health.kill();
                    }
                }
                _ => (),
            }
        }
        self.enemies.extend(spawn_enemies);
    }

    fn area_effects(&mut self, delta_time: f32) {
        for area_effect in &mut self.area_effects {
            area_effect.lifetime.change(-delta_time);

            if self.player.health.is_alive() {
                let distance = (area_effect.position - self.player.body.position).length();
                if distance <= self.player.body.collider.radius + area_effect.radius {
                    match &area_effect.effect {
                        Effect::Heal { heal } => {
                            self.player.health.change(*heal * delta_time);
                        }
                    }
                }
            }
        }
        self.area_effects
            .retain(|area_effect| area_effect.lifetime.is_alive());
    }

    fn move_player(&mut self, delta_time: f32) {
        // Move
        self.player.body.movement(delta_time);
        self.player.head.movement(delta_time);

        if self.player.health.is_alive() {
            // Calculate head target velocity
            let direction = self.player.head.position - self.player.body.position;
            let target = self.player.head_target - self.player.body.position;
            let angle = direction.angle_between(target).abs();
            let speed = angle.min(0.2) / 0.2;
            let direction = vec2(direction.y, -direction.x).normalize();
            let signum = direction.dot(target).signum();
            let direction = direction * signum * speed;
            self.player.target_head_velocity = direction * HEAD_SPEED + self.player.body.velocity;

            // Accelerate towards target velocity
            let target_change = self.player.target_body_velocity - self.player.body.velocity;
            self.player.body.velocity += target_change * BODY_ACCELERATION * delta_time;

            let target_change = self.player.target_head_velocity - self.player.head.velocity;
            self.player.head.velocity += target_change * HEAD_ACCELERATION * delta_time;
        }

        // Clamp distance between body and head
        let offset = self.player.head.position - self.player.body.position;
        let distance = offset.length() - self.player.chain_length;
        self.player.head.position -= offset.normalize_or_zero() * distance;
    }

    fn move_enemies(&mut self, delta_time: f32) {
        for enemy in &mut self.enemies {
            enemy.rigidbody.movement(delta_time);

            if enemy.rigidbody.velocity.length() > enemy.movement_speed {
                enemy.rigidbody.drag(delta_time);
            }
            match &enemy.enemy_type {
                EnemyType::Crawler | EnemyType::Attacker { .. } => {
                    let target_direction = self.player.body.position - enemy.rigidbody.position;
                    let target_velocity = target_direction.normalize() * enemy.movement_speed;
                    enemy.rigidbody.velocity +=
                        (target_velocity - enemy.rigidbody.velocity) * delta_time;
                }
                _ => (),
            }
        }
    }

    fn collide(&mut self) {
        // Collide bounds
        if self.player.body.bounce_bounds(&self.bounds) {
            self.events.push(Event::Sound {
                sound: EventSound::Bounce,
            });
        }
        self.player.head.bounce_bounds(&self.bounds);
        for enemy in &mut self.enemies {
            if enemy.rigidbody.bounce_bounds(&self.bounds) {
                if let EnemyType::Projectile { lifetime } = &mut enemy.enemy_type {
                    lifetime.kill();
                }

                self.events.push(Event::Sound {
                    sound: EventSound::Bounce,
                });
            }
        }

        let mut particles = Vec::new();

        // Collide player body
        for enemy in &mut self.enemies {
            if !enemy.is_alive() {
                continue;
            }

            if let Some(collision) = enemy.rigidbody.collide(&self.player.body) {
                enemy.rigidbody.position += collision.normal * collision.penetration;
                let relative_velocity = self.player.body.velocity - enemy.rigidbody.velocity;
                let hit_strength = collision.normal.dot(relative_velocity).abs();
                enemy.rigidbody.velocity +=
                    BODY_HIT_SPEED * collision.normal * self.player.body.mass
                        / enemy.rigidbody.mass;
                self.player.body.velocity -=
                    BODY_IMPACT * collision.normal * enemy.rigidbody.mass / self.player.body.mass;

                let contact = self.player.body.position + collision.normal * collision.penetration;
                let player_alive = self.player.health.is_alive();
                self.player.health.change(-hit_strength);
                particles.push((contact, hit_strength * 5.0, PLAYER_COLOR));
                enemy.health.change(-hit_strength);
                particles.push((contact, hit_strength, enemy.color));
                self.events.push(Event::Sound {
                    sound: EventSound::BodyHit,
                });
                if player_alive && !self.player.health.is_alive() {
                    self.events.push(Event::Sound {
                        sound: EventSound::Death,
                    })
                }
            }
        }

        // Collide player head
        for enemy in &mut self.enemies {
            if !enemy.is_alive() {
                continue;
            }

            if let Some(collision) = enemy.rigidbody.collide(&self.player.head) {
                enemy.rigidbody.position += collision.normal * collision.penetration;
                let relative_velocity = self.player.head.velocity - enemy.rigidbody.velocity;
                let hit_strength = collision.normal.dot(relative_velocity).abs();
                enemy.rigidbody.velocity +=
                    hit_strength * collision.normal * self.player.head.mass / enemy.rigidbody.mass;
                self.player.head.velocity -=
                    hit_strength * collision.normal * enemy.rigidbody.mass / self.player.body.mass;

                let contact = self.player.head.position + collision.normal * collision.penetration;
                enemy.health.change(-hit_strength);
                particles.push((contact, hit_strength, enemy.color));
                self.events.push(Event::Sound {
                    sound: EventSound::HeadHit,
                });
            }
        }

        // Particles
        for (position, damage, color) in particles {
            self.spawn_particles_hit(position, damage, color);
        }
    }

    fn check_dead(&mut self, delta_time: f32) {
        let mut dead_enemies = Vec::new();
        let mut particles = Vec::new();
        for (index, enemy) in self.enemies.iter_mut().enumerate() {
            if !enemy.is_alive() {
                match &mut enemy.enemy_type {
                    EnemyType::Corpse { lifetime } => {
                        lifetime.change(-delta_time);
                        if !lifetime.is_alive() {
                            dead_enemies.push(index);
                        }
                        enemy.color.a = lifetime.hp_frac() * 0.5;
                    }
                    EnemyType::Attacker { attack } if !attack.attack_time.is_alive() => {
                        match attack.attack_type {
                            AttackType::Bomb { .. } => {
                                particles.push((enemy.rigidbody.position, 500.0, BOMB_COLOR));
                                dead_enemies.push(index);
                            }
                            _ => (),
                        }
                    }
                    _ => {
                        enemy.enemy_type = EnemyType::Corpse {
                            lifetime: Health::new(CORPSE_LIFETIME),
                        }
                    }
                }
            }
        }
        dead_enemies.reverse();
        for dead_index in dead_enemies {
            self.enemies.remove(dead_index);
        }
        for (position, damage, color) in particles {
            self.spawn_particles_hit(position, damage, color);
        }
    }
}
