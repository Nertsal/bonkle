mod body;
mod collider;
mod health;
mod logic;
mod shape;

pub use self::body::*;
pub use self::collider::*;
pub use self::health::*;
use self::logic::Logic;
pub use self::shape::*;

use crate::assets::EntitiesAssets;
use crate::{assets::Config, util::Vec2RealConversions};

use ecs::{
    arena::{Arena, Index},
    prelude::*,
};
use geng::prelude::*;

pub type Id = Index;
pub type Color = Rgba<f32>;
pub type Time = R32;
pub type Coord = R32;
pub type Mass = R32;
pub type Bounds = Aabb2<Coord>;

#[derive(Debug, Clone)]
pub struct Player {
    pub body: Id,
    pub head: Id,
}

#[derive(Debug, Clone)]
pub struct PlayerInput {
    pub target_move_dir: vec2<Coord>,
    pub head_target: RotationTarget,
}

#[derive(Debug, Clone)]
pub enum RotationTarget {
    Relative { delta: Angle<R32> },
    LookAt { position: vec2<Coord> },
}

impl RotationTarget {
    pub fn get_target(&self, origin: vec2<R32>, angle: Angle<R32>) -> Angle<R32> {
        match self {
            RotationTarget::Relative { delta } => (angle + *delta).normalized_pi(),
            RotationTarget::LookAt { position } => {
                let delta = *position - origin;
                Angle::from_radians(delta.arg())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum BodyAI {
    Crawler,
}

#[derive(StructOf, Debug, Clone)]
pub struct Particle {
    pub collider: Collider,
    pub velocity: vec2<Coord>,
    pub lifetime: Health,
}

pub struct Model {
    pub config: Config,
    pub entities: EntitiesAssets,
    pub current_stage: usize,
    pub camera: Camera2d,
    pub bounds: Bounds,
    // pub spawn_bounds: Bounds,
    pub player: Player,
    pub bodies: StructOf<Arena<BonkleBody>>,
    pub corpses: StructOf<Arena<BodyCorpse>>,
    // pub enemies: Vec<Box<dyn EntityObject>>,
    // pub minions: Vec<Box<dyn EntityObject>>,
    pub particles: StructOf<Arena<Particle>>,
    // pub area_effects: Vec<AreaEffect>,
    // pub spawners: Vec<Spawner>,
}

impl Model {
    pub fn new(config: Config, entities: EntitiesAssets) -> Self {
        let mut bodies = StructOf::<Arena<BonkleBody>>::new();

        let player_body = bodies.insert(BonkleBody::new(config.player.body, vec2::ZERO));

        let mut player_head = BonkleBody::new(
            config.player.head,
            vec2::UNIT_Y * config.player.orbit_distance,
        );
        player_head.attachment = Some(BodyAttachment {
            to_body: player_body,
            ty: AttachmentType::Orbit {
                distance: config.player.orbit_distance,
            },
        });
        let player_head = bodies.insert(player_head);

        bodies.insert(BonkleBody::new(
            entities.configs["crawler"],
            vec2(20.0, 0.0).as_r32(),
        ));

        Self {
            current_stage: 0,
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: 0.0,
                fov: 100.0,
            },
            bounds: Bounds {
                min: -config.arena.size / r32(2.0),
                max: config.arena.size / r32(2.0),
            },
            player: Player {
                body: player_body,
                head: player_head,
            },
            bodies,
            corpses: StructOf::new(),
            particles: StructOf::new(),
            config,
            entities,
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
