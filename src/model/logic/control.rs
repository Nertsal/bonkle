use super::*;

impl Logic<'_> {
    pub fn player_control(&mut self) {
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

    pub fn body_ai(&mut self) {
        // Calculate actions
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            collider: &'a Collider,
            speed: &'a Coord,
            #[query(optic = "._Some")]
            controller: &'a BodyController,
        }

        let mut actions: HashMap<Id, BodyController> = HashMap::new();
        let query = query_body_ref!(self.model.bodies);
        for (body_id, body) in &query_body_ref!(self.model.bodies) {
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
        #[derive(StructQuery)]
        struct UpdateRef<'a> {
            #[query(optic = "._Some")]
            controller: &'a mut BodyController,
        }

        let mut query = query_update_ref!(self.model.bodies);
        for (body_id, new_controller) in actions {
            let body = query.get_mut(body_id).unwrap();
            *body.controller = new_controller;
        }
    }

    pub fn body_control(&mut self) {
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
}
