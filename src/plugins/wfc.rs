use crate::resources::grid::{GridLayout, TileGrid};
use crate::resources::river::RiverConstraints;
use crate::systems::menu::GameState;
use crate::systems::wfc::{wfc_debug_system, wfc_generation_system, GenerateWorldEvent};
use crate::wfc::solver::WfcSolver;
use bevy::prelude::*;

pub struct WfcPlugin;

impl Plugin for WfcPlugin {
    fn build(&self, app: &mut App) {
        // Resources
        app.insert_resource(TileGrid::new())
            .insert_resource(WfcSolver::new(
                GridLayout::default().width,
                GridLayout::default().length,
                GridLayout::default().max_height,
            ))
            .init_resource::<RiverConstraints>();

        // Events
        app.add_event::<GenerateWorldEvent>();

        // Systems

        app.add_systems(Update, (wfc_generation_system, wfc_debug_system).run_if(in_state(GameState::InGame)));

        // Génère automatiquement au démarrage
        app.add_systems(OnEnter(GameState::InGame), |mut events: EventWriter<GenerateWorldEvent>| {
            events.write(GenerateWorldEvent);
        });
    }
}
