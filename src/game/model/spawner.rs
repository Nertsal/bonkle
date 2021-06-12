use super::*;
use ::rand::{thread_rng, Rng};

pub struct Spawner {
    pub position: Vec2,
    pub spawn_group: WaveGroup,
    pub time_left: f32,
}

impl Spawner {
    pub fn new(position: Vec2, spawn_group: WaveGroup, spawn_time: f32) -> Self {
        Self {
            position,
            spawn_group,
            time_left: spawn_time,
        }
    }
}

impl Model {
    pub fn spawn_group(&mut self, position: Vec2, group: WaveGroup) {
        for enemy_info in group.enemies {
            self.enemies.push(Enemy::new(
                Self::get_random_position_circle(position, group.radius),
                enemy_info,
            ));
        }
    }

    pub fn get_random_position_bounds(bounds: &Bounds) -> Vec2 {
        let mut random = thread_rng();
        let x = random.gen::<f32>() * (bounds.max.x - bounds.min.x) + bounds.min.x;
        let y = random.gen::<f32>() * (bounds.max.y - bounds.min.y) + bounds.min.y;
        vec2(x, y)
    }

    pub fn get_random_position_circle(position: Vec2, radius: f32) -> Vec2 {
        let mut random = thread_rng();
        let angle = random.gen_range(0.0..std::f32::consts::PI * 2.0);
        let distance = random.gen_range(0.0..=1.0);
        let (sin, cos) = angle.sin_cos();
        vec2(cos, sin) * distance * radius + position
    }
}
