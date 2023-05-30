use crate::unwrap_or_panic;

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
    }

    fn player_control(&mut self) {
        let player = &self.model.player;
        let body = self
            .model
            .bodies
            .get_mut(player.body)
            .expect("Player body not found");
        let controller =
            unwrap_or_panic!(body.controller.as_mut(), "Player has no body controller");
        controller.target_velocity =
            self.player_input.target_move_dir.normalize_or_zero() * *body.movement_speed;
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
            position: &'a mut vec2<Coord>,
            velocity: &'a vec2<Coord>,
        }

        let mut query = query_body_ref!(self.model.bodies);
        let mut iter = query.iter_mut();
        while let Some((_body_id, body)) = iter.next() {
            *body.position += *body.velocity * self.delta_time;
        }
    }
}
