use crate::constants::*;
use crate::movement::move_paddle;
use crate::physics::*;
use bevy::prelude::*;

#[derive(Resource)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
    Impossible,
}

pub fn computer_movement(
    time: Res<Time>,
    mut paddle_query: Single<&mut Transform, (With<Paddle>, With<Computer>, Without<Ball>)>,
    ball_query: Single<&Transform, With<Ball>>,
) {
    let mut paddle_transform = paddle_query.into_inner();
    let ball_position = ball_query.translation;
    let mut direction = 0.;

    let ball_y = ball_position.y;
    let current_y = paddle_transform.translation.y;

    if ball_y > (current_y + PADDLE_SIZE.y / 2.) {
        direction += 1.;
    } else if ball_y < (current_y - PADDLE_SIZE.y / 2.) {
        direction -= 1.;
    }

    move_paddle(&mut paddle_transform, direction, time.delta_secs());
}
