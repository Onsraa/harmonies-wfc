use crate::systems::wfc::v1::GenerateWorldEvent;
use crate::systems::wfc::v2::wfc::step_controller;
use bevy::prelude::*;

pub struct WfcPlugin;

impl Plugin for WfcPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GenerateWorldEvent>();
        app.add_systems(Update, step_controller);
    }
}
