mod components;
mod globals;
mod plugins;
mod resources;
mod systems;
mod wfc;

use crate::plugins::camera::CameraPlugin;
use crate::plugins::setup::SetupPlugin;
use crate::plugins::wfc::WfcPlugin;
use bevy::prelude::*;
use crate::plugins::visualisation::VisualizationPlugin;

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WfcPlugin)
        .add_plugins(VisualizationPlugin)
        //.add_plugins(UiPlugin)
        .run();
}
