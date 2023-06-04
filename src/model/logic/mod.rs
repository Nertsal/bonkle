mod collisions;
mod control;
mod misc;
mod movement;

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
        self.update_lifetimes();
    }
}
