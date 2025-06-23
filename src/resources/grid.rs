use crate::components::grid::tile::tile_type::TileType;
use crate::components::grid::Orientation;
use crate::globals::{DEFAULT_GRID_SIZE, MAX_HEIGHT};
use crate::wfc::v1::cell::CellCoord;
use bevy::math::Vec2;
use bevy::prelude::Resource;
use std::collections::HashMap;

#[derive(Resource)]
pub struct GridLayout {
    pub orientation: Orientation,
    pub width: i32,
    pub length: i32,
    pub max_height: u8,
    pub hex_size: f32,
    pub origin: Vec2,
}

impl Default for GridLayout {
    fn default() -> Self {
        GridLayout {
            orientation: Orientation::POINTY,
            width: DEFAULT_GRID_SIZE,
            length: DEFAULT_GRID_SIZE,
            max_height: MAX_HEIGHT,
            hex_size: 2.0,
            origin: Vec2::ZERO,
        }
    }
}

#[derive(Resource, Default)]
pub struct TileGrid {
    pub tiles: HashMap<CellCoord, TileType>,
}

impl TileGrid {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }
    pub fn set_tile(&mut self, coord: CellCoord, tile_type: TileType) {
        self.tiles.insert(coord, tile_type);
    }
    pub fn get_tile(&self, coord: &CellCoord) -> Option<&TileType> {
        self.tiles.get(coord)
    }
    pub fn count_tiles_of_type(&self, tile_type: TileType) -> usize {
        self.tiles.values().filter(|&&t| t == tile_type).count()
    }
    pub fn clear(&mut self) {
        self.tiles.clear();
    }
}
