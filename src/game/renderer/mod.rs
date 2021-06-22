use super::*;

mod draw;
mod elements;
mod ui_state;

use elements::*;
use ui_state::*;

const STAGE_SHOW_TIME: f32 = 2.0;
const DEFAULT_WIDTH: f32 = 800.0;
const DEFAULT_HEIGHT: f32 = 600.0;

pub struct Renderer {
    pub game_camera: Camera2D,
    camera_zoom: f32,
    camera_zoom_speed: f32,
    ui_state: UIState,
}

impl Renderer {
    pub fn new(assets: &Rc<Assets>) -> Self {
        Self {
            game_camera: Camera2D::default(),
            camera_zoom: 0.02,
            camera_zoom_speed: 0.001,
            ui_state: UIState {
                state: GameState::Menu,
                ui_scale: vec2(1.0, 1.0),
                camera_scale: vec2(1.0, 1.0),
                debug_mode: false,
                player_alive: true,
                stage: 0,
                stage_timer: 0.0,
                play_button: ButtonElement::new(
                    5.0,
                    10.0,
                    20.0,
                    5.0,
                    10.0,
                    TextElement::new(
                        "PLAY".to_owned(),
                        50.0,
                        WHITE,
                        UIObject::new(vec2(0.5, 0.5), vec2(0.0, -20.0), UIScaleMode::World),
                    ),
                ),
                quit_button: ButtonElement::new(
                    5.0,
                    10.0,
                    20.0,
                    5.0,
                    10.0,
                    TextElement::new(
                        "QUIT".to_owned(),
                        50.0,
                        WHITE,
                        UIObject::new(vec2(0.5, 0.5), vec2(0.0, 20.0), UIScaleMode::World),
                    ),
                ),
                tutorial_texture: TextureElement::new(
                    assets.tutorial,
                    WHITE,
                    Some(vec2(160.0, 40.0)),
                    UIObject::new(vec2(0.5, 0.0), vec2(0.0, 60.0), UIScaleMode::KeepRatio),
                ),
                fps_element: FPSElement::new(
                    0.5,
                    TextElement::new(
                        "".to_owned(),
                        20.0,
                        WHITE,
                        UIObject::new(vec2(0.0, 0.0), vec2(50.0, 20.0), UIScaleMode::KeepRatio),
                    ),
                ),
                stage_element: TextElement::new(
                    "".to_owned(),
                    50.0,
                    WHITE,
                    UIObject::new(vec2(0.5, 0.0), vec2(0.0, 100.0), UIScaleMode::KeepRatio),
                ),
                death_element: TextElement::new(
                    "YOU DIED".to_owned(),
                    50.0,
                    WHITE,
                    UIObject::new(vec2(0.5, 0.5), vec2(0.0, 0.0), UIScaleMode::KeepRatio),
                ),
                reset_element: TextElement::new(
                    "PRESS R TO RESET".to_owned(),
                    50.0,
                    WHITE,
                    UIObject::new(vec2(0.5, 0.5), vec2(0.0, 50.0), UIScaleMode::KeepRatio),
                ),
            },
        }
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        state: GameState,
        model: &Model,
    ) -> Option<GameUpdate> {
        let target_zoom = match state {
            GameState::Menu => 0.02,
            GameState::Pregame | GameState::Game => 0.0055,
        };
        let delta = target_zoom - self.camera_zoom;
        if delta.abs() > 1e-4 {
            self.camera_zoom += delta.signum() * delta.abs().min(self.camera_zoom_speed);
        }
        self.game_camera = Camera2D {
            offset: vec2(0.0, 0.0),
            zoom: vec2(
                self.camera_zoom,
                self.camera_zoom * screen_width() / screen_height(),
            ),
            ..Default::default()
        };

        self.ui_state.state = state;
        self.ui_state.player_alive = model.player.entity.health.is_alive();

        self.ui_state.update(
            self.game_camera.zoom.xx() / 2.0,
            delta_time,
            self.game_camera.world_to_screen(model.player.head.position),
        )
    }

    pub fn next_wave(&mut self, stage: usize) {
        self.ui_state.stage = stage;
        self.ui_state.stage_timer = STAGE_SHOW_TIME;
    }
}
