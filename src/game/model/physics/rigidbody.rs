use super::*;

#[derive(Debug, Clone)]
pub struct RigidBody {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
    pub collider: Collider,
    pub physics_material: PhysicsMaterial,
}

impl RigidBody {
    pub fn new(
        position: Vec2,
        mass: f32,
        collider: Collider,
        physics_material: PhysicsMaterial,
    ) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            mass,
            collider,
            physics_material,
        }
    }

    pub fn movement(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
    }

    pub fn drag(&mut self, delta_time: f32) {
        self.velocity *= 1.0 - self.physics_material.drag * delta_time;
    }

    pub fn collide(&self, other: &Self) -> Option<Collision> {
        let offset = self.position - other.position;
        let penetration = self.collider.radius + other.collider.radius - offset.length();
        if penetration >= 0.0 {
            Some(Collision {
                normal: offset.normalize(),
                penetration,
            })
        } else {
            None
        }
    }

    pub fn clamp_bounds(&mut self, bounds: &Bounds) {
        let size = vec2(self.collider.radius, self.collider.radius);
        self.position = self.position.clamp(bounds.min + size, bounds.max - size);
    }

    pub fn bounce_bounds(&mut self, bounds: &Bounds) -> bool {
        let size = vec2(self.collider.radius, self.collider.radius);
        let min = self.position - size;
        let max = self.position + size;
        let mut bounce = false;
        if min.x < bounds.min.x || max.x > bounds.max.x {
            self.velocity.x *= -self.physics_material.bounciness;
            bounce = true;
        }
        if min.y < bounds.min.y || max.y > bounds.max.y {
            self.velocity.y *= -self.physics_material.bounciness;
            bounce = true;
        }
        self.clamp_bounds(bounds);
        bounce
    }
}
