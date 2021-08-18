use macroquad::prelude::Color;
use specs::{Component, VecStorage};

#[derive(Debug, Clone)]
pub struct ColorComponent(pub Color);

impl Component for ColorComponent {
    type Storage = VecStorage<Self>;
}
