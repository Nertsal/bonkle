use std::collections::HashMap;

use super::*;

impl Model {
    pub fn update(&mut self, delta_time: f32) {
        let wave_active = self.wave();
        if !wave_active {
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
        let mut commands = Commands::new();

        self.attack(delta_time, &mut commands);
        self.area_effects(delta_time);
        self.decide_movement(delta_time);
        self.move_entities(delta_time);
        self.collide(&mut commands);
        self.check_dead(delta_time, &mut commands);

        self.perform_commands(commands);
    }

    fn wave(&mut self) -> bool {
        !self.player.entity.is_alive()
            || self.spawners.len() > 0
            || self
                .entities
                .iter()
                .any(|entity| entity.entity_type() == EntityType::Enemy)
    }

    fn attack(&mut self, delta_time: f32, commands: &mut Commands) {
        let mut targets = HashMap::new();
        for (index, entity) in self.entities.iter().enumerate() {
            let target_types = entity.attack_targets();
            let entity_pos = entity.entity().rigidbody.position;
            if let Some(target_pos) = self.find_closest(entity_pos, target_types) {
                targets.insert(index, target_pos);
            }
        }
        for (index, entity) in self.entities.iter_mut().enumerate() {
            entity.attack(targets.get(&index).copied(), delta_time, commands);
        }
    }

    fn find_closest(&self, origin: Vec2, target_types: Vec<EntityType>) -> Option<Vec2> {
        self.entities(target_types)
            .map(|entity| entity.rigidbody.position)
            .min_by(|&pos_a, &pos_b| {
                let dist_a = (pos_a - origin).length();
                let dist_b = (pos_b - origin).length();
                dist_a.partial_cmp(&dist_b).unwrap()
            })
    }

    fn entities(&self, target_types: Vec<EntityType>) -> impl Iterator<Item = &Entity> {
        let include_player = target_types.contains(&self.player.entity_type());
        self.entities
            .iter()
            .filter(move |entity| target_types.contains(&entity.entity_type()))
            .map(|entity| entity.entity())
            .chain(if include_player {
                vec![self.player.entity()].into_iter()
            } else {
                vec![].into_iter()
            })
    }

    fn area_effects(&mut self, delta_time: f32) {
        for area_effect in &mut self.area_effects {
            area_effect.lifetime.change(-delta_time);

            if self.player.entity.is_alive() {
                let distance =
                    (area_effect.position - self.player.entity.rigidbody.position).length();
                if distance <= self.player.entity.rigidbody.collider.radius + area_effect.radius {
                    match &area_effect.effect {
                        Effect::Heal { heal } => {
                            self.player.entity.health.change(*heal * delta_time);
                        }
                    }
                }
            }
        }
        self.area_effects
            .retain(|area_effect| area_effect.lifetime.is_alive());
    }

    fn decide_movement(&mut self, delta_time: f32) {
        let mut targets = HashMap::new();
        for (index, entity) in self.entities.iter().enumerate() {
            let target_types = entity.movement_targets();
            let entity_pos = entity.entity().rigidbody.position;
            if let Some(target_pos) = self.find_closest(entity_pos, target_types) {
                targets.insert(index, target_pos);
            }
        }
        self.player.decide_movement(None, delta_time);
        for (index, entity) in self.entities.iter_mut().enumerate() {
            entity.decide_movement(targets.get(&index).copied(), delta_time);
        }
    }

    fn move_entities(&mut self, delta_time: f32) {
        self.player.movement(delta_time);
        for entity in &mut self.entities {
            entity.movement(delta_time);
        }
    }

    fn collide(&mut self, commands: &mut Commands) {
        // Collide bounds
        self.player.collide_bounds(&self.bounds, commands);
        self.player.head.bounce_bounds(&self.bounds);
        for entity in &mut self.entities {
            entity.collide_bounds(&self.bounds, commands);
        }

        // Collide player body
        for entity in &mut self.entities {
            if !entity.entity().is_alive() {
                continue;
            }

            if let Some(collision) = entity
                .entity()
                .rigidbody
                .collide(&self.player.entity.rigidbody)
            {
                entity.entity_mut().rigidbody.position += collision.normal * collision.penetration;
                let relative_velocity =
                    self.player.entity.rigidbody.velocity - entity.entity().rigidbody.velocity;
                let hit_strength = collision.normal.dot(relative_velocity).abs();
                let velocity_change =
                    BODY_HIT_SPEED * collision.normal * self.player.entity.rigidbody.mass
                        / entity.entity().rigidbody.mass;
                entity.entity_mut().rigidbody.velocity += velocity_change;
                self.player.entity.rigidbody.velocity -=
                    BODY_IMPACT * collision.normal * entity.entity().rigidbody.mass
                        / self.player.entity.rigidbody.mass;

                let contact = self.player.entity.rigidbody.position
                    + collision.normal * collision.penetration;
                let player_alive = self.player.entity.is_alive();
                self.player.entity.health.change(-hit_strength);
                commands.spawn_particles(contact, hit_strength * 5.0, PLAYER_COLOR);
                entity.entity_mut().health.change(-hit_strength);
                commands.spawn_particles(contact, hit_strength, entity.entity().color);
                self.events.push(Event::Sound {
                    sound: EventSound::BodyHit,
                });
                if player_alive && !self.player.entity.is_alive() {
                    self.events.push(Event::Sound {
                        sound: EventSound::Death,
                    })
                }
            }
        }

        // Collide player head
        for entity in &mut self.entities {
            if !entity.entity().is_alive() {
                continue;
            }

            if let Some(collision) = entity.entity().rigidbody.collide(&self.player.head) {
                entity.entity_mut().rigidbody.position += collision.normal * collision.penetration;
                let relative_velocity =
                    self.player.head.velocity - entity.entity().rigidbody.velocity;
                let hit_strength = collision.normal.dot(relative_velocity).abs();
                let velocity_change = hit_strength * collision.normal * self.player.head.mass
                    / entity.entity().rigidbody.mass;
                entity.entity_mut().rigidbody.velocity += velocity_change;
                self.player.head.velocity -=
                    hit_strength * collision.normal * entity.entity().rigidbody.mass
                        / self.player.entity.rigidbody.mass;

                let contact = self.player.head.position + collision.normal * collision.penetration;
                entity.entity_mut().health.change(-hit_strength);
                commands.spawn_particles(contact, hit_strength, entity.entity().color);
                self.events.push(Event::Sound {
                    sound: EventSound::HeadHit,
                });
            }
        }
    }

    fn check_dead(&mut self, delta_time: f32, commands: &mut Commands) {
        let mut dead_enemies = Vec::new();
        for (index, entity) in self.entities.iter_mut().enumerate() {
            if entity.entity().destroy {
                dead_enemies.push(index);
            } else if !entity.entity().is_alive() {
                match entity.dead(delta_time) {
                    DeadState::Destroy => dead_enemies.push(index),
                    DeadState::Corpse => {
                        dead_enemies.push(index);
                        commands.spawn_entity(
                            Box::new(CorpseInfo::new(
                                entity.entity_type(),
                                Health::new(CORPSE_LIFETIME),
                                entity.entity().rigidbody.velocity,
                                entity.entity().entity_info(),
                            ))
                            .into_entity_object(entity.entity().rigidbody.position),
                        );
                    }
                    DeadState::Idle => (),
                }
            }
        }
        dead_enemies.reverse();
        for dead_index in dead_enemies {
            self.entities.remove(dead_index);
        }
    }
}
