use super::*;

impl Model {
    pub fn update(&mut self, delta_time: f32) {
        if self.enemies.len() == 0 && self.spawners.len() == 0 {
            self.next_wave();
        }
        self.update_spawners(delta_time);
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

    pub fn fixed_update(&mut self, delta_time: f32) {
        self.move_player(delta_time);
        self.move_enemies(delta_time);
        self.collide();
        self.check_dead();
    }

    fn move_player(&mut self, delta_time: f32) {
        // Move
        self.player.body.position += self.player.body.velocity * delta_time;
        self.player.head.position += self.player.head.velocity * delta_time;

        // Calculate head movement direction
        let direction = self.player.head.position - self.player.body.position;
        let target = self.player.head_target - self.player.body.position;
        let angle = direction.angle_between(target).abs();
        let speed = angle.min(0.2) / 0.2;
        let direction = vec2(direction.y, -direction.x).normalize();
        let signum = direction.dot(target).signum();
        let direction = direction * signum * speed;
        self.player.head.velocity = direction * HEAD_SPEED + self.player.body.velocity;

        // Clamp distance between body and head
        let offset = self.player.head.position - self.player.body.position;
        let distance = offset.length() - self.player.chain_length;
        self.player.head.position -= offset.normalize_or_zero() * distance;
    }

    fn move_enemies(&mut self, delta_time: f32) {
        for enemy in &mut self.enemies {
            enemy.rigidbody.position += enemy.rigidbody.velocity * delta_time;
            if enemy.rigidbody.velocity.length() > enemy.movement_speed {
                enemy.rigidbody.velocity *= 1.0 - DRAG * delta_time;
            }
        }
    }

    fn collide(&mut self) {
        // Collide bounds
        self.player.body.clamp_bounds(&self.bounds);
        self.player.head.clamp_bounds(&self.bounds);
        for enemy in &mut self.enemies {
            enemy.rigidbody.bounce_bounds(&self.bounds);
        }

        // Collide player body
        for enemy in &mut self.enemies {
            if let Some(collision) = enemy.rigidbody.collide(&self.player.body) {
                enemy.rigidbody.position += collision.normal * collision.penetration;
                let relative_velocity = self.player.body.velocity - enemy.rigidbody.velocity;
                let hit_strength = collision.normal.dot(relative_velocity);
                self.player.health -= hit_strength;
                enemy.rigidbody.velocity +=
                    BODY_HIT_SPEED * collision.normal * self.player.body.mass
                        / enemy.rigidbody.mass;
            }
        }

        // Collide player head
        for enemy in &mut self.enemies {
            if let Some(collision) = enemy.rigidbody.collide(&self.player.head) {
                enemy.rigidbody.position += collision.normal * collision.penetration;
                let relative_velocity = self.player.head.velocity - enemy.rigidbody.velocity;
                let hit_strength = collision.normal.dot(relative_velocity);
                enemy.health -= hit_strength;
                enemy.rigidbody.velocity +=
                    hit_strength * collision.normal * self.player.head.mass / enemy.rigidbody.mass;
            }
        }

        // Collide enemies
    }

    fn check_dead(&mut self) {
        self.enemies.retain(|enemy| enemy.health > 0.0);
    }
}
