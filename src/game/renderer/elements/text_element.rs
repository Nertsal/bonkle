use super::*;

pub struct TextElement {
    pub text: String,
    pub font_size: f32,
    pub color: Color,
    pub ui_object: UIObject,
}

impl TextElement {
    pub fn new(text: String, font_size: f32, color: Color, ui_object: UIObject) -> Self {
        Self {
            text,
            font_size,
            color,
            ui_object,
        }
    }
}

impl UIElement for TextElement {
    fn ui_element(&self) -> (&UIObject, UIContent) {
        (
            &self.ui_object,
            UIContent::Text {
                text: self.text.clone(),
                color: self.color.clone(),
                font_size: self.font_size,
            },
        )
    }
}
