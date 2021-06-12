use super::*;

impl Model {
    pub fn update(&mut self, delta_time: f32) {}

    pub fn fixed_update(&mut self, delta_time: f32) {
        self.move_player(delta_time);
        self.collide();
    }

    fn move_player(&mut self, delta_time: f32) {
        self.player.body.position += self.player.body.velocity * delta_time;
        self.player.head.position += self.player.head.velocity * delta_time;

        let offset = self.player.head.position - self.player.body.position;
        let distance = offset.length() - self.player.chain_length;
        self.player.head.position -= offset.normalize_or_zero() * distance;
    }

    fn collide(&mut self) {}
}
