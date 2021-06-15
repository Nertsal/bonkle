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
        self.ui_state.player_alive = model.player.health.is_alive();

        self.ui_state.update(delta_time);
    }

    pub fn next_wave(&mut self, stage: usize) {
        self.ui_state.stage = stage;
        self.ui_state.stage_timer = STAGE_SHOW_TIME;
    }
}
