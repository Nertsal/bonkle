use super::*;

impl Model {
    pub fn update(&mut self, delta_time: f32) {}

    pub fn fixed_update(&mut self, delta_time: f32) {
        self.move_player(delta_time);
    }

    fn move_player(&mut self, delta_time: f32) {
        self.player.body.position += self.player.body.velocity * delta_time;
    }
}
