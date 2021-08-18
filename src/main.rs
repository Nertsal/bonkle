use components::Transform;
// #![windows_subsystem = "windows"]
use macroquad::prelude::*;
use specs::{Component, Entity, Join, World, WorldExt};
use std::time::Instant;

mod actions;
mod actors;
mod components;
mod constants;
mod game;
mod physics;
mod resources;
mod systems;

use game::Game;

const FIXED_DELTA_TIME: f32 = 1.0 / 60.0;

#[macroquad::main("Bonkle")]
async fn main() {
    let mut game = Game::new(FIXED_DELTA_TIME);

    // let mut game = Game::new().await;
    let mut frame_time = 0.0;
    loop {
        println!("---- next frame ----");
        let delta_time = get_frame_time();
        frame_time += delta_time;
        let time = Instant::now();
        game.update(delta_time);
        println!("update: {}ms", time.elapsed().as_millis());

        let time = Instant::now();
        let mut frames = 0;
        while frame_time >= FIXED_DELTA_TIME {
            game.fixed_update(FIXED_DELTA_TIME);
            frame_time -= FIXED_DELTA_TIME;
            frames += 1;
        }
        println!(
            "fixed_update: {}ms / {} frames",
            time.elapsed().as_millis(),
            frames
        );

        let time = Instant::now();
        game.draw();
        println!("draw: {}ms", time.elapsed().as_millis());
        next_frame().await;
    }
}

pub fn find_closest_entity_world<T: Component>(origin: Vec2, world: &World) -> Option<Entity> {
    find_closest_entity(
        origin,
        (
            &world.entities(),
            &world.read_component::<T>(),
            &world.read_component::<Transform>(),
        )
            .join()
            .map(|(entity, _, transform)| (entity, transform.position)),
    )
}

pub fn find_closest_entity(
    origin: Vec2,
    entities: impl Iterator<Item = (Entity, Vec2)>,
) -> Option<Entity> {
    entities
        .map(|(entity, position)| (entity, origin.distance_squared(position)))
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(entity, _)| entity)
}
