use crate::components::grid::hex::Hex;
use crate::systems::wfc::v1::GenerateWorldEvent;
use crate::wfc::v2::controller::WfcController;
use crate::wfc::v2::WfcError;
use bevy::prelude::{Commands, EventWriter, Local, Query, ResMut};

pub fn setup_controller(mut commands: Commands, hexes: Query<&Hex>) {
    let hex_vec = hexes.iter().copied().collect::<Vec<_>>();
    let controller = WfcController::from_hexes(hex_vec);
    commands.insert_resource(controller);
}

pub fn step_controller(
    controller: Option<ResMut<WfcController>>,
    mut regenerate_world_event: EventWriter<GenerateWorldEvent>,
    mut completed: Local<bool>,
) {
    let Some(mut controller) = controller else {
        return;
    };
    if *completed {
        return;
    }
    if let Err(e) = controller.step() {
        if e != WfcError::Complete {
            eprintln!("WFC Error: {:?}", e);
        } else {
            *completed = true;
            regenerate_world_event.write(GenerateWorldEvent);
        }
    }
}
