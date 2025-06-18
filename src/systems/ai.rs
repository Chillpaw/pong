use crate::constants::*;
use crate::physics::*;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Copy, Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Impossible,
}

impl Difficulty {
    fn accuracy_factor(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.6,
            Difficulty::Medium => 0.75,
            Difficulty::Hard => 0.8,
            Difficulty::Impossible => 0.95,
        }
    }

    fn reaction_time(&self) -> f32 {
        match self {
            Difficulty::Easy => 1.0,
            Difficulty::Medium => 0.4,
            Difficulty::Hard => 0.2,
            Difficulty::Impossible => 0.0,
        }
    }

    fn error_margin(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.4,
            Difficulty::Medium => 0.25,
            Difficulty::Hard => 0.1,
            Difficulty::Impossible => 0.0,
        }
    }
    fn prediction_horizon(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.2,
            Difficulty::Medium => 0.5,
            Difficulty::Hard => 1.0,
            Difficulty::Impossible => 1.5,
        }
    }
}

#[derive(Component)]
pub struct ComputerState {
    target_y: f32,
    perceived_ball_y: f32,
    last_update_time: f32,
    last_reaction_time: f32,
    difficulty: Difficulty,
    reaction_timer: Timer,
}

impl ComputerState {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            target_y: 0.0,
            perceived_ball_y: 0.0,
            last_update_time: 0.0,
            last_reaction_time: 0.0,
            difficulty,
            reaction_timer: Timer::from_seconds(difficulty.reaction_time(), TimerMode::Repeating),
        }
    }

    fn calculate_target(&self, actual_y: f32) -> f32 {
        let mut rng = rand::rng();
        let error_margin = self.difficulty.error_margin();
        let offset = rng.random_range(-error_margin..error_margin);
        actual_y + offset
    }
}

pub fn computer_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ComputerState), With<Computer>>,
) {
    for (mut transform, computer_state) in query.iter_mut() {
        let current_y = transform.translation.y;
        let target_y = computer_state.target_y;

        let delta = time.delta_secs();

        let direction = (target_y - current_y).signum();
        let distance = (target_y - current_y).abs();

        let movement = f32::min(distance, PADDLE_SPEED * delta);

        let upper_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.y / 2.0 - PADDLE_PADDING;
        let lower_bound = BTM_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.y / 2.0 + PADDLE_PADDING;

        let new_y = transform.translation.y + movement * direction;
        transform.translation.y = new_y.clamp(lower_bound, upper_bound);
    }
}

pub fn update_computer_targets(
    time: Res<Time>,
    mut query: Query<(&mut ComputerState, &Transform), With<Computer>>,
    ball_query: Single<&Transform, With<Ball>>,
) {
    let ball_y = ball_query.translation.y;

    for (mut computer_state, _transform) in query.iter_mut() {
        computer_state.reaction_timer.tick(time.delta());

        if computer_state.reaction_timer.finished() {
            let perceived = computer_state.calculate_target(ball_y);
            computer_state.perceived_ball_y = perceived;
            computer_state.target_y = perceived;
            computer_state.last_reaction_time = time.elapsed_secs();
        }
    }
}
