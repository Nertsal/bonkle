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

        let direction = self.player.head.position - self.player.body.position;
        let target = self.player.head_target - self.player.body.position;
        let angle = direction.angle_between(target).abs();
        let speed = angle.min(0.2) / 0.2;
        let direction = vec2(direction.y, -direction.x).normalize();
        let signum = direction.dot(target).signum();
        let direction = direction * signum * speed;
        self.player.head.velocity = direction * HEAD_SPEED;

        let offset = self.player.head.position - self.player.body.position;
        let distance = offset.length() - self.player.chain_length;
        self.player.head.position -= offset.normalize_or_zero() * distance;
    }

    fn collide(&mut self) {}
}
