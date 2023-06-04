use super::*;

impl Logic<'_> {
    pub fn body_collisions(&mut self) {
        #[derive(StructQuery, Clone, Copy)]
        struct BodyRef<'a> {
            collider: &'a Collider,
            velocity: &'a vec2<Coord>,
            mass: &'a Mass,
        }

        let query = query_body_ref!(self.model.bodies);

        // Evaluate collisions
        struct CollisionInfo<'a> {
            body_id: Id,
            body: BodyRef<'a>,
            other_id: Id,
            other: BodyRef<'a>,
            collision: Collision,
        }

        let mut collisions: Vec<CollisionInfo> = Vec::new();
        // TODO: optimize with quad-tree or aabb's or smth
        for (body_id, body) in &query {
            for (other_id, other) in &query {
                if body_id == other_id {
                    continue;
                }
                if let Some(collision) = body.collider.collide(other.collider) {
                    collisions.push(CollisionInfo {
                        body_id,
                        body,
                        other_id,
                        other,
                        collision,
                    });
                }
            }
        }

        // Resolve collisions
        struct Correction {
            position: vec2<Coord>,
            velocity: vec2<Coord>,
            damage: Hp,
        }

        let mut particles = Vec::new();
        let mut corrections: HashMap<Id, Correction> = HashMap::new();
        for info in collisions {
            let mut body_correction = Correction {
                position: info.body.collider.position,
                velocity: *info.body.velocity,
                damage: Hp::ZERO,
            };
            let mut other_correction = Correction {
                position: info.other.collider.position,
                velocity: *info.other.velocity,
                damage: Hp::ZERO,
            };

            // Translate
            let translation = info.collision.normal * info.collision.penetration / r32(2.0);
            body_correction.position -= translation;
            other_correction.position += translation;

            // Linear bounce
            let relative_velocity = *info.body.velocity - *info.other.velocity;
            let hit_strength = vec2::dot(info.collision.normal, relative_velocity).abs();
            let hit_self = hit_strength * *info.other.mass / *info.body.mass;
            let hit_other = hit_strength * *info.body.mass / *info.other.mass;
            body_correction.velocity -= info.collision.normal * hit_self;
            other_correction.velocity += info.collision.normal * hit_other;

            // Damage
            let damage_fn = |hit: R32| -> Hp { hit / r32(5.0) };
            body_correction.damage = damage_fn(hit_self);
            other_correction.damage = damage_fn(hit_other);

            // TODO: Angular bounce

            corrections.extend([
                (info.body_id, body_correction),
                (info.other_id, other_correction),
            ]);

            // Particles
            particles.extend([(info.collision.point, hit_strength)]);
        }

        // Particles
        for (position, intensity) in particles {
            self.spawn_particles_hit(position, intensity);
        }

        // Apply corrections
        #[derive(StructQuery)]
        struct BodyUpdate<'a> {
            #[query(optic = ".collider._get", component = "Collider")]
            position: &'a mut vec2<Coord>,
            velocity: &'a mut vec2<Coord>,
            health: &'a mut Option<Health>,
        }

        let mut query = query_body_update!(self.model.bodies);
        for (body, correction) in corrections {
            let body = query.get_mut(body).unwrap(); // Body guaranteed to be valid
            *body.position = correction.position;
            *body.velocity = correction.velocity;
            if let Some(health) = body.health {
                health.damage(correction.damage);
            }
        }
    }

    /// Collide bodies and corpses with level bounds.
    pub fn collide_bounds(&mut self) {
        macro_rules! process {
            ($storage:expr) => {
                let bounds = self.model.bounds;
                let mut query = query_body_ref!($storage);
                let mut iter = query.iter_mut();
                while let Some((_, body)) = iter.next() {
                    let aabb = body.collider.compute_aabb();

                    let left = (bounds.min.x - aabb.min.x).as_f32();
                    let right = (aabb.max.x - bounds.max.x).as_f32();

                    let (nx, dx) = if right > left && right > 0.0 {
                        (1.0, right)
                    } else if left > 0.0 {
                        (-1.0, left)
                    } else {
                        (0.0, 0.0)
                    };

                    let down = (bounds.min.y - aabb.min.y).as_f32();
                    let up = (aabb.max.y - bounds.max.y).as_f32();

                    let (ny, dy) = if up > down && up > 0.0 {
                        (1.0, up)
                    } else if down > 0.0 {
                        (-1.0, down)
                    } else {
                        (0.0, 0.0)
                    };

                    let normal = vec2(nx, ny).as_r32();
                    let penetration = vec2(dx, dy).as_r32();

                    // Translate
                    body.collider.position -= normal * penetration;

                    // Linear bounce
                    let bounciness = r32(0.7);
                    let projection = vec2::dot(normal, *body.velocity);
                    *body.velocity -= normal * projection * (Coord::ONE + bounciness);

                    // TODO: angular bounce
                }
            };
        }

        {
            #[derive(StructQuery)]
            struct BodyRef<'a> {
                collider: &'a mut Collider,
                velocity: &'a mut vec2<Coord>,
            }
            process!(self.model.bodies);
        }
        {
            #[derive(StructQuery)]
            struct BodyRef<'a> {
                #[query(nested = ".body")]
                collider: &'a mut Collider,
                #[query(nested = ".body")]
                velocity: &'a mut vec2<Coord>,
            }
            process!(self.model.corpses);
        }
    }
}
