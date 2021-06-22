use super::*;

pub struct ButtonElement {
    pub text_element: TextElement,
    pub hovering: bool,
    size: f32,
    size_min: f32,
    size_max: f32,
    size_speed: f32,
    font_min: f32,
    font_max: f32,
}

impl ButtonElement {
    pub fn new(
        size_min: f32,
        size_max: f32,
        size_speed: f32,
        font_min: f32,
        font_max: f32,
        text_element: TextElement,
    ) -> Self {
        Self {
            hovering: false,
            size: size_min,
            size_min,
            size_max,
            text_element,
            size_speed,
            font_min,
            font_max,
        }
    }

    pub fn point_inside(&self, point: Vec2, ui_scale: Vec2, camera_scale: Vec2) -> bool {
        let distance = (point
            - self
                .text_element
                .ui_object
                .global_position(ui_scale, camera_scale))
        .length();
        distance
            <= self.size
                * self
                    .text_element
                    .ui_object
                    .scale(ui_scale, camera_scale)
                    .max_element()
    }
}

impl UIElement for ButtonElement {
    fn ui_element(&self) -> (&UIObject, UIContent) {
        self.text_element.ui_element()
    }

    fn update(&mut self, delta_time: f32) {
        if self.hovering {
            self.size += self.size_speed * delta_time;
        } else {
            self.size -= self.size_speed * delta_time;
        }
        self.size = self.size.clamp(self.size_min, self.size_max);
        self.text_element.font_size = (self.size - self.size_min) / (self.size_max - self.size_min)
            * (self.font_max - self.font_min)
            + self.font_min;
    }
}
