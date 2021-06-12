use super::*;

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
            self.enemies.push(Enemy::new(position, enemy_info));
        }
    }
}
