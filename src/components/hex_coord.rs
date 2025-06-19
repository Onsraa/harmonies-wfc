use crate::globals::MAX_HEIGHT;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
    pub height: u8, // 0 = sol, 1 = niveau 1, 2 = niveau 2
}

impl HexCoord {
    pub fn new(q: i32, r: i32, height: u8) -> Self {
        Self { q, r, height }
    }

    pub fn at_ground(q: i32, r: i32) -> Self {
        Self { q, r, height: 0 }
    }

    /// Voisins dans le même niveau de hauteur
    pub fn neighbors(&self) -> [HexCoord; 6] {
        [
            HexCoord::new(self.q + 1, self.r, self.height), // E
            HexCoord::new(self.q + 1, self.r - 1, self.height), // NE
            HexCoord::new(self.q, self.r - 1, self.height), // NW
            HexCoord::new(self.q - 1, self.r, self.height), // W
            HexCoord::new(self.q - 1, self.r + 1, self.height), // SW
            HexCoord::new(self.q, self.r + 1, self.height), // SE
        ]
    }

    /// Cellule au-dessus
    pub fn above(&self) -> Option<HexCoord> {
        if self.height < MAX_HEIGHT {
            Some(HexCoord::new(self.q, self.r, self.height + 1))
        } else {
            None
        }
    }

    /// Cellule en-dessous
    pub fn below(&self) -> Option<HexCoord> {
        if self.height > 0 {
            Some(HexCoord::new(self.q, self.r, self.height - 1))
        } else {
            None
        }
    }

    /// Convertit en position mondiale pour le rendu
    pub fn to_world_pos(&self) -> Vec3 {
        let hex_size = 1.0;
        let x = hex_size * (3.0_f32.sqrt() * self.q as f32 + 3.0_f32.sqrt() / 2.0 * self.r as f32);
        let z = hex_size * (3.0 / 2.0 * self.r as f32);
        let y = self.height as f32 * 0.5; // Hauteur d'une tuile

        Vec3::new(x, y, z)
    }
}
