use crate::resources::grid::GridLayout;
use crate::systems::grid::setup::setup_grid;
use bevy::prelude::{App, Plugin, Startup};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridLayout>()
            .add_systems(Startup, setup_grid);
    }
}
