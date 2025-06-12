pub mod display;
pub mod ops;

use crate::components::grid::Orientation;
use bevy::prelude::Component;

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Hex {
    pub coords: [i32; 3],
}

const HEX_DIRECTIONS: [Hex; 6] = [
    /// Hexagonal directions, in trigonometric order (starting from East)
    Hex { coords: [1, 0, -1] }, // 0 : E
    Hex { coords: [1, -1, 0] }, // 1 : NE
    Hex { coords: [0, -1, 1] }, // 2 : NW
    Hex { coords: [-1, 0, 1] }, // 3 : W
    Hex { coords: [-1, 1, 0] }, // 4 : SW
    Hex { coords: [0, 1, -1] }, // 5 : SE
];

impl Hex {
    // --- CONSTRUCTORS ---
    #[inline]
    pub fn new(q: i32, r: i32, s: i32) -> Self {
        debug_assert_eq!(
            q + r + s,
            0,
            "Invalid hex coordinates: q + r + s must equal 0"
        );
        Self { coords: [q, r, s] }
    }

    #[inline]
    pub fn from_axial(q: i32, r: i32) -> Self {
        Self {
            coords: [q, r, -q - r],
        }
    }

    // --- ACCESSORS ---
    #[inline]
    pub fn q(&self) -> i32 {
        self.coords[0]
    }

    #[inline]
    pub fn r(&self) -> i32 {
        self.coords[1]
    }

    #[inline]
    pub fn s(&self) -> i32 {
        self.coords[2]
    }

    // --- METHODS ---
    #[inline]
    pub fn length(&self) -> i32 {
        (self.coords[0].abs() + self.coords[1].abs() + self.coords[2].abs()) / 2
    }

    #[inline]
    pub fn distance_from(&self, other: Hex) -> i32 {
        (*self - other).length()
    }
}
