use super::*;

pub struct Particle {
    pub rigidbody: RigidBody,
    pub color: Color,
    pub lifetime: Health,
}

impl Model {
    pub fn spawn_particles_hit(&mut self, position: Vec2, damage: f32, color: Color) {
        let particles_count = macroquad::rand::gen_range(1, (damage / 10.0).min(50.0) as usize);
        for _ in 0..particles_count {
            let direction = Self::get_random_direction();
            let velocity = macroquad::rand::gen_range(10.0, 30.0);
            let velocity = direction * velocity;
            self.particles.push(Particle {
                rigidbody: RigidBody {
                    position,
                    velocity,
                    mass: 1.0,
                    collider: Collider::new(1.0),
                },
                color,
                lifetime: Health::new(PARTICLE_LIFETIME),
            })
        }
    }

    pub fn get_random_direction() -> Vec2 {
        let angle = macroquad::rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let (sin, cos) = angle.sin_cos();
        vec2(cos, sin)
    }
}
