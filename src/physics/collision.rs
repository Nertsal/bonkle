use crate::{
    components::{BonkleBody, Transform},
    resources::Bounds,
};

use super::*;

#[derive(Debug, Clone)]
pub struct Collision {
    pub normal: Vec2,
    pub penetration: f32,
}

#[derive(Debug, Clone)]
pub struct HitInfo {
    pub contact: Vec2,
    pub hit_self: f32,
    pub hit_other: f32,
}

pub fn collision_info(
    body: &BonkleBody,
    transform: &Transform,
    other: &BonkleBody,
    other_transform: &Transform,
) -> Option<(Collision, HitInfo)> {
    collision(body, transform, other, other_transform).map(|collision| {
        // self.position += collision.normal * collision.penetration;
        let relative_velocity = body.velocity - other.velocity;
        let hit_strength = collision.normal.dot(relative_velocity).abs();
        let hit_self = hit_strength * other.mass / body.mass;
        let hit_other = hit_strength * body.mass / other.mass;
        let contact =
            other_transform.position + collision.normal * (other.radius - collision.penetration);
        (
            collision,
            HitInfo {
                contact,
                hit_self,
                hit_other,
            },
        )
    })
}

pub fn collision(
    body: &BonkleBody,
    transform: &Transform,
    other: &BonkleBody,
    other_transform: &Transform,
) -> Option<Collision> {
    let offset = transform.position - other_transform.position;
    let penetration = body.radius + other.radius - offset.length();
    if penetration >= 0.0 {
        Some(Collision {
            normal: offset.normalize(),
            penetration,
        })
    } else {
        None
    }
}

pub fn clamp_bounds(body: &BonkleBody, transform: &mut Transform, bounds: &Bounds) {
    let size = vec2(body.radius, body.radius);
    transform.position = transform
        .position
        .clamp(bounds.min + size, bounds.max - size);
}

pub fn bounce_bounds(body: &mut BonkleBody, transform: &mut Transform, bounds: &Bounds) -> bool {
    let size = vec2(body.radius, body.radius);
    let min = transform.position - size;
    let max = transform.position + size;
    let mut bounce = false;
    if min.x < bounds.min.x || max.x > bounds.max.x {
        body.velocity.x *= -body.physics_material.bounciness;
        bounce = true;
    }
    if min.y < bounds.min.y || max.y > bounds.max.y {
        body.velocity.y *= -body.physics_material.bounciness;
        bounce = true;
    }
    clamp_bounds(body, transform, bounds);
    bounce
}
