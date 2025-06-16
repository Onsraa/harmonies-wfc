use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TileType {
    Empty,   // Vide (seulement en hauteur > 0)
    City,    // Ville
    River,   // Rivière
    Rock,    // Roche
    Trunk,   // Tronc d'arbre
    Leaves,  // Feuilles
    Field,   // Champs
}

impl TileType {
    /// Toutes les tuiles possibles
    pub fn all() -> &'static [TileType] {
        &[
            TileType::City,
            TileType::River,
            TileType::Rock,
            TileType::Trunk,
            TileType::Leaves,
            TileType::Field,
        ]
    }

    /// Tuiles qui peuvent être placées au sol (height = 0)
    pub fn ground_tiles() -> &'static [TileType] {
        &[
            TileType::City,
            TileType::River,
            TileType::Rock,
            TileType::Trunk,
            TileType::Field,
        ]
    }

    /// Peut être placé au sol ?
    pub fn can_be_on_ground(&self) -> bool {
        match self {
            TileType::Empty => false,   // Vide seulement en hauteur
            TileType::Leaves => false,  // Feuilles jamais au sol
            _ => true,
        }
    }

    /// Peut être placé sur une autre tuile spécifique ?
    pub fn can_be_placed_on(&self, below: &TileType) -> bool {
        match self {
            TileType::City => matches!(below, TileType::City | TileType::Rock | TileType::Trunk),
            TileType::River => false, // Rivière seulement au sol
            TileType::Rock => matches!(below, TileType::Rock),
            TileType::Trunk => matches!(below, TileType::Trunk),
            TileType::Leaves => matches!(below, TileType::Trunk),
            TileType::Field => false, // Champs seulement au sol
            TileType::Empty => true,  // Vide peut être mis partout en hauteur
        }
    }

    /// Couleur pour le rendu debug
    pub fn color(&self) -> Color {
        match self {
            TileType::Empty => Color::srgba(0.0, 0.0, 0.0, 0.0),
            TileType::City => Color::srgb(0.7, 0.7, 0.7),   // Gris
            TileType::River => Color::srgb(0.2, 0.5, 0.9),   // Bleu
            TileType::Rock => Color::srgb(0.5, 0.5, 0.5),    // Gris foncé
            TileType::Trunk => Color::srgb(0.4, 0.3, 0.2),   // Marron
            TileType::Leaves => Color::srgb(0.2, 0.7, 0.2),  // Vert
            TileType::Field => Color::srgb(0.8, 0.7, 0.4),   // Beige
        }
    }
}

/// Composant pour une tuile placée dans le monde
#[derive(Component)]
pub struct Tile {
    pub coord: super::hex_coord::HexCoord,
    pub tile_type: TileType,
}

/// Contraintes spéciales pour les rivières
#[derive(Resource, Default)]
pub struct RiverConstraints {
    /// Positions de toutes les rivières existantes
    pub river_positions: HashSet<(i32, i32)>, // (q, r) sans hauteur car rivières au sol
}

impl RiverConstraints {
    /// Vérifie si une rivière peut être placée à cette position
    pub fn can_place_river(&self, q: i32, r: i32) -> bool {
        use super::hex_coord::HexCoord;

        let coord = HexCoord::at_ground(q, r);
        let neighbors = coord.neighbors();

        // Compte les voisins rivière
        let river_neighbors: Vec<_> = neighbors
            .iter()
            .filter(|n| self.river_positions.contains(&(n.q, n.r)))
            .collect();

        match river_neighbors.len() {
            0 => {
                // Peut commencer une nouvelle rivière seulement s'il n'y en a pas déjà
                self.river_positions.is_empty()
            }
            1 => true, // Peut continuer une rivière
            2 => {
                // Vérifie que les deux voisins ne forment pas un "Y"
                let pos1 = (river_neighbors[0].q, river_neighbors[0].r);
                let pos2 = (river_neighbors[1].q, river_neighbors[1].r);

                // Calcul de l'angle entre les deux voisins
                let diff_q = pos2.0 - pos1.0;
                let diff_r = pos2.1 - pos1.1;

                // Si les voisins sont opposés ou forment un angle >= 120°, c'est OK
                diff_q.abs() >= 2 || diff_r.abs() >= 2 || (diff_q + diff_r).abs() >= 2
            }
            _ => false, // Plus de 2 voisins rivière = non autorisé
        }
    }
}