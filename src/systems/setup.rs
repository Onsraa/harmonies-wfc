use crate::components::cell::Cell;
use crate::components::grid::Grid;
use bevy::prelude::*;

pub fn initialize(mut commands: Commands, grid: Option<Res<Grid>>) {
    if let Some(grid) = &grid {
        for x in 0..grid.width {
            for y in 0..grid.height {
                commands.spawn(Cell::new(x, y));
            }
        }
    }
}
