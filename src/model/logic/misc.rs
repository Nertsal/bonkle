use super::*;

use crate::util::RealConversions;

impl Logic<'_> {
    pub fn check_deaths(&mut self) {
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            #[query(optic = "._Some")]
            health: &'a Health,
        }

        let deaths: Vec<Id> = query_body_ref!(self.model.bodies)
            .iter()
            .filter(|(_, body)| body.health.is_dead())
            .map(|(id, _)| id)
            .collect();
        for body_id in deaths {
            let body = self.model.bodies.remove(body_id).unwrap();
            self.model.corpses.insert(BodyCorpse {
                body,
                lifetime: Health::new(r32(1.0)),
            });
            // TODO: particles
        }
    }

    pub fn update_lifetimes(&mut self) {
        #[derive(StructQuery)]
        struct ItemRef<'a> {
            lifetime: &'a mut Health,
        }

        macro_rules! process {
            ($storage:expr) => {
                let mut query = query_item_ref!($storage);
                let mut iter = query.iter_mut();
                let mut deaths = Vec::new();
                while let Some((id, item)) = iter.next() {
                    item.lifetime.damage(self.delta_time);
                    if item.lifetime.is_dead() {
                        deaths.push(id);
                    }
                }
                for id in deaths {
                    $storage.remove(id);
                }
            };
        }

        process!(self.model.corpses);
        process!(self.model.particles);
    }

    pub fn spawn_particles_hit(&mut self, config: ParticlesSpawn) {
        let mut rng = thread_rng();
        let particles_count =
            rng.gen_range(3..(config.intensity.as_f32() / 10.0).clamp(4.0, 50.0) as usize);
        for _ in 0..particles_count {
            let angle =
                rng.gen_range(config.angle.start().as_radians()..=config.angle.end().as_radians());
            let direction = Angle::from_radians(angle).unit_vec().as_r32();
            let speed = rng.gen_range(*config.speed.start()..*config.speed.end());
            let velocity = direction * speed;
            self.model.particles.insert(Particle {
                name: config.name.clone(),
                collider: Collider::new(
                    config.position,
                    Shape::Circle {
                        radius: 0.3.as_r32(),
                    },
                ),
                velocity,
                lifetime: Health::new(1.0.as_r32()),
            });
        }
    }
}
