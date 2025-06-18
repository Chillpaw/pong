pub mod systems;

use systems::*;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            //.insert_resource(Difficulty::Easy) No longer implementing difficulty as a resource but instead attaching it to the ComputerState on each paddle
            .add_event::<CollisionEvent>()
            .add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
                (
                    apply_velocity,
                    player_movement,
                    update_computer_targets,
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
