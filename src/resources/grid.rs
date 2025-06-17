use crate::components::grid::Orientation;
use bevy::math::Vec2;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GridLayout {
    pub orientation: Orientation,
    pub grid_size: Vec2,
    pub hex_size: f32,
    pub origin: Vec2,
}

#[derive(Resource, Default)]
pub struct HexGrid {
    pub tiles: HashMap<HexCoord, TileType>,
    pub width: i32,
    pub height: i32,
}

impl HexGrid {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            tiles: HashMap::new(),
            width,
            height,
        }
    }
    pub fn set_tile(&mut self, coord: HexCoord, tile_type: TileType) {
        self.tiles.insert(coord, tile_type);
    }
    pub fn get_tile(&self, coord: &HexCoord) -> Option<&TileType> {
        self.tiles.get(coord)
    }
    pub fn count_tiles_of_type(&self, tile_type: TileType) -> usize {
        self.tiles.values()
            .filter(|&&t| t == tile_type)
            .count()
    }
    pub fn clear(&mut self) {
        self.tiles.clear();
    }
}

impl Default for GridLayout {
    fn default() -> Self {
        GridLayout {
            orientation: Orientation::POINTY,
            grid_size: Vec2::new(100.0, 100.0),
            hex_size: 2.0,
            origin: Vec2::ZERO,
        }
    }
}