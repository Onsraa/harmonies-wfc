use crate::components::grid::Hex;
use std::ops;

// +
impl ops::Add<Hex> for Hex {
    type Output = Hex;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Hex {
            coords: [
                self.coords[0] + rhs.coords[0],
                self.coords[1] + rhs.coords[1],
                self.coords[2] + rhs.coords[2],
            ],
            level: self.level + rhs.level,
        }
    }
}

impl ops::AddAssign<Hex> for Hex {
    #[inline]
    fn add_assign(&mut self, rhs: Hex) {
        self.coords[0] += rhs.coords[0];
        self.coords[1] += rhs.coords[1];
        self.coords[2] += rhs.coords[2];
        self.level += rhs.level;
    }
}

// -
impl ops::Sub<Hex> for Hex {
    type Output = Hex;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Hex {
            coords: [
                self.coords[0] - rhs.coords[0],
                self.coords[1] - rhs.coords[1],
                self.coords[2] - rhs.coords[2],
            ],
            level: self.level - rhs.level,
        }
    }
}

impl ops::SubAssign<Hex> for Hex {
    #[inline]
    fn sub_assign(&mut self, rhs: Hex) {
        self.coords[0] -= rhs.coords[0];
        self.coords[1] -= rhs.coords[1];
        self.coords[2] -= rhs.coords[2];
        self.level -= rhs.level;
    }
}

// *
impl ops::Mul<Hex> for Hex {
    type Output = Hex;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Hex {
            coords: [
                self.coords[0] * rhs.coords[0],
                self.coords[1] * rhs.coords[1],
                self.coords[2] * rhs.coords[2],
            ],
            level: self.level * rhs.level,
        }
    }
}

impl ops::MulAssign<Hex> for Hex {
    #[inline]
    fn mul_assign(&mut self, rhs: Hex) {
        self.coords[0] *= rhs.coords[0];
        self.coords[1] *= rhs.coords[1];
        self.coords[2] *= rhs.coords[2];
        self.level *= rhs.level;
    }
}
