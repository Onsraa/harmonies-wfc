use bevy::prelude::*;
use crate::components::tile::RiverConstraints;
use crate::resources::grid::HexGrid;
use crate::systems::wfc::{
    wfc_generation_system, wfc_debug_system, GenerateWorldEvent
};
use crate::wfc::solver::WfcSolver;

pub struct WfcPlugin;

impl Plugin for WfcPlugin {
    fn build(&self, app: &mut App) {
        // Resources
        app.insert_resource(HexGrid::new(20, 20))
            .insert_resource(WfcSolver::new(20, 20))
            .init_resource::<RiverConstraints>();

        // Events
        app.add_event::<GenerateWorldEvent>();

        // Systems
        app.add_systems(Update, (
            wfc_generation_system,
            wfc_debug_system,
        ));

        // Génère automatiquement au démarrage
        app.add_systems(Startup, |mut events: EventWriter<GenerateWorldEvent>| {
            events.send(GenerateWorldEvent);
        });
    }
}