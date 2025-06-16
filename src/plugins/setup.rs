use bevy::prelude::*;
use crate::systems::setup::initialize;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize);
    }
}