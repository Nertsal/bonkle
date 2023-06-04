use crate::{unwrap_or_panic, util::RealConversions};

use super::*;

pub struct Logic<'a> {
    pub model: &'a mut Model,
    pub player_input: PlayerInput,
    pub delta_time: Time,
}

impl Logic<'_> {
    pub fn process(&mut self) {
        // Control
        self.player_control();
        self.body_ai();
        self.body_control();

        // Movement/collisions
        self.movement();
        self.body_attachment();
        self.body_collisions();
        self.collide_bounds();

        // Misc
        self.check_deaths();
        self.process_corpses();
    }

    fn player_control(&mut self) {
        let player = &self.model.player;

        // Body
        let body = self
            .model
            .bodies
            .get_mut(player.body)
            .expect("Player body not found");
        let controller =
            unwrap_or_panic!(body.controller.as_mut(), "Player has no body controller");
        controller.target_velocity =
            self.player_input.target_move_dir.normalize_or_zero() * *body.speed;

        // Head
        let body_position = body.collider.position;
        let body_velocity = *body.velocity;
        let head = self
            .model
            .bodies
            .get_mut(player.head)
            .expect("Player head not found");
        let controller =
            unwrap_or_panic!(head.controller.as_mut(), "Player has no head controller");

        let pos = head.collider.position;
        let delta = pos - body_position;
        let angle = Angle::from_radians(delta.arg());

        let target_angle = self
            .player_input
            .head_target
            .get_target(body_position, angle);

        let angle_delta = angle.angle_to(target_angle).as_radians();
        let sign = angle_delta.signum();
        let dir = angle.unit_vec().rotate_90() * sign; // Move along the tangent

        let speed_coef = (angle_delta.abs().as_f32() / 0.1).min(1.0).as_r32();
        let speed = *head.speed * speed_coef;

        controller.target_velocity = body_velocity + dir * speed; // TODO: better velocity calculation
    }

    fn body_ai(&mut self) {
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            collider: &'a Collider,
            speed: &'a Coord,
            #[query(optic = "._Some")]
            controller: &'a mut BodyController,
        }

        // Calculate actions
        let mut actions: HashMap<Id, BodyController> = HashMap::new();
        let mut query = query_body_ref!(self.model.bodies);
        for (body_id, body) in &query {
            let mut controller = body.controller.clone();
            let Some(ai) = &controller.ai else { continue; };
            match ai {
                BodyAI::Crawler => {
                    if let Some(player_body) = query.get(self.model.player.body) {
                        let delta = player_body.collider.position - body.collider.position;
                        let dir = delta.normalize_or_zero();
                        controller.target_velocity = dir * *body.speed;
                    }
                }
            };
            actions.insert(body_id, controller);
        }

        // Apply actions
        for (body_id, new_controller) in actions {
            let body = query.get_mut(body_id).unwrap();
            *body.controller = new_controller;
        }
    }

    fn body_control(&mut self) {
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            velocity: &'a mut vec2<Coord>,
            #[query(optic = "._Some")]
            controller: &'a BodyController,
        }

        let mut query = query_body_ref!(self.model.bodies);
        let mut iter = query.iter_mut();
        while let Some((_body_id, body)) = iter.next() {
            let acceleration = if vec2::dot(body.controller.target_velocity, *body.velocity)
                < Coord::ZERO
                || body.controller.target_velocity.len() < body.velocity.len()
            {
                body.controller.deceleration
            } else {
                body.controller.acceleration
            };
            *body.velocity +=
                (body.controller.target_velocity - *body.velocity) * acceleration * self.delta_time;
        }
    }

    /// Move bodies and corpses according to their velocity.
    fn movement(&mut self) {
        macro_rules! process {
            ($storage:expr) => {
                let mut query = query_body_ref!($storage);
                let mut iter = query.iter_mut();
                while let Some((_body_id, body)) = iter.next() {
                    body.collider.position += *body.velocity * self.delta_time;
                }
            };
        }

        {
            #[derive(StructQuery)]
            struct BodyRef<'a> {
                collider: &'a mut Collider,
                velocity: &'a vec2<Coord>,
            }
            process!(self.model.bodies);
        }

        {
            #[derive(StructQuery)]
            struct BodyRef<'a> {
                #[query(optic = ".body.collider._get._id")]
                collider: &'a mut Collider,
                #[query(optic = ".body.velocity._get._id")]
                velocity: &'a vec2<Coord>,
            }
            process!(self.model.corpses);
        }
    }

    /// Correct bodies' attachments by moving them in a position that satifies the constraint.
    fn body_attachment(&mut self) {
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            collider: &'a mut Collider,
            velocity: &'a mut vec2<Coord>,
            attachment: &'a Option<BodyAttachment>,
        }

        let mut query = query_body_ref!(self.model.bodies);

        // Collect corrections
        struct Correction {
            position: vec2<Coord>,
            velocity: vec2<Coord>,
        }
        let mut corrections = HashMap::<Id, Correction>::new();
        for (body_id, body) in &query {
            if let Some(attachment) = body.attachment {
                if let Some(to_body) = query.get(attachment.to_body) {
                    match attachment.ty {
                        AttachmentType::Orbit { distance } => {
                            let angle = Angle::from_radians(
                                (body.collider.position - to_body.collider.position).arg(),
                            );
                            let position = to_body.collider.position + angle.unit_vec() * distance;

                            // Preserve speed - change direction
                            let speed = body.velocity.len();
                            let dir = angle.unit_vec().rotate_90();
                            let velocity = dir * speed * vec2::dot(dir, *body.velocity).signum();

                            corrections.insert(body_id, Correction { position, velocity });
                        }
                    }
                } else {
                    // Body attachment not found
                    // TODO: idk
                }
            }
        }

        // Apply corrections
        for (body, correction) in corrections {
            let body = query.get_mut(body).unwrap(); // Body guaranteed to be valid
            body.collider.position = correction.position;
            *body.velocity = correction.velocity;
        }
    }

    fn body_collisions(&mut self) {
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
        }

        // Apply corrections
        #[derive(StructQuery)]
        struct BodyUpdate<'a> {
            collider: &'a mut Collider,
            velocity: &'a mut vec2<Coord>,
            health: &'a mut Option<Health>,
        }

        let mut query = query_body_update!(self.model.bodies);
        for (body, correction) in corrections {
            let body = query.get_mut(body).unwrap(); // Body guaranteed to be valid
            body.collider.position = correction.position;
            *body.velocity = correction.velocity;
            if let Some(health) = body.health {
                health.damage(correction.damage);
            }
        }
    }

    /// Collide bodies and corpses with level bounds.
    fn collide_bounds(&mut self) {
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
                #[query(optic = ".body.collider._get._id")]
                collider: &'a mut Collider,
                #[query(optic = ".body.velocity._get._id")]
                velocity: &'a mut vec2<Coord>,
            }
            process!(self.model.corpses);
        }
    }

    fn check_deaths(&mut self) {
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            #[query(optic = "._Some")]
            health: &'a Health,
        }

        let deaths: Vec<Id> = query_body_ref!(self.model.bodies)
            .iter()
            .filter(|(_, body)| body.health.is_dead())
            .map(|(id, _)| id)
            .collect();
        for body_id in deaths {
            let body = self.model.bodies.remove(body_id).unwrap();
            self.model.corpses.insert(BodyCorpse {
                body,
                lifetime: Health::new(r32(1.0)),
            });
            // TODO: particles
        }
    }

    fn process_corpses(&mut self) {
        #[derive(StructQuery)]
        struct CorpseRef<'a> {
            lifetime: &'a mut Health,
        }

        let mut query = query_corpse_ref!(self.model.corpses);
        let mut iter = query.iter_mut();
        let mut deaths = Vec::new();
        while let Some((id, corpse)) = iter.next() {
            corpse.lifetime.damage(self.delta_time);
            if corpse.lifetime.is_dead() {
                deaths.push(id);
            }
        }

        for id in deaths {
            self.model.corpses.remove(id);
        }
    }
}
