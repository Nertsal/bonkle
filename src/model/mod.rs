mod logic;

use self::logic::Logic;

use crate::{
    collection::{Collection, Id},
    config::Config,
    util::Vec2RealConversions,
};

use ecs::prelude::*;
use geng::prelude::*;

pub type Color = Rgba<f32>;
pub type Time = R32;
pub type Coord = R32;
pub type Mass = R32;
pub type Bounds = Aabb2<Coord>;

#[derive(Debug, Clone)]
pub struct Player {
    pub body: Id,
}

#[derive(Debug, Clone)]
pub struct PlayerInput {
    pub target_move_dir: vec2<Coord>,
}

#[derive(StructOf, Debug, Clone)]
pub struct BonkleBody {
    pub position: vec2<Coord>,
    pub velocity: vec2<Coord>,
    pub radius: Coord,
    pub mass: Mass,
    pub speed: Coord,
    // TODO: #[structof(flatten)] or smth
    pub controller: Option<BodyController>,
    // pub material: PhysicsMaterial, // TODO
}

#[derive(StructOf, Debug, Clone)]
pub struct BodyController {
    pub target_velocity: vec2<Coord>,
    pub acceleration: Coord,
}

pub struct Model {
    pub current_stage: usize,
    pub camera: Camera2d,
    pub bounds: Bounds,
    // pub spawn_bounds: Bounds,
    pub player: Player,
    pub bodies: StructOf<Collection<BonkleBody>>,
    // pub enemies: Vec<Box<dyn EntityObject>>,
    // pub minions: Vec<Box<dyn EntityObject>>,
    // pub particles: Vec<Particle>,
    // pub area_effects: Vec<AreaEffect>,
    // pub spawners: Vec<Spawner>,
}

impl Model {
    pub fn new(config: Config) -> Self {
        let mut bodies = StructOf::<Collection<BonkleBody>>::new();
        let player_body = bodies.insert(BonkleBody {
            position: vec2::ZERO,
            velocity: vec2::ZERO,
            radius: config.player.radius,
            mass: config.player.mass,
            speed: config.player.speed,
            controller: Some(BodyController {
                target_velocity: vec2::ZERO,
                acceleration: config.player.acceleration,
            }),
        });

        Self {
            current_stage: 0,
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: 0.0,
                fov: 50.0,
            },
            bounds: Bounds {
                min: vec2(-160.0, -90.0).as_r32(),
                max: vec2(160.0, 90.0).as_r32(),
            },
            player: Player { body: player_body },
            bodies,
        }
    }

    pub fn update(&mut self, player_input: PlayerInput, delta_time: Time) {
        let mut logic = Logic {
            model: self,
            player_input,
            delta_time,
        };
        logic.process();
    }
}
