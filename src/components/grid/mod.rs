pub mod hex;

use hex::*;

pub struct Orientation {
    f0: f32,
    f1: f32,
    f2: f32,
    f3: f32,

    b0: f32,
    b1: f32,
    b2: f32,
    b3: f32,

    start_angle: f32,
}

impl Orientation {
    pub const POINTY: Self = Self {
        f0: 1.7320508075688772, // 3f32.sqrt()
        f1: 0.8660254037844386, // 3f32.sqrt() / 2.0
        f2: 0.0,
        f3: 1.5,

        b0: 0.5773502691896257, // 3f32.sqrt() / 3.0
        b1: -1.0 / 3.0,
        b2: 0.0,
        b3: 2.0 / 3.0,

        start_angle: 0.5,
    };

    pub const FLAT: Self = Self {
        f0: 1.5, // 3f32 / 2.0
        f1: 0.0,
        f2: 0.8660254037844386, // 3f32.sqrt() / 2.0
        f3: 1.7320508075688772, // 3f32.sqrt()

        b0: 2.0 / 3.0,
        b1: 0.0,
        b2: -1.0 / 3.0,
        b3: 0.5773502691896257, // 3f32.sqrt() / 3.0

        start_angle: 0.0,
    };
}
