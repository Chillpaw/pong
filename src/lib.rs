pub mod systems;

use systems::*;

use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

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
