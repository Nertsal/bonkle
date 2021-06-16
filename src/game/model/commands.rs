use super::*;

pub struct Commands {
    commands: Vec<Command>,
    pub events: Vec<Event>,
}

enum Command {
    SpawnEntity {
        entity: EntityObject,
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

    pub fn spawn_entity(&mut self, entity: EntityObject) {
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
                Command::SpawnEntity { entity } => match entity {
                    EntityObject::Player(_) => unimplemented!(),
                    EntityObject::Minion(minion) => self.minions.push(minion),
                    EntityObject::Enemy(enemy) => self.enemies.push(enemy),
                },
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
