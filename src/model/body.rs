use crate::config::BodyConfig;

use super::*;

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
    pub attachment: Option<BodyAttachment>,
}

#[derive(StructOf, Debug, Clone)]
pub struct BodyController {
    pub target_velocity: vec2<Coord>,
    pub acceleration: Coord,
}

#[derive(Debug, Clone)]
pub struct BodyAttachment {
    pub to_body: Id,
    pub ty: AttachmentType,
}

#[derive(Debug, Clone)]
pub enum AttachmentType {
    Orbit { distance: Coord },
}

impl BonkleBody {
    pub fn new(config: BodyConfig, position: vec2<Coord>) -> Self {
        Self {
            position,
            velocity: vec2::ZERO,
            radius: config.radius,
            mass: config.mass,
            speed: config.speed,
            controller: Some(BodyController {
                target_velocity: vec2::ZERO,
                acceleration: config.acceleration,
            }),
            attachment: None,
        }
    }
}
