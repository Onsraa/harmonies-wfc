use crate::systems::grid::tile::visualisation::*;
use bevy::prelude::*;

pub struct VisualizationPlugin;

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_shared_meshes);
        app.add_systems(
            Update,
            (
                render_tiles_system,
                // regenerate_world_system,
            ), // .run_if(in_state(GameState::InGame)),
        );
    }
}
