use crate::components::cell::Tile;
use crate::globals::{GRID_HEIGHT, GRID_WIDTH};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub rule: Vec<(Tile, Tile)>,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
            rule: vec![
                (Tile::Grass, Tile::Sand),
                (Tile::Grass, Tile::Water),
                (Tile::Water, Tile::Water),
            ],
        }
    }
}
