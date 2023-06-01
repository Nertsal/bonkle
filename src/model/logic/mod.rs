use crate::{unwrap_or_panic, util::RealConversions};

use super::*;

pub struct Logic<'a> {
    pub model: &'a mut Model,
    pub player_input: PlayerInput,
    pub delta_time: Time,
}

impl Logic<'_> {
    pub fn process(&mut self) {
        self.player_control();
        self.body_control();
        self.body_movement();
        self.body_attachment();
        self.body_collisions();
        self.body_bounds();
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

    fn body_control(&mut self) {
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            velocity: &'a mut vec2<Coord>,
            #[query(optional)]
            controller: &'a BodyController,
        }

        let mut query = query_body_ref!(self.model.bodies);
        let mut iter = query.iter_mut();
        while let Some((_body_id, body)) = iter.next() {
            *body.velocity += (body.controller.target_velocity - *body.velocity)
                * body.controller.acceleration
                * self.delta_time;
        }
    }

    fn body_movement(&mut self) {
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            collider: &'a mut Collider,
            velocity: &'a vec2<Coord>,
        }

        let mut query = query_body_ref!(self.model.bodies);
        let mut iter = query.iter_mut();
        while let Some((_body_id, body)) = iter.next() {
            body.collider.position += *body.velocity * self.delta_time;
        }
    }

    /// Correct bodies' attachments by moving them in a position that satifies the constraint.
    fn body_attachment(&mut self) {
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            collider: &'a mut Collider,
            attachment: &'a Option<BodyAttachment>,
        }

        let mut query = query_body_ref!(self.model.bodies);

        // Collect corrections
        let mut corrections = HashMap::<Id, vec2<Coord>>::new();
        for (body_id, body) in &query {
            if let Some(attachment) = body.attachment {
                if let Some(to_body) = query.get(attachment.to_body) {
                    match attachment.ty {
                        AttachmentType::Orbit { distance } => {
                            let angle = Angle::from_radians(
                                (body.collider.position - to_body.collider.position).arg(),
                            );
                            let target = to_body.collider.position + angle.unit_vec() * distance;
                            corrections.insert(body_id, target);
                        }
                    }
                } else {
                    // Body attachment not found
                    // TODO: idk
                }
            }
        }

        // Apply corrections
        for (body, target_pos) in corrections {
            let body = query.get_mut(body).unwrap(); // Body guaranteed to be valid
            body.collider.position = target_pos;
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
        }

        let mut corrections: HashMap<Id, Correction> = HashMap::new();
        for info in collisions {
            let mut body_correction = Correction {
                position: info.body.collider.position,
                velocity: *info.body.velocity,
            };
            let mut other_correction = Correction {
                position: info.other.collider.position,
                velocity: *info.other.velocity,
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
        }

        let mut query = query_body_update!(self.model.bodies);
        for (body, correction) in corrections {
            let body = query.get_mut(body).unwrap(); // Body guaranteed to be valid
            body.collider.position = correction.position;
            *body.velocity = correction.velocity;
        }
    }

    /// Collide with level bounds.
    fn body_bounds(&mut self) {
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            collider: &'a mut Collider,
        }

        let bounds = self.model.bounds;
        let mut query = query_body_ref!(self.model.bodies);
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
                (1.0, 0.0)
            };

            let down = (bounds.min.y - aabb.min.y).as_f32();
            let up = (aabb.max.y - bounds.max.y).as_f32();

            let (ny, dy) = if up > down && up > 0.0 {
                (1.0, up)
            } else if down > 0.0 {
                (-1.0, down)
            } else {
                (1.0, 0.0)
            };

            let normal = vec2(nx, ny).as_r32();
            let penetration = vec2(dx, dy).as_r32();

            // Translate
            body.collider.position -= normal * penetration;

            // Linear bounce

            // TODO: angular bounce
        }
    }
}
