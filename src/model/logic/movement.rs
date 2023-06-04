use super::*;

impl Logic<'_> {
    /// Move bodies, corpses, and particles according to their velocity.
    pub fn movement(&mut self) {
        macro_rules! process {
            ($storage:expr) => {
                let mut query = query_move_ref!($storage);
                let mut iter = query.iter_mut();
                while let Some((_, item)) = iter.next() {
                    item.collider.position += *item.velocity * self.delta_time;
                }
            };
        }

        {
            #[derive(StructQuery)]
            struct MoveRef<'a> {
                collider: &'a mut Collider,
                velocity: &'a vec2<Coord>,
            }
            process!(self.model.bodies);
            process!(self.model.particles);
        }
        {
            #[derive(StructQuery)]
            struct MoveRef<'a> {
                #[query(nested = ".body")]
                collider: &'a mut Collider,
                #[query(nested = ".body")]
                velocity: &'a vec2<Coord>,
            }
            process!(self.model.corpses);
        }
    }

    /// Correct bodies' attachments by moving them in a position that satifies the constraint.
    pub fn body_attachment(&mut self) {
        // Collect corrections
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            collider: &'a Collider,
            velocity: &'a vec2<Coord>,
            attachment: &'a Option<BodyAttachment>,
        }

        struct Correction {
            position: vec2<Coord>,
            velocity: vec2<Coord>,
        }

        let mut corrections = HashMap::<Id, Correction>::new();
        let query = query_body_ref!(self.model.bodies);
        for (body_id, body) in &query {
            if let Some(attachment) = body.attachment {
                if let Some(to_body) = query.get(attachment.to_body) {
                    match attachment.ty {
                        AttachmentType::Orbit { distance } => {
                            let angle = Angle::from_radians(
                                (body.collider.position - to_body.collider.position).arg(),
                            );
                            let position = to_body.collider.position + angle.unit_vec() * distance;

                            // Preserve speed - change direction
                            let speed = body.velocity.len();
                            let dir = angle.unit_vec().rotate_90();
                            let velocity = dir * speed * vec2::dot(dir, *body.velocity).signum();

                            corrections.insert(body_id, Correction { position, velocity });
                        }
                    }
                } else {
                    // Body attachment not found
                    // TODO: idk
                }
            }
        }

        // Apply corrections
        #[derive(StructQuery)]
        struct CorrectionRef<'a> {
            #[query(optic = ".collider._get", component = "Collider")]
            position: &'a mut vec2<Coord>,
            velocity: &'a mut vec2<Coord>,
        }

        let mut query = query_correction_ref!(self.model.bodies);
        for (body, correction) in corrections {
            let body = query.get_mut(body).unwrap(); // Body guaranteed to be valid
            *body.position = correction.position;
            *body.velocity = correction.velocity;
        }
    }
}
