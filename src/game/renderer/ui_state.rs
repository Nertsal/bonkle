use super::*;

pub struct UIState {
    pub state: GameState,
    pub ui_scale: Vec2,
    pub camera_scale: Vec2,
    pub debug_mode: bool,
    pub player_alive: bool,
    pub stage: usize,
    pub stage_timer: f32,
    pub play_button: ButtonElement,
    pub quit_button: ButtonElement,
    pub tutorial_texture: TextureElement,
    pub fps_element: FPSElement,
    pub stage_element: TextElement,
    pub death_element: TextElement,
    pub reset_element: TextElement,
}

impl UIState {
    pub fn update(
        &mut self,
        camera_scale: Vec2,
        delta_time: f32,
        position: Vec2,
    ) -> Option<GameUpdate> {
        self.camera_scale = camera_scale;
        self.ui_scale = vec2(
            screen_width() / DEFAULT_WIDTH,
            screen_height() / DEFAULT_HEIGHT,
        );

        if self.stage_timer > 0.0 {
            self.stage_timer -= delta_time;
        }

        if is_key_pressed(KeyCode::F6) {
            self.debug_mode = !self.debug_mode;
        }

        let mut game_update = None;
        match self.state {
            GameState::Menu => {
                let press =
                    is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Enter);
                self.play_button.hovering =
                    self.play_button
                        .point_inside(position, self.ui_scale, self.camera_scale);
                self.play_button.update(delta_time);
                self.quit_button.hovering =
                    self.quit_button
                        .point_inside(position, self.ui_scale, self.camera_scale);
                self.quit_button.update(delta_time);
                if press {
                    if self.play_button.hovering {
                        game_update = Some(GameUpdate::Start);
                    } else if self.quit_button.hovering {
                        game_update = Some(GameUpdate::Quit);
                    }
                }
            }
            _ => (),
        }

        self.fps_element.update(delta_time);
        self.stage_element.text = format!("STAGE {}", self.stage);
        game_update
    }

    pub fn draw(&self) {
        match self.state {
            GameState::Menu => {
                self.play_button.draw(self.ui_scale, self.camera_scale);
                self.quit_button.draw(self.ui_scale, self.camera_scale);
            }
            GameState::Pregame => {
                self.tutorial_texture.draw(self.ui_scale, self.camera_scale);
            }
            _ => (),
        }

        if self.debug_mode {
            self.fps_element.draw(self.ui_scale, self.camera_scale);
        }

        if !self.player_alive {
            self.stage_element.draw(self.ui_scale, self.camera_scale);
            self.death_element.draw(self.ui_scale, self.camera_scale);
            self.reset_element.draw(self.ui_scale, self.camera_scale);
        } else if self.stage_timer > 0.0 {
            self.stage_element.draw(self.ui_scale, self.camera_scale);
        }
    }
}

pub trait UIElement {
    fn update(&mut self, _delta_time: f32) {}

    fn ui_element(&self) -> (&UIObject, UIContent);

    fn draw(&self, ui_scale: Vec2, camera_scale: Vec2) {
        let (object, content) = self.ui_element();
        object.draw(ui_scale, camera_scale, content);
    }
}

pub struct UIObject {
    pub anchor: Vec2,
    pub position: Vec2,
    pub scale_mode: UIScaleMode,
}

impl UIObject {
    pub fn new(anchor: Vec2, position: Vec2, scale_mode: UIScaleMode) -> Self {
        Self {
            anchor,
            position,
            scale_mode,
        }
    }

    pub fn scale(&self, ui_scale: Vec2, camera_scale: Vec2) -> Vec2 {
        match self.scale_mode {
            UIScaleMode::World => camera_scale * DEFAULT_WIDTH * ui_scale.x,
            UIScaleMode::KeepRatio => {
                let scale = ui_scale.min_element();
                vec2(scale, scale)
            }
        }
    }

    pub fn global_position(&self, ui_scale: Vec2, camera_scale: Vec2) -> Vec2 {
        let anchor = self.anchor * vec2(DEFAULT_WIDTH, DEFAULT_HEIGHT) * ui_scale;
        anchor + self.position * self.scale(ui_scale, camera_scale)
    }

    pub fn draw(&self, ui_scale: Vec2, camera_scale: Vec2, content: UIContent) {
        let scale = self.scale(ui_scale, camera_scale);
        let min_scale = scale.x.min(scale.y);
        let position = self.global_position(ui_scale, camera_scale);
        match content {
            UIContent::Text {
                text,
                font_size,
                color,
            } => {
                let font_size = font_size * min_scale;
                draw_text(
                    &text,
                    position.x - font_size / 4.0 * text.len() as f32,
                    position.y + font_size / 4.0,
                    font_size,
                    color.clone(),
                );
            }
            UIContent::Texture {
                texture,
                color,
                dest_size,
            } => {
                let dest_size = dest_size.map(|dest_size| dest_size * scale);
                draw_texture_ex(
                    texture,
                    position.x - dest_size.map_or(0.0, |size| size.x / 2.0),
                    position.y + dest_size.map_or(0.0, |size| size.y / 2.0),
                    color,
                    DrawTextureParams {
                        dest_size,
                        ..Default::default()
                    },
                );
            }
        }
    }
}

pub enum UIContent {
    Text {
        text: String,
        font_size: f32,
        color: Color,
    },
    Texture {
        texture: Texture2D,
        color: Color,
        dest_size: Option<Vec2>,
    },
}

#[derive(Clone, Copy)]
pub enum UIScaleMode {
    World,
    KeepRatio,
}
