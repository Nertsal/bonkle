use super::*;

pub struct UIState {
    pub paused: bool,
    pub debug_mode: bool,
    pub player_alive: bool,
    pub stage: usize,
    pub stage_timer: f32,
    pub tutorial_texture: TextureElement,
    pub fps_element: FPSElement,
    pub stage_element: TextElement,
    pub death_element: TextElement,
    pub reset_element: TextElement,
}

impl UIState {
    pub fn update(&mut self, delta_time: f32) {
        if self.stage_timer > 0.0 {
            self.stage_timer -= delta_time;
        }

        if is_key_pressed(KeyCode::F6) {
            self.debug_mode = !self.debug_mode;
        }

        self.tutorial_texture.update(delta_time);
        self.fps_element.update(delta_time);
        self.stage_element.text = format!("STAGE {}", self.stage);
    }

    pub fn draw(&self) {
        let ui_scale = vec2(
            screen_width() / DEFAULT_WIDTH,
            screen_height() / DEFAULT_HEIGHT,
        );

        if self.paused {
            self.tutorial_texture.draw(ui_scale);
        }

        if self.debug_mode {
            self.fps_element.draw(ui_scale);
        }

        if !self.player_alive {
            self.stage_element.draw(ui_scale);
            self.death_element.draw(ui_scale);
            self.reset_element.draw(ui_scale);
        } else if self.stage_timer > 0.0 {
            self.stage_element.draw(ui_scale);
        }
    }
}

pub trait UIElement {
    fn update(&mut self, _delta_time: f32) {}

    fn ui_element(&self) -> (&UIObject, UIContent);

    fn draw(&self, ui_scale: Vec2) {
        let (object, content) = self.ui_element();
        object.draw(ui_scale, content);
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

    pub fn draw(&self, ui_scale: Vec2, content: UIContent) {
        let scale = match self.scale_mode {
            UIScaleMode::Full => ui_scale,
            UIScaleMode::KeepRatio => {
                let scale = ui_scale.x.min(ui_scale.y);
                vec2(scale, scale)
            }
        };
        let min_scale = scale.x.min(scale.y);
        let anchor = self.anchor * vec2(DEFAULT_WIDTH, DEFAULT_HEIGHT) * ui_scale;
        match content {
            UIContent::Text {
                text,
                font_size,
                color,
            } => {
                draw_text(
                    &text,
                    anchor.x + self.position.x * scale.x,
                    anchor.y + self.position.y * scale.y,
                    font_size * min_scale,
                    color.clone(),
                );
            }
            UIContent::Texture {
                texture,
                color,
                dest_size,
            } => {
                draw_texture_ex(
                    texture,
                    anchor.x + self.position.x * scale.x,
                    anchor.y + self.position.y * scale.y,
                    color,
                    DrawTextureParams {
                        dest_size: dest_size.map(|dest_size| dest_size * scale),
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
    KeepRatio,
    Full,
}
