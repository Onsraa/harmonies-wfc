use crate::wfc::v1::cell::CellCoord;
use bevy::prelude::Resource;
use std::collections::HashSet;

/// Contraintes spéciales pour les rivières
#[derive(Resource, Default)]
pub struct RiverConstraints {
    /// Positions de toutes les rivières existantes
    pub river_positions: HashSet<(i32, i32)>, // (q, r) sans hauteur car rivières au sol
}

impl RiverConstraints {
    /// Vérifie si une rivière peut être placée à cette position
    pub fn can_place_river(&self, q: i32, r: i32) -> bool {
        let coord = CellCoord::at_ground(q, r);
        let neighbors = coord.neighbors();

        // Compte les voisins rivière
        let river_neighbors: Vec<_> = neighbors
            .iter()
            .filter(|n| self.river_positions.contains(&(n.0, n.1)))
            .collect();

        match river_neighbors.len() {
            0 => {
                // Peut commencer une nouvelle rivière seulement s'il n'y en a pas déjà
                self.river_positions.is_empty()
            }
            1 => true, // Peut continuer une rivière
            2 => {
                // Vérifie que les deux voisins ne forment pas un "Y"
                let pos1 = (river_neighbors[0].0, river_neighbors[0].1);
                let pos2 = (river_neighbors[1].0, river_neighbors[1].1);

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
