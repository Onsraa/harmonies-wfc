use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::tile::TileType;

/// Resource pour stocker les poids de chaque type de tuile
#[derive(Resource)]
pub struct TileWeights {
    /// Poids par type de tuile et par niveau
    weights: HashMap<(TileType, u8), f32>,
    /// Afficher ou non l'interface
    pub show_ui: bool,
}

impl Default for TileWeights {
    fn default() -> Self {
        let mut weights = HashMap::new();

        // Poids par défaut pour le niveau 0 (sol)
        weights.insert((TileType::City, 0), 15.0);
        weights.insert((TileType::River, 0), 10.0);  // Augmenté pour plus de rivières
        weights.insert((TileType::Rock, 0), 20.0);
        weights.insert((TileType::Trunk, 0), 20.0);
        weights.insert((TileType::Field, 0), 35.0);

        // Poids pour le niveau 1
        weights.insert((TileType::City, 1), 30.0);
        weights.insert((TileType::Rock, 1), 30.0);
        weights.insert((TileType::Trunk, 1), 20.0);
        weights.insert((TileType::Leaves, 1), 10.0);
        weights.insert((TileType::Empty, 1), 10.0);

        // Poids pour le niveau 2
        weights.insert((TileType::City, 2), 20.0);
        weights.insert((TileType::Rock, 2), 20.0);
        weights.insert((TileType::Leaves, 2), 30.0);
        weights.insert((TileType::Empty, 2), 30.0);

        Self {
            weights,
            show_ui: true,
        }
    }
}

impl TileWeights {
    /// Obtient le poids d'un type de tuile à un niveau donné
    pub fn get_weight(&self, tile_type: TileType, height: u8) -> f32 {
        *self.weights.get(&(tile_type, height)).unwrap_or(&1.0)
    }

    /// Définit le poids d'un type de tuile à un niveau donné
    pub fn set_weight(&mut self, tile_type: TileType, height: u8, weight: f32) {
        self.weights.insert((tile_type, height), weight.max(0.1)); // Minimum 0.1
    }

    /// Obtient une référence mutable au poids
    pub fn get_weight_mut(&mut self, tile_type: TileType, height: u8) -> &mut f32 {
        self.weights.entry((tile_type, height)).or_insert(1.0)
    }

    /// Normalise les poids pour un niveau donné (somme = 100)
    pub fn normalize_weights(&mut self, height: u8) {
        let tiles = match height {
            0 => vec![TileType::City, TileType::River, TileType::Rock, TileType::Trunk, TileType::Field],
            1 | 2 => vec![TileType::City, TileType::Rock, TileType::Trunk, TileType::Leaves, TileType::Empty],
            _ => return,
        };

        let sum: f32 = tiles.iter()
            .map(|t| self.get_weight(*t, height))
            .sum();

        if sum > 0.0 {
            for tile in tiles {
                let current = self.get_weight(tile, height);
                self.set_weight(tile, height, current * 100.0 / sum);
            }
        }
    }

    /// Choisit une tuile selon les poids pour un ensemble de possibilités
    pub fn weighted_choice(&self, possibilities: &[TileType], height: u8) -> Option<TileType> {
        if possibilities.is_empty() {
            return None;
        }

        // Calcule les poids cumulés
        let mut cumulative_weights = Vec::new();
        let mut total_weight = 0.0;

        for tile_type in possibilities {
            total_weight += self.get_weight(*tile_type, height);
            cumulative_weights.push((total_weight, *tile_type));
        }

        if total_weight <= 0.0 {
            // Si tous les poids sont zéro, choix uniforme
            return possibilities.get(rand::random::<i64>() as usize % possibilities.len()).copied();
        }

        // Choix aléatoire pondéré
        let random_value = rand::random::<f32>() * total_weight;

        for (cumulative, tile_type) in cumulative_weights {
            if random_value <= cumulative {
                return Some(tile_type);
            }
        }

        // Par sécurité
        possibilities.last().copied()
    }

    /// Réinitialise les poids par défaut
    pub fn reset_to_default(&mut self) {
        *self = Self::default();
    }
}