use macroquad::prelude::*;
use specs::{Builder, Dispatcher, DispatcherBuilder, Join, World, WorldExt};

use crate::{
    components::*,
    constants::*,
    physics::PhysicsMaterial,
    resources::{Bounds, Time},
    systems::*,
};

pub struct Game<'a, 'b, 'c, 'd> {
    game_camera: Camera2D,
    world: World,
    update_dispatcher: Dispatcher<'a, 'b>,
    fixed_update_dispatcher: Dispatcher<'c, 'd>,
}

impl<'a, 'b, 'c, 'd> Game<'a, 'b, 'c, 'd> {
    pub fn new(fixed_delta_time: f32) -> Self {
        let mut world = World::new();

        world.register::<Transform>();
        world.register::<BonkleBody>();
        world.register::<PlayerBody>();
        world.register::<PlayerHead>();
        world.register::<FriendTarget>();
        world.register::<FriendCollider>();
        world.register::<EnemyCollider>();
        world.register::<ActorComponent>();
        world.register::<Health>();
        world.register::<Corpse>();
        world.register::<Collidable>();
        world.register::<ColorComponent>();
        world.register::<BonkleBodyController>();
        world.register::<Lifetime>();

        world.insert(Time {
            delta_time: 0.0,
            fixed_delta_time,
        });
        world.insert(Bounds {
            min: vec2(-160.0, -90.0),
            max: vec2(160.0, 90.0),
        });
        world.insert(Camera2D::default());

        spawn_player(&mut world, Vec2::ZERO);

        let mut update_dispatcher = DispatcherBuilder::new()
            .with(HandleInputSystem::default(), "handle_input", &[])
            .with(
                BonkleBodyControlSystem,
                "bonkle_body_control",
                &["handle_input"],
            )
            .with(MovementSystem, "movement", &["bonkle_body_control"])
            .with(DragSystem, "drag", &["movement"])
            .with(PlayerHeadSystem, "player_head", &["movement"])
            .with(CollisionSystem, "collision", &["player_head"])
            .with(LifetimeSystem, "lifetime", &["collision"])
            .with(CheckDeadSystem, "dead", &["lifetime"])
            .with(SpawnerSystem, "spawner", &["dead"])
            .with(WaveSystem { current_stage: 0 }, "wave", &["spawner"])
            .with(UpdateCorpsesSystem, "corpses", &[])
            .build();
        update_dispatcher.setup(&mut world);

        let mut fixed_update_dispatcher = DispatcherBuilder::new().build();
        fixed_update_dispatcher.setup(&mut world);

        Self {
            game_camera: Camera2D::default(),
            world,
            update_dispatcher,
            fixed_update_dispatcher,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let zoom = 0.0055;
        self.game_camera = Camera2D {
            offset: vec2(0.0, 0.0),
            zoom: vec2(zoom, zoom * screen_width() / screen_height()),
            ..Default::default()
        };

        self.world.insert(self.game_camera.clone());
        self.world.write_resource::<Time>().delta_time = delta_time;
        self.update_dispatcher.dispatch(&mut self.world);
        self.world.maintain();
        self.perform_actions(delta_time);
    }

    pub fn fixed_update(&mut self, delta_time: f32) {
        self.world.write_resource::<Time>().fixed_delta_time = delta_time;
        self.fixed_update_dispatcher.dispatch(&mut self.world);
        self.world.maintain();
    }

    fn perform_actions(&mut self, delta_time: f32) {
        let mut actions = Vec::new();
        for (entity, actor) in (
            &self.world.entities(),
            &mut self.world.write_component::<ActorComponent>(),
        )
            .join()
        {
            for action in actor.0.update(entity, &self.world, delta_time) {
                actions.push((action, entity));
            }
        }
        for (action, actor) in actions {
            action.perform(&mut self.world, actor);
        }
    }

    pub fn draw(&self) {
        set_camera(&self.game_camera);

        for (entity, transform, bonkle_body, color) in (
            &self.world.entities(),
            &self.world.read_storage::<Transform>(),
            &self.world.read_storage::<BonkleBody>(),
            &self.world.read_storage::<ColorComponent>(),
        )
            .join()
        {
            if let Some(inner) = self
                .world
                .read_storage::<Lifetime>()
                .get(entity)
                .map(|lifetime| lifetime.0.hp_frac())
                .or_else(|| {
                    self.world
                        .read_storage::<Health>()
                        .get(entity)
                        .map(|health| health.hp_frac())
                })
            {
                draw_circle(
                    transform.position.x,
                    transform.position.y,
                    inner * bonkle_body.radius,
                    color.0,
                );
            }
            draw_circle_outline(transform.position, bonkle_body.radius, color.0);
        }

        let bounds = self.world.read_resource::<Bounds>();
        let bounds_size = bounds.max - bounds.min;
        draw_rectangle_lines(
            bounds.min.x,
            bounds.min.y,
            bounds_size.x,
            bounds_size.y,
            0.5,
            BORDER_COLOR,
        );
    }
}

fn draw_circle_outline(position: Vec2, radius: f32, color: Color) {
    draw_poly_lines(position.x, position.y, 50, radius, 0.0, 0.2, color);
}

fn spawn_player(world: &mut World, position: Vec2) {
    let player_body = world
        .create_entity()
        .with(FriendTarget)
        .with(FriendCollider)
        .with(PlayerBody {
            movement_speed: BODY_SPEED,
        })
        .with(Health::new(PLAYER_HP))
        .with(ColorComponent(PLAYER_COLOR))
        .with(Collidable)
        .with(Transform { position })
        .with(BonkleBody {
            mass: BODY_MASS,
            radius: BODY_RADIUS,
            velocity: Vec2::ZERO,
            physics_material: PhysicsMaterial::new(0.0, 1.0),
        })
        .with(BonkleBodyController {
            target_velocity: Vec2::ZERO,
            acceleration: BODY_ACCELERATION,
        })
        .build();

    let position = position + Vec2::new(HEAD_CONNECTION_LENGTH, 0.0);
    world
        .create_entity()
        .with(FriendCollider)
        .with(PlayerHead {
            movement_speed: HEAD_SPEED,
            target_pos: position,
            connected_to: player_body,
            connection_length: HEAD_CONNECTION_LENGTH,
        })
        .with(ColorComponent(PLAYER_COLOR))
        .with(Collidable)
        .with(Transform { position })
        .with(BonkleBody {
            mass: HEAD_MASS,
            radius: HEAD_RADIUS,
            velocity: Vec2::ZERO,
            physics_material: PhysicsMaterial::new(0.0, BOUNCINESS),
        })
        .with(BonkleBodyController {
            target_velocity: Vec2::ZERO,
            acceleration: HEAD_ACCELERATION,
        })
        .build();
}
