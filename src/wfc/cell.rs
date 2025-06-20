use crate::components::grid::tile::tile_type::TileType;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct CellCoord(pub i32, pub i32, pub u8);

impl CellCoord {
    #[inline]
    pub fn at_ground(q: i32, r: i32) -> Self {
        Self { 0: q, 1: r, 2: 0 }
    }

    #[inline]
    pub fn neighbors(&self) -> [CellCoord; 6] {
        [
            CellCoord(self.0 + 1, self.1, self.2),     // E
            CellCoord(self.0 + 1, self.1 - 1, self.2), // NE
            CellCoord(self.0, self.1 - 1, self.2),     // NW
            CellCoord(self.0 - 1, self.1, self.2),     // W
            CellCoord(self.0 - 1, self.1 + 1, self.2), // SW
            CellCoord(self.0, self.1 + 1, self.2),     // SE
        ]
    }

    #[inline]
    pub fn above(&self) -> CellCoord {
        CellCoord(self.0, self.1, self.2 + 1)
    }

    #[inline]
    pub fn below(&self) -> CellCoord {
        CellCoord(self.0, self.1, self.2 - 1)
    }
}

/// Cellule du WFC
#[derive(Clone, Debug)]
pub struct WfcCell {
    pub possibilities: HashSet<TileType>,
    pub collapsed: bool,
}

impl WfcCell {
    pub fn new(height: u8) -> Self {
        let mut possibilities = HashSet::new();

        // Au sol, toutes les tuiles sauf Empty et Leaves
        if height == 0 {
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
