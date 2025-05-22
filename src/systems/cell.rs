use crate::components::cell::Cell;
use bevy::prelude::*;

pub fn check_cells(query: Query<&Cell>) {
    for cell in query {
        println!("[{}, {}]", cell.x_coord, cell.y_coord);
    }
}
