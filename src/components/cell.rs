use bevy::prelude::Component;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq)]
pub enum Tile {
    Grass,
    Water,
    Sand,
}

#[derive(Component)]
pub struct Cell {
    pub x_coord: usize,
    pub y_coord: usize,
    pub possibilities: HashSet<Tile>,
    pub collapsed: bool,
}

impl Cell {
    pub fn new(x_coord: usize, y_coord: usize) -> Self {
        let possibilities: HashSet<Tile> = vec![Tile::Grass, Tile::Water, Tile::Sand]
            .into_iter()
            .collect();

        Self {
            x_coord,
            y_coord,
            possibilities: possibilities,
            collapsed: false,
        }
    }
}
