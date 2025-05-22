use crate::components::grid::Grid;
use crate::systems::cell::*;
use crate::systems::setup::*;
use bevy::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid>();
        app.add_systems(Startup, (initialize, check_cells).chain());
    }
}
