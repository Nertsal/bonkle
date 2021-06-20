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
            || self.entities_type(vec![EntityType::Enemy]).any(|_| true)
    }

    fn attack(&mut self, delta_time: f32, commands: &mut Commands) {
        let mut targets = HashMap::new();
        for (index, entity) in self.entities().enumerate() {
            let target_types = entity.attack_targets();
            let entity_pos = entity.rigidbody.position;
            if let Some(target_pos) = self.find_closest(entity_pos, target_types) {
                targets.insert(index, target_pos);
            }
        }
        self.player.attack(None, delta_time, commands);
        for (index, entity) in self.entities_mut().enumerate() {
            entity.attack(targets.get(&index).copied(), delta_time, commands);
        }
    }

    fn find_closest(&self, origin: Vec2, target_types: Vec<EntityType>) -> Option<Vec2> {
        self.entities_type(target_types)
            .map(|entity| entity.rigidbody.position)
            .min_by(|&pos_a, &pos_b| {
                let dist_a = (pos_a - origin).length();
                let dist_b = (pos_b - origin).length();
                dist_a.partial_cmp(&dist_b).unwrap()
            })
    }

    fn entities_type(&self, target_types: Vec<EntityType>) -> impl Iterator<Item = &Entity> {
        let include_player = target_types.contains(&self.player.entity_type());
        self.entities()
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
        for (index, entity) in self.entities().enumerate() {
            let target_types = entity.movement_targets();
            let entity_pos = entity.rigidbody.position;
            if let Some(target_pos) = self.find_closest(entity_pos, target_types) {
                targets.insert(index, target_pos);
            }
        }
        self.player.decide_movement(None, delta_time);
        for (index, entity) in self.entities_mut().enumerate() {
            entity.decide_movement(targets.get(&index).copied(), delta_time);
        }
    }

    pub fn entities(&self) -> impl Iterator<Item = &Box<dyn EntityObject>> {
        self.enemies.iter().chain(self.minions.iter())
    }

    fn entities_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn EntityObject>> {
        self.enemies.iter_mut().chain(self.minions.iter_mut())
    }

    fn move_entities(&mut self, delta_time: f32) {
        self.player.movement(delta_time);
        for entity in self.entities_mut() {
            entity.movement(delta_time);
        }
    }

    fn collide(&mut self, commands: &mut Commands) {
        // Collide bounds
        let bounds = self.bounds;
        self.player.collide_bounds(&bounds, commands);
        self.player.head.bounce_bounds(&bounds);
        for entity in self.entities_mut() {
            entity.collide_bounds(&bounds, commands);
        }

        // Collide player body
        for enemy in self.enemies.iter_mut().filter(|enemy| enemy.is_alive()) {
            if let Some(hit_info) = self.player.collide(enemy) {
                let player_alive = self.player.entity.is_alive();
                self.player.entity.health.change(-hit_info.hit_self);
                commands.spawn_particles(hit_info.contact, hit_info.hit_self * 5.0, PLAYER_COLOR);
                enemy.health.change(-hit_info.hit_other);
                commands.spawn_particles(hit_info.contact, hit_info.hit_other, enemy.color);
                commands.event(Event::Sound {
                    sound: EventSound::BodyHit,
                });
                if player_alive && !self.player.entity.is_alive() {
                    commands.event(Event::Sound {
                        sound: EventSound::Death,
                    })
                }
            }
        }

        // Collide player head
        for enemy in self.enemies.iter_mut().filter(|enemy| enemy.is_alive()) {
            if let Some(hit_info) = self.player.head.collide(&mut enemy.rigidbody, None, None) {
                enemy.health.change(-hit_info.hit_other);
                commands.spawn_particles(hit_info.contact, hit_info.hit_other, enemy.color);
                commands.event(Event::Sound {
                    sound: EventSound::HeadHit,
                });
            }
        }

        // Collide minions
        for enemy in self.enemies.iter_mut().filter(|enemy| enemy.is_alive()) {
            for minion in self.minions.iter_mut().filter(|minion| minion.is_alive()) {
                if let Some(hit_info) = enemy.collide(minion) {
                    enemy.health.change(-hit_info.hit_self);
                    commands.spawn_particles(hit_info.contact, hit_info.hit_self, enemy.color);
                    minion.health.change(-hit_info.hit_other);
                    commands.spawn_particles(hit_info.contact, hit_info.hit_other, minion.color);
                    commands.event(Event::Sound {
                        sound: EventSound::HeadHit,
                    });
                }
            }
        }
    }

    fn check_dead(&mut self, delta_time: f32, commands: &mut Commands) {
        let mut dead_enemies = Vec::new();
        for (index, entity) in self.entities_mut().enumerate() {
            if entity.destroy {
                dead_enemies.push(index);
            } else if !entity.is_alive() {
                match entity.dead(delta_time) {
                    DeadState::Destroy => dead_enemies.push(index),
                    DeadState::Corpse => {
                        dead_enemies.push(index);
                        commands.spawn_entity(
                            Box::new(CorpseInfo::new(
                                entity.entity_type(),
                                Health::new(CORPSE_LIFETIME),
                                entity.rigidbody.velocity,
                                entity.entity_info(),
                            ))
                            .into_entity_object(entity.rigidbody.position),
                        );
                    }
                    DeadState::Idle => (),
                }
            }
        }
        dead_enemies.reverse();
        for dead_index in dead_enemies {
            if dead_index >= self.enemies.len() {
                self.minions.remove(dead_index - self.enemies.len());
            } else {
                self.enemies.remove(dead_index);
            }
        }
    }
}
