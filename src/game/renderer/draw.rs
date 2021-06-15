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

        // Player health
        let coefficient = (model.player.health / model.player.max_health).max(0.0);
        let player_life_color = Color::new(
            PLAYER_LIFE_COLOR.r,
            PLAYER_LIFE_COLOR.g,
            PLAYER_LIFE_COLOR.b,
            0.5,
        );
        draw_circle(
            model.player.body.position.x,
            model.player.body.position.y,
            model.player.chain_length * coefficient,
            player_life_color,
        );

        // Particles
        for particle in &model.particles {
            self.draw_rigidbody(&particle.rigidbody, particle.color);
        }

        // Enemies
        for enemy in &model.enemies {
            self.draw_rigidbody(&enemy.rigidbody, enemy.color);
        }

        // Player border
        draw_circle_lines(
            model.player.body.position.x,
            model.player.body.position.y,
            model.player.chain_length,
            0.2,
            PLAYER_BORDER_COLOR,
        );

        // Player body & head
        self.draw_rigidbody(&model.player.body, PLAYER_COLOR);
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
        draw_circle(
            rigidbody.position.x,
            rigidbody.position.y,
            rigidbody.collider.radius,
            color,
        );
    }

    fn draw_ui(&self) {
        set_default_camera();
        self.ui_state.draw();
    }
}
