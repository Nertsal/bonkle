use super::*;

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

    pub fn process_corpses(&mut self) {
        #[derive(StructQuery)]
        struct CorpseRef<'a> {
            lifetime: &'a mut Health,
        }

        let mut query = query_corpse_ref!(self.model.corpses);
        let mut iter = query.iter_mut();
        let mut deaths = Vec::new();
        while let Some((id, corpse)) = iter.next() {
            corpse.lifetime.damage(self.delta_time);
            if corpse.lifetime.is_dead() {
                deaths.push(id);
            }
        }

        for id in deaths {
            self.model.corpses.remove(id);
        }
    }
}
