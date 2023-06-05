use crate::{
    assets::*,
    model::*,
    util::{Mat3RealConversions, Vec2RealConversions},
};

use ecs::prelude::*;
use geng::prelude::*;

pub struct GameRender {
    geng: Geng,
    // assets: Rc<Assets>,
    theme: ColorTheme,
}

impl GameRender {
    pub fn new(geng: &Geng, _assets: &Rc<Assets>, theme: ColorTheme) -> Self {
        Self {
            geng: geng.clone(),
            // assets: assets.clone(),
            theme,
        }
    }

    pub fn draw(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(self.theme.background), None, None);
        self.draw_corpses(model, framebuffer);
        self.draw_particles(model, framebuffer);
        self.draw_bodies(model, framebuffer);
        self.draw_bounds(model, framebuffer);
    }

    fn draw_bounds(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        let [bl, br, tr, tl] = model.bounds.map(Coord::as_f32).corners();
        let bm = (bl + br) / 2.0;
        let points = vec![bm, br, tr, tl, bl, bm];
        let chain = Chain::new(points);
        self.geng.draw2d().draw2d(
            framebuffer,
            &model.camera,
            &draw2d::Chain::new(chain, 0.1, self.theme.border, 2),
        );
    }

    fn draw_bodies(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[derive(StructQuery)]
        struct Item<'a> {
            name: &'a String,
            collider: &'a Collider,
            health: &'a Option<Health>,
        }

        for (_body_id, body) in &query_item!(model.bodies) {
            let color = self.theme.get_entity(body.name).unwrap_or(Color::BLUE);
            if let Some(health) = body.health {
                let fill = health.ratio().as_f32().clamp(0.0, 1.0);
                self.draw_collider(
                    body.collider,
                    color,
                    mat3::scale_uniform(fill),
                    &model.camera,
                    framebuffer,
                );
            }
            self.draw_collider_outline(
                body.collider,
                0.1,
                color,
                mat3::identity(),
                &model.camera,
                framebuffer,
            );
        }
    }

    fn draw_corpses(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[derive(StructQuery)]
        struct Item<'a> {
            #[query(nested = ".body")]
            name: &'a String,
            #[query(nested = ".body")]
            collider: &'a Collider,
            lifetime: &'a Health,
        }

        for (_body_id, body) in &query_item!(model.corpses) {
            let mut color = self.theme.get_entity(body.name).unwrap_or(Color::BLUE);
            color.a = body.lifetime.ratio().as_f32() * 0.5;
            self.draw_collider_outline(
                body.collider,
                0.1,
                color,
                mat3::identity(),
                &model.camera,
                framebuffer,
            );
        }
    }

    fn draw_particles(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[derive(StructQuery)]
        struct Item<'a> {
            name: &'a String,
            collider: &'a Collider,
            lifetime: &'a Health,
        }

        for (_id, particle) in &query_item!(model.particles) {
            let mut color = self.theme.get_entity(particle.name).unwrap_or(Color::BLUE);
            let t = particle.lifetime.ratio().as_f32();
            let t = tween::Tweener::cubic_in_out(0.0, 1.0, 1.0).move_by(t);
            color.a = t * 0.9;
            self.draw_collider_outline(
                particle.collider,
                0.1,
                color,
                mat3::scale_uniform(t),
                &model.camera,
                framebuffer,
            );
        }
    }

    fn draw_collider_outline(
        &self,
        collider: &Collider,
        outline_width: f32,
        color: Color,
        transform: mat3<f32>,
        camera: &Camera2d,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let transform = collider.transform_mat().as_f32() * transform;
        match collider.shape {
            Shape::Circle { radius } => {
                let radius = radius.as_f32();
                self.geng.draw2d().draw2d_transformed(
                    framebuffer,
                    camera,
                    &draw2d::Ellipse::circle_with_cut(
                        vec2::ZERO,
                        (radius - outline_width).max(1e-2),
                        radius,
                        color,
                    ),
                    transform,
                );
            }
            Shape::Rectangle { width, height } => {
                let aabb = Aabb2::ZERO.extend_symmetric(vec2(width, height).as_f32() / 2.0);
                let [bl, br, tr, tl] = aabb.corners();
                let bm = (bl + br) / 2.0;
                let points = vec![bm, br, tr, tl, bl, bm];
                let chain = Chain::new(points);
                self.geng.draw2d().draw2d(
                    framebuffer,
                    camera,
                    &draw2d::Chain::new(chain, outline_width, Color::GRAY, 2),
                );
            }
        }
    }

    fn draw_collider(
        &self,
        collider: &Collider,
        color: Color,
        transform: mat3<f32>,
        camera: &Camera2d,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let transform = collider.transform_mat().as_f32() * transform;
        self.draw_shape(collider.shape, color, transform, camera, framebuffer)
    }

    fn draw_shape(
        &self,
        shape: Shape,
        color: Color,
        transform: mat3<f32>,
        camera: &Camera2d,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        match shape {
            Shape::Circle { radius } => {
                self.geng.draw2d().draw2d_transformed(
                    framebuffer,
                    camera,
                    &draw2d::Ellipse::circle(vec2::ZERO, radius.as_f32(), color),
                    transform,
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
