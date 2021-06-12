use super::*;

pub struct Wave {
    pub groups: Vec<WaveGroup>,
}

pub struct WaveGroup {
    pub enemies: Vec<EnemyInfo>,
    pub radius: f32,
}

impl Model {
    pub fn next_wave(&mut self) {
        if let Some(wave) = self.waves.pop_front() {
            for group in wave.groups {
                let group_position = Self::get_random_position_bounds(&self.spawn_bounds);
                self.spawners.push(Spawner::new(group_position, group, 2.0));
            }
        }
    }
}
