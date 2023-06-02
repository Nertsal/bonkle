use geng::prelude::*;

use crate::model::{BodyAI, Coord, Shape};

#[derive(geng::Load, Serialize, Deserialize, Debug, Clone)]
#[load(serde = "toml")]
pub struct Config {
    pub arena: ArenaConfig,
    pub player: PlayerConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ArenaConfig {
    pub size: vec2<Coord>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PlayerConfig {
    pub body: BodyConfig,
    pub head: BodyConfig,
    pub orbit_distance: Coord,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct BodyConfig {
    pub shape: Shape,
    pub mass: Coord,
    pub speed: Coord,
    pub ai: Option<BodyAI>,
    pub acceleration: Coord,
    pub deceleration: Coord,
}
