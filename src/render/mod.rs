use crate::{assets::*, model::*, util::Vec2RealConversions};

use ecs::prelude::*;
use geng::prelude::*;

pub struct GameRender {
    geng: Geng,
    assets: Rc<Assets>,
}

impl GameRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
        }
    }

    pub fn draw(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        self.draw_bodies(model, framebuffer);
    }

    fn draw_bodies(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[derive(StructQuery)]
        struct Item<'a> {
            position: &'a vec2<Coord>,
            radius: &'a Coord,
        }

        for (_body_id, body) in &query_item!(model.bodies) {
            self.geng.draw2d().draw2d(
                framebuffer,
                &model.camera,
                &draw2d::Ellipse::circle(body.position.as_f32(), body.radius.as_f32(), Color::BLUE),
            );
        }
    }
}
