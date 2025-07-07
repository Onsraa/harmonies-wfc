use crate::systems::wfc::v2::wfc::{check_wfc_progress, setup_wfc_task_when_ready};
use bevy::prelude::*;

pub struct WfcPlugin;

impl Plugin for WfcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (setup_wfc_task_when_ready, check_wfc_progress).chain(),
        );
    }
}
