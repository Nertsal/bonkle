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
            collider: &'a Collider,
        }

        for (_body_id, body) in &query_item!(model.bodies) {
            self.draw_collider(body.collider, &model.camera, framebuffer);
        }
    }

    fn draw_collider(
        &self,
        collider: &Collider,
        camera: &Camera2d,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let position = collider.position.as_f32();
        let rotation = collider.rotation.as_radians().as_f32();
        let transform = mat3::translate(position) * mat3::rotate(rotation);

        let color = Color::BLUE; // TODO
        match collider.shape {
            Shape::Circle { radius } => {
                // Rotation does not impact circles (TODO: for now)
                self.geng.draw2d().draw2d(
                    framebuffer,
                    camera,
                    &draw2d::Ellipse::circle(position, radius.as_f32(), color),
                );
            }
            Shape::Rectangle { width, height } => {
                let aabb = Aabb2::ZERO.extend_symmetric(vec2(width, height).as_f32() / 2.0);
                self.geng.draw2d().draw2d_transformed(
                    framebuffer,
                    camera,
                    &draw2d::Quad::new(aabb, color),
                    transform,
                );
            }
        }
    }
}
