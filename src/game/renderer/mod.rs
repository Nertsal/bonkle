use super::*;

mod elements;
mod ui_state;

use elements::*;
use ui_state::*;

const STAGE_SHOW_TIME: f32 = 2.0;
const DEFAULT_WIDTH: f32 = 800.0;
const DEFAULT_HEIGHT: f32 = 600.0;

pub struct Renderer {
    pub game_camera: Camera2D,
    ui_state: UIState,
}

impl Renderer {
    pub fn new(assets: &Rc<Assets>) -> Self {
        Self {
            game_camera: Camera2D::default(),
            ui_state: UIState {
                paused: true,
                debug_mode: false,
                player_alive: true,
                stage: 0,
                stage_timer: 0.0,
                tutorial_texture: TextureElement::new(
                    assets.tutorial,
                    WHITE,
                    Some(vec2(160.0, 40.0)),
                    UIObject::new(vec2(0.5, 0.0), vec2(-80.0, 60.0), UIScaleMode::KeepRatio),
                ),
                fps_element: FPSElement::new(
                    0.5,
                    TextElement::new(
                        "".to_owned(),
                        20.0,
                        WHITE,
                        UIObject::new(vec2(0.0, 0.0), vec2(10.0, 20.0), UIScaleMode::KeepRatio),
                    ),
                ),
                stage_element: TextElement::new(
                    "".to_owned(),
                    50.0,
                    WHITE,
                    UIObject::new(vec2(0.5, 0.0), vec2(-75.0, 100.0), UIScaleMode::KeepRatio),
                ),
                death_element: TextElement::new(
                    "YOU DIED".to_owned(),
                    50.0,
                    WHITE,
                    UIObject::new(vec2(0.5, 0.5), vec2(-85.0, 0.0), UIScaleMode::KeepRatio),
                ),
                reset_element: TextElement::new(
                    "PRESS R TO RESET".to_owned(),
                    50.0,
                    WHITE,
                    UIObject::new(vec2(0.5, 0.5), vec2(-165.0, 50.0), UIScaleMode::KeepRatio),
                ),
            },
        }
    }

    pub fn update(&mut self, delta_time: f32, paused: bool, model: &Model) {
        let zoom = 0.0055;
        self.game_camera = Camera2D {
            offset: vec2(0.0, 0.0),
            zoom: vec2(zoom, zoom * screen_width() / screen_height()),
            ..Default::default()
        };

        self.ui_state.paused = paused;
        self.ui_state.player_alive = model.player.health > 0.0;

        self.ui_state.update(delta_time);
    }

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

    pub fn next_wave(&mut self, stage: usize) {
        self.ui_state.stage = stage;
        self.ui_state.stage_timer = STAGE_SHOW_TIME;
    }
}
