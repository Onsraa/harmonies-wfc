use bevy::prelude::Component;

#[derive(Component)]
pub struct Hex {
    pub coords: [i32; 3],
}

impl Hex {
    // CONSTRUCTORS
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

    // ACCESSORS
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
}
