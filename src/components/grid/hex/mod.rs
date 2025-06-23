pub mod display;
pub mod ops;

use crate::components::grid::Orientation;
use bevy::prelude::Component;

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Hex {
    pub coords: [i32; 3],
    pub level: i32,
}

impl Hex {
    // --- CONSTRUCTORS ---
    #[inline]
    pub fn new(q: i32, r: i32, s: i32, level: i32) -> Self {
        debug_assert_eq!(
            q + r + s,
            0,
            "Invalid hex coordinates: q + r + s must equal 0"
        );
        Self {
            coords: [q, r, s],
            level,
        }
    }

    #[inline]
    pub fn from_axial(q: i32, r: i32, level: i32) -> Self {
        Self {
            coords: [q, r, -q - r],
            level,
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

    #[inline]
    pub fn level(&self) -> i32 {
        self.level
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
