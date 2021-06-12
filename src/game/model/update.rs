use super::*;

impl Model {
    pub fn update(&mut self, delta_time: f32) {}

    pub fn fixed_update(&mut self, delta_time: f32) {
        self.move_player(delta_time);
        self.move_enemies(delta_time);
        self.collide();
    }

    fn move_player(&mut self, delta_time: f32) {
        // Move
        self.player.body.position += self.player.body.velocity * delta_time;
        self.player.head.position += self.player.head.velocity * delta_time;

        // Calculate head movement direction
        let direction = self.player.head.position - self.player.body.position;
        let target = self.player.head_target - self.player.body.position;
        let angle = direction.angle_between(target).abs();
        let speed = angle.min(0.2) / 0.2;
        let direction = vec2(direction.y, -direction.x).normalize();
        let signum = direction.dot(target).signum();
        let direction = direction * signum * speed;
        self.player.head.velocity = direction * HEAD_SPEED + self.player.body.velocity;

        // Clamp distance between body and head
        let offset = self.player.head.position - self.player.body.position;
        let distance = offset.length() - self.player.chain_length;
        self.player.head.position -= offset.normalize_or_zero() * distance;
    }

    fn move_enemies(&mut self, delta_time: f32) {
        for enemy in &mut self.enemies {
            enemy.rigidbody.position += enemy.rigidbody.velocity * delta_time;
        }
    }

    fn collide(&mut self) {
        // Collide player body
        for enemy in &mut self.enemies {
            if let Some(collision) = enemy.rigidbody.collide(&self.player.body) {
                enemy.rigidbody.position += collision.normal * collision.penetration;
                enemy.rigidbody.velocity +=
                    BODY_HIT_SPEED * collision.normal * self.player.body.mass
                        / enemy.rigidbody.mass;
            }
        }

        // Collide player head
        for enemy in &mut self.enemies {
            if let Some(collision) = enemy.rigidbody.collide(&self.player.head) {
                enemy.rigidbody.position += collision.normal * collision.penetration;
                enemy.rigidbody.velocity += collision.normal.dot(self.player.head.velocity)
                    * collision.normal
                    * self.player.head.mass
                    / enemy.rigidbody.mass;
            }
        }

        // Collide enemies
    }
}
