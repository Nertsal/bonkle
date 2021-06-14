use super::*;

pub struct TextureElement {
    texture: Texture2D,
    color: Color,
    dest_size: Option<Vec2>,
    ui_object: UIObject,
}

impl TextureElement {
    pub fn new(
        texture: Texture2D,
        color: Color,
        dest_size: Option<Vec2>,
        ui_object: UIObject,
    ) -> Self {
        Self {
            texture,
            color,
            dest_size,
            ui_object,
        }
    }
}

impl UIElement for TextureElement {
    fn ui_element(&self) -> (&UIObject, UIContent) {
        (
            &self.ui_object,
            UIContent::Texture {
                texture: self.texture,
                color: self.color,
                dest_size: self.dest_size.clone(),
            },
        )
    }
}
