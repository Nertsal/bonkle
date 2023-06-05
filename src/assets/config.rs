#![allow(clippy::needless_question_mark)] // `#[derive(geng::asset::Load)]` gives that idk why

use crate::model::{BodyAI, Color, Coord, Hp, Shape};

use geng::prelude::*;

#[derive(geng::asset::Load, Serialize, Deserialize, Debug, Clone)]
#[load(serde = "ron")]
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
    pub health: Option<Hp>,
    pub ai: Option<BodyAI>,
    pub acceleration: Coord,
    pub deceleration: Coord,
}

#[derive(geng::asset::Load, Serialize, Deserialize, Debug, Clone)]
#[load(serde = "toml")]
pub struct ColorTheme {
    pub background: Color,
    pub border: Color,
    pub player: Color,
    pub entities: HashMap<String, Color>,
}

impl ColorTheme {
    pub fn get_entity(&self, name: impl AsRef<str>) -> Option<Color> {
        match name.as_ref() {
            "player" => Some(self.player),
            name => self.entities.get(name).copied(),
        }
    }
}
