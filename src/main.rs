#![feature(new_range_api)]
extern crate core;

mod components;
mod globals;
mod plugins;
mod resources;
mod systems;

use crate::plugins::camera::CameraPlugin;
use crate::plugins::grid::GridPlugin;
use crate::plugins::setup::SetupPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_plugins(SetupPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(GridPlugin)
        .run();
}
