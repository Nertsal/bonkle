use super::*;

pub struct Spawner {
    pub position: Vec2,
    pub spawn_group: WaveGroup,
    pub time_left: f32,
    pub time_left_max: f32,
}

impl Spawner {
    pub fn new(position: Vec2, spawn_group: WaveGroup, spawn_time: f32) -> Self {
        Self {
            position,
            spawn_group,
            time_left: spawn_time,
            time_left_max: spawn_time,
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
        let x = macroquad::rand::gen_range(bounds.min.x, bounds.max.x);
        let y = macroquad::rand::gen_range(bounds.min.y, bounds.max.y);
        vec2(x, y)
    }

    pub fn get_random_position_circle(position: Vec2, radius: f32) -> Vec2 {
        let angle = macroquad::rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let distance = macroquad::rand::gen_range(0.0, 1.0);
        let (sin, cos) = angle.sin_cos();
        vec2(cos, sin) * distance * radius + position
    }
}
