use macroquad::{
    camera::Camera2D,
    prelude::{is_key_down, mouse_position, vec2, KeyCode, Vec2},
};
use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{BonkleBody, BonkleBodyController, PlayerBody, PlayerHead, Transform};

#[derive(Default)]
pub struct HandleInputSystem {
    last_mouse_position: Vec2,
    head_control_mode: HeadControlMode,
}

impl<'s> System<'s> for HandleInputSystem {
    type SystemData = (
        ReadStorage<'s, PlayerBody>,
        WriteStorage<'s, PlayerHead>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, BonkleBodyController>,
        ReadStorage<'s, BonkleBody>,
        Read<'s, Camera2D>,
    );

    fn run(
        &mut self,
        (
            player_bodies,
            mut player_heads,
            transforms,
            mut bonkle_body_controllers,
            bonkle_bodies,
            game_camera,
        ): Self::SystemData,
    ) {
        for (body, controller) in (&player_bodies, &mut bonkle_body_controllers).join() {
            control_body(body.movement_speed, controller);
        }
        for (head, head_transform, controller) in
            (&mut player_heads, &transforms, &mut bonkle_body_controllers).join()
        {
            if let Some(body_transform) = transforms.get(head.connected_to) {
                let head_position = head_transform.position;
                let body_position = body_transform.position;
                self.control_head(head, head_position, body_position, &game_camera);
                let direction = head_position - body_position;
                let target = head.target_pos - body_position;
                let angle = direction.angle_between(target).abs();
                let speed = angle.min(0.2) / 0.2;
                let direction = vec2(direction.y, -direction.x).normalize();
                let signum = direction.dot(target).signum();
                let direction = direction * signum * speed;
                let body_velocity = bonkle_bodies
                    .get(head.connected_to)
                    .map(|body| body.velocity)
                    .unwrap_or_default();
                controller.target_velocity = direction * head.movement_speed + body_velocity;
            }
        }
    }
}

impl HandleInputSystem {
    fn control_head(
        &mut self,
        head: &mut PlayerHead,
        head_position: Vec2,
        body_position: Vec2,
        game_camera: &Camera2D,
    ) {
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_position = vec2(mouse_x, mouse_y);
        if mouse_position != self.last_mouse_position {
            self.head_control_mode = HeadControlMode::Mouse;
            head.target_pos = game_camera.screen_to_world(mouse_position);
        } else {
            let mut direction = 0.0;
            if is_key_down(KeyCode::Left) {
                direction -= 1.0;
            }
            if is_key_down(KeyCode::Right) {
                direction += 1.0;
            }
            if direction != 0.0 {
                let target = head_position - body_position;
                head.target_pos =
                    vec2(target.y, -target.x).normalize() * direction * 5.0 + head_position;
                self.head_control_mode = HeadControlMode::Keys;
            } else {
                match self.head_control_mode {
                    HeadControlMode::Mouse => (),
                    HeadControlMode::Keys => {
                        head.target_pos = head_position;
                    }
                }
            }
        };
        self.last_mouse_position = mouse_position;
    }
}

enum HeadControlMode {
    Mouse,
    Keys,
}

impl Default for HeadControlMode {
    fn default() -> Self {
        Self::Mouse
    }
}

fn control_body(movement_speed: f32, controller: &mut BonkleBodyController) {
    let mut move_x = 0.0;
    if is_key_down(KeyCode::A) {
        move_x -= 1.0;
    }
    if is_key_down(KeyCode::D) {
        move_x += 1.0;
    }

    let mut move_y = 0.0;
    if is_key_down(KeyCode::S) {
        move_y -= 1.0;
    }
    if is_key_down(KeyCode::W) {
        move_y += 1.0;
    }

    controller.target_velocity.x = move_x * movement_speed;
    controller.target_velocity.y = move_y * movement_speed;
}
