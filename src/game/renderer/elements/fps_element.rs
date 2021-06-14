use super::*;

pub struct FPSElement {
    pub fps_update_time: f32,
    pub text_element: TextElement,
    current_fps: f32,
    fps_update: f32,
}

impl FPSElement {
    pub fn new(fps_update_time: f32, text_element: TextElement) -> Self {
        Self {
            current_fps: 0.0,
            fps_update: 0.0,
            fps_update_time,
            text_element,
        }
    }
}

impl UIElement for FPSElement {
    fn update(&mut self, delta_time: f32) {
        self.fps_update -= delta_time;
        if self.fps_update <= 0.0 {
            self.fps_update += self.fps_update_time;
            self.current_fps = 1.0 / delta_time;
        }
        self.text_element.text = format!("FPS: {:.0}", self.current_fps);
    }

    fn ui_element(&self) -> (&UIObject, UIContent) {
        self.text_element.ui_element()
    }
}
