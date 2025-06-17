use bevy::prelude::*;

use crate::systems::*;

pub fn player_movement(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Paddle>, With<Player>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += 1.0;
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction -= 1.0;
        }

        move_paddle(&mut transform, direction, time.delta_secs());
    }
}

pub fn move_paddle(paddle_transform: &mut Transform, direction: f32, delta_time: f32) {
    let new_y = paddle_transform.translation.y + direction * PADDLE_SPEED * delta_time;
    let upper_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.y / 2.0 - PADDLE_PADDING;
    let lower_bound = BTM_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.y / 2.0 + PADDLE_PADDING;

    paddle_transform.translation.y = new_y.clamp(lower_bound, upper_bound);
}
