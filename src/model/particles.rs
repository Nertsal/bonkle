use crate::util::RealConversions;

use super::*;

#[derive(StructOf, Debug, Clone)]
pub struct Particle {
    pub name: String,
    pub collider: Collider,
    pub velocity: vec2<Coord>,
    pub lifetime: Health,
}

#[derive(Debug, Clone)]
pub struct ParticlesSpawn {
    pub name: String,
    /// Controls the number of particles spawned.
    /// The number is not specified directly to allow settings to lower the total amount of particles.
    pub intensity: R32,
    pub position: vec2<Coord>,
    pub angle: std::ops::RangeInclusive<Angle<R32>>,
    pub speed: std::ops::RangeInclusive<Coord>,
}

impl ParticlesSpawn {
    pub fn new(name: impl Into<String>, intensity: impl Float, position: vec2<Coord>) -> Self {
        Self {
            name: name.into(),
            intensity: intensity.as_r32(),
            position,
            angle: Angle::ZERO..=Angle::from_degrees(r32(360.0)),
            speed: r32(5.0)..=r32(10.0),
        }
    }

    pub fn directed(self, angle: Angle<R32>, range: Angle<R32>) -> Self {
        let range = range / r32(2.0);
        self.with_angle((angle - range)..=(angle + range))
    }

    pub fn with_angle(self, range: std::ops::RangeInclusive<Angle<R32>>) -> Self {
        Self {
            angle: range,
            ..self
        }
    }

    pub fn sped(self, speed: Coord, range: Coord) -> Self {
        let range = range / r32(2.0);
        self.with_speed((speed - range)..=(speed + range))
    }

    pub fn with_speed(self, range: std::ops::RangeInclusive<Coord>) -> Self {
        Self {
            speed: range,
            ..self
        }
    }
}
