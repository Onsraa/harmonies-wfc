use crate::resources::tile_weights::TileWeights;
use crate::systems::menu::GameState;
use crate::systems::setup::initialize;
use crate::systems::setup::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Harmonies".into(),
                    present_mode: PresentMode::AutoNoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ));

        // Add the state
        app.init_state::<GameState>();

        // Only initialize game world when entering the game state
        app.add_systems(OnEnter(GameState::InGame), initialize);
        app.init_resource::<TileWeights>();
    }
}
