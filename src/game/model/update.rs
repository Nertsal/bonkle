use super::*;

impl Model {
    pub fn update(&mut self, delta_time: f32) {
        if self.player.health > 0.0 && self.enemies.len() == 0 && self.spawners.len() == 0 {
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
            particle.rigidbody.drag(DRAG, delta_time);
            particle.lifetime -= delta_time;
            particle.color.a = particle.lifetime / PARTICLE_LIFETIME * 0.5;
        }
        self.particles.retain(|particle| particle.lifetime > 0.0);
    }

    pub fn fixed_update(&mut self, delta_time: f32) {
        self.attack(delta_time);
        self.move_player(delta_time);
        self.move_enemies(delta_time);
        self.collide();
        self.check_dead(delta_time);
    }

    fn attack(&mut self, delta_time: f32) {
        let mut spawn_enemies = Vec::new();
        for enemy in &mut self.enemies {
            match &mut enemy.enemy_type {
                EnemyType::Ranger {
                    projectile,
                    attack_cooldown,
                    attack_time,
                } => {
                    *attack_cooldown -= delta_time;
                    if *attack_cooldown <= 0.0 {
                        *attack_cooldown = *attack_time;
                        let mut projectile =
                            Enemy::new(enemy.rigidbody.position, (**projectile).clone());
                        projectile.rigidbody.velocity =
                            (self.player.body.position - projectile.rigidbody.position).normalize()
                                * projectile.movement_speed;
                        spawn_enemies.push(projectile);
                    }
                }
                EnemyType::Projectile { lifetime } => {
                    *lifetime -= delta_time;
                    if *lifetime <= 0.0 {
                        enemy.health = 0.0;
                    }
                }
                _ => (),
            }
        }
        self.enemies.extend(spawn_enemies);
    }

    fn move_player(&mut self, delta_time: f32) {
        // Move
        self.player.body.movement(delta_time);
        self.player.head.movement(delta_time);

        if self.player.health > 0.0 {
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
                enemy.rigidbody.drag(DRAG, delta_time);
            }
            match &enemy.enemy_type {
                EnemyType::Melee | EnemyType::Ranger { .. } => {
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
        self.player.body.bounce_bounds(&self.bounds);
        self.player.head.bounce_bounds(&self.bounds);
        for enemy in &mut self.enemies {
            enemy.rigidbody.bounce_bounds(&self.bounds);
        }

        let mut particles = Vec::new();

        // Collide player body
        for enemy in &mut self.enemies {
            if enemy.health <= 0.0 {
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
                self.player.health -= hit_strength;
                particles.push((contact, hit_strength * 5.0, PLAYER_COLOR));
                enemy.health -= hit_strength;
                particles.push((contact, hit_strength, enemy.color));
            }
        }

        // Collide player head
        for enemy in &mut self.enemies {
            if enemy.health <= 0.0 {
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
                enemy.health -= hit_strength;
                particles.push((contact, hit_strength, enemy.color));
            }
        }

        // Particles
        for (position, damage, color) in particles {
            self.spawn_particles_hit(position, damage, color);
        }
    }

    fn check_dead(&mut self, delta_time: f32) {
        let mut dead_enemies = Vec::new();
        for (index, enemy) in self.enemies.iter_mut().enumerate() {
            if enemy.health <= 0.0 {
                match &mut enemy.enemy_type {
                    EnemyType::Corpse { lifetime } => {
                        *lifetime -= delta_time;
                        let lifetime = *lifetime;
                        if lifetime <= 0.0 {
                            dead_enemies.push(index);
                        }
                        enemy.color.a = lifetime / CORPSE_LIFETIME * 0.5;
                    }
                    _ => {
                        enemy.enemy_type = EnemyType::Corpse {
                            lifetime: CORPSE_LIFETIME,
                        }
                    }
                }
            }
        }
        dead_enemies.reverse();
        for dead_index in dead_enemies {
            self.enemies.remove(dead_index);
        }
    }
}
