use super::*;

pub struct Renderer {
    pub game_camera: Camera2D,
    current_fps: f32,
    fps_update_time: f32,
    fps_update: f32,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            game_camera: Camera2D {
                offset: vec2(0.0, -1.0),
                zoom: vec2(0.01, 0.01 * screen_width() / screen_height()),
                ..Default::default()
            },
            current_fps: 0.0,
            fps_update_time: 0.5,
            fps_update: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.fps_update -= delta_time;
        if self.fps_update <= 0.0 {
            self.fps_update += self.fps_update_time;
            self.current_fps = 1.0 / delta_time;
        }
    }

    pub fn draw(&mut self, model: &Model) {
        clear_background(BLACK);
        self.draw_game(model);
        self.draw_ui();
    }

    fn draw_game(&mut self, model: &Model) {
        set_camera(&self.game_camera);
    }

    fn draw_ui(&self) {
        set_default_camera();

        draw_text(
            &format!("FPS: {:.0}", self.current_fps),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
    }
}
