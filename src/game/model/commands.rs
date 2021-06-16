use super::*;

pub struct Commands {
    commands: Vec<Command>,
}

enum Command {
    SpawnEnemy {
        enemy: Enemy,
    },
    SpawnParticles {
        position: Vec2,
        intensity: f32,
        color: Color,
    },
}

impl Commands {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }

    pub fn spawn_enemy(&mut self, enemy: Enemy) {
        self.commands.push(Command::SpawnEnemy { enemy });
    }

    pub fn spawn_particles(&mut self, position: Vec2, intensity: f32, color: Color) {
        self.commands.push(Command::SpawnParticles {
            position,
            intensity,
            color,
        });
    }
}

impl Model {
    pub fn perform_commands(&mut self, commands: Commands) {
        for command in commands.commands {
            match command {
                Command::SpawnEnemy { enemy } => self.enemies.push(enemy),
                Command::SpawnParticles {
                    position,
                    intensity,
                    color,
                } => {
                    self.spawn_particles_hit(position, intensity, color);
                }
            }
        }
    }
}
