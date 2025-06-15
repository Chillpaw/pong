use std::thread::current;

use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

const PADDLE_SIZE: Vec2 = Vec2::new(20.0, 120.0);
const BALL_DIAMETER: f32 = 5.0;
const PLAYER_COLOR: Color = Color::hsl(200.0, 1.0, 1.0);
const COMPUTER_COLOR: Color = Color::hsl(100.0, 1.0, 1.0);
const BALL_COLOR: Color = Color::hsl(50.0, 1.0, 1.0);
const WALL_COLOR: Color = Color::srgb(1.0, 0.0, 0.5);

const PADDLE_PADDING: f32 = 30.0;
const PADDLE_SPEED: f32 = 200.0;
const BALL_SPEED: f32 = 250.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

const TOP_WALL: f32 = 300.;
const BTM_WALL: f32 = -300.;
const LEFT_WALL: f32 = -600.;
const RIGHT_WALL: f32 = 600.;
const WALL_THICKNESS: f32 = 10.;
const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Computer;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Resource, Deref)]
struct CollisionSound(Handle<AudioSource>);

#[derive(Component, Default)]
struct Collider;

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Component)]
#[require(Sprite, Transform, Collider)]
struct Wall;

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BTM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BTM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl Wall {
    fn new(location: WallLocation) -> (Wall, Sprite, Transform) {
        (
            Wall,
            Sprite::from_color(WALL_COLOR, Vec2::ONE),
            Transform {
                translation: location.position().extend(0.0),
                scale: location.size().extend(1.0),
                ..default()
            },
        )
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_event::<CollisionEvent>()
            .add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
                (
                    apply_velocity,
                    player_movement,
                    computer_movement,
                    check_for_collisions,
                )
                    .chain(),
            );
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);
    //TODO: make collision sound
    //let ball_collision_sound = asset_server.load("sounds/pong_collision.ogg");
    //let bgm = asset_server.load("sounds/pong_bgm");

    let paddle = meshes.add(Rectangle::new(1.0, 1.0));
    let ball = meshes.add(Circle::new(BALL_DIAMETER));
    let paddle_x = LEFT_WALL + WALL_THICKNESS + PADDLE_PADDING;

    //spawn player paddle
    commands.spawn((
        Sprite::from_color(PLAYER_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(paddle_x, 0.0, 0.0),
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
        Paddle,
        Collider,
        Player,
    ));
    //spawn computer paddle
    commands.spawn((
        Sprite::from_color(COMPUTER_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(-paddle_x, 0.0, 0.0),
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
        Paddle,
        Collider,
        Computer,
    ));
    //spawn ball
    commands.spawn((
        Mesh2d(ball),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    ));
    //spawn walls
    commands.spawn(Wall::new(WallLocation::Bottom));
    commands.spawn(Wall::new(WallLocation::Top));
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Right));

    println!("Setup steps complete.")
}

fn player_movement(
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

#[derive(Resource)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
    Impossible,
}

fn computer_movement(
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

fn move_paddle(paddle_transform: &mut Transform, direction: f32, delta_time: f32) {
    let new_y = paddle_transform.translation.y + direction * PADDLE_SPEED * delta_time;
    let upper_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.y / 2.0 - PADDLE_PADDING;
    let lower_bound = BTM_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.y / 2.0 + PADDLE_PADDING;

    paddle_transform.translation.y = new_y.clamp(lower_bound, upper_bound);
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut score: ResMut<Score>,
    ball_query: Single<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.into_inner();

    for (collider_entity, collider_transform) in &collider_query {
        let collision = ball_collision(
            BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER / 2.),
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            collision_events.write_default();

            println!("Collision detected.");
            println!(
                "Ball x velocity: {}, ball y velocity: {}",
                ball_velocity.x, ball_velocity.y
            );

            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
            }

            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
                println!("Ball x velocity reversed.");
            }

            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
                println!("Ball y velocity reversed.");
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
