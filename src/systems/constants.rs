use bevy::prelude::*;

pub const TOP_WALL: f32 = 300.;
pub const BTM_WALL: f32 = -300.;
pub const LEFT_WALL: f32 = -600.;
pub const RIGHT_WALL: f32 = 600.;
pub const WALL_THICKNESS: f32 = 10.;

pub const PADDLE_SIZE: Vec2 = Vec2::new(20.0, 120.0);
pub const BALL_DIAMETER: f32 = 5.0;
pub const PLAYER_COLOR: Color = Color::hsl(200.0, 1.0, 1.0);
pub const COMPUTER_COLOR: Color = Color::hsl(100.0, 1.0, 1.0);
pub const BALL_COLOR: Color = Color::hsl(50.0, 1.0, 1.0);
pub const WALL_COLOR: Color = Color::srgb(1.0, 0.0, 0.5);

pub const PADDLE_PADDING: f32 = 30.0;
pub const PADDLE_SPEED: f32 = 200.0;
pub const BALL_SPEED: f32 = 250.0;
pub const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

pub const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
