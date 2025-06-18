use crate::ai::ComputerState;
use crate::constants::*;
use crate::physics::*;
use crate::systems::Difficulty;
use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub usize);

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

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    //asset_server: Res<AssetServer>,
    //difficulty: Res<Difficulty>,
) {
    commands.spawn(Camera2d);
    //TODO: make collision sound
    //let ball_collision_sound = asset_server.load("sounds/pong_collision.ogg");
    //let bgm = asset_server.load("sounds/pong_bgm");

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
        Computer,
        ComputerState::new(Difficulty::Medium),
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
        ComputerState::new(Difficulty::Medium),
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
