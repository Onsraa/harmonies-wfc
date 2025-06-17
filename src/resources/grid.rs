use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::{hex_coord::HexCoord, tile::TileType};

/// Grille hexagonale 3D du monde
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

    /// Ajoute une tuile à la grille
    pub fn set_tile(&mut self, coord: HexCoord, tile_type: TileType) {
        self.tiles.insert(coord, tile_type);
    }

    /// Obtient une tuile de la grille
    pub fn get_tile(&self, coord: &HexCoord) -> Option<&TileType> {
        self.tiles.get(coord)
    }

    /// Compte le nombre de tuiles d'un certain type
    pub fn count_tiles_of_type(&self, tile_type: TileType) -> usize {
        self.tiles.values()
            .filter(|&&t| t == tile_type)
            .count()
    }

    /// Vide la grille
    pub fn clear(&mut self) {
        self.tiles.clear();
    }
}