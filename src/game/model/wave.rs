use super::*;
use ::rand::{thread_rng, Rng};

pub struct Wave {
    pub groups: Vec<WaveGroup>,
}

pub struct WaveGroup {
    pub enemies: Vec<EnemyInfo>,
}

impl Model {
    pub fn next_wave(&mut self) {
        if let Some(wave) = self.waves.pop_front() {
            for group in wave.groups {
                let group_position = self.get_random_position();
                self.spawners.push(Spawner::new(group_position, group, 2.0));
            }
        }
    }

    pub fn get_random_position(&self) -> Vec2 {
        let mut random = thread_rng();
        let x = random.gen::<f32>() * (self.bounds.max.x - self.bounds.min.x) + self.bounds.min.x;
        let y = random.gen::<f32>() * (self.bounds.max.y - self.bounds.min.y) + self.bounds.min.y;
        vec2(x, y)
    }
}
