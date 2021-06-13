use super::*;

const STAGE_SHOW_TIME: f32 = 2.0;

pub struct Renderer {
    assets: Rc<Assets>,
    pub game_camera: Camera2D,
    ui_scale: Vec2,
    current_fps: f32,
    fps_update_time: f32,
    fps_update: f32,
    debug_mode: bool,
    player_life_color: Color,
    stage: usize,
    stage_timer: f32,
}

impl Renderer {
    pub fn new(assets: &Rc<Assets>) -> Self {
        Self {
            assets: assets.clone(),
            game_camera: Camera2D::default(),
            ui_scale: vec2(1.0, 1.0),
            current_fps: 0.0,
            fps_update_time: 0.5,
            fps_update: 0.0,
            debug_mode: false,
            player_life_color: PLAYER_LIFE_COLOR,
            stage: 0,
            stage_timer: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if is_key_pressed(KeyCode::F6) {
            self.debug_mode = !self.debug_mode;
        }

        self.fps_update -= delta_time;
        if self.fps_update <= 0.0 {
            self.fps_update += self.fps_update_time;
            self.current_fps = 1.0 / delta_time;
        }

        if self.stage_timer > 0.0 {
            self.stage_timer -= delta_time;
        }
    }

    pub fn draw(&mut self, model: &Model, paused: bool) {
        clear_background(BACKGROUND_COLOR);
        self.draw_game(model);
        self.draw_ui(model, paused);
    }

    fn draw_game(&mut self, model: &Model) {
        let zoom = 0.0055;
        self.game_camera = Camera2D {
            offset: vec2(0.0, 0.0),
            zoom: vec2(zoom, zoom * screen_width() / screen_height()),
            ..Default::default()
        };
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
        for spawner in &model.spawners {
            draw_circle_lines(
                spawner.position.x,
                spawner.position.y,
                spawner.spawn_group.radius,
                0.2,
                SPAWNER_COLOR,
            );
        }

        // Player health
        let coefficient = (model.player.health / model.player.max_health).max(0.0);
        self.player_life_color = Color::new(
            PLAYER_LIFE_COLOR.r,
            PLAYER_LIFE_COLOR.g,
            PLAYER_LIFE_COLOR.b,
            0.5,
        );
        draw_circle(
            model.player.body.position.x,
            model.player.body.position.y,
            model.player.chain_length * coefficient,
            self.player_life_color,
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

    fn draw_ui(&mut self, model: &Model, paused: bool) {
        set_default_camera();
        self.ui_scale = vec2(screen_width() / 800.0, screen_height() / 600.0);

        if paused {
            draw_texture_ex(
                self.assets.tutorial,
                screen_width() / 2.0 - 80.0 * self.ui_scale.x,
                60.0 * self.ui_scale.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(160.0, 40.0) * self.ui_scale),
                    ..Default::default()
                },
            );
        }

        if self.debug_mode {
            draw_text(
                &format!("FPS: {:.0}", self.current_fps),
                10.0 * self.ui_scale.x,
                20.0 * self.ui_scale.y,
                20.0 * self.ui_scale(),
                WHITE,
            );
        }

        if model.player.health <= 0.0 {
            draw_text(
                &format!("STAGE {}", self.stage),
                screen_width() / 2.0 - 75.0 * self.ui_scale.x,
                screen_height() / 2.0 - 50.0 * self.ui_scale.y,
                50.0 * self.ui_scale(),
                WHITE,
            );
            draw_text(
                "YOU DIED",
                screen_width() / 2.0 - 75.0 * self.ui_scale.x,
                screen_height() / 2.0,
                50.0 * self.ui_scale(),
                WHITE,
            );
            draw_text(
                "PRESS R TO RESET",
                screen_width() / 2.0 - 150.0 * self.ui_scale.x,
                screen_height() / 2.0 + 50.0 * self.ui_scale.y,
                50.0 * self.ui_scale(),
                WHITE,
            );
        }

        if self.stage_timer > 0.0 {
            draw_text(
                &format!("STAGE {}", self.stage),
                screen_width() / 2.0 - 60.0 * self.ui_scale.x,
                100.0 * self.ui_scale.y,
                40.0 * self.ui_scale(),
                WHITE,
            );
        }
    }

    pub fn next_wave(&mut self, stage: usize) {
        self.stage = stage;
        self.stage_timer = STAGE_SHOW_TIME;
    }

    fn ui_scale(&self) -> f32 {
        self.ui_scale.x.min(self.ui_scale.y)
    }
}
