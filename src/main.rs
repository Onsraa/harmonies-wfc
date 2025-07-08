mod components;
mod events;
mod globals;
mod plugins;
mod resources;
mod systems;
mod wfc;

use crate::plugins::boids::BoidsPlugin;
use crate::plugins::camera::CameraPlugin;
use crate::plugins::clouds::CloudsPlugin;
use crate::plugins::grid::GridPlugin;
use crate::plugins::menu::MenuPlugin;
use crate::plugins::setup::SetupPlugin;
use crate::plugins::spatial::SpatialPlugin;
use crate::plugins::ui::UiPlugin;
use crate::plugins::visualisation::VisualizationPlugin;
use crate::plugins::wfc::WfcPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(WfcPlugin)
        .add_plugins(VisualizationPlugin)
        .add_plugins(SpatialPlugin)
        .add_plugins(BoidsPlugin)
        .add_plugins(CloudsPlugin)
        .add_plugins(UiPlugin)
        .run();
}
