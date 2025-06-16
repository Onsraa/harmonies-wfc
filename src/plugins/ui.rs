use crate::resources::tile_weights::TileWeights;
use crate::systems::ui::{controls_help_ui_system, tile_weights_ui_system};
use bevy::prelude::*;
use bevy_egui::{EguiContextPass, EguiPlugin};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        });
        app.add_systems(EguiContextPass, (tile_weights_ui_system, controls_help_ui_system));
    }
}
