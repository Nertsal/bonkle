use super::*;

pub struct Spawner {
    pub position: vec2<f32>,
    pub spawn_group: WaveGroup,
    pub time_left: f32,
    pub time_left_max: f32,
}

impl Spawner {
    pub fn new(position: vec2<f32>, spawn_group: WaveGroup, spawn_time: f32) -> Self {
        Self {
            position,
            spawn_group,
            time_left: spawn_time,
            time_left_max: spawn_time,
        }
    }
}

impl Model {
    pub fn spawn_group(&mut self, position: vec2<f32>, group: WaveGroup) {
        for entity_info in group.entities {
            self.enemies.push(
                entity_info
                    .into_entity_object(Self::get_random_position_circle(position, group.radius)),
            );
        }
    }

    pub fn get_random_position_bounds(bounds: &Bounds) -> vec2<f32> {
        let mut rng = thread_rng();
        let x = rng.gen_range(bounds.min.x..bounds.max.x);
        let y = rng.gen_range(bounds.min.y..bounds.max.y);
        vec2(x, y)
    }

    pub fn get_random_position_circle(position: vec2<f32>, radius: f32) -> vec2<f32> {
        let mut rng = thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        let distance = rng.gen_range(0.0..1.0);
        let (sin, cos) = angle.sin_cos();
        vec2(cos, sin) * distance * radius + position
    }
}
