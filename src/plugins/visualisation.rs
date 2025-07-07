use crate::systems::grid::tile::visualisation::*;
use bevy::prelude::*;

pub struct VisualizationPlugin;

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResetGridVisuals>();
        app.add_event::<UpdateGridVisuals>();
        app.add_systems(Startup, setup_shared_meshes);
        app.add_systems(
            Update,
            (render_new_tiles,), // .run_if(in_state(GameState::InGame)),
        );
    }
}
