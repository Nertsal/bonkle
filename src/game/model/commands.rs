use super::*;

pub struct Commands {
    commands: Vec<Command>,
    pub events: Vec<Event>,
}

enum Command {
    SpawnEntity {
        entity: Entity,
    },
    SpawnParticles {
        position: Vec2,
        intensity: f32,
        color: Color,
    },
}

impl Commands {
    pub fn new() -> Self {
        Self {
            commands: vec![],
            events: vec![],
        }
    }

    pub fn spawn_entity(&mut self, entity: Entity) {
        self.commands.push(Command::SpawnEntity { entity });
    }

    pub fn spawn_particles(&mut self, position: Vec2, intensity: f32, color: Color) {
        self.commands.push(Command::SpawnParticles {
            position,
            intensity,
            color,
        });
    }

    pub fn event(&mut self, event: Event) {
        self.events.push(event);
    }
}

impl Model {
    pub fn perform_commands(&mut self, commands: Commands) {
        for command in commands.commands {
            match command {
                Command::SpawnEntity { entity } => self.entities.push(entity),
                Command::SpawnParticles {
                    position,
                    intensity,
                    color,
                } => {
                    self.spawn_particles_hit(position, intensity, color);
                }
            }
        }
        self.events.extend(commands.events);
    }
}
