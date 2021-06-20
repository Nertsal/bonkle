use super::*;

impl Renderer {
    pub fn draw(&self, model: &Model) {
        clear_background(BACKGROUND_COLOR);
        self.draw_game(model);
        self.draw_ui();
    }

    fn draw_game(&self, model: &Model) {
        set_camera(&self.game_camera);

        // Area effects
        for area_effect in &model.area_effects {
            let area_color = match &area_effect.effect {
                Effect::Heal { .. } => Color::new(0.0, 1.0, 0.0, 0.5),
            };
            draw_circle(
                area_effect.position.x,
                area_effect.position.y,
                area_effect.radius,
                area_color,
            );
        }

        // Player health
        let coefficient = model.player.entity.health.hp_frac();
        let player_life_color = color_alpha(PLAYER_LIFE_COLOR, 0.5);
        draw_circle(
            model.player.entity.rigidbody.position.x,
            model.player.entity.rigidbody.position.y,
            model.player.chain_length * coefficient,
            player_life_color,
        );

        // Spawners
        let spawner_color = Color::new(SPAWNER_COLOR.r, SPAWNER_COLOR.g, SPAWNER_COLOR.b, 0.5);
        for spawner in &model.spawners {
            draw_circle_lines(
                spawner.position.x,
                spawner.position.y,
                spawner.spawn_group.radius,
                0.2,
                spawner_color,
            );
            draw_circle(
                spawner.position.x,
                spawner.position.y,
                spawner.time_left / spawner.time_left_max * spawner.spawn_group.radius,
                spawner_color,
            );
        }

        // Particles
        for particle in &model.particles {
            self.draw_rigidbody(&particle.rigidbody, particle.color);
        }

        // Other entities
        for enemy in model.entities() {
            self.draw_rigidbody(&enemy.entity().rigidbody, enemy.entity().color);
            let health_frac = enemy.health_frac();
            draw_circle(
                enemy.entity().rigidbody.position.x,
                enemy.entity().rigidbody.position.y,
                health_frac * enemy.entity().rigidbody.collider.radius,
                enemy.entity().color,
            );
        }

        // Player border
        draw_circle_lines(
            model.player.entity.rigidbody.position.x,
            model.player.entity.rigidbody.position.y,
            model.player.chain_length,
            0.2,
            PLAYER_BORDER_COLOR,
        );

        // Player body & head
        self.draw_rigidbody(&model.player.entity.rigidbody, PLAYER_COLOR);
        self.draw_rigidbody(&model.player.head, PLAYER_COLOR);

        // Bounds
        let bounds_size = model.bounds.max - model.bounds.min;
        draw_rectangle_lines(
            model.bounds.min.x,
            model.bounds.min.y,
            bounds_size.x,
            bounds_size.y,
            0.5,
            BORDER_COLOR,
        );
    }

    fn draw_rigidbody(&self, rigidbody: &RigidBody, color: Color) {
        draw_circle_lines(
            rigidbody.position.x,
            rigidbody.position.y,
            rigidbody.collider.radius,
            0.5,
            color,
        );
    }

    fn draw_ui(&self) {
        set_default_camera();
        self.ui_state.draw();
    }
}

fn color_alpha(color: Color, alpha: f32) -> Color {
    Color::new(color.r, color.g, color.b, alpha)
}
