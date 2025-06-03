mod components;
mod globals;
mod plugins;
mod resources;
mod systems;

use bevy::prelude::*;
use crate::plugins::setup::SetupPlugin;
use crate::plugins::camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SetupPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
