use bevy::color::Color;
use bevy::prelude::Component;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TileType {
    Empty,  // Vide (seulement en hauteur > 0)
    City,   // Ville
    River,  // Rivière
    Rock,   // Roche
    Trunk,  // Tronc d'arbre
    Leaves, // Feuilles
    Field,  // Champs
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
            TileType::Empty => false,  // Vide seulement en hauteur
            TileType::Leaves => false, // Feuilles jamais au sol
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
            TileType::City => Color::srgb(0.7, 0.7, 0.7), // Gris
            TileType::River => Color::srgb(0.2, 0.5, 0.9), // Bleu
            TileType::Rock => Color::srgb(0.5, 0.5, 0.5), // Gris foncé
            TileType::Trunk => Color::srgb(0.4, 0.3, 0.2), // Marron
            TileType::Leaves => Color::srgb(0.2, 0.7, 0.2), // Vert
            TileType::Field => Color::srgb(0.8, 0.7, 0.4), // Beige
        }
    }
}
