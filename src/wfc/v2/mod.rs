use crate::components::grid::hex::Hex;
use crate::wfc::v2::cell::TileId;
pub mod cell;
pub mod controller;
pub mod grid;
pub mod solver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WfcError {
    Complete,
    NoValidTiles,
    PropagationFailed,
}

pub const TILE_COUNT: u8 = 8;

pub const RIVER: TileId = 0;
pub const FIELD: TileId = 1;
pub const TRUNK: TileId = 2;
pub const LEAVES: TileId = 3;
pub const CITY: TileId = 4;
pub const CITY_TOP: TileId = 5;
pub const ROCK: TileId = 6;
pub const ROCK_TOP: TileId = 7;
pub const EMPTY: TileId = 8; // Special value, outside normal range

pub type Direction = u8;

pub const EAST: Direction = 0;
pub const WEST: Direction = 1;
pub const NORTHEAST: Direction = 2;
pub const NORTHWEST: Direction = 3;
pub const SOUTHWEST: Direction = 4;
pub const SOUTHEAST: Direction = 5;
pub const UP: Direction = 6;
pub const DOWN: Direction = 7;

const HEX_3D_DIRECTIONS: [(Hex, Direction); 8] = [
    // Horizontal neighbors (level unchanged)
    (
        Hex {
            coords: [1, 0, -1],
            level: 0,
        },
        EAST,
    ), // East
    (
        Hex {
            coords: [1, -1, 0],
            level: 0,
        },
        NORTHEAST,
    ), // Northeast
    (
        Hex {
            coords: [0, -1, 1],
            level: 0,
        },
        NORTHWEST,
    ), // Northwest
    (
        Hex {
            coords: [-1, 0, 1],
            level: 0,
        },
        WEST,
    ), // West
    (
        Hex {
            coords: [-1, 1, 0],
            level: 0,
        },
        SOUTHWEST,
    ), // Southwest
    (
        Hex {
            coords: [0, 1, -1],
            level: 0,
        },
        SOUTHEAST,
    ), // Southeast
    // Vertical neighbors
    (
        Hex {
            coords: [0, 0, 0],
            level: 1,
        },
        UP,
    ), // Up
    (
        Hex {
            coords: [0, 0, 0],
            level: -1,
        },
        DOWN,
    ), // Down
];

pub fn get_opposite_direction(direction: Direction) -> Direction {
    match direction {
        EAST => WEST,
        WEST => EAST,
        NORTHEAST => SOUTHWEST,
        NORTHWEST => SOUTHEAST,
        SOUTHWEST => NORTHEAST,
        SOUTHEAST => NORTHWEST,
        UP => DOWN,
        DOWN => UP,
        _ => panic!("Invalid direction"),
    }
}
