use crate::components::grid::tile::tile_type::TileType;
use bevy::prelude::Component;

pub mod tile_type;

#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub top_level: bool,
}
