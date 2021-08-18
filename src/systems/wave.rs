use macroquad::prelude::{vec2, Vec2};
use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

use crate::{
    actors::{EnemyMelee, EnemyRanger},
    components::{
        ActorComponent, BonkleBody, BonkleBodyController, Collidable, ColorComponent,
        EnemyCollider, EnemyTarget, Health, SpawnGroup, Spawner, Transform,
    },
    constants::{
        BOUNCINESS, DRAG, GROUP_SPAWN_TIME, MELEE_ACCELERATION, MELEE_COLOR, MELEE_SPEED,
        RANGER_ACCELERATION, RANGER_COLOR, RANGER_SPEED,
    },
    physics::PhysicsMaterial,
    resources::{Bounds, Time},
};

pub struct WaveSystem {
    pub current_stage: usize,
}

impl<'s> System<'s> for WaveSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, EnemyCollider>,
        WriteStorage<'s, Spawner>,
        WriteStorage<'s, Transform>,
        Read<'s, Bounds>,
    );

    fn run(
        &mut self,
        (entities, enemies, mut spawners, mut transforms, spawn_bounds): Self::SystemData,
    ) {
        if enemies.is_empty() && spawners.is_empty() {
            self.current_stage += 1;
            let max_groups = (self.current_stage as f32).sqrt().floor() as usize;
            let groups_count =
                macroquad::rand::gen_range(max_groups.max(2) - 1, max_groups.max(1) + 1);
            for _ in 0..groups_count {
                let max_enemies = (self.current_stage as f32).sqrt().floor() as usize;
                let enemies_count =
                    macroquad::rand::gen_range(max_enemies.max(3) - 2, max_enemies.max(1) + 1);
                let group_position = get_random_position_bounds(&spawn_bounds);
                let group = SpawnGroup {
                    enemies_count,
                    radius: macroquad::rand::gen_range(10.0, 15.0),
                };
                entities
                    .build_entity()
                    .with(Spawner::new(GROUP_SPAWN_TIME, group), &mut spawners)
                    .with(
                        Transform {
                            position: group_position,
                        },
                        &mut transforms,
                    )
                    .build();
            }
        }
    }
}

pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Spawner>,
        WriteStorage<'s, EnemyTarget>,
        WriteStorage<'s, EnemyCollider>,
        WriteStorage<'s, ActorComponent>,
        WriteStorage<'s, Health>,
        WriteStorage<'s, ColorComponent>,
        WriteStorage<'s, Collidable>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, BonkleBody>,
        WriteStorage<'s, BonkleBodyController>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut spawners,
            mut enemy_targets,
            mut enemy_colliders,
            mut actors,
            mut healths,
            mut colors,
            mut collidables,
            mut transforms,
            mut bonkle_bodies,
            mut bonkle_body_controllers,
            time,
        ): Self::SystemData,
    ) {
        for (entity, spawner) in (&entities, &mut spawners).join() {
            spawner.spawn_time.change(-time.delta_time);
            if !spawner.spawn_time.is_alive() {
                enum SpawnEnemy {
                    Melee,
                    Ranger,
                }
                let weights = [(2.0, SpawnEnemy::Melee), (1.0, SpawnEnemy::Ranger)]; //, (0.5, &bomber)];
                let total_weight: f32 = weights.iter().map(|(weight, _)| weight).sum();
                for _ in 0..spawner.spawn_group.enemies_count {
                    let mut random = macroquad::rand::gen_range(0.0, 1.0);
                    let mut enemy = None;
                    for (weight, spawn_enemy) in &weights {
                        let chance = *weight / total_weight;
                        random -= chance;
                        if random <= 0.0 {
                            enemy = Some(spawn_enemy);
                            break;
                        }
                    }

                    if let Some(enemy) = enemy {
                        let position = transforms
                            .get(entity)
                            .map(|transform| transform.position)
                            .unwrap_or_default();
                        let position =
                            get_random_position_circle(position, spawner.spawn_group.radius);
                        let builder = entities
                            .build_entity()
                            .with(EnemyTarget, &mut enemy_targets)
                            .with(EnemyCollider, &mut enemy_colliders)
                            .with(Collidable, &mut collidables)
                            .with(Transform { position }, &mut transforms);

                        let builder = match enemy {
                            SpawnEnemy::Melee => builder
                                .with(
                                    ActorComponent(Box::new(EnemyMelee {
                                        movement_speed: MELEE_SPEED,
                                    })),
                                    &mut actors,
                                )
                                .with(Health::new(300.0), &mut healths)
                                .with(ColorComponent(MELEE_COLOR), &mut colors)
                                .with(
                                    BonkleBody {
                                        mass: 5.0,
                                        radius: 2.0,
                                        velocity: vec2(0.0, 0.0),
                                        physics_material: PhysicsMaterial::new(DRAG, BOUNCINESS),
                                    },
                                    &mut bonkle_bodies,
                                )
                                .with(
                                    BonkleBodyController {
                                        acceleration: MELEE_ACCELERATION,
                                        target_velocity: Vec2::ZERO,
                                    },
                                    &mut bonkle_body_controllers,
                                ),
                            SpawnEnemy::Ranger => builder
                                .with(
                                    ActorComponent(Box::new(EnemyRanger {
                                        movement_speed: RANGER_SPEED,
                                        attack_cooldown: Health::new(2.0),
                                    })),
                                    &mut actors,
                                )
                                .with(Health::new(300.0), &mut healths)
                                .with(ColorComponent(RANGER_COLOR), &mut colors)
                                .with(
                                    BonkleBody {
                                        mass: 5.0,
                                        radius: 2.0,
                                        velocity: vec2(0.0, 0.0),
                                        physics_material: PhysicsMaterial::new(DRAG, BOUNCINESS),
                                    },
                                    &mut bonkle_bodies,
                                )
                                .with(
                                    BonkleBodyController {
                                        acceleration: RANGER_ACCELERATION,
                                        target_velocity: Vec2::ZERO,
                                    },
                                    &mut bonkle_body_controllers,
                                ),
                        };

                        builder.build();
                    }
                }

                entities.delete(entity).unwrap();
            }
        }
    }
}

fn get_random_position_bounds(bounds: &Bounds) -> Vec2 {
    let x = macroquad::rand::gen_range(bounds.min.x, bounds.max.x);
    let y = macroquad::rand::gen_range(bounds.min.y, bounds.max.y);
    vec2(x, y)
}

fn get_random_position_circle(position: Vec2, radius: f32) -> Vec2 {
    let angle = macroquad::rand::gen_range(0.0, std::f32::consts::PI * 2.0);
    let distance = macroquad::rand::gen_range(0.0, 1.0);
    let (sin, cos) = angle.sin_cos();
    vec2(cos, sin) * distance * radius + position
}
