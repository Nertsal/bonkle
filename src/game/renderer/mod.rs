use super::*;

const STAGE_SHOW_TIME: f32 = 2.0;

pub struct Renderer {
    assets: Rc<Assets>,
    pub game_camera: Camera2D,
    current_fps: f32,
    fps_update_time: f32,
    fps_update: f32,
    debug_mode: bool,
    player_life_color: Color,
    stage: Option<usize>,
    stage_timer: f32,
}

impl Renderer {
    pub fn new(assets: &Rc<Assets>) -> Self {
        Self {
            assets: assets.clone(),
            game_camera: Camera2D::default(),
            current_fps: 0.0,
            fps_update_time: 0.5,
            fps_update: 0.0,
            debug_mode: false,
            player_life_color: PLAYER_LIFE_COLOR,
            stage: None,
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

        if self.stage.is_some() {
            self.stage_timer -= delta_time;
            if self.stage_timer <= 0.0 {
                self.stage = None;
            }
        }
    }

    pub fn draw(&mut self, model: &Model) {
        clear_background(BACKGROUND_COLOR);
        self.draw_game(model);
        self.draw_ui(model);
    }

    fn draw_game(&mut self, model: &Model) {
        let zoom = 0.005;
        self.game_camera = Camera2D {
            offset: vec2(0.0, 0.0),
            zoom: vec2(zoom, zoom * screen_width() / screen_height()),
            ..Default::default()
        };
        set_camera(&self.game_camera);

        for particle in &model.particles {
            self.draw_rigidbody(&particle.rigidbody, particle.color);
        }

        self.draw_rigidbody(&model.player.body, PLAYER_COLOR);
        let coefficient = model.player.health / model.player.max_health;
        self.player_life_color = Color::new(
            coefficient * PLAYER_LIFE_COLOR.r,
            coefficient * PLAYER_LIFE_COLOR.g,
            coefficient * PLAYER_LIFE_COLOR.b,
            0.5,
        );
        draw_circle(
            model.player.body.position.x,
            model.player.body.position.y,
            model.player.chain_length,
            self.player_life_color,
        );
        draw_circle_lines(
            model.player.body.position.x,
            model.player.body.position.y,
            model.player.chain_length,
            0.2,
            PLAYER_BORDER_COLOR,
        );
        self.draw_rigidbody(&model.player.head, PLAYER_COLOR);
        for enemy in &model.enemies {
            self.draw_rigidbody(&enemy.rigidbody, enemy.color);
        }

        for spawner in &model.spawners {
            draw_circle_lines(
                spawner.position.x,
                spawner.position.y,
                spawner.spawn_group.radius,
                0.2,
                SPAWNER_COLOR,
            );
        }

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

    fn draw_ui(&self, model: &Model) {
        set_default_camera();

        if model.game_start_timer > 0.0 {
            draw_texture_ex(
                self.assets.tutorial,
                screen_width() / 2.0 - 80.0,
                100.0 - 40.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(160.0, 40.0)),
                    ..Default::default()
                },
            );
        }

        set_default_camera();

        if self.debug_mode {
            draw_text(
                &format!("FPS: {:.0}", self.current_fps),
                10.0,
                20.0,
                20.0,
                WHITE,
            );
        }

        if model.player.health <= 0.0 {
            draw_text(
                "YOU DIED",
                screen_width() / 2.0 - 75.0,
                screen_height() / 2.0,
                50.0,
                WHITE,
            );
            draw_text(
                "PRESS R TO RESET",
                screen_width() / 2.0 - 150.0,
                screen_height() / 2.0 + 50.0,
                50.0,
                WHITE,
            );
        }

        if let Some(stage) = self.stage {
            draw_text(
                &format!("STAGE {}", stage),
                screen_width() / 2.0 - 60.0,
                100.0,
                40.0,
                WHITE,
            );
        }
    }

    pub fn next_wave(&mut self, stage: usize) {
        self.stage = Some(stage);
        self.stage_timer = STAGE_SHOW_TIME;
    }
}
