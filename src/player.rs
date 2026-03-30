use avian2d::prelude::*;
use bevy::prelude::*;
#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct CursorCircle;

use crate::constants::{MAX_VELOCITY,GRAVITY};


pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut ConstantLinearAcceleration, &mut LinearVelocity), With<Player>>,
) {
    for (mut acceleration, mut velocity) in &mut query {
        let accel_rate = 5000.0;
        let jump_force = 500.0;
        let friction = 0.1;
        if keyboard_input.pressed(KeyCode::KeyA) {
            acceleration.x = -accel_rate * (1.0 - velocity.x.abs() / MAX_VELOCITY);
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            acceleration.x = accel_rate * (1.0 - velocity.x / MAX_VELOCITY);
        } else {
            acceleration.x = 0.0;
            velocity.x *= 1.0 - friction;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            acceleration.y = jump_force
                * (1.0 - velocity.y / MAX_VELOCITY)
                + GRAVITY;
        } else {
            acceleration.y = 0.0;
        }
        if velocity.y < -MAX_VELOCITY*3.0 {
            velocity.y = -MAX_VELOCITY*3.0;
        }
    }
}
pub fn update_cursor(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut cursor_query: Query<&mut Transform, With<CursorCircle>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single().unwrap();

    if let Some(screen_position) = window
        .expect("Error: couldn't get cursor pos.")
        .cursor_position()
    {
        if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, screen_position) {
            if let Ok(mut cursor_transform) = cursor_query.single_mut() {
                cursor_transform.translation.x = world_position.x;
                cursor_transform.translation.y = world_position.y;
            }
        }
    }
}
