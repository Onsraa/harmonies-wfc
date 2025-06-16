use crate::components::{hex_coord::HexCoord, tile::TileType};
use std::collections::HashSet;

/// Cellule du WFC
#[derive(Clone, Debug)]
pub struct WfcCell {
    pub coord: HexCoord,
    pub possibilities: HashSet<TileType>,
    pub collapsed: bool,
}

impl WfcCell {
    pub fn new(coord: HexCoord) -> Self {
        let mut possibilities = HashSet::new();

        // Au sol, toutes les tuiles sauf Empty et Leaves
        if coord.height == 0 {
            for tile in TileType::ground_tiles() {
                possibilities.insert(*tile);
            }
        } else {
            // En hauteur, on commence avec toutes les possibilités
            // Elles seront filtrées selon ce qu'il y a en dessous
            for tile in TileType::all() {
                possibilities.insert(*tile);
            }
            possibilities.insert(TileType::Empty); // Toujours possible en hauteur
        }

        Self {
            coord,
            possibilities,
            collapsed: false,
        }
    }

    /// Entropie = nombre de possibilités (avec un peu de bruit pour éviter les égalités)
    pub fn entropy(&self) -> f32 {
        self.possibilities.len() as f32 + rand::random::<f32>() * 0.1
    }

    /// Obtient le type de tuile si la cellule est effondrée
    pub fn get_tile_type(&self) -> Option<TileType> {
        if self.collapsed && self.possibilities.len() == 1 {
            self.possibilities.iter().next().copied()
        } else {
            None
        }
    }
}