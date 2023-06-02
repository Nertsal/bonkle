use geng::prelude::*;

use crate::model::{Coord, Shape};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub acceleration: Coord,
    pub deceleration: Coord,
}

impl Config {
    pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let s = file::load_string(path)
            .await
            .context("failed to open config file")?;
        ron::from_str(&s).context("failed to parse config")
    }
}
