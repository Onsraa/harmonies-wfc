mod components;
mod globals;
mod plugins;
mod resources;
mod systems;
mod wfc;

use crate::plugins::camera::CameraPlugin;
use crate::plugins::grid::GridPlugin;
use crate::plugins::menu::MenuPlugin;
use crate::plugins::setup::SetupPlugin;
use crate::plugins::ui::UiPlugin;
use crate::plugins::visualisation::VisualizationPlugin;
use crate::plugins::wfc::WfcPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MenuPlugin)
        .add_plugins(MeshPickingPlugin)
        .add_plugins(SetupPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(WfcPlugin)
        .add_plugins(VisualizationPlugin)
        .add_plugins(UiPlugin)
        .run();
}
