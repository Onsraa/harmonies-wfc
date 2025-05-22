mod components;
mod globals;
mod plugins;
mod resources;
mod systems;

use crate::plugins::setup::SetupPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SetupPlugin)
        .run();
}
